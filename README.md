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

Compatibility depends on the exact app version, virtual device profile, host GPU/driver, and emulator build. **Playable** means gameplay has been reported; **In-game** means the app gets past startup but may have issues; **Experimental** means only partial startup or limited testing is confirmed. Wishlist entries below are **not compatibility claims**: they are candidate 32-bit iOS app versions from the iOS 6–10 era for which this repository currently has no confirmed test report. An app may have both 32-bit and 64-bit releases, so the exact IPA/build must be checked.

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

| App | Current status |
| --- | --- |
| My Talking Tom 2 | Work in progress. Current investigation targets startup and the birth-year selection screen; it is **not yet listed as playable**. |

### Community wishlist: classic 32-bit iOS games and apps

These are **wanted test targets**, not confirmed working games. The focus is on 32-bit versions released for or compatible with the iOS 6–10 era. Some titles have multiple releases; check the exact binary architecture and minimum iOS version before testing. “No report” means no confirmed compatibility result is documented in this repository yet, not that nobody anywhere has tested it.

| App / game | What we'd like to test or fix | Status in this repository |
| --- | --- | --- |
| My Talking Angela | Fix gameplay, UI, and animation bugs | No report; wishlist |
| Real Madrid Talking Players | Fix character animations and transitions | No report; wishlist |
| Talking Tom (classic) | Test older 32-bit releases and core interactions | No report; wishlist |
| Talking Tom Cat 2 | Test touch interactions, sound, and animations | No report; wishlist |
| Talking Ginger | Test character interactions and mini-games | No report; wishlist |
| Talking Ben the Dog | Test voice interaction and character responses | No report; wishlist |
| Talking Santa | Test legacy character interactions | No report; wishlist |
| Where's My Water? (very old version) | Confirmed working in the user's test; version-specific | Tested working; exact version not recorded here |
| Where's My Perry? | Tested by the project helper; does not work in the tested setup | Tested not working; exact version and failure details still needed |
| Cut the Rope | Reported to work reliably in testing | Reported working; exact version not recorded here |
| Cut the Rope: Experiments | Test physics and level progression | No report; wishlist |
| Angry Birds | Reported to work reliably in testing | Reported working; exact version not recorded here |
| Angry Birds Seasons | Test level packs and rendering | No report; wishlist |
| Fruit Ninja | Reported working in testing | Reported working; exact version not recorded here |
| Jetpack Joyride | Possibly works; needs a fresh confirmation | Uncertain; retest needed |
| Tiny Wings | Reported working in testing | Reported working; exact version not recorded here |
| Doodle Jump | Reported working in testing | Reported working; exact version not recorded here |
| Temple Run | Test 3D rendering and controls | No report; wishlist |
| Subway Surfers (older 32-bit build) | Test 3D rendering and gameplay | No report; wishlist |
| Plants vs. Zombies | Test touch controls, animations, and levels | No report; wishlist |
| The Sims FreePlay (older 32-bit build) | Test simulation and UI | No report; wishlist |
| The Room | Test 3D scenes, puzzles, and touch interactions | No report; wishlist |
| BADLAND | Test physics, effects, and level gameplay | No report; wishlist |
| LIMBO (iOS) | Test rendering and touch controls | No report; wishlist |
| Shadowgun | Test 3D graphics and controls | No report; wishlist |
| Dead Trigger | Test 3D rendering, input, and gameplay | No report; wishlist |
| Modern Combat 4: Zero Hour | Test startup, graphics, and controls | No report; wishlist |
| N.O.V.A. 2 | Test 3D rendering and gameplay | No report; wishlist |
| Gangstar Rio: City of Saints | Test startup and 3D gameplay | No report; wishlist |
| FIFA 13 (older iOS build) | Test menus, rendering, and match gameplay | No report; wishlist |
| LEGO Harry Potter: Years 1–4 | Test 3D scenes and touch controls | No report; wishlist |
| Scribblenauts Remix | Test touch input and object interactions | No report; wishlist |
| Toca Kitchen | Test touch interactions and animations | No report; wishlist |
| Pango Playground | Test scene interactions and animations | No report; wishlist |
| Pango StoryTime | Test story scenes, audio, and interactions | No report; wishlist |
| Pango Zoo | Test scene rendering and interactions | No report; wishlist |

### Reporting results

Please include the app name, exact app version, whether the binary is ARMv7/32-bit or ARM64/64-bit, minimum iOS version, device/GPU, emulator build or commit, and what works or fails. A title should only move from the wishlist into a compatibility category after someone provides a reproducible test report.

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
