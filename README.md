<p align="center">
  <a href="https://github.com/dotenv-linter/dotenv-linter">
    <img alt="dotenv-linter"
         width="350" height="192"
         src="https://raw.githubusercontent.com/dotenv-linter/dotenv-linter/master/logo.svg?sanitize=true">
  </a>
</p>

<h2 align="center">
⚡️Lightning-fast linter for <code>.env</code> files. Written in Rust 🦀
</h2>

<p align="center">
  <a href="https://github.com/dotenv-linter/dotenv-linter/actions">
    <img alt="GitHub Actions" src="https://github.com/dotenv-linter/dotenv-linter/workflows/CI/badge.svg">
  </a>
  <a href="https://codecov.io/gh/dotenv-linter/dotenv-linter">
    <img alt="Coverage Status" src="https://codecov.io/gh/dotenv-linter/dotenv-linter/branch/master/graph/badge.svg">
  </a>
  <a href="https://github.com/dotenv-linter/dotenv-linter/blob/master/LICENSE">
    <img alt="License" src="https://img.shields.io/github/license/dotenv-linter/dotenv-linter">
  </a>
  <a href="https://github.com/dotenv-linter/dotenv-linter/releases">
    <img alt="Releases" src="https://img.shields.io/github/release/dotenv-linter/dotenv-linter">
  </a>
</p>

It checks `.env` files for problems that may cause the application to malfunction:
<p>
&nbsp;&nbsp;&nbsp;&nbsp;✅&nbsp;<a href="#duplicated-key">Duplicated Key</a><br />
&nbsp;&nbsp;&nbsp;&nbsp;✅&nbsp;<a href="#ending-blank-line">Ending Blank Line</a><br />
&nbsp;&nbsp;&nbsp;&nbsp;✅&nbsp;<a href="#extra-blank-line">Extra Blank Line</a><br />
&nbsp;&nbsp;&nbsp;&nbsp;✅&nbsp;<a href="#incorrect-delimiter">Incorrect delimiter</a><br />
&nbsp;&nbsp;&nbsp;&nbsp;✅&nbsp;<a href="#key-without-value">Key without value</a><br />
&nbsp;&nbsp;&nbsp;&nbsp;✅&nbsp;<a href="#leading-character">Leading character</a><br />
&nbsp;&nbsp;&nbsp;&nbsp;✅&nbsp;<a href="#lowercase-key">Lowercase key</a><br />
&nbsp;&nbsp;&nbsp;&nbsp;✅&nbsp;<a href="#quote-character">Quote character</a><br />
&nbsp;&nbsp;&nbsp;&nbsp;✅&nbsp;<a href="#space-character">Space character</a><br />
&nbsp;&nbsp;&nbsp;&nbsp;✅&nbsp;<a href="#trailing-whitespace">Trailing whitespace</a><br />
&nbsp;&nbsp;&nbsp;&nbsp;✅&nbsp;<a href="#unordered-Key">Unordered Key</a><br />
</p>

The key features:
<p>
&nbsp;&nbsp;&nbsp;&nbsp;⚡️&nbsp;Lightning-fast because it is written in Rust 🦀<br />
&nbsp;&nbsp;&nbsp;&nbsp;💣&nbsp;Can be used on any project regardless of the programming language 💥<br />
&nbsp;&nbsp;&nbsp;&nbsp;🚀&nbsp;Can be integrated with <a href="https://github.com/reviewdog/reviewdog">reviewdog</a> and other CI services (including <a href="https://github.com/dotenv-linter/action-dotenv-linter">GitHub Actions</a>) 🔥
</p>

Articles about dotenv-linter:
* [In English](https://evrone.com/dotenv-linter?utm_source=github&utm_campaign=dotenv-linter)
* [In Russian](https://www.mgrachev.com/2020/04/20/dotenv-linter)

[Dotenv-linter](https://evrone.com/dotenv-linter?utm_source=github&utm_campaign=dotenv-linter) is created & supported by [Evrone](https://evrone.com/?utm_source=github&utm_campaign=dotenv-linter). What else we develop with [Rust](https://evrone.com/rust?utm_source=github&utm_campaign=dotenv-linter).

## 👨‍💻 Installation

### Binary

```shell script
# Linux
$ curl https://github.com/dotenv-linter/dotenv-linter/releases/download/v2.0.0/dotenv-linter-linux-x86_64.tar.gz -sSfL | tar -xzf -

# Alpine Linux
$ wget https://github.com/dotenv-linter/dotenv-linter/releases/download/v2.0.0/dotenv-linter-alpine-x86_64.tar.gz -O - -q | tar -xzf -

# macOS
$ curl https://github.com/dotenv-linter/dotenv-linter/releases/download/v2.0.0/dotenv-linter-darwin-x86_64.tar.gz -sSfL | tar -xzf -
```

### Homebrew / Linuxbrew

```shell script
$ brew install dotenv-linter/tap/dotenv-linter
```

### Arch Linux / AUR

```shell script
# use your favourite AUR-helper
$ trizen -S dotenv-linter-bin # for the binary distribution
$ trizen -S dotenv-linter-git # for the current master branch
```

### Windows / Scoop

```shell script
$ scoop bucket add dotenv-linter https://github.com/dotenv-linter/scoop.git
$ scoop install dotenv-linter/dotenv-linter
````

### Docker

```shell script
$ docker run --rm -v `pwd`:/app -w /app dotenvlinter/dotenv-linter
```

### Cargo

If you are a **Rust** programmer, you can install `dotenv-linter` via `cargo`:

```shell script
$ cargo install dotenv-linter
```

### GitHub Action

<details>
<summary>Example: <code>.github/workflows/dotenv_linter.yml</code></summary>

```yaml
name: dotenv-linter
on: [pull_request]
jobs:
  dotenv-linter:
    name: runner / dotenv-linter
    runs-on: ubuntu-latest
    steps:
      - name: Check out code
        uses: actions/checkout@v1
      - name: dotenv-linter
        uses: dotenv-linter/action-dotenv-linter@v2
        with:
          github_token: ${{ secrets.github_token }}
```

In the example above, [action-dotenv-linter](https://github.com/dotenv-linter/action-dotenv-linter) is used to run `dotenv-linter`.
</details>

### CircleCI

<details>
<summary>Example: <code>.circleci/config.yml</code></summary>

```yaml
version: 2.1
jobs:
  dotenv-linter:
    docker:
      - image: circleci/rust:latest
    steps:
      - checkout
      - run:
          name: Run dotenv-linter
          command: |
            DOTENV_LINTER_VERSION=v2.0.0
            wget https://github.com/dotenv-linter/dotenv-linter/releases/download/$DOTENV_LINTER_VERSION/dotenv-linter-alpine-x86_64.tar.gz \
            -O - -q | tar -xzf -
            ./dotenv-linter
```
</details>

## 🚀 Usage

By default, `dotenv-linter` checks all `.env` files in the current directory:

```shell script
$ dotenv-linter
.env:2 DuplicatedKey: The FOO key is duplicated
.env:3 UnorderedKey: The BAR key should go before the FOO key
.env.test:1 LeadingCharacter: Invalid leading character detected
```

To check another directory, just pass its path as an argument. The same approach works if you need to check any files individually:

```shell script
$ dotenv-linter dir1 dir2/.my-env-file
dir1/.env:1 LeadingCharacter: Invalid leading character detected
dir1/.env:3 IncorrectDelimiter: The FOO-BAR key has incorrect delimiter
dir2/.my-env-file:1 LowercaseKey: The bar key should be in uppercase
```

If you need to exclude a file from check, you can use the argument `--exclude FILE_PATH` or its short version `-e FILE_PATH`:

```shell script
$ dotenv-linter --exclude .env.test
.env:2 DuplicatedKey: The FOO key is duplicated
.env:3 UnorderedKey: The BAR key should go before the FOO key
```

If you need to skip some checks, you can use the argument `--skip CHECK_NAME` or its short version `-s CHECK_NAME`:

```shell script
$ dotenv-linter --skip UnorderedKey EndingBlankLine
.env:2 DuplicatedKey: The FOO key is duplicated
```

If you need to view all available checks, you can use the argument `--show-checks`:

```shell script
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

## ✅ Checks

### Duplicated Key

Detects if a key is not unique:

```env
❌ Wrong
FOO=BAR
FOO=BAR

✅ Correct
FOO=BAR
BAR=FOO
```

### Ending Blank Line

Detects if a file doesn't have a blank line at the end:

```env
❌ Wrong
FOO=BAR
```

```env
✅ Correct
FOO=BAR

```

### Extra Blank Line

Detects if a file contains more than one blank line in a row:

```env
❌ Wrong
A=B


FOO=BAR
```

```env
❌ Wrong
A=B
FOO=BAR


```

```env
✅ Correct
A=B

FOO=BAR

```

```env
✅ Correct
A=B
FOO=BAR

```

### Incorrect delimiter

Detects if a key does not use an underscore to separate words:
```env
❌ Wrong
FOO-BAR=FOOBAR

✅ Correct
FOO_BAR=FOOBAR
```

### Key without value

Detects if a line has a key without a value:
```env
❌ Wrong
FOO

✅ Correct
FOO=

✅ Correct
FOO=BAR
```

### Leading character

Detects if a line starts with an unallowed character (characters from `A` to `Z` and `_` (underscore) are allowed):

```env
❌ Wrong
 FOO=BAR

❌ Wrong
.FOO=BAR

❌ Wrong
*FOO=BAR

❌ Wrong
1FOO=BAR

✅ Correct
FOO=BAR

✅ Correct
_FOO=BAR
```

### Lowercase key

Detects if a key has lowercase characters:

```env
❌ Wrong
FOo_BAR=FOOBAR

❌ Wrong
foo_bar=FOOBAR

✅ Correct
FOO_BAR=FOOBAR
```

### Quote character

Detects if a value is wrapped in quotes:

```env
❌ Wrong
FOO="BAR"

❌ Wrong
FOO='BAR'

✅ Correct
FOO=BAR
```

### Space character

Detects lines with a whitespace around equal sign character `=`:

```env
❌ Wrong
FOO =BAR

❌ Wrong
FOO= BAR

❌ Wrong
FOO = BAR

✅ Correct
FOO=BAR
```

### Trailing whitespace

Detects if a line has a trailing whitespace.

### Unordered Key

Detects if a key is not alphabetically ordered:

```env
❌ Wrong
FOO=BAR
BAR=FOO

✅ Correct
BAR=FOO
FOO=BAR
```

## 🤝 Contributing

If you've ever wanted to contribute to open source, now you have a great opportunity:

* [How to set up the project](/CONTRIBUTING.md#how-to-set-up-the-project)
* [How to add a new check](/CONTRIBUTING.md#how-to-add-a-new-check)

## 👍 Similar projects

* [wemake-services/dotenv-linter](https://github.com/wemake-services/dotenv-linter) (Python)

## ✨ Contributors

This project exists thanks to all the people who contribute. [[Contribute](CONTRIBUTING.md)].
<a href="https://github.com/dotenv-linter/dotenv-linter/graphs/contributors"><img src="https://opencollective.com/dotenv-linter/contributors.svg?width=890&button=false" /></a>

## ♥️ Sponsors

<p>
  <a href="https://evrone.com/?utm_source=github&utm_campaign=dotenv-linter">
    <img src="https://www.mgrachev.com/assets/static/evrone-sponsored-300.png"
      alt="Sponsored by Evrone" width="210">
  </a>
</p>

Become a financial contributor and help us sustain our community.

<a href="https://opencollective.com/dotenv-linter"><img src="https://opencollective.com/dotenv-linter/individuals.svg?width=890"></a>

## 📃 License

[MIT](https://choosealicense.com/licenses/mit)
