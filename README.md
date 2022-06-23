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

Dotenv-linter can **[check](#-check)** / **[fix](#-fix)** / **[compare](#-compare)** `.env` files for problems that may cause the application to malfunction.

**Available checks**:

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
&nbsp;&nbsp;&nbsp;&nbsp;✅&nbsp;<a href="https://dotenv-linter.github.io/#/checks/substitution_key">Substitution Key</a><br />
&nbsp;&nbsp;&nbsp;&nbsp;✅&nbsp;<a href="https://dotenv-linter.github.io/#/checks/trailing_whitespace">Trailing whitespace</a><br />
&nbsp;&nbsp;&nbsp;&nbsp;✅&nbsp;<a href="https://dotenv-linter.github.io/#/checks/unordered_key">Unordered Key</a><br />
</p>

**What is a `.env` file?**

<p>
&nbsp;&nbsp;&nbsp;&nbsp;💡&nbsp;A <code>.env</code> file or <code>dotenv</code> file is a simple text file containing all the environment variables of a project.<br /> &nbsp;&nbsp;&nbsp;&nbsp;Storing <a href="https://12factor.net/config">configuration in the environment variables</a> is one of the tenets of the <a href="https://12factor.net">Manifesto of Twelve-Factor App</a>.<br />
&nbsp;&nbsp;&nbsp;&nbsp;The <code>.env</code> file has a simple key-value format, for example: <code>FOO=BAR</code>.<br />
&nbsp;&nbsp;&nbsp;&nbsp;More information you can find in articles in <a href="https://evrone.com/dotenv-linter?utm_source=github&utm_campaign=dotenv-linter">English</a> and <a href="https://www.mgrachev.com/2020/04/20/dotenv-linter">Russian</a>.
</p>

**The key features**:

<p>
&nbsp;&nbsp;&nbsp;&nbsp;⚡️&nbsp;Lightning-fast because it is written in Rust 🦀<br />
&nbsp;&nbsp;&nbsp;&nbsp;💣&nbsp;Can be used on any project regardless of the programming language 💥<br />
&nbsp;&nbsp;&nbsp;&nbsp;🚀&nbsp;Can be integrated with <a href="https://github.com/reviewdog/reviewdog">reviewdog</a> and other CI services (including <a href="https://github.com/dotenv-linter/action-dotenv-linter">GitHub Actions</a> and <a href="https://github.com/github/super-linter">Super-Linter</a>) 🔥
</p>

**Articles about dotenv-linter**:

- [EN] [Dotenv-linter: looking after the environment for you](https://evrone.com/dotenv-linter?utm_source=github&utm_campaign=dotenv-linter)
- [EN] [What's new in dotenv-linter v2.2.0?](https://evrone.com/dotenv-linter-v220?utm_source=github&utm_campaign=dotenv-linter)
- [EN] [What are the key changes in dotenv-linter v3.0.0 release?](https://evrone.com/dotenv-linter-v300?utm_source=github&utm_campaign=dotenv-linter)
- [RU] [Dotenv-linter: линтер .env файлов](https://www.mgrachev.com/2020/04/20/dotenv-linter)
- [RU] [Что нового в dotenv-linter v2.2.1?](https://evrone.ru/dotenv-linter-v220?utm_source=github&utm_campaign=dotenv-linter)
- [RU] [Что нового в dotenv-linter v3.0.0?](https://evrone.ru/dotenv-linter-v300?utm_source=github&utm_campaign=dotenv-linter)

[Dotenv-linter](https://evrone.com/dotenv-linter?utm_source=github&utm_campaign=dotenv-linter) is created & supported by [Evrone](https://evrone.com/?utm_source=github&utm_campaign=dotenv-linter). What else we develop with [Rust](https://evrone.com/rust?utm_source=github&utm_campaign=dotenv-linter).

## 👨‍💻 Installation

### Pre-compiled binary

```shell script
# Linux / macOS / Windows (MINGW and etc). Installs it into ./bin/ by default
$ curl -sSfL https://raw.githubusercontent.com/dotenv-linter/dotenv-linter/master/install.sh | sh -s

# Or a shorter way
$ curl -sSfL https://git.io/JLbXn | sh -s

# Specify installation directory and version
$ curl -sSfL https://git.io/JLbXn | sh -s -- -b usr/local/bin v2.0.0

# Alpine Linux (using wget)
$ wget -q -O - https://git.io/JLbXn | sh -s
```

You can find other installation methods here: https://dotenv-linter.github.io/#/installation

## 🚀 Usage

#### ✅ Check

By default, `dotenv-linter` checks all `.env` files in the current directory:

```shell
$ dotenv-linter
Checking .env
.env:2 DuplicatedKey: The FOO key is duplicated
.env:3 UnorderedKey: The BAR key should go before the FOO key

Checking .env.test
.env.test:1 LeadingCharacter: Invalid leading character detected

Found 3 problems
```

#### 🛠 Fix

It can also fix the found warnings with the `fix` command:

```shell
$ dotenv-linter fix
Fixing .env
Original file was backed up to: ".env_1601378896"

.env:2 DuplicatedKey: The BAR key is duplicated
.env:3 LowercaseKey: The foo key should be in uppercase

All warnings are fixed. Total: 2
```

#### 🤲 Compare

In addition, `dotenv-linter` can compare `.env` files with each other and output the difference between them:

```shell
$ dotenv-linter compare .env .env.example
Comparing .env
Comparing .env.example
.env is missing keys: BAR
.env.example is missing keys: FOO
```

Other use cases you can find on the documentation site (https://dotenv-linter.github.io):

- [Check](https://dotenv-linter.github.io/#/usage/check)
- [Fix](https://dotenv-linter.github.io/#/usage/fix)
- [Compare](https://dotenv-linter.github.io/#/usage/compare)

## 🚦 Continuous Integration

`dotenv-linter` can also be used with CI services such as: [GitHub Actions](https://dotenv-linter.github.io/#/integrations/github_actions) and [Circle CI](https://dotenv-linter.github.io/#/integrations/circleci).

## 🚧 Benchmark

Benchmarking [dotenv-linter/dotenv-linter](https://github.com/dotenv-linter/dotenv-linter) and [wemake-services/dotenv-linter](https://github.com/wemake-services/dotenv-linter) has done using the [hyperfine](https://github.com/sharkdp/hyperfine) utility:

| Command                              |    Mean [ms] | Min [ms] | Max [ms] |      Relative |
| :----------------------------------- | -----------: | -------: | -------: | ------------: |
| `dotenv-linter/dotenv-linter .env`   |    2.7 ± 0.4 |      2.0 |      4.3 |          1.00 |
| `wemake-services/dotenv-linter .env` | 162.6 ± 12.1 |    153.0 |    201.3 | 60.83 ± 10.20 |

<details>
<summary>Content of <code>.env</code> file used for benchmarking</summary>

```dotenv
 SPACED=

KEY = VALUE

SECRET="my value"

SECRET=Already defined

kebab-case-name=1
snake_case_name=2
```

</details>

## ✌️ Mentorship

`Dotenv-linter` is not just a linter for `.env` files — it is also a **contributor-friendly open-source project** with the purpose of helping others learn Rust using a simple, but useful tool. 😊

In addition to studying Rust, this project has another goal — to **promote love for open-source**, help you with the first steps in it and give an opportunity to contribute to the open-source project written in Rust. ❤️

We act [as a mentor](https://rustbeginners.github.io/awesome-rust-mentors) within this project and **help developers** follow the path of a novice contributor from start to the top. 🤗

## 🤝 Contributing

If you've ever wanted to contribute to open source, now you have a great opportunity:

- [How to set up the project](/CONTRIBUTING.md#how-to-set-up-the-project)
- [How to add a new check](/CONTRIBUTING.md#how-to-add-a-new-check)

## 👍 Similar projects

- [wemake-services/dotenv-linter](https://github.com/wemake-services/dotenv-linter) (Python)

## ✨ Contributors

This project exists thanks to all the people who contribute. [[Contribute](/CONTRIBUTING.md)].

<a href="https://github.com/dotenv-linter/dotenv-linter/graphs/contributors">
  <img src="https://opencollective.com/dotenv-linter/contributors.svg?width=890&button=false" />
</a>

## ♥️ Sponsors

<p>
  <a href="https://evrone.com/?utm_source=github&utm_campaign=dotenv-linter">
    <img src="https://www.mgrachev.com/assets/static/sponsored_by_evrone.svg?sanitize=true"
      alt="Sponsored by Evrone">
  </a>
</p>

Become a financial contributor and help us sustain our community.

<a href="https://opencollective.com/dotenv-linter"><img src="https://opencollective.com/dotenv-linter/individuals.svg?width=890"></a>

## 📃 License

[MIT](https://choosealicense.com/licenses/mit)
