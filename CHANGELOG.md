# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### 🚀 Added
- Add --quiet argument [#242](https://github.com/dotenv-linter/dotenv-linter/pull/242) ([@wesleimp](https://github.com/wesleimp), [@mgrachev](https://github.com/mgrachev))
- Add installation CI test for Windows (via `install.sh`) [#235](https://github.com/dotenv-linter/dotenv-linter/pull/235) ([@DDtKey](https://github.com/DDtKey))

### 🔧 Changed

## [v2.1.0] - 2020-07-13
### 🚀 Added
- Add install.sh for provide more short way to install [#220](https://github.com/dotenv-linter/dotenv-linter/pull/220) ([@DDtKey](https://github.com/DDtKey))
- Add flag to enable recursive search for `.env` files [#223](https://github.com/dotenv-linter/dotenv-linter/pull/223) ([@DDtKey](https://github.com/DDtKey))
- Add docs [#226](https://github.com/dotenv-linter/dotenv-linter/pull/226) ([@wesleimp](https://github.com/wesleimp))
- Add Windows publishing to release workflow [#211](https://github.com/dotenv-linter/dotenv-linter/pull/211) ([@DDtKey](https://github.com/DDtKey))
- Add support canonicalize path for Windows [#213](https://github.com/dotenv-linter/dotenv-linter/pull/213) ([@DDtKey](https://github.com/DDtKey))
- Add build and test steps running on Windows [#216](https://github.com/dotenv-linter/dotenv-linter/pull/216) ([@mgrachev](https://github.com/mgrachev))

### 🔧 Changed
- Add the field with a checker name to Warning [#234](https://github.com/dotenv-linter/dotenv-linter/pull/234) ([@evgeniy-r](https://github.com/evgeniy-r))
- Remove `FileEntry::get_content_by_path` in favor of `fs::read_to_string` [#233](https://github.com/dotenv-linter/dotenv-linter/pull/233) ([@mstruebing](https://github.com/mstruebing))
- Move show-checks flag to main.rs [#227](https://github.com/dotenv-linter/dotenv-linter/pull/227) ([@mgrachev](https://github.com/mgrachev))
- Fix `total_lines` in some tests [#224](https://github.com/dotenv-linter/dotenv-linter/pull/224) ([@DDtKey](https://github.com/DDtKey))
- Consider blank lines in `UnorderedKey` check [#221](https://github.com/dotenv-linter/dotenv-linter/pull/221) ([@mgrachev](https://github.com/mgrachev))
- Optimize integration tests [#218](https://github.com/dotenv-linter/dotenv-linter/pull/218) ([@mgrachev](https://github.com/mgrachev))

## [v2.0.0] - 2020-06-15
### 🚀 Added
- Add check: TrailingWhitespace [#190](https://github.com/dotenv-linter/dotenv-linter/pull/190) ([@pineapplethief](https://github.com/pineapplethief))
- Add an argument to show available checks [#202](https://github.com/dotenv-linter/dotenv-linter/pull/202) ([@DDtKey](https://github.com/DDtKey))
- Add the ability to skip checks [#178](https://github.com/dotenv-linter/dotenv-linter/pull/178) ([@mgrachev](https://github.com/mgrachev))
- Add check: ExtraBlankLine [#180](https://github.com/dotenv-linter/dotenv-linter/pull/180) ([@evgeniy-r](https://github.com/evgeniy-r))
- Add check: EndingBlankLine [#170](https://github.com/dotenv-linter/dotenv-linter/pull/170) ([@evgeniy-r](https://github.com/evgeniy-r))
- Add check: Quote characters [#174](https://github.com/dotenv-linter/dotenv-linter/pull/174) ([@sourabhmarathe](https://github.com/sourabhmarathe))
- Github Actions: Add caching in the CI workflow [#163](https://github.com/dotenv-linter/dotenv-linter/pull/163) ([@evgeniy-r](https://github.com/evgeniy-r))
- Add GitHub Workflow for AUR publishing [#161](https://github.com/dotenv-linter/dotenv-linter/pull/161) ([@mstruebing](https://github.com/mstruebing))

### 🔧 Changed
- Enable checks for outside current directory [#209](https://github.com/dotenv-linter/dotenv-linter/pull/209) ([@tisorlawan](https://github.com/tisorlawan))
- Refactor `EndingBlankLineChecker`: change last line check logic (add `total_lines` to `FileEntry`) [#207](https://github.com/dotenv-linter/dotenv-linter/pull/207) ([@DDtKey](https://github.com/DDtKey))
- Fix check (for several successive blank lines): ExtraBlankLine [#208](https://github.com/dotenv-linter/dotenv-linter/pull/208) ([@evgeniy-r](https://github.com/evgeniy-r))
- Replace `PathBuf` with `FileEntry` for `LineEntry` [#203](https://github.com/dotenv-linter/dotenv-linter/pull/203) ([@mgrachev](https://github.com/mgrachev))
- Replace `&'static str` with `&'a str` for ` LeadingCharacterChecker` [#200](https://github.com/dotenv-linter/dotenv-linter/pull/200) ([@rossjones](https://github.com/rossjones))
- Replace `&'static str` with `&'a str` for `QuoteCharacterChecker` [#198](https://github.com/dotenv-linter/dotenv-linter/pull/198) ([@duncandean](https://github.com/duncandean))
- Replace `&'static str` with `&'a str` for `EndingBlankLineChecker` [#197](https://github.com/dotenv-linter/dotenv-linter/pull/197) ([@rossjones](https://github.com/rossjones))
- Replace `String` with `&'a str` for `UnorderedKeyChecker` [#196](https://github.com/dotenv-linter/dotenv-linter/pull/196) ([@k0va1](https://github.com/k0va1))
- Replace `String` with `&'a str` for `SpaceCharacterChecker` [#195](https://github.com/dotenv-linter/dotenv-linter/pull/195) ([@k0va1](https://github.com/k0va1))
- Replace `String` with `&'a str` for `LowercaseKeyChecker` [#194](https://github.com/dotenv-linter/dotenv-linter/pull/194) ([@tisorlawan](https://github.com/tisorlawan))
- Replace `&'static str` with `&'a str` for `ExtraBlankLineChecker` [#193](https://github.com/dotenv-linter/dotenv-linter/pull/193) ([@vishalsodani](https://github.com/vishalsodani))
- Replace `String` with `&'a str` for `DuplicatedKeyChecker` [#192](https://github.com/dotenv-linter/dotenv-linter/pull/192) ([@iamsaquib](https://github.com/iamsaquib))
- Replace `String` with `&'a str` for `KeyWithoutValueChecker` [#177](https://github.com/dotenv-linter/dotenv-linter/pull/177) ([@mgrachev](https://github.com/mgrachev))
- Fix docker image [#160](https://github.com/dotenv-linter/dotenv-linter/pull/160) ([@mgrachev](https://github.com/mgrachev))
- Replace `&'static str` with `&'a str` for `IncorrectDelimiterChecker` [#191](https://github.com/dotenv-linter/dotenv-linter/pull/191) ([@DDtKey](https://github.com/DDtKey))

## [v1.2.0] - 2020-04-20
### 🔧 Changed
- SpaceCharacter: Show check name in the message [#149](https://github.com/dotenv-linter/dotenv-linter/pull/149) ([@pineapplethief](https://github.com/pineapplethief))
- LeadingCharacter: Show check name in message [#144](https://github.com/dotenv-linter/dotenv-linter/pull/144) ([@michaetto](https://github.com/michaetto))
- Rename `UnorderedKeys` check to singular `UnorderedKey` [#147](https://github.com/dotenv-linter/dotenv-linter/pull/147) ([@pineapplethief](https://github.com/pineapplethief))
- KeyWithoutValue: Show check name in the message [#139](https://github.com/dotenv-linter/dotenv-linter/pull/139) ([@harshu4](https://github.com/harshu4))
- LowercaseKey: Show check name in the message [#131](https://github.com/dotenv-linter/dotenv-linter/pull/131) ([@qelphybox](https://github.com/qelphybox))
- DuplicatedKey: Show check name in message [#138](https://github.com/dotenv-linter/dotenv-linter/pull/138)([@SaMuRa1ReM1X](https://github.com/SaMuRa1ReM1X))
- IncorrectDelimiter: Show check name in the message [#146](https://github.com/dotenv-linter/dotenv-linter/pull/146) ([@undef1nd](https://github.com/undef1nd))
- Replaced kcov with grcov in Github Actions [#143](https://github.com/dotenv-linter/dotenv-linter/pull/143) ([@pmk21](https://github.com/pmk21))
- Streamline CLI tests and split into smaller files [#137](https://github.com/dotenv-linter/dotenv-linter/pull/137) ([@sonro](https://github.com/sonro))
- UnorderedKey: Added check name to the message [#140](https://github.com/dotenv-linter/dotenv-linter/pull/140) ([@pmk21](https://github.com/pmk21))
- Add test coverage for CLI --exclude arguments [#135](https://github.com/dotenv-linter/dotenv-linter/pull/135) ([@sonro](https://github.com/sonro))
- Renamed check SpacesAroundEqual to SpaceCharacter [#134](https://github.com/dotenv-linter/dotenv-linter/pull/134) ([@SaMuRa1ReM1X](https://github.com/SaMuRa1ReM1X))
- Rename check DuplicatedKeys to DuplicatedKey [#133](https://github.com/dotenv-linter/dotenv-linter/pull/133) ([@sonro](https://github.com/sonro))
- Minimizing Rust Binary Size [#132](https://github.com/dotenv-linter/dotenv-linter/pull/132) ([@akirill0v](https://github.com/akirill0v))
- Remove the unwrap method and use platform native OsString to fetch the information about current directory [#115](https://github.com/dotenv-linter/dotenv-linter/pull/115) ([@kanapuli](https://github.com/kanapuli))
- Use HashSet for DuplicateKeyChecker [#113](https://github.com/dotenv-linter/dotenv-linter/pull/113) ([@TamasFlorin](https://github.com/TamasFlorin))
- Use reference for the LineEntry as part of the run method for checks [#111](https://github.com/dotenv-linter/dotenv-linter/pull/111) ([@TamasFlorin](https://github.com/TamasFlorin))
- New CLI API: Ability to check multiple directories [#99](https://github.com/dotenv-linter/dotenv-linter/pull/99) ([@mgrachev](https://github.com/mgrachev))
- Add exit with the code 0 when there are no warnings [#105](https://github.com/dotenv-linter/dotenv-linter/pull/105) ([@simPod](https://github.com/simPod))
- Use `get` method to get result of item in `Vec` and use ? operator unwrap the result if it's `Some` [#108](https://github.com/dotenv-linter/dotenv-linter/pull/108) ([@boybird](https://github.com/boybird))

## [v1.1.2] - 2020-03-13
### 🔧 Changed
- Fix --path CLI parameter not canonizing filepaths from directory path passed as argument and not working as intended as a result [#97](https://github.com/dotenv-linter/dotenv-linter/pull/97) ([@pineapplethief](https://github.com/pineapplethief))
- Fix incorrect delimiter check for keys contains numeric [#95](https://github.com/dotenv-linter/dotenv-linter/pull/95) ([@alter369](https://github.com/alter369))
- Add `LineEntry.is_empty_or_comment` method to DRY and simplify `Check.run` [#94](https://github.com/dotenv-linter/dotenv-linter/pull/94) ([@pineapplethief](https://github.com/pineapplethief))
- Refactor `Github Actions` and reduce `Docker Image size` [#90](https://github.com/dotenv-linter/dotenv-linter/pull/90) ([@Macbet](https://github.com/Macbet))
- Use `Line.get_key` in all checks [#89](https://github.com/dotenv-linter/dotenv-linter/pull/89) ([@pineapplethief](https://github.com/pineapplethief))

## [v1.1.1] - 2020-02-18
### 🔧 Changed
- Show the full path to the file relative to the current directory [#85](https://github.com/dotenv-linter/dotenv-linter/pull/85) ([@mgrachev](https://github.com/mgrachev))

## [v1.1.0] - 2020-01-27
### 🚀 Added
- Add check: Unordered keys [#72](https://github.com/dotenv-linter/dotenv-linter/pull/72) ([@mgrachev](https://github.com/mgrachev))
- Add the ability to specify the directory where to run [#65](https://github.com/dotenv-linter/dotenv-linter/pull/65) ([@Louis-Lesage](https://github.com/Louis-Lesage))
- Add check: Duplicated keys [#33](https://github.com/dotenv-linter/dotenv-linter/pull/33) ([@mstruebing](https://github.com/mstruebing))
- Add exit with the code 1 on warnings found [#58](https://github.com/dotenv-linter/dotenv-linter/pull/58) ([@Louis-Lesage](https://github.com/Louis-Lesage))
- Add exclude argument to exclude specific files [#49](https://github.com/dotenv-linter/dotenv-linter/pull/49) ([@mstruebing](https://github.com/mstruebing))
- Add the ability to include files to check [#48](https://github.com/dotenv-linter/dotenv-linter/pull/48) ([@mgrachev](https://github.com/mgrachev))
- Add check: Spaces around equal sign [#35](https://github.com/dotenv-linter/dotenv-linter/pull/35) ([@artem-russkikh](https://github.com/artem-russkikh))
- Add skipping of commented or empty lines [#37](https://github.com/dotenv-linter/dotenv-linter/pull/37) ([@mstruebing](https://github.com/mstruebing))

### 🔧 Changed
- Rename `leading_space` to `leading_character` and check for allowed chars [#63](https://github.com/dotenv-linter/dotenv-linter/pull/63) ([@mstruebing](https://github.com/mstruebing))
- Remove multiple checks of the same file [#62](https://github.com/dotenv-linter/dotenv-linter/pull/62) ([@mstruebing](https://github.com/mstruebing))
- Add mutability support for checks [#52](https://github.com/dotenv-linter/dotenv-linter/pull/52) ([@mgrachev](https://github.com/mgrachev))

## [v1.0.0] - 2020-01-01
### 🚀 Added
- Add check: Keys without values [#22](https://github.com/dotenv-linter/dotenv-linter/pull/22) ([@mstruebing](https://github.com/mstruebing))
- Add check: Lowercase keys [#21](https://github.com/dotenv-linter/dotenv-linter/pull/21) ([@qelphybox](https://github.com/qelphybox))
- Add check: Incorrect delimiter [#20](https://github.com/dotenv-linter/dotenv-linter/pull/20) ([@sonro](https://github.com/sonro))
- Add `Display` trait for `LineEntry` [#19](https://github.com/dotenv-linter/dotenv-linter/pull/19) ([@mstruebing](https://github.com/mstruebing))

### 🔧 Changed
- Add support for command line arguments [#31](https://github.com/dotenv-linter/dotenv-linter/pull/31) ([@mgrachev](https://github.com/mgrachev))
- Replace field warning with template for all check structs [#26](https://github.com/dotenv-linter/dotenv-linter/pull/26) ([@mgrachev](https://github.com/mgrachev))
- Prepare a template for easy adding new checks [#14](https://github.com/dotenv-linter/dotenv-linter/pull/14) ([@mgrachev](https://github.com/mgrachev))

[v2.1.0]: https://github.com/dotenv-linter/dotenv-linter/releases/tag/v2.1.0
[v2.0.0]: https://github.com/dotenv-linter/dotenv-linter/releases/tag/v2.0.0
[v1.2.0]: https://github.com/dotenv-linter/dotenv-linter/releases/tag/v1.2.0
[v1.1.2]: https://github.com/dotenv-linter/dotenv-linter/releases/tag/v1.1.2
[v1.1.1]: https://github.com/dotenv-linter/dotenv-linter/releases/tag/v1.1.1
[v1.1.0]: https://github.com/dotenv-linter/dotenv-linter/releases/tag/v1.1.0
[v1.0.0]: https://github.com/dotenv-linter/dotenv-linter/releases/tag/v1.0.0
