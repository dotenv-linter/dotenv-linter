<p align="center">
  <a href="https://github.com/mgrachev">
    <img alt="dotenv-linter"
         width="350" height="192"
         src="https://raw.githubusercontent.com/mgrachev/dotenv-linter/new-logo/logo.svg?sanitize=true">
  </a>
</p>

<h2 align="center">
  ⚡️Blazing-fast linter for <code>.env</code> files. Written in Rust 🦀
</h2>

<p align="center">
  <a href="https://github.com/mgrachev/dotenv-linter/actions">
    <img alt="GitHub Actions" src="https://github.com/mgrachev/dotenv-linter/workflows/CI/badge.svg">
  </a>
  <a href="https://codecov.io/gh/mgrachev/dotenv-linter">
    <img alt="Coverage Status" src="https://codecov.io/gh/mgrachev/dotenv-linter/branch/master/graph/badge.svg">
  </a>
  <a href="https://github.com/mgrachev/dotenv-linter/blob/master/LICENSE">
    <img alt="License" src="https://img.shields.io/github/license/mgrachev/dotenv-linter">
  </a>
  <a href="https://github.com/mgrachev/dotenv-linter/releases">
    <img alt="Releases" src="https://img.shields.io/github/release/mgrachev/dotenv-linter">
  </a>
</p>

It checks `.env` files for problems that may cause the application to malfunction 😱

<p>
    ✅<a href="/mgrachev/dotenv-linter#duplicated-key">Duplicated Key</a><br />
    ✅<a href="/mgrachev/dotenv-linter#incorrect-delimiter">Incorrect delimiter</a><br />
    ✅<a href="/mgrachev/dotenv-linter#key-without-value">Key without value</a><br />
    ✅<a href="/mgrachev/dotenv-linter#leading-character">Leading character</a><br />
    ✅<a href="/mgrachev/dotenv-linter#lowercase-key">Lowercase key</a><br />
    ✅<a href="/mgrachev/dotenv-linter#space-character">Space character</a><br />
    ✅<a href="/mgrachev/dotenv-linter#unordered-Key">Unordered Key</a>
</p>

* ✅[Duplicated Key](#duplicated-key)
* ✅[Incorrect delimiter](#incorrect-delimiter)
* ✅[Key without value](#key-without-value)
* ✅[Leading character](#leading-character)
* ✅[Lowercase key](#lowercase-key)
* ✅[Space character](#space-character)
* ✅[Unordered Key](#unordered-Key)

Its key features:
* It is blazing-fast because it is written in Rust 🦀
* It can be used on any project, regardless of the programming language 🔥
* It can integrate with [reviewdog](https://github.com/reviewdog/reviewdog) and other CI services (including [GitHub Actions](https://github.com/mgrachev/action-dotenv-linter)) 🚀

dotenv-linter is created & supported by [Evrone](https://evrone.com/?utm_source=github&utm_campaign=dotenv-linter). What else we develop with [Rust](https://evrone.com/rust?utm_source=github&utm_campaign=dotenv-linter).

## 👨‍💻 Installation

### Binary

```shell script
# Linux
$ curl https://github.com/mgrachev/dotenv-linter/releases/download/v1.1.2/dotenv-linter-linux-x86_64.tar.gz -sSfL | tar -xzf -

# Alpine Linux
$ wget https://github.com/mgrachev/dotenv-linter/releases/download/v1.1.2/dotenv-linter-alpine-x86_64.tar.gz -O - -q | tar -xzf -

# macOS
$ curl https://github.com/mgrachev/dotenv-linter/releases/download/v1.1.2/dotenv-linter-darwin-x86_64.tar.gz -sSfL | tar -xzf -
```

### Homebrew / Linuxbrew

```shell script
$ brew install mgrachev/tap/dotenv-linter
```

### Arch Linux / AUR

```shell script
# use your favourite AUR-helper
$ trizen -S dotenv-linter-bin # for the binary distribution
$ trizen -S dotenv-linter-git # for the current master branch
```

### Docker

```shell script
$ docker run --rm -v `pwd`:/app -w /app mgrachev/dotenv-linter
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
        uses: mgrachev/action-dotenv-linter@v1
        with:
          github_token: ${{ secrets.github_token }}
```

In the example above, [action-dotenv-linter](https://github.com/mgrachev/action-dotenv-linter) is used to run `dotenv-linter`.
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
            DOTENV_LINTER_VERSION=v1.1.2
            wget https://github.com/mgrachev/dotenv-linter/releases/download/$DOTENV_LINTER_VERSION/dotenv-linter-alpine-x86_64.tar.gz \
            -O - -q | tar -xzf -
            ./dotenv-linter
```
</details>

## 🚀 Usage

By default, `dotenv-linter` checks all files that start and end with `.env`. For example: `.env`, `test.env`, `.env.qa`:

```shell script
$ dotenv-linter
.env:1 Invalid leading character detected
.env:2 The FOO-BAR key has incorrect delimiter
.env:3 The FOo_BAR key should be in uppercase
.env:4 SpaceCharacter:The line has spaces around equal sign
test.env:5 The foo_bar key should be in uppercase
test.env:6 The FOO key should be with a value or have an equal sign
```

If you want to include a file with a specific name to check,
you can use the argument `--include FILE_NAME` or its short version `-i FILE_NAME`:

```shell script
$ dotenv-linter -i test.dotenv .my-env-file
.env:1 Invalid leading character detected
test.dotenv:2 The FOO-BAR key has incorrect delimiter
.my-env-file:3 SpaceCharacter: The line has spaces around equal sign
```

If you want to exclude a file with a specific name from check,
you can use the argument `--exclude FILE_NAME` or its short version `-e FILE_NAME`:

```shell script
$ dotenv-linter -e .env .env.test
```

If you want to specify the directory where to run dotenv-linter,
you can use the argument `--path DIRECTORY_PATH` or its short version `-p DIRECTORY_PATH`:

```shell script
$ dotenv-linter -p /directory/where/to/run
```

## ✅ Checks

### Duplicated Key

Detects if a key is not unique:

```env
❌Wrong
FOO=BAR
FOO=BAR

✅Correct
FOO=BAR
BAR=FOO
```

### Incorrect delimiter

Detects if a key does not use an underscore to separate words:
```env
❌Wrong
FOO-BAR=FOOBAR

✅Correct
FOO_BAR=FOOBAR
```

### Key without value

Detects if a line has a key without a value:
```env
❌Wrong
FOO

✅Correct
FOO=

✅Correct
FOO=BAR
```

### Leading character

Detects if a line starts with an unallowed character (characters from `A` to `Z` and `_` (underscore) are allowed):

```env
❌Wrong
 FOO=BAR

❌Wrong
.FOO=BAR

❌Wrong
*FOO=BAR

❌Wrong
1FOO=BAR

✅Correct
FOO=BAR

✅Correct
_FOO=BAR
```

### Lowercase key

Detects if a key has lowercase characters:

```env
❌Wrong
FOo_BAR=FOOBAR

❌Wrong
foo_bar=FOOBAR

✅Correct
FOO_BAR=FOOBAR
```

### Space character

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

### Unordered Key

Detects if a key is not alphabetically ordered:

```env
❌Wrong
FOO=BAR
BAR=FOO

✅Correct
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
<a href="https://github.com/mgrachev/dotenv-linter/graphs/contributors"><img src="https://opencollective.com/dotenv-linter/contributors.svg?width=890&button=false" /></a>

## ♥️ Sponsors

<p>
  <a href="https://evrone.com/?utm_source=github&utm_campaign=dotenv-linter">
    <img src="https://www.mgrachev.com/assets/static/evrone-sponsored-300.png"
      alt="Sponsored by Evrone" width="210">
  </a>
</p>

## 📃 License

[MIT](https://choosealicense.com/licenses/mit)
