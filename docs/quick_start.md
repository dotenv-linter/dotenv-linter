## 🚀 Quick Start

By default, `dotenv-linter` checks all `.env` files in the current directory:

```sh
$ dotenv-linter

.env:2 DuplicatedKey: The FOO key is duplicated
.env:3 UnorderedKey: The BAR key should go before the FOO key
.env.test:1 LeadingCharacter: Invalid leading character detected
```

To check another directory, just pass its path as an argument. The same approach works if you need to check any files individually:

```sh
$ dotenv-linter dir1 dir2/.my-env-file

dir1/.env:1 LeadingCharacter: Invalid leading character detected
dir1/.env:3 IncorrectDelimiter: The FOO-BAR key has incorrect delimiter
dir2/.my-env-file:1 LowercaseKey: The bar key should be in uppercase
```

If you need to exclude a file from check, you can use the argument `--exclude FILE_PATH` or its short version `-e FILE_PATH`:

```sh
$ dotenv-linter --exclude .env.test

.env:2 DuplicatedKey: The FOO key is duplicated
.env:3 UnorderedKey: The BAR key should go before the FOO key
```

If you need to skip some checks, you can use the argument `--skip CHECK_NAME` or its short version `-s CHECK_NAME`:

```sh
$ dotenv-linter --skip UnorderedKey EndingBlankLine

.env:2 DuplicatedKey: The FOO key is duplicated
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
