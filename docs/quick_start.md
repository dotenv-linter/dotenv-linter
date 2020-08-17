## 🚀 Quick Start

By default, `dotenv-linter` checks all `.env` files in the current directory:

```sh
$ dotenv-linter
.env:2 DuplicatedKey: The FOO key is duplicated
.env:3 UnorderedKey: The BAR key should go before the FOO key
.env.test:1 LeadingCharacter: Invalid leading character detected

Found 3 problems
```

To check another directory, just pass its path as an argument. The same approach works if you need to check any files individually:

```sh
$ dotenv-linter dir1 dir2/.my-env-file
dir1/.env:1 LeadingCharacter: Invalid leading character detected
dir1/.env:3 IncorrectDelimiter: The FOO-BAR key has incorrect delimiter
dir2/.my-env-file:1 LowercaseKey: The bar key should be in uppercase

Found 3 problems
```

If you need to exclude a file from check, you can use the argument `--exclude FILE_PATH` or its short version `-e FILE_PATH`:

```sh
$ dotenv-linter --exclude .env.test
.env:2 DuplicatedKey: The FOO key is duplicated
.env:3 UnorderedKey: The BAR key should go before the FOO key

Found 2 problems
```

If you need a recursive search inside directories (deeper than 1 level), you can use the flag `--recursive` or its short version `-r`:

```shell script
$ dotenv-linter --recursive
dir1/.env:2 DuplicatedKey: The FOO key is duplicated
dir2/subdir/.env:3 IncorrectDelimiter: The FOO-BAR key has incorrect delimiter

Found 2 problems
```

If you need to skip some checks, you can use the argument `--skip CHECK_NAME` or its short version `-s CHECK_NAME`:

```sh
$ dotenv-linter --skip UnorderedKey EndingBlankLine
.env:2 DuplicatedKey: The FOO key is duplicated

Found 1 problem
```

If you want to see only warnings without additional information, use the argument `--quiet` or its short version `-q` (will be available in [v2.2.0](https://github.com/dotenv-linter/dotenv-linter/issues/238)):

```shell script
$ dotenv-linter --quiet
.env:2 DuplicatedKey: The FOO key is duplicated
.env:3 UnorderedKey: The BAR key should go before the FOO key
.env.test:1 LeadingCharacter: Invalid leading character detected
```

If you need to view all available checks, you can use the argument `--show-checks`:

```sh
$ dotenv-linter --show-checks
DuplicatedKey
EndingBlankLine
ExtraBlankLine
IncorrectDelimiter
KeyWithoutValue
LeadingCharacter
LowercaseKey
QuoteCharacter
SpaceCharacter
TrailingWhitespace
UnorderedKey
```

`dotenv-linter` can also automatically fix warnings in the files. You should use the argument `--fix` (or its short version `-f`) for this (will be available in [v2.2.0](https://github.com/dotenv-linter/dotenv-linter/issues/238)):

```shell script
$ dotenv-linter -f
Fixed warnings:
.env:3 LowercaseKey: The foo key should be in uppercase

Unfixed warnings:
.env:2 DuplicatedKey: The BAR key is duplicated
```
