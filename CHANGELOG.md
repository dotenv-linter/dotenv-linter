# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### 🚀 Added

### 🔧 Changed
- Get rid of the name field in checks and fixes structs [#432](https://github.com/dotenv-linter/dotenv-linter/pull/432) ([@mgrachev](https://github.com/mgrachev))
- Replace filter with unwrap on flatten [#434](https://github.com/dotenv-linter/dotenv-linter/pull/434) ([@Fcukit](https://github.com/Fcukit))

## [v3.1.0] - 2021-06-09
### 🚀 Added
- Add fixer: Substitution Key [#428](https://github.com/dotenv-linter/dotenv-linter/pull/428) ([@DDtKey](https://github.com/DDtKey))
- Refactor check unit tests [#421](https://github.com/dotenv-linter/dotenv-linter/pull/421) ([@mc1098](https://github.com/mc1098))
- Add missing integration tests [#420](https://github.com/dotenv-linter/dotenv-linter/pull/420) ([@gosolivs](https://github.com/gosolivs))
- Add checker: Substitution Key [#414](https://github.com/dotenv-linter/dotenv-linter/pull/414) ([@de-sh](https://github.com/de-sh))
- Print a message if the amount of checks doesn't match the amount of fixes [#415](https://github.com/dotenv-linter/dotenv-linter/pull/415) ([@marcel-baur](https://github.com/marcel-baur))
- Print a message "Nothing to compare" [#398](https://github.com/dotenv-linter/dotenv-linter/pull/398) ([@jakecorrenti](https://github.com/jakecorrenti))
- Add action-hadolint [#400](https://github.com/dotenv-linter/dotenv-linter/pull/400) ([@iovanom](https://github.com/iovanom))
- Add method to get substitution keys to LineEntry [#391](https://github.com/dotenv-linter/dotenv-linter/pull/391) ([@zotho](https://github.com/zotho))
- Added a Fix Benchmark [#399](https://github.com/dotenv-linter/dotenv-linter/pull/399) ([@a4blue](https://github.com/a4blue))
- Add benchmark for the compare function [#395](https://github.com/dotenv-linter/dotenv-linter/pull/395) ([@FrancisMurillo](https://github.com/FrancisMurillo))
- Print a message when there are no input files for fix subcommand [#394](https://github.com/dotenv-linter/dotenv-linter/pull/394) ([@mdycz](https://github.com/mdycz))
- Print a message when there are no input files [#392](https://github.com/dotenv-linter/dotenv-linter/pull/392) ([@jodli](https://github.com/jodli))
- Add a GitHub Action to compare benchmarks [#378](https://github.com/dotenv-linter/dotenv-linter/pull/378) ([@mgrachev](https://github.com/mgrachev))
- Add benchmark for the check function [#376](https://github.com/dotenv-linter/dotenv-linter/pull/376) ([@mgrachev](https://github.com/mgrachev))

### 🔧 Changed
- Refactor hard-coded mandatory fixer [#413](https://github.com/dotenv-linter/dotenv-linter/pull/413) ([@akhtariev](https://github.com/akhtariev))
- Don't fix key order for substitution variables [#406](https://github.com/dotenv-linter/dotenv-linter/pull/406) ([@stygian-coffee](https://github.com/stygian-coffee))
- Allow unordered keys for substitution variables [#401](https://github.com/dotenv-linter/dotenv-linter/pull/401) ([@Ru5ty0ne](https://github.com/Ru5ty0ne))
- Replace `String` with `Into<String>` in `LineEntry::new`  [#404](https://github.com/dotenv-linter/dotenv-linter/pull/404) ([@miDeb](https://github.com/miDeb))
- Replace String on Into<String> for all TestDir methods [#397](https://github.com/dotenv-linter/dotenv-linter/pull/397) ([@ebobrow](https://github.com/ebobrow))
- Use Rc<FileEntry> internally to reduce memory consumption [#393](https://github.com/dotenv-linter/dotenv-linter/pull/393) ([@Tom01098](https://github.com/Tom01098))
- Use [actions-rs/clippy-check](https://github.com/actions-rs/clippy-check) to run clippy [#375](https://github.com/dotenv-linter/dotenv-linter/pull/375) ([@mgrachev](https://github.com/mgrachev))
- Remove `Result` from the return type [#374](https://github.com/dotenv-linter/dotenv-linter/pull/374) ([@DDtKey](https://github.com/DDtKey))
- Add `.bak` extension to backup files and don't lint backup files [#367](https://github.com/dotenv-linter/dotenv-linter/pull/367) ([@mstruebing](https://github.com/mstruebing))
- Add `.env` explanation [#363](https://github.com/dotenv-linter/dotenv-linter/pull/363) ([@henryboisdequin](https://github.com/henryboisdequin))
- Contemplate variables containing $ character [#418](https://github.com/dotenv-linter/dotenv-linter/pull/418) ([@JuanMarchetto](https://github.com/JuanMarchetto))

## [v3.0.0] - 2021-01-11
### 🚀 Added
- Add `compare`-command [#282](https://github.com/dotenv-linter/dotenv-linter/pull/282) ([@mstruebing](https://github.com/mstruebing))
- Add colored output feature and `--no-color` flag to disable colors [#307](https://github.com/dotenv-linter/dotenv-linter/pull/307) ([@Nikhil0487](https://github.com/Nikhil0487))
- Display linted files when run [#311](https://github.com/dotenv-linter/dotenv-linter/pull/311) ([@Anthuang](https://github.com/anthuang))
- Add export prefix support [#340](https://github.com/dotenv-linter/dotenv-linter/pull/340) ([@skonik](https://github.com/skonik))
- Add colored output for compare command [#356](https://github.com/dotenv-linter/dotenv-linter/pull/356) ([@mgrachev](https://github.com/mgrachev))

### 🔧 Changed
- Refactoring to get rid of unnecessary heap allocations and some improvements in API ergonomics [#350](https://github.com/dotenv-linter/dotenv-linter/pull/350) ([@vbrandl](https://github.com/vbrandl))
- Add benchmark to README [#351](https://github.com/dotenv-linter/dotenv-linter/pull/351) ([@mgrachev](https://github.com/mgrachev))
- Fix QuoteCharacterChecker to not raise warning when quote characters are used for values with whitespaces [#349](https://github.com/dotenv-linter/dotenv-linter/pull/349) ([@sebastiantoh](https://github.com/sebastiantoh))
- Find all problems on the first run for `KeyWithoutValue` [#348](https://github.com/dotenv-linter/dotenv-linter/pull/348) ([@vbrandl](https://github.com/vbrandl))
- Add [commitlint](https://github.com/conventional-changelog/commitlint) action [#347](https://github.com/dotenv-linter/dotenv-linter/pull/347) ([@mgrachev](https://github.com/mgrachev))
- Add [cargo-deny](https://github.com/EmbarkStudios/cargo-deny) action [#346](https://github.com/dotenv-linter/dotenv-linter/pull/346) ([@mgrachev](https://github.com/mgrachev))
- Remade flags to subcommands [#342](https://github.com/dotenv-linter/dotenv-linter/pull/342) ([@mgrachev](https://github.com/mgrachev))
- Changed behavior of QuoteCharacterChecker for multiline values support [#341](https://github.com/dotenv-linter/dotenv-linter/pull/341) ([@artem-russkikh](http://github.com/artem-russkikh))
- Make an output on-the-fly [#336](https://github.com/dotenv-linter/dotenv-linter/pull/336) ([@DDtKey](https://github.com/DDtKey))

## [v2.2.1] - 2020-10-24
### 🚀 Added
- Add `.gitattributes` to improve `git diff` [#330](https://github.com/dotenv-linter/dotenv-linter/pull/330) ([@DDtKey](https://github.com/DDtKey))
- Add action-yamllint [#317](https://github.com/dotenv-linter/dotenv-linter/pull/317) ([@vk26](https://github.com/vk26))
- Add default exclude list [#324](https://github.com/dotenv-linter/dotenv-linter/pull/324) ([@ametalon](https://github.com/ametalon))

### 🔧 Changed
- Fix bug where symlinks would cycle in recursive mode [#328](https://github.com/dotenv-linter/dotenv-linter/pull/328) ([@sonro](https://github.com/sonro))
- Fix linter rechecking files if they were listed more than once [#327](https://github.com/dotenv-linter/dotenv-linter/pull/327) ([@Aashu23](https://github.com/Aashu23))
- Added docker build step to the CI pipeline [#322](https://github.com/dotenv-linter/dotenv-linter/pull/322) ([@JoeAmedeo](https://github.com/JoeAmedeo))
- Change soon deprecated `set-env` action [#320](https://github.com/dotenv-linter/dotenv-linter/pull/320) ([@marcodenisi](https://github.com/marcodenisi))
- Fix docker release [#319](https://github.com/dotenv-linter/dotenv-linter/pull/319) ([@mgrachev](https://github.com/mgrachev))

## [v2.2.0] - 2020-10-12
### 🚀 Added
- Add integration test for autofix [#309](https://github.com/dotenv-linter/dotenv-linter/pull/309) ([@evgeniy-r](https://github.com/evgeniy-r))
- Add action-misspell [#304](https://github.com/dotenv-linter/dotenv-linter/pull/304) ([@PurpleMyst](https://github.com/PurpleMyst))
- Add action-shellcheck [#303](https://github.com/dotenv-linter/dotenv-linter/pull/303) ([@amd-9](https://github.com/amd-9))
- Add fixer: UnorderedKeyFixer [#261](https://github.com/dotenv-linter/dotenv-linter/pull/261) ([@evgeniy-r](https://github.com/evgeniy-r))
- Add backup feature and `--no-backup` flag for fixers [#272](https://github.com/dotenv-linter/dotenv-linter/pull/272) ([@baile320](https://github.com/baile320))
- Add fixer: ExtraBlankLineFixer [#260](https://github.com/dotenv-linter/dotenv-linter/pull/260) ([@diggymo](https://github.com/diggymo))
- Add fixer: DuplicatedKeyFixer [#270](https://github.com/dotenv-linter/dotenv-linter/pull/270) ([@utter-step](https://github.com/utter-step))
- Add fixer: LeadingCharacter [#259](https://github.com/dotenv-linter/dotenv-linter/pull/259) ([@baile320](https://github.com/baile320))
- Add fixer: IncorrectDelimiter [#258](https://github.com/dotenv-linter/dotenv-linter/pull/258) ([@gillespiecd](https://github.com/gillespiecd))
- Add support of comments to disable checks [#239](https://github.com/dotenv-linter/dotenv-linter/pull/239) ([@mgrachev](https://github.com/mgrachev))
- Add autofix for EndingBlankLine [#263](https://github.com/dotenv-linter/dotenv-linter/pull/263) ([@baile320](https://github.com/baile320))
- Add test for two keys that only differ in case [#269](https://github.com/dotenv-linter/dotenv-linter/pull/269) ([@yanakad](https://github.com/yanakad))
- Add autofix for TrailingWhitespace [#255](https://github.com/dotenv-linter/dotenv-linter/pull/255) ([@gregcline](https://github.com/gregcline))
- Add fixer: KeyWithoutValueFixer [#254](https://github.com/dotenv-linter/dotenv-linter/pull/254) ([@unexge](https://github.com/unexge))
- Add fixer: QuoteCharacterFixer [#257](https://github.com/dotenv-linter/dotenv-linter/pull/257) ([@lensvol](https://github.com/lensvol))
- Add fixer: SpaceCharacterFixer [#253](https://github.com/dotenv-linter/dotenv-linter/pull/253) ([@DDtKey](https://github.com/DDtKey))
- Add total problems to output and `--quiet` argument [#242](https://github.com/dotenv-linter/dotenv-linter/pull/242) ([@wesleimp](https://github.com/wesleimp), [@mgrachev](https://github.com/mgrachev))
- Add autofix feature (for LowercaseKey check) [#228](https://github.com/dotenv-linter/dotenv-linter/pull/228) ([@evgeniy-r](https://github.com/evgeniy-r))
- Add installation CI test for Windows (via `install.sh`) [#235](https://github.com/dotenv-linter/dotenv-linter/pull/235) ([@DDtKey](https://github.com/DDtKey))

### 🔧 Changed
- Update docs [#315](https://github.com/dotenv-linter/dotenv-linter/pull/315) ([@mgrachev](https://github.com/mgrachev))
- Remove `CARGO_TERM_COLOR` from the shellcheck workflow [#313](https://github.com/dotenv-linter/dotenv-linter/pull/313) ([@MusiKid](https://github.com/MusiKid))
- Add `check_output` helper function for integration tests [#305](https://github.com/dotenv-linter/dotenv-linter/pull/305) ([@Anthuang](https://github.com/anthuang))
- Add an additional test for `LineEntry.get_value` [#306](https://github.com/dotenv-linter/dotenv-linter/pull/306) ([@vvkpd](https://github.com/vvkpd))
- Update args help [#299](https://github.com/dotenv-linter/dotenv-linter/pull/299) ([@mgrachev](https://github.com/mgrachev))
- Move `remove_invalid_leading_chars_test` to `tests` module [#298](https://github.com/dotenv-linter/dotenv-linter/pull/298) ([@mgrachev](https://github.com/mgrachev))
- Add command to install latest version via `Homebrew` [#297](https://github.com/dotenv-linter/dotenv-linter/pull/297) ([@mgrachev](https://github.com/mgrachev))
- Add `CARGO_TERM_COLOR` env variable to the actions [#296](https://github.com/dotenv-linter/dotenv-linter/pull/296) ([@skippednote](https://github.com/skippednote))
- Remove code related to "Fixed/Unfixed warnings" [#289](https://github.com/dotenv-linter/dotenv-linter/pull/289) ([@mgrachev](https://github.com/mgrachev))
- Refactoring integration tests [#288](https://github.com/dotenv-linter/dotenv-linter/pull/288) ([@mgrachev](https://github.com/mgrachev))
- Fix a bug with fixers spawning new warnings [#287](https://github.com/dotenv-linter/dotenv-linter/pull/287) ([@evgeniy-r](https://github.com/evgeniy-r))
- Fix a bug with `UnorderedKeyChecker` and control comments [#283](https://github.com/dotenv-linter/dotenv-linter/pull/283) ([@mgrachev](https://github.com/mgrachev))
- Change the line grouping for the `UnorderedKey` checker [#281](https://github.com/dotenv-linter/dotenv-linter/pull/281) ([@evgeniy-r](https://github.com/evgeniy-r))
- Fix a bug with `ExtraBlankLineFixer` and control comments [#279](https://github.com/dotenv-linter/dotenv-linter/pull/279) ([@mgrachev](https://github.com/mgrachev))
- Move logic for creating `LineEntry` for tests to `common` module [#280](https://github.com/dotenv-linter/dotenv-linter/pull/280) ([@mgrachev](https://github.com/mgrachev))
- Simplify UnorderedKeyChecker [#277](https://github.com/dotenv-linter/dotenv-linter/pull/277) ([@mgrachev](https://github.com/mgrachev))
- Partition fixed/unfixed warnings [#275](https://github.com/dotenv-linter/dotenv-linter/pull/275) ([@gillespiecd](https://github.com/gillespiecd))
- Add missing test for IncorrectDelimiterChecker [#273](https://github.com/dotenv-linter/dotenv-linter/pull/273) ([@mgrachev](https://github.com/mgrachev))
- Add *.env to gitignore [#271](https://github.com/dotenv-linter/dotenv-linter/pull/271) ([@baile320](https://github.com/baile320))
- Actions uses cache@v2 [#262](https://github.com/dotenv-linter/dotenv-linter/pull/262) ([@gillespiecd](https://github.com/gillespiecd))
- Update logic for IncorrectDelimiterCheck [#267](https://github.com/dotenv-linter/dotenv-linter/pull/267) ([@baile320](https://github.com/baile320))
- Add tests for default implementation of Fix::fix_warnings [#266](https://github.com/dotenv-linter/dotenv-linter/pull/266) ([@kilotaras](https://github.com/kilotaras))
- Modularize common.rs [#264](https://github.com/dotenv-linter/dotenv-linter/pull/264) ([@gillespiecd](https://github.com/gillespiecd))

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

[v3.1.0]: https://github.com/dotenv-linter/dotenv-linter/releases/tag/v3.1.0
[v3.0.0]: https://github.com/dotenv-linter/dotenv-linter/releases/tag/v3.0.0
[v2.2.1]: https://github.com/dotenv-linter/dotenv-linter/releases/tag/v2.2.1
[v2.2.0]: https://github.com/dotenv-linter/dotenv-linter/releases/tag/v2.2.0
[v2.1.0]: https://github.com/dotenv-linter/dotenv-linter/releases/tag/v2.1.0
[v2.0.0]: https://github.com/dotenv-linter/dotenv-linter/releases/tag/v2.0.0
[v1.2.0]: https://github.com/dotenv-linter/dotenv-linter/releases/tag/v1.2.0
[v1.1.2]: https://github.com/dotenv-linter/dotenv-linter/releases/tag/v1.1.2
[v1.1.1]: https://github.com/dotenv-linter/dotenv-linter/releases/tag/v1.1.1
[v1.1.0]: https://github.com/dotenv-linter/dotenv-linter/releases/tag/v1.1.0
[v1.0.0]: https://github.com/dotenv-linter/dotenv-linter/releases/tag/v1.0.0
