# ✌️dotenv-linter

![](https://github.com/mgrachev/dotenv-linter/workflows/CI/badge.svg)
![](https://img.shields.io/github/license/mgrachev/dotenv-linter)
![](https://img.shields.io/github/v/release/mgrachev/dotenv-linter)

Linter for files with prefix `.env`. For example: `.env`, `.env.test`, `.env.docker`.

## Installation

### Binary

```bash
# Linux
$ curl https://github.com/mgrachev/dotenv-linter/releases/download/v1.0.0/dotenv-linter-v1.0.0-linux-x86_64.tar.gz -sSfL | tar -xzf - 

# Alpine Linux
$ wget https://github.com/mgrachev/dotenv-linter/releases/download/v1.0.0/dotenv-linter-v1.0.0-alpine-x86_64.tar.gz -O - -q | tar -xzf -

# macOS
$ curl https://github.com/mgrachev/dotenv-linter/releases/download/v1.0.0/dotenv-linter-v1.0.0-darwin-x86_64.tar.gz -sSfL | tar -xzf -
```

## Usage

By default, `dotenv-linter` checks all files with the `.env` prefix:

```bash
$ dotenv-linter
.env:1 Leading space detected
.env:2 The FOO-BAR key has incorrect delimiter
.env:3 The FOo_BAR key should be in uppercase
.env:4 The line has spaces around equal sign
.env.test:5 The foo_bar key should be in uppercase
.env.test:6 The FOO key should be with a value or have an equal sign
```

If you want to include a file with a specific name to check,
you can use the argument `--include FILE_NAME` or its short version `-i FILE_NAME`:

```bash
$ dotenv-linter -i test.env --include .my-env-file
.env:1 Leading space detected
test.env:2 The FOO-BAR key has incorrect delimiter
.my-env-file:3 The line has spaces around equal sign
```

## Checks

### ⭐️ Leading space

Detects if a line starts with a space or a tab character:

```env
❌Wrong
 FOO=BAR

✅Correct
FOO=BAR
```

### ⭐️ Keys without values

Detects if a line has a key without a value:
```env
❌Wrong
FOO

✅Correct
FOO=

✅Correct
FOO=BAR
```

### ⭐️ Incorrect delimiter

Detects if a key does not use an underscore to separate words:
```env
❌Wrong
FOO-BAR=FOOBAR

✅Correct
FOO_BAR=FOOBAR
```

### ⭐️ Lowercase key

Detects if a key has lowercase characters:

```env
❌Wrong
FOo_BAR=FOOBAR

❌Wrong
foo_bar=FOOBAR

✅Correct
FOO_BAR=FOOBAR
```

### ⭐ Spaces around equal sign

Detects lines with a whitespace around equal sign character `=`:

```env
❌Wrong
FOO =BAR

❌Wrong
FOO= BAR

❌Wrong
FOO = BAR

✅Correct
FOO=BAR
```

## Roadmap
- [ ] Add more checks:
  - [x] Leading Space;
  - [x] Keys without values;
  - [x] Incorrect delimiter;
  - [x] Lowercase keys;
  - [x] Spaces around equal sign;
  - [ ] [Unordered keys](https://github.com/mgrachev/dotenv-linter/issues/4);
  - [ ] [Duplicated keys](https://github.com/mgrachev/dotenv-linter/issues/5);
  - [ ] Other checks.
- [ ] Support [reviewdog](https://github.com/reviewdog/reviewdog);
- [ ] Create a GitHub Action for easily using `dotenv-linter`.

## Contributing

[How to add a new check](/CONTRIBUTING.md#how-to-add-a-new-check)

## Similar projects
* [wemake-services/dotenv-linter](https://github.com/wemake-services/dotenv-linter) (Python)

## Sponsor

<p>
  <a href="https://evrone.com/?utm_source=dotenv-linter">
    <img src="https://www.mgrachev.com/assets/static/evrone-sponsored-300.png"
      alt="Sponsored by Evrone" width="210">
  </a>
</p>
