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

### Recent test reports on the latest fork

Results reported by the project helper. These are specific to the tested app versions and setup; they are not guarantees for other builds.

| App / version | Result |
| --- | --- |
| Angry Birds Rio 2.6.7 | Does not boot |
| Toca Kitchen 1.1.6 | Works, but the image is stretched incorrectly |
| Bad Piggies 1.7.0 | Crashes |
| Sonic CD 1.0.1 | Does not boot |
| Where's My Mickey? for iPad 1.1.0 | Does not work; exact failure details not provided |
| Tiny Tower 2.2.8 | Works normally |
| Infinity Blade v7895 | Starts, then hangs after the Unreal logo moves upward; had to be closed from the terminal |
| Rayman Jungle Run (version not provided) | Works, but the letters/text look strange or unclear; exact visual issue not yet identified |
| Kingdom Rush (version not provided) | Works, but HyperHLE detects or reports the display as 9:16 |
| My Talking Angela (version not provided) | Opens a window, but the game display remains completely black in the screenshot |
| Real Madrid Talking Players (version not provided) | Reaches the character screen and renders the player and 3D background; screenshot appears to show Di María selected/available in the free game |

### Community wishlist: 200 classic iOS 5–10 game and app test targets

These are **candidate test targets, not compatibility claims**. The list contains 200 distinct entries assembled around games and apps released during or with versions from the iOS 5–10 era. Some later ports or variants may be 64-bit-only, require a newer iOS version, or have a title/version mismatch; check the exact IPA, CPU architecture, and minimum iOS version before testing. Existing test reports are preserved as reported by the project helper; all other entries remain untested here.

| App / game | What we'd like to test or fix | Status in this repository |
| --- | --- | --- |
| My Talking Angela | Screenshot report: app window opens but the game area stays black | Tested; black screen at startup in the provided screenshot |
| Real Madrid Talking Players | Screenshot report: character screen renders with a 3D player and stadium background; Di María appears in the free game | Tested; character screen renders in the provided screenshot |
| Talking Tom (classic) | Test older 32-bit releases and core interactions | No report; wishlist |
| Talking Tom Cat 2 | Test touch interactions, sound, and animations | No report; wishlist |
| Talking Ginger | Test character interactions and mini-games | No report; wishlist |
| Talking Ben the Dog | Test voice interaction and character responses | No report; wishlist |
| Talking Santa | Investigate incorrect frame ordering during animation | Tested; frame-order problem reported |
| Where's My Water? (very old version) | Confirmed working in the user's test; version-specific | Tested working; exact version not recorded here |
| Where's My Perry? | Tested by the project helper; does not work in the tested setup | Tested not working; exact version and failure details still needed |
| Cut the Rope | Reported to work reliably in testing | Reported working; exact version not recorded here |
| Cut the Rope: Experiments | Played by the project helper; compatibility result not specified yet | Played; result needs confirmation |
| Angry Birds | Reported to work reliably in testing | Reported working; exact version not recorded here |
| Angry Birds Seasons | Not yet played by the project helper; remains a test target | Never played; wishlist |
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
| Toca Kitchen | Version 1.1.6: runs, but the display is stretched incorrectly | Tested; visual scaling/aspect-ratio issue |
| Pango Playground | Test scene interactions and animations | No report; wishlist |
| Pango StoryTime | Test story scenes, audio, and interactions | No report; wishlist |
| Pango Zoo | Test scene rendering and interactions | No report; wishlist |
| Angry Birds Rio | Version 2.6.7 does not boot | Tested; fails to boot |
| Angry Birds Space | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Angry Birds Star Wars | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Angry Birds Star Wars II | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Angry Birds Friends | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Angry Birds Trilogy companion apps | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Bad Piggies | Version 1.7.0 crashes | Tested; crash on launch/run (exact point not specified) |
| Amazing Alex | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Where's My Mickey? (iPad) | Version 1.1.0 does not work in the tested setup | Tested; failure details not specified |
| Where's My Water? 2 | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Where's My Valentine? | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Where's My Holiday? | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Where's My Summer? | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Where's My Robot? | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Where's My Perry? (older build) | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Sprinkle | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Sprinkle Junior | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Pudding Monsters | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Cut the Rope: Time Travel | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Cut the Rope: Magic | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| King of Thieves | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Plants vs. Zombies 2 (older 32-bit build) | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Peggle | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Peggle Classic | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Bejeweled 2 | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Bejeweled Blitz | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Bookworm | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Chuzzle | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Puzzle Bobble | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Tetris | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Tetris Blitz | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Tetris 2011 | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Mahjong Artifacts | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Solitaire | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Spider Solitaire | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| UNO | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Monopoly | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Scrabble | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Yahtzee Adventures | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| The Game of Life | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Risk | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Clue: The Classic Mystery Game | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Infinity Blade | Version v7895 starts normally, then the Unreal logo moves upward and the app stops responding; it had to be closed from the terminal | Tested; hangs after Unreal logo appears |
| Rayman Jungle Run | Reported working, but letters/text look strange or unclear; exact visual issue not identified yet | Tested working; text rendering issue reported |
| Kingdom Rush | Reported working, but HyperHLE detects/reports the display as 9:16 | Tested working; aspect-ratio detection issue reported |
| Infinity Blade II | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Infinity Blade III | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Epic Citadel | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Chair Entertainment's Vengeance demo | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Dungeon Hunter 2 | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Dungeon Hunter 3 | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Dungeon Hunter 4 (older build) | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Order & Chaos Online | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Wild Blood | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| BackStab | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Hero of Sparta | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Gangstar: Miami Vindication | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Gangstar: West Coast Hustle | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Gangstar 2: Kings of L.A. | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Gangstar: New Orleans (check 32-bit build) | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Six-Guns | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Brothers in Arms 2: Global Front | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Brothers in Arms 3 (check architecture) | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Tom Clancy's Splinter Cell Conviction | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Tom Clancy's Rainbow Six: Shadow Vanguard | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Assassin's Creed: Altaïr's Chronicles | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Assassin's Creed II Discovery | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Prince of Persia: Warrior Within | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Prince of Persia: The Forgotten Sands | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Rayman Jungle Run | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Rayman Fiesta Run | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Rayman Classic | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Sonic the Hedgehog 4 Episode I | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Sonic the Hedgehog 4 Episode II | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Sonic CD | Version 1.0.1 does not boot | Tested; fails to boot |
| Sonic the Hedgehog 2 (2013) | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Sonic the Hedgehog (2013) | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Crazy Taxi | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Jet Set Radio | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Virtua Tennis Challenge | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Street Fighter IV Volt | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Street Fighter X Tekken Mobile | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| King of Fighters-i | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Samurai Shodown II | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Final Fantasy III | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Final Fantasy IV | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Final Fantasy Dimensions | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Chrono Trigger | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Secret of Mana | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Chaos Rings | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Chaos Rings II | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| The World Ends with You: Solo Remix | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Bastion | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Transistor (check architecture) | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Superbrothers: Sword & Sworcery EP | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Osmos | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| World of Goo | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Machinarium | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Botanicula | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Year Walk | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Device 6 | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| The Silent Age | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| The Walking Dead: Season One | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| The Wolf Among Us | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Broken Age | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| The Cave | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Monument Valley | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Monument Valley 2 (check architecture) | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Threes! | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| 2048 | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Drop7 | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Flow Free | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Unblock Me | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Where's My Water? Free | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Contre Jour | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Lazors | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Slice It! | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Cut the Buttons | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Fruit Ninja: Puss in Boots | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Fruit Ninja Frenzy | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Paper Toss | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Paper Toss 2.0 | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Angry Gran Run | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Agent Dash | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Minion Rush (older 32-bit build) | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Despicable Me: Minion Rush | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Ski Safari | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Ski Safari 2 (check architecture) | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Mega Jump | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Mega Run | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Doodle Jump Christmas Special | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Doodle Jump SpongeBob SquarePants | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Nyan Cat: Lost in Space | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Robot Unicorn Attack 2 | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Canabalt | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Super Hexagon | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| VVVVVV | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Bit.Trip Beat | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Geometry Wars: Touch | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Tilt to Live | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Boost 2 | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Flight Control | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Harbor Master | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Tiny Wings HD | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Jetpack Joyride: Back to the Future | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Temple Run 2 (older 32-bit build) | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Temple Run: Brave | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Temple Run: Oz | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Pitfall! | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Subway Surfers (older build variant) | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Rail Rush | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Sonic Dash (older 32-bit build) | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Smash Hit | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Riptide GP | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Riptide GP2 (check architecture) | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Shine Runner | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Reckless Racing | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Reckless Racing 2 | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Asphalt 6: Adrenaline | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Asphalt 5 | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Need for Speed Undercover | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Need for Speed Shift | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Need for Speed Hot Pursuit | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Real Racing 2 | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| Colin McRae Rally | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |
| F1 2011 | Check startup, graphics, input, audio, and gameplay; verify the exact iOS-era build | No report; wishlist |

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
