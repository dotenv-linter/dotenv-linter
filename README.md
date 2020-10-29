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
&nbsp;&nbsp;&nbsp;&nbsp;✅&nbsp;<a href="https://dotenv-linter.github.io/#/checks/duplicated_key">Duplicated Key</a><br />
&nbsp;&nbsp;&nbsp;&nbsp;✅&nbsp;<a href="https://dotenv-linter.github.io/#/checks/ending_blank_line">Ending Blank Line</a><br />
&nbsp;&nbsp;&nbsp;&nbsp;✅&nbsp;<a href="https://dotenv-linter.github.io/#/checks/extra_blank_line">Extra Blank Line</a><br />
&nbsp;&nbsp;&nbsp;&nbsp;✅&nbsp;<a href="https://dotenv-linter.github.io/#/checks/incorrect_delimiter">Incorrect delimiter</a><br />
&nbsp;&nbsp;&nbsp;&nbsp;✅&nbsp;<a href="https://dotenv-linter.github.io/#/checks/key_without_value">Key without value</a><br />
&nbsp;&nbsp;&nbsp;&nbsp;✅&nbsp;<a href="https://dotenv-linter.github.io/#/checks/leading_character">Leading character</a><br />
&nbsp;&nbsp;&nbsp;&nbsp;✅&nbsp;<a href="https://dotenv-linter.github.io/#/checks/lowercase_key">Lowercase key</a><br />
&nbsp;&nbsp;&nbsp;&nbsp;✅&nbsp;<a href="https://dotenv-linter.github.io/#/checks/quote_character">Quote character</a><br />
&nbsp;&nbsp;&nbsp;&nbsp;✅&nbsp;<a href="https://dotenv-linter.github.io/#/checks/space_character">Space character</a><br />
&nbsp;&nbsp;&nbsp;&nbsp;✅&nbsp;<a href="https://dotenv-linter.github.io/#/checks/trailing_whitespace">Trailing whitespace</a><br />
&nbsp;&nbsp;&nbsp;&nbsp;✅&nbsp;<a href="https://dotenv-linter.github.io/#/checks/unordered_key">Unordered Key</a><br />
</p>

And automatically fixes them all 😱

**The key features**:
<p>
&nbsp;&nbsp;&nbsp;&nbsp;⚡️&nbsp;Lightning-fast because it is written in Rust 🦀<br />
&nbsp;&nbsp;&nbsp;&nbsp;💣&nbsp;Can be used on any project regardless of the programming language 💥<br />
&nbsp;&nbsp;&nbsp;&nbsp;🚀&nbsp;Can be integrated with <a href="https://github.com/reviewdog/reviewdog">reviewdog</a> and other CI services (including <a href="https://github.com/dotenv-linter/action-dotenv-linter">GitHub Actions</a>) 🔥
</p>

**Articles about dotenv-linter**:
* [In English](https://evrone.com/dotenv-linter?utm_source=github&utm_campaign=dotenv-linter)
* [In Russian](https://www.mgrachev.com/2020/04/20/dotenv-linter)

[Dotenv-linter](https://evrone.com/dotenv-linter?utm_source=github&utm_campaign=dotenv-linter) is created & supported by [Evrone](https://evrone.com/?utm_source=github&utm_campaign=dotenv-linter). What else we develop with [Rust](https://evrone.com/rust?utm_source=github&utm_campaign=dotenv-linter).

## 👨‍💻 Installation

### Pre-compiled binary

```shell script
# Linux / macOS / Windows (MINGW and etc). Installs it into ./bin/ by default
$ curl -sSfL https://raw.githubusercontent.com/dotenv-linter/dotenv-linter/master/install.sh | sh -s

# Specify installation directory and version
$ curl -sSfL https://raw.githubusercontent.com/dotenv-linter/dotenv-linter/master/install.sh | sh -s -- -b usr/local/bin v2.0.0

# Alpine Linux (wget)
$ wget -q -O - https://raw.githubusercontent.com/dotenv-linter/dotenv-linter/master/install.sh | sh -s
```

You can find other installation methods here: https://dotenv-linter.github.io/#/installation

## 🚀 Usage

By default, `dotenv-linter` checks all `.env` files in the current directory:

```shell script
$ dotenv-linter
Checking .env
.env:2 DuplicatedKey: The FOO key is duplicated
.env:3 UnorderedKey: The BAR key should go before the FOO key

Checking .env.test
.env.test:1 LeadingCharacter: Invalid leading character detected

Found 3 problems
```

It can also fix found warnings. You should use the argument `--fix` (or its short version `-f`) for this:

```shell script
$ dotenv-linter -f
Fixing .env
Original file was backed up to: ".env_1601378896"

.env:2 DuplicatedKey: The BAR key is duplicated
.env:3 LowercaseKey: The foo key should be in uppercase

All warnings are fixed. Total: 2
```

Other use cases you can find here: https://dotenv-linter.github.io/#/usage

## 🚦 Continuous Integration 

`dotenv-linter` can also be used with CI services such as: [GitHub Actions](https://dotenv-linter.github.io/#/ci/github_actions) and [Circle CI](https://dotenv-linter.github.io/#/ci/circleci).

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
