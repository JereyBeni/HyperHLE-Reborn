# HyperHLE-Reborn

<p align="center"><img src="./Picsart_26-10-20_10-43-04-335.jpg" alt="HyperHLE-Reborn" width="700"></p>

**HyperHLE-Reborn** is a community-maintained fork of [HyperHLE](https://github.com/HyperHLE/HyperHLE), itself based on [touchHLE](https://github.com/touchHLE/touchHLE). The user-facing name of this project is **HyperHLE**.

HyperHLE is a high-level emulator for early iPhone OS apps. It reimplements selected iOS frameworks on the host, allowing compatible apps to run without booting iOS. This fork focuses on Android/mobile usability, fullscreen and orientation handling, graphics compatibility, and app-specific compatibility fixes. It is not an official Apple product.

The repository does not distribute commercial apps or iOS firmware. Put an app bundle (`.app`) or IPA (`.ipa`) that you are entitled to use in `touchHLE_apps/`; it will appear in the app picker.

## Platforms and features

- Builds for Android, iOS, Windows, macOS, and Linux. The iOS app comes from [Applesauce](https://github.com/johnny901901901/Applesauce); see [`platform/ios`](platform/ios/README.md).
- Virtual iPhone and iPad device profiles.
- Android touch input and fullscreen/orientation handling.
- Ongoing GLES, EAGL, Core Animation, audio, and iOS framework compatibility work.
- Download build artifacts from the [HyperHLE GitHub Actions workflow](https://github.com/KlugKlugTG/HyperHLE-Reborn/actions/workflows/HyperHLE_release.yml).

## Game and app compatibility

Compatibility depends on the exact app version, virtual device profile, host GPU/driver, and emulator build. **Playable** means gameplay has been reported; **In-game** means the app gets past startup but may have issues; **Experimental** means only partial startup or limited testing is confirmed. These are community-reported results, not guarantees that every feature or a full playthrough works.

### Reported playable

| App | Reported status |
| --- | --- |
| N.O.V.A. 3 | Playable |
| Geometry Dash | Playable |
| Modern Combat 3 | Playable |
| Asphalt 7 | Playable |
| Turbo Dismount | Playable |
| Scarface | Playable |
| Zombie Safari | Playable |
| Real Racing 1 | Playable |
| Real Racing 3 | Playable |
| Silent Ops | Playable |
| Need for Speed: Most Wanted | Playable |
| Minecraft 0.14.2–0.16.2 | Playable |
| Oceanhorn | Playable |
| Asphalt 8 | Playable |
| Talking Tom Hero Dash | Playable |
| My Talking Hank | Playable |
| Talking Tom Candy Run | Reported playable on HyperHLE-Reborn; app-version-specific testing is recommended |

### Starts, but may have issues

| App | Reported status |
| --- | --- |
| GTA 3 | In-game |
| Granny | In-game |

### Known compatibility issue

| App | Issue |
| --- | --- |
| Gangstar Vegas | Reported unplayable on Adreno GPUs; results may differ on other devices/drivers |

### Compatibility work in progress

This fork also investigates apps that need additional iOS framework or graphics support beyond the existing upstream implementation. A successful launch on this fork does **not** automatically mean the same app was tested against the original HyperHLE/touchHLE build.

| App | Current status |
| --- | --- |
| My Talking Tom 2 | Work in progress. Current investigation targets startup and the birth-year selection screen; it is **not yet listed as playable**. |

**Reporting results:** Please include the app name, exact app version, device/GPU, emulator build or commit, and what works or fails. An update can change compatibility, so older reports may no longer apply.

## Build and documentation

Build and run a desktop version with Cargo:

```sh
cargo run --release -- path/to/app.app
```

For iOS build steps and the unsigned IPAs, see [`platform/ios`](platform/ios/README.md). For Android build steps, see the `android/` directory and the [CI workflow](https://github.com/KlugKlugTG/HyperHLE-Reborn/actions/workflows/HyperHLE_release.yml). Command-line options are documented in [`OPTIONS_HELP.txt`](OPTIONS_HELP.txt); notable compatibility changes are tracked in [`CHANGELOG.md`](CHANGELOG.md).

## Community

Join the [HyperHLE Discord server](https://discord.gg/ZpEkAV47H9) to discuss the project and contribute.

## Credits and license

HyperHLE-Reborn carries forward work from HyperHLE and touchHLE contributors and uses open-source libraries. It is licensed under the [Mozilla Public License 2.0](LICENSE); see the individual projects for their respective notices and licenses.
