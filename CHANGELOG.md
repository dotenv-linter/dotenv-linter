# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### 🚀 Added

### 🔧 Changed

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
