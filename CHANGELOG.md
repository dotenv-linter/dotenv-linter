# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### 🚀 Added

### 🔧 Changed
- Rename `UnorderedKeys` check to singular `UnorderedKey` [#147](https://github.com/mgrachev/dotenv-linter/pull/147) ([@pineapplethief](https://github.com/pineapplethief))
- KeyWithoutValue: Show check name in the message [#139](https://github.com/mgrachev/dotenv-linter/pull/139) ([@harshu4](https://github.com/harshu4))
- LowercaseKey: Show check name in the message [#131](https://github.com/mgrachev/dotenv-linter/pull/131) ([@qelphybox](https://github.com/qelphybox))
- DuplicatedKey: Show check name in message [#138](https://github.com/mgrachev/dotenv-linter/pull/138)([@SaMuRa1ReM1X](https://github.com/SaMuRa1ReM1X))
- IncorrectDelimiter: Show check name in the message [#146](https://github.com/mgrachev/dotenv-linter/pull/146) ([undef1nd](https://github.com/undef1nd))
- Replaced kcov with grcov in Github Actions [#143](https://github.com/mgrachev/dotenv-linter/pull/143) ([@pmk21](https://github.com/pmk21))
- Streamline CLI tests and split into smaller files [#137](https://github.com/mgrachev/dotenv-linter/pull/137) ([@sonro](https://github.com/sonro))
- UnorderedKey: Added check name to the message [#140](https://github.com/mgrachev/dotenv-linter/pull/140) ([@pmk21](https://github.com/pmk21))
- Add test coverage for CLI --exclude arguments [#135](https://github.com/mgrachev/dotenv-linter/pull/135) ([@sonro](https://github.com/sonro))
- Renamed check SpacesAroundEqual to SpaceCharacter [#134](https://github.com/mgrachev/dotenv-linter/pull/134) ([@SaMuRa1ReM1X](https://github.com/SaMuRa1ReM1X))
- Rename check DuplicatedKeys to DuplicatedKey [#133](https://github.com/mgrachev/dotenv-linter/pull/133) ([@sonro](https://github.com/sonro))
- Minimizing Rust Binary Size [#132](https://github.com/mgrachev/dotenv-linter/pull/132) ([@akirill0v](https://github.com/akirill0v))
- Remove the unwrap method and use platform native OsString to fetch the information about current directory [#115](https://github.com/mgrachev/dotenv-linter/pull/115) ([@kanapuli](https://github.com/kanapuli))
- Use HashSet for DuplicateKeyChecker [#113](https://github.com/mgrachev/dotenv-linter/pull/113) ([@TamasFlorin](https://github.com/TamasFlorin))
- Use reference for the LineEntry as part of the run method for checks [#111](https://github.com/mgrachev/dotenv-linter/pull/111) ([@TamasFlorin](https://github.com/TamasFlorin))
- New CLI API: Ability to check multiple directories [#99](https://github.com/mgrachev/dotenv-linter/pull/99)
- Add exit with the code 0 when there are no warnings [#105](https://github.com/mgrachev/dotenv-linter/pull/105) ([@simPod](https://github.com/simPod))
- Use `get` method to get result of item in `Vec` and use ? operator unwrap the result if it's `Some` [#108](https://github.com/mgrachev/dotenv-linter/pull/108) ([@boybird](https://github.com/boybird))

## [v1.1.2] - 2020-03-13
### 🔧 Changed
- Fix --path CLI parameter not canonizing filepaths from directory path passed as argument and not working as intended as a result [#97](https://github.com/mgrachev/dotenv-linter/pull/97) ([@pineapplethief](https://github.com/pineapplethief))
- Fix incorrect delimiter check for keys contains numeric [#95](https://github.com/mgrachev/dotenv-linter/pull/95) ([@alter369](https://github.com/alter369))
- Add `LineEntry.is_empty_or_comment` method to DRY and simplify `Check.run` [#94](https://github.com/mgrachev/dotenv-linter/pull/94) ([@pineapplethief](https://github.com/pineapplethief))
- Refactor `Github Actions` and reduce `Docker Image size` [#90](https://github.com/mgrachev/dotenv-linter/pull/90) ([@Macbet](https://github.com/Macbet))
- Use `Line.get_key` in all checks [#89](https://github.com/mgrachev/dotenv-linter/pull/89) ([@pineapplethief](https://github.com/pineapplethief))

## [v1.1.1] - 2020-02-18
### 🔧 Changed
- Show the full path to the file relative to the current directory [#85](https://github.com/mgrachev/dotenv-linter/pull/85)

## [v1.1.0] - 2020-01-27
### 🚀 Added
- Add check: Unordered keys [#72](https://github.com/mgrachev/dotenv-linter/pull/72)
- Add the ability to specify the directory where to run [#65](https://github.com/mgrachev/dotenv-linter/pull/65) ([@Louis-Lesage](https://github.com/Louis-Lesage))
- Add check: Duplicated keys [#33](https://github.com/mgrachev/dotenv-linter/pull/33) ([@mstruebing](https://github.com/mstruebing))
- Add exit with the code 1 on warnings found [#58](https://github.com/mgrachev/dotenv-linter/pull/58) ([@Louis-Lesage](https://github.com/Louis-Lesage))
- Add exclude argument to exclude specific files [#49](https://github.com/mgrachev/dotenv-linter/pull/49) ([@mstruebing](https://github.com/mstruebing))
- Add the ability to include files to check [#48](https://github.com/mgrachev/dotenv-linter/pull/48)
- Add check: Spaces around equal sign [#35](https://github.com/mgrachev/dotenv-linter/pull/35) ([@artem-russkikh](https://github.com/artem-russkikh))
- Add skipping of commented or empty lines [#37](https://github.com/mgrachev/dotenv-linter/pull/37) ([@mstruebing](https://github.com/mstruebing))

### 🔧 Changed
- Rename `leading_space` to `leading_character` and check for allowed chars [#63](https://github.com/mgrachev/dotenv-linter/pull/63) ([@mstruebing](https://github.com/mstruebing))
- Remove multiple checks of the same file [#62](https://github.com/mgrachev/dotenv-linter/pull/62) ([@mstruebing](https://github.com/mstruebing))
- Add mutability support for checks [#52](https://github.com/mgrachev/dotenv-linter/pull/52)

## [v1.0.0] - 2020-01-01
### 🚀 Added
- Add check: Keys without values [#22](https://github.com/mgrachev/dotenv-linter/pull/22) ([@mstruebing](https://github.com/mstruebing))
- Add check: Lowercase keys [#21](https://github.com/mgrachev/dotenv-linter/pull/21) ([@qelphybox](https://github.com/qelphybox))
- Add check: Incorrect delimiter [#20](https://github.com/mgrachev/dotenv-linter/pull/20) ([@sonro](https://github.com/sonro))
- Add `Display` trait for `LineEntry` [#19](https://github.com/mgrachev/dotenv-linter/pull/19) ([@mstruebing](https://github.com/mstruebing))

### 🔧 Changed
- Add support for command line arguments [#31](https://github.com/mgrachev/dotenv-linter/pull/31)
- Replace field warning with template for all check structs [#26](https://github.com/mgrachev/dotenv-linter/pull/26)
- Prepare a template for easy adding new checks [#14](https://github.com/mgrachev/dotenv-linter/pull/14)

[v1.1.2]: https://github.com/mgrachev/dotenv-linter/releases/tag/v1.1.2
[v1.1.1]: https://github.com/mgrachev/dotenv-linter/releases/tag/v1.1.1
[v1.1.0]: https://github.com/mgrachev/dotenv-linter/releases/tag/v1.1.0
[v1.0.0]: https://github.com/mgrachev/dotenv-linter/releases/tag/v1.0.0
