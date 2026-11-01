/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0.
 * If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `NSURLComponents` (iOS 7+).
//!
//! Reference: Apple's `NSURLComponents` class documentation (iOS 7.0+) and
//! RFC 3986 section 3 for the component grammar. Apple's behaviour that this
//! implementation follows:
//!
//! - `-initWithString:` / `+componentsWithString:` return `nil` when the
//!   string is not a valid RFC 3986 URI reference.
//! - The plain properties (`user`, `password`, `host`, `path`, `query`,
//!   `fragment`) are percent-*decoded*; the `percentEncoded*` properties are
//!   the raw form. Setters of the plain properties encode, setters of the
//!   `percentEncoded*` properties validate.
//! - `-string` and `-URL` return `nil` if the components cannot form a valid
//!   URL string (for example a host together with a relative path).
//!
//! The components are stored in their percent-encoded form, because that is
//! lossless: decoding is only done when a plain getter is called.
//!
//! Deliberately not implemented yet: `queryItems` (needs `NSURLQueryItem`),
//! `-URLRelativeToURL:`, `rangeOfScheme` and friends, and IDN host handling.

use super::ns_string::{from_rust_string, to_rust_string};
use crate::objc::{
    autorelease, id, msg, msg_class, nil, objc_classes, release, ClassExports, HostObject,
    NSZonePtr,
};
use crate::Environment;

/// Which component a string belongs to; decides which characters may appear
/// unescaped (RFC 3986 sections 3.2 and 3.3 to 3.5, matching the
/// `NSCharacterSet` `URL*AllowedCharacterSet` definitions).
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Part {
    User,
    Password,
    Host,
    Path,
    Query,
    Fragment,
}

/// All components, percent-encoded (the "raw" form).
#[derive(Default, Clone, PartialEq, Eq, Debug)]
struct Components {
    scheme: Option<String>,
    user: Option<String>,
    password: Option<String>,
    host: Option<String>,
    port: Option<u16>,
    /// Never absent: Foundation reports an empty path as `@""`.
    path: String,
    query: Option<String>,
    fragment: Option<String>,
}

#[derive(Default)]
struct NSURLComponentsHostObject {
    components: Components,
}
impl HostObject for NSURLComponentsHostObject {}

// MARK: - Percent-encoding

fn is_unreserved(b: u8) -> bool {
    b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.' | b'_' | b'~')
}

fn is_sub_delim(b: u8) -> bool {
    matches!(
        b,
        b'!' | b'$' | b'&' | b'\'' | b'(' | b')' | b'*' | b'+' | b',' | b';' | b'='
    )
}

/// Is this byte allowed unescaped within `part`?
fn is_allowed(part: Part, b: u8) -> bool {
    if is_unreserved(b) || is_sub_delim(b) {
        // `User` and `Password` follow `URLUserAllowedCharacterSet` and
        // `URLPasswordAllowedCharacterSet`, which exclude the sub-delimiters
        // that terminate userinfo.
        return match part {
            Part::User => b != b';',
            Part::Password => b != b';',
            _ => true,
        };
    }
    match part {
        Part::User => false,
        Part::Password => false,
        Part::Host => false,
        Part::Path => matches!(b, b':' | b'@' | b'/'),
        Part::Query | Part::Fragment => matches!(b, b':' | b'@' | b'/' | b'?'),
    }
}

fn hex_value(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

/// Percent-encodes everything that is not allowed in `part`. A literal `%` is
/// encoded too (as Foundation does for the plain setters), so this is not
/// idempotent.
fn percent_encode(s: &str, part: Part) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut out = String::with_capacity(s.len());
    for &b in s.as_bytes() {
        if is_allowed(part, b) {
            out.push(b as char);
        } else {
            out.push('%');
            out.push(HEX[(b >> 4) as usize] as char);
            out.push(HEX[(b & 0xf) as usize] as char);
        }
    }
    out
}

/// Decodes `%XX` escapes. `None` if an escape is malformed or the result is
/// not valid UTF-8 (Foundation returns `nil` for the plain properties then).
fn percent_decode(s: &str) -> Option<String> {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            let hi = hex_value(*bytes.get(i + 1)?)?;
            let lo = hex_value(*bytes.get(i + 2)?)?;
            out.push(hi << 4 | lo);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8(out).ok()
}

/// Is `s` a well-formed already-encoded string for `part`? Every byte must be
/// either allowed unescaped or part of a valid `%XX` escape.
fn is_valid_encoded(s: &str, part: Part) -> bool {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'%' {
            let ok = bytes.get(i + 1).copied().and_then(hex_value).is_some()
                && bytes.get(i + 2).copied().and_then(hex_value).is_some();
            if !ok {
                return false;
            }
            i += 3;
        } else if is_allowed(part, b) || (part == Part::Host && is_ip_literal_byte(b)) {
            i += 1;
        } else {
            return false;
        }
    }
    true
}

/// Bytes only valid in a bracketed IPv6/IPvFuture host literal.
fn is_ip_literal_byte(b: u8) -> bool {
    matches!(b, b'[' | b']' | b':')
}

fn is_valid_scheme(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'))
}

// MARK: - Parsing and composing

impl Components {
    /// Parses an RFC 3986 URI reference. Returns `None` for strings Foundation
    /// would reject (illegal characters, malformed escapes, invalid port).
    fn parse(s: &str) -> Option<Components> {
        // Every byte must be a URI character or part of a `%XX` escape.
        let bytes = s.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            let b = bytes[i];
            if b == b'%' {
                hex_value(*bytes.get(i + 1)?)?;
                hex_value(*bytes.get(i + 2)?)?;
                i += 3;
                continue;
            }
            let ok = is_unreserved(b)
                || is_sub_delim(b)
                || matches!(b, b':' | b'/' | b'?' | b'#' | b'[' | b']' | b'@');
            if !ok {
                return None;
            }
            i += 1;
        }

        let mut c = Components::default();
        let mut rest = s;

        if let Some(hash) = rest.find('#') {
            c.fragment = Some(rest[hash + 1..].to_string());
            rest = &rest[..hash];
        }
        if let Some(q) = rest.find('?') {
            c.query = Some(rest[q + 1..].to_string());
            rest = &rest[..q];
        }
        if let Some(colon) = rest.find(':') {
            let candidate = &rest[..colon];
            if is_valid_scheme(candidate) {
                c.scheme = Some(candidate.to_string());
                rest = &rest[colon + 1..];
            }
        }
        if let Some(after) = rest.strip_prefix("//") {
            let end = after.find('/').unwrap_or(after.len());
            let (authority, path) = after.split_at(end);
            c.parse_authority(authority)?;
            c.path = path.to_string();
        } else {
            c.path = rest.to_string();
        }
        Some(c)
    }

    fn parse_authority(&mut self, authority: &str) -> Option<()> {
        let hostport = match authority.rfind('@') {
            Some(at) => {
                let userinfo = &authority[..at];
                match userinfo.find(':') {
                    Some(colon) => {
                        self.user = Some(userinfo[..colon].to_string());
                        self.password = Some(userinfo[colon + 1..].to_string());
                    }
                    None => self.user = Some(userinfo.to_string()),
                }
                &authority[at + 1..]
            }
            None => authority,
        };
        let (host, port) = if hostport.starts_with('[') {
            // IP literal: the port separator is the first colon after `]`.
            let close = hostport.find(']')?;
            let host = &hostport[..=close];
            match &hostport[close + 1..] {
                "" => (host, None),
                tail => (host, Some(tail.strip_prefix(':')?)),
            }
        } else {
            match hostport.rfind(':') {
                Some(colon) => (&hostport[..colon], Some(&hostport[colon + 1..])),
                None => (hostport, None),
            }
        };
        self.host = Some(host.to_string());
        self.port = match port {
            None | Some("") => None,
            Some(p) => {
                if !p.bytes().all(|b| b.is_ascii_digit()) {
                    return None;
                }
                Some(p.parse::<u16>().ok()?)
            }
        };
        Some(())
    }

    fn has_authority(&self) -> bool {
        self.host.is_some() || self.user.is_some() || self.password.is_some() || self.port.is_some()
    }

    /// The URL string, or `None` if the components are inconsistent (as
    /// Foundation does, e.g. an authority followed by a relative path).
    fn compose(&self) -> Option<String> {
        let has_authority = self.has_authority();
        if has_authority && !self.path.is_empty() && !self.path.starts_with('/') {
            return None;
        }
        if !has_authority && self.path.starts_with("//") {
            return None;
        }
        let mut out = String::new();
        if let Some(scheme) = &self.scheme {
            out.push_str(scheme);
            out.push(':');
        }
        if has_authority {
            out.push_str("//");
            if self.user.is_some() || self.password.is_some() {
                if let Some(user) = &self.user {
                    out.push_str(user);
                }
                if let Some(password) = &self.password {
                    out.push(':');
                    out.push_str(password);
                }
                out.push('@');
            }
            if let Some(host) = &self.host {
                out.push_str(host);
            }
            if let Some(port) = self.port {
                out.push(':');
                out.push_str(&port.to_string());
            }
        }
        out.push_str(&self.path);
        if let Some(query) = &self.query {
            out.push('?');
            out.push_str(query);
        }
        if let Some(fragment) = &self.fragment {
            out.push('#');
            out.push_str(fragment);
        }
        Some(out)
    }

    /// Plain-setter semantics: encode `value` for `part`.
    fn set_plain(&mut self, part: Part, value: Option<String>) {
        let encoded = value.map(|v| {
            // A bracketed IP literal must not have its brackets escaped.
            if part == Part::Host && v.starts_with('[') && v.ends_with(']') {
                v
            } else {
                percent_encode(&v, part)
            }
        });
        self.store(part, encoded);
    }

    /// `percentEncoded*` setter semantics: store as-is if well-formed.
    /// Returns `false` (leaving `self` unchanged) otherwise.
    fn set_encoded(&mut self, part: Part, value: Option<String>) -> bool {
        if let Some(v) = &value {
            if !is_valid_encoded(v, part) {
                return false;
            }
        }
        self.store(part, value);
        true
    }

    fn store(&mut self, part: Part, value: Option<String>) {
        match part {
            Part::User => self.user = value,
            Part::Password => self.password = value,
            Part::Host => self.host = value,
            Part::Path => self.path = value.unwrap_or_default(),
            Part::Query => self.query = value,
            Part::Fragment => self.fragment = value,
        }
    }

    fn encoded(&self, part: Part) -> Option<String> {
        match part {
            Part::User => self.user.clone(),
            Part::Password => self.password.clone(),
            Part::Host => self.host.clone(),
            Part::Path => Some(self.path.clone()),
            Part::Query => self.query.clone(),
            Part::Fragment => self.fragment.clone(),
        }
    }
}

// MARK: - Guest-object glue

fn optional_to_ns_string(env: &mut Environment, s: Option<String>) -> id {
    match s {
        Some(s) => {
            let ns = from_rust_string(env, s);
            autorelease(env, ns)
        }
        None => nil,
    }
}

fn ns_string_to_optional(env: &mut Environment, s: id) -> Option<String> {
    if s == nil {
        None
    } else {
        Some(to_rust_string(env, s).into_owned())
    }
}

fn components_of(env: &Environment, this: id) -> Components {
    env.objc
        .borrow::<NSURLComponentsHostObject>(this)
        .components
        .clone()
}

fn set_components(env: &mut Environment, this: id, components: Components) {
    env.objc
        .borrow_mut::<NSURLComponentsHostObject>(this)
        .components = components;
}

/// Getter for a decoded property.
fn get_decoded(env: &mut Environment, this: id, part: Part) -> id {
    let value = components_of(env, this)
        .encoded(part)
        .and_then(|s| percent_decode(&s));
    optional_to_ns_string(env, value)
}

/// Getter for a `percentEncoded*` property.
fn get_encoded(env: &mut Environment, this: id, part: Part) -> id {
    let value = components_of(env, this).encoded(part);
    optional_to_ns_string(env, value)
}

fn set_decoded(env: &mut Environment, this: id, part: Part, value: id) {
    let value = ns_string_to_optional(env, value);
    env.objc
        .borrow_mut::<NSURLComponentsHostObject>(this)
        .components
        .set_plain(part, value);
}

fn set_encoded(env: &mut Environment, this: id, part: Part, selector: &str, value: id) {
    let value = ns_string_to_optional(env, value);
    let ok = env
        .objc
        .borrow_mut::<NSURLComponentsHostObject>(this)
        .components
        .set_encoded(part, value.clone());
    if !ok {
        // Foundation raises NSInvalidArgumentException here. The raise would
        // kill the guest for a malformed optional component, so the value is
        // rejected (the component stays unchanged) and the call is reported.
        log!(
            "Warning: -[NSURLComponents {}] given invalid percent-encoded \
            string {:?}, ignored (real iOS raises NSInvalidArgumentException)",
            selector,
            value
        );
    }
}

/// Initializes `this` from `string`. Releases `this` and returns `nil` if the
/// string is not a valid URI reference.
fn init_with_rust_string(env: &mut Environment, this: id, string: Option<&str>) -> id {
    let Some(string) = string else {
        release(env, this);
        return nil;
    };
    match Components::parse(string) {
        Some(components) => {
            set_components(env, this, components);
            this
        }
        None => {
            release(env, this);
            nil
        }
    }
}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation NSURLComponents: NSObject

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<NSURLComponentsHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

+ (id)componentsWithString:(id)url_string { // NSString*
    let new: id = msg![env; this alloc];
    let new: id = msg![env; new initWithString:url_string];
    autorelease(env, new)
}

+ (id)componentsWithURL:(id)url // NSURL*
 resolvingAgainstBaseURL:(bool)resolve {
    let new: id = msg![env; this alloc];
    let new: id = msg![env; new initWithURL:url resolvingAgainstBaseURL:resolve];
    autorelease(env, new)
}

// `+new` is inherited from NSObject (`alloc` then `init`); `-init` leaves all
// components empty, as on iOS.

- (id)initWithString:(id)url_string { // NSString*
    let string = ns_string_to_optional(env, url_string);
    init_with_rust_string(env, this, string.as_deref())
}

- (id)initWithURL:(id)url // NSURL*
resolvingAgainstBaseURL:(bool)_resolve {
    // NSURL here does not track base URLs (see `-[NSURL relativeString]`),
    // so `resolve` makes no difference: the absolute string is parsed.
    let string = if url == nil {
        None
    } else {
        let absolute: id = msg![env; url absoluteString];
        ns_string_to_optional(env, absolute)
    };
    init_with_rust_string(env, this, string.as_deref())
}

- (())dealloc {
    env.objc.dealloc_object(this, &mut env.mem)
}

- (id)copyWithZone:(NSZonePtr)_zone {
    let new: id = msg_class![env; NSURLComponents alloc];
    let components = components_of(env, this);
    set_components(env, new, components);
    new
}

// MARK: - Whole URL

- (id)string {
    let string = components_of(env, this).compose();
    optional_to_ns_string(env, string)
}

- (id)URL {
    let string = components_of(env, this).compose();
    match string {
        Some(string) => {
            let ns = from_rust_string(env, string);
            let url: id = msg_class![env; NSURL URLWithString:ns];
            release(env, ns);
            url
        }
        None => nil,
    }
}

// MARK: - Scheme and port (no percent-encoding variants)

- (id)scheme {
    let scheme = components_of(env, this).scheme;
    optional_to_ns_string(env, scheme)
}
- (())setScheme:(id)scheme { // NSString*
    let scheme = ns_string_to_optional(env, scheme);
    match scheme {
        Some(ref s) if !is_valid_scheme(s) => {
            // Real iOS raises NSInvalidArgumentException; see `set_encoded`.
            log!(
                "Warning: -[NSURLComponents setScheme:] given invalid scheme \
                {:?}, ignored (real iOS raises NSInvalidArgumentException)",
                s
            );
        }
        _ => {
            env.objc
                .borrow_mut::<NSURLComponentsHostObject>(this)
                .components
                .scheme = scheme;
        }
    }
}

- (id)port { // NSNumber*
    match components_of(env, this).port {
        Some(port) => msg_class![env; NSNumber numberWithInt:(port as i32)],
        None => nil,
    }
}
- (())setPort:(id)port { // NSNumber*
    if port == nil {
        env.objc
            .borrow_mut::<NSURLComponentsHostObject>(this)
            .components
            .port = None;
        return;
    }
    let value: i32 = msg![env; port intValue];
    match u16::try_from(value) {
        Ok(value) => {
            env.objc
                .borrow_mut::<NSURLComponentsHostObject>(this)
                .components
                .port = Some(value);
        }
        Err(_) => {
            // Real iOS raises NSInvalidArgumentException; see `set_encoded`.
            log!(
                "Warning: -[NSURLComponents setPort:] given out-of-range \
                port {}, ignored (real iOS raises NSInvalidArgumentException)",
                value
            );
        }
    }
}

// MARK: - Decoded properties

- (id)user { get_decoded(env, this, Part::User) }
- (())setUser:(id)value { set_decoded(env, this, Part::User, value) }
- (id)password { get_decoded(env, this, Part::Password) }
- (())setPassword:(id)value { set_decoded(env, this, Part::Password, value) }
- (id)host { get_decoded(env, this, Part::Host) }
- (())setHost:(id)value { set_decoded(env, this, Part::Host, value) }
- (id)path { get_decoded(env, this, Part::Path) }
- (())setPath:(id)value { set_decoded(env, this, Part::Path, value) }
- (id)query { get_decoded(env, this, Part::Query) }
- (())setQuery:(id)value { set_decoded(env, this, Part::Query, value) }
- (id)fragment { get_decoded(env, this, Part::Fragment) }
- (())setFragment:(id)value { set_decoded(env, this, Part::Fragment, value) }

// MARK: - Percent-encoded properties

- (id)percentEncodedUser { get_encoded(env, this, Part::User) }
- (())setPercentEncodedUser:(id)value {
    set_encoded(env, this, Part::User, "setPercentEncodedUser:", value)
}
- (id)percentEncodedPassword { get_encoded(env, this, Part::Password) }
- (())setPercentEncodedPassword:(id)value {
    set_encoded(env, this, Part::Password, "setPercentEncodedPassword:", value)
}
- (id)percentEncodedHost { get_encoded(env, this, Part::Host) }
- (())setPercentEncodedHost:(id)value {
    set_encoded(env, this, Part::Host, "setPercentEncodedHost:", value)
}
- (id)percentEncodedPath { get_encoded(env, this, Part::Path) }
- (())setPercentEncodedPath:(id)value {
    set_encoded(env, this, Part::Path, "setPercentEncodedPath:", value)
}
- (id)percentEncodedQuery { get_encoded(env, this, Part::Query) }
- (())setPercentEncodedQuery:(id)value {
    set_encoded(env, this, Part::Query, "setPercentEncodedQuery:", value)
}
- (id)percentEncodedFragment { get_encoded(env, this, Part::Fragment) }
- (())setPercentEncodedFragment:(id)value {
    set_encoded(env, this, Part::Fragment, "setPercentEncodedFragment:", value)
}

@end

};

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(s: &str) -> Components {
        Components::parse(s).unwrap_or_else(|| panic!("{s:?} should parse"))
    }

    #[test]
    fn parses_all_components() {
        let c = parse("https://user:p%40ss@example.com:8080/a/b%20c?x=1&y=2#frag");
        assert_eq!(c.scheme.as_deref(), Some("https"));
        assert_eq!(c.user.as_deref(), Some("user"));
        assert_eq!(c.password.as_deref(), Some("p%40ss"));
        assert_eq!(
            percent_decode(c.password.as_deref().unwrap()).unwrap(),
            "p@ss"
        );
        assert_eq!(c.host.as_deref(), Some("example.com"));
        assert_eq!(c.port, Some(8080));
        assert_eq!(c.path, "/a/b%20c");
        assert_eq!(c.query.as_deref(), Some("x=1&y=2"));
        assert_eq!(c.fragment.as_deref(), Some("frag"));
    }

    #[test]
    fn parse_then_compose_round_trips() {
        for s in [
            "http://example.com",
            "http://example.com/",
            "https://a.b/c?d#e",
            "http://[::1]:80/x",
            "file:///var/mobile/x.txt",
            "mailto:someone@example.com",
            "/relative/path?q=1",
            "relative",
            "?only=query",
            "#only-fragment",
            "http://example.com?",
            "http://example.com#",
            "http://u@h/",
        ] {
            assert_eq!(
                parse(s).compose().as_deref(),
                Some(s),
                "round trip of {s:?}"
            );
        }
    }

    #[test]
    fn absent_versus_empty_components() {
        let c = parse("http://example.com");
        assert_eq!(c.path, "");
        assert_eq!(c.query, None);
        assert_eq!(c.fragment, None);
        assert_eq!(c.port, None);
        let c = parse("http://example.com/?#");
        assert_eq!(c.query.as_deref(), Some(""));
        assert_eq!(c.fragment.as_deref(), Some(""));
        // `file:///x` has an empty (not absent) host.
        assert_eq!(parse("file:///x").host.as_deref(), Some(""));
    }

    #[test]
    fn rejects_invalid_strings() {
        assert!(Components::parse("http://exa mple.com").is_none());
        assert!(Components::parse("http://example.com/%zz").is_none());
        assert!(Components::parse("http://example.com/%2").is_none());
        assert!(Components::parse("http://example.com:80a/").is_none());
        assert!(Components::parse("http://example.com:99999/").is_none());
        assert!(Components::parse("http://example.com/<>").is_none());
        assert!(Components::parse("http://[::1/").is_none());
    }

    #[test]
    fn empty_string_is_an_empty_relative_reference() {
        let c = parse("");
        assert_eq!(c, Components::default());
        assert_eq!(c.compose().as_deref(), Some(""));
    }

    #[test]
    fn non_scheme_prefix_before_colon_is_path() {
        let c = parse("/a:b");
        assert_eq!(c.scheme, None);
        assert_eq!(c.path, "/a:b");
        let c = parse("1abc:rest");
        assert_eq!(c.scheme, None);
    }

    #[test]
    fn plain_setters_encode_and_encoded_setters_validate() {
        let mut c = Components::default();
        c.set_plain(Part::Path, Some("/a b/100%".to_string()));
        assert_eq!(c.path, "/a%20b/100%25");
        c.set_plain(Part::Query, Some("a=b c&d=é".to_string()));
        assert_eq!(c.query.as_deref(), Some("a=b%20c&d=%C3%A9"));
        c.set_plain(Part::Fragment, Some("x#y".to_string()));
        assert_eq!(c.fragment.as_deref(), Some("x%23y"));
        c.set_plain(Part::Host, Some("[::1]".to_string()));
        assert_eq!(c.host.as_deref(), Some("[::1]"));

        assert!(c.set_encoded(Part::Path, Some("/ok%20path".to_string())));
        assert_eq!(c.path, "/ok%20path");
        let before = c.clone();
        assert!(!c.set_encoded(Part::Path, Some("/not ok".to_string())));
        assert!(!c.set_encoded(Part::Query, Some("a=%G1".to_string())));
        assert_eq!(c, before, "rejected values must not change the components");
    }

    #[test]
    fn setting_nil_clears_components() {
        let mut c = parse("http://u:p@h:1/p?q#f");
        c.set_plain(Part::User, None);
        c.set_plain(Part::Password, None);
        c.set_plain(Part::Host, None);
        c.port = None;
        c.set_plain(Part::Path, None);
        c.set_plain(Part::Query, None);
        c.set_plain(Part::Fragment, None);
        assert_eq!(c.compose().as_deref(), Some("http:"));
    }

    #[test]
    fn compose_rejects_inconsistent_components() {
        let mut c = Components::default();
        c.host = Some("example.com".to_string());
        c.path = "relative".to_string();
        assert_eq!(c.compose(), None);
        c.path = "/absolute".to_string();
        assert_eq!(c.compose().as_deref(), Some("//example.com/absolute"));

        let mut c = Components::default();
        c.path = "//looks-like-authority".to_string();
        assert_eq!(c.compose(), None);
    }

    #[test]
    fn builds_a_url_from_scratch() {
        // The `[NSURLComponents new]` + setters pattern.
        let mut c = Components::default();
        c.scheme = Some("https".to_string());
        c.set_plain(Part::Host, Some("api.example.com".to_string()));
        c.set_plain(Part::Path, Some("/v1/some thing".to_string()));
        c.set_plain(Part::Query, Some("k=v".to_string()));
        c.port = Some(443);
        assert_eq!(
            c.compose().as_deref(),
            Some("https://api.example.com:443/v1/some%20thing?k=v")
        );
    }

    #[test]
    fn percent_decode_rejects_bad_input() {
        assert_eq!(percent_decode("a%20b").as_deref(), Some("a b"));
        assert_eq!(percent_decode("%C3%A9").as_deref(), Some("é"));
        assert_eq!(percent_decode("%"), None);
        assert_eq!(percent_decode("%1"), None);
        assert_eq!(percent_decode("%zz"), None);
        assert_eq!(percent_decode("%FF"), None, "not valid UTF-8");
    }

    #[test]
    fn scheme_validation() {
        assert!(is_valid_scheme("http"));
        assert!(is_valid_scheme("x-callback+url.2"));
        assert!(!is_valid_scheme(""));
        assert!(!is_valid_scheme("1http"));
        assert!(!is_valid_scheme("ht tp"));
    }
}
