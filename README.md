# ✌️dotenv-linter

![](https://github.com/mgrachev/dotenv-linter/workflows/CI/badge.svg)
![](https://img.shields.io/github/license/mgrachev/dotenv-linter)
![](https://img.shields.io/github/v/release/mgrachev/dotenv-linter)
[![](https://codecov.io/gh/mgrachev/dotenv-linter/branch/master/graph/badge.svg)](https://codecov.io/gh/mgrachev/dotenv-linter)

> Linter for `.env` files. Written in Rust 🦀

## 👨‍💻 Installation

### Binary

```bash
# Linux
$ curl https://github.com/mgrachev/dotenv-linter/releases/download/v1.0.0/dotenv-linter-v1.0.0-linux-x86_64.tar.gz -sSfL | tar -xzf - 

# Alpine Linux
$ wget https://github.com/mgrachev/dotenv-linter/releases/download/v1.0.0/dotenv-linter-v1.0.0-alpine-x86_64.tar.gz -O - -q | tar -xzf -

# macOS
$ curl https://github.com/mgrachev/dotenv-linter/releases/download/v1.0.0/dotenv-linter-v1.0.0-darwin-x86_64.tar.gz -sSfL | tar -xzf -
```

### GitHub Action

Use [action-dotenv-linter](https://github.com/mgrachev/action-dotenv-linter) to run `dotenv-linter`:

```yml
# .github/workflows/dotenv_linter.yml
name: reviewdog
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

### Docker

```bash
$ docker run --rm -v `pwd`:/app -w /app mgrachev/dotenv-linter
```

## 🚀 Usage

By default, `dotenv-linter` checks all files with the `.env` prefix. For example: `.env`, `.env.test`, `.env.qa`:

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

If you want to exclude a file with a specific name from check,
you can use the argument `--exclude FILE_NAME` or its short version `-e FILE_NAME`:

```bash
$ dotenv-linter -e .env --exclude .env.test
```

If you want to specify the directory where to run dotenv-linter,
you can use the argument `--path DIRECTORY_PATH` or its short version `-p DIRECTORY_PATH`:

```bash
$ dotenv-linter -p /directory/where/to/run
```

## ✅ Checks

### Duplicated Keys

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

### Keys without values

Detects if a line has a key without a value:
```env
❌Wrong
FOO

✅Correct
FOO=

✅Correct
FOO=BAR
```

### Leading space

Detects if a line starts with a space or a tab character:

```env
❌Wrong
 FOO=BAR

✅Correct
FOO=BAR
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

### Spaces around equal sign

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

### Unordered Keys

Detects if a key is not alphabetically ordered:

```env
❌Wrong
FOO=BAR
BAR=FOO

✅Correct
BAR=FOO
FOO=BAR
```

## 📋 Roadmap

- [x] Add more checks:
  - [x] Duplicated keys;
  - [x] Incorrect delimiter;
  - [x] Keys without values;
  - [x] Leading Space;
  - [x] Lowercase keys;
  - [x] Spaces around equal sign;
  - [x] Unordered keys;
- [x] Support [reviewdog](https://github.com/reviewdog/reviewdog);
- [x] Create a GitHub Action for easily using `dotenv-linter`.

## 🤝 Contributing

If you've ever wanted to contribute to open source, now you have a great opportunity:

* [How to set up the project](/CONTRIBUTING.md#how-to-set-up-the-project)
* [How to add a new check](/CONTRIBUTING.md#how-to-add-a-new-check)

## 👍 Similar projects

* [wemake-services/dotenv-linter](https://github.com/wemake-services/dotenv-linter) (Python)

## ✨ Contributors

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tr>
    <td align="center"><a href="https://github.com/mgrachev"><img src="https://avatars0.githubusercontent.com/u/700998?v=4" width="100px;" alt=""/><br /><sub><b>Grachev Mikhail</b></sub></a><br /><a href="https://github.com/mgrachev/dotenv-linter/commits?author=mgrachev" title="Code">💻</a> <a href="https://github.com/mgrachev/dotenv-linter/commits?author=mgrachev" title="Documentation">📖</a> <a href="#ideas-mgrachev" title="Ideas, Planning, & Feedback">🤔</a> <a href="https://github.com/mgrachev/dotenv-linter/commits?author=mgrachev" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://github.com/mstruebing"><img src="https://avatars0.githubusercontent.com/u/12071529?v=4" width="100px;" alt=""/><br /><sub><b>Max Strübing</b></sub></a><br /><a href="https://github.com/mgrachev/dotenv-linter/commits?author=mstruebing" title="Code">💻</a> <a href="#example-mstruebing" title="Examples">💡</a> <a href="#ideas-mstruebing" title="Ideas, Planning, & Feedback">🤔</a> <a href="https://github.com/mgrachev/dotenv-linter/commits?author=mstruebing" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://github.com/Louis-Lesage"><img src="https://avatars2.githubusercontent.com/u/31346705?v=4" width="100px;" alt=""/><br /><sub><b>Louis Lesage</b></sub></a><br /><a href="https://github.com/mgrachev/dotenv-linter/commits?author=Louis-Lesage" title="Code">💻</a> <a href="#example-Louis-Lesage" title="Examples">💡</a></td>
    <td align="center"><a href="https://github.com/artem-russkikh"><img src="https://avatars0.githubusercontent.com/u/3540978?v=4" width="100px;" alt=""/><br /><sub><b>Artem Russkikh</b></sub></a><br /><a href="https://github.com/mgrachev/dotenv-linter/commits?author=artem-russkikh" title="Code">💻</a> <a href="#example-artem-russkikh" title="Examples">💡</a> <a href="#ideas-artem-russkikh" title="Ideas, Planning, & Feedback">🤔</a> <a href="https://github.com/mgrachev/dotenv-linter/commits?author=artem-russkikh" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://github.com/sonro"><img src="https://avatars2.githubusercontent.com/u/11620521?v=4" width="100px;" alt=""/><br /><sub><b>Christopher Morton</b></sub></a><br /><a href="https://github.com/mgrachev/dotenv-linter/commits?author=sonro" title="Code">💻</a> <a href="#example-sonro" title="Examples">💡</a> <a href="https://github.com/mgrachev/dotenv-linter/commits?author=sonro" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://github.com/qelphybox"><img src="https://avatars3.githubusercontent.com/u/8515324?v=4" width="100px;" alt=""/><br /><sub><b>Kirill Bobykin</b></sub></a><br /><a href="https://github.com/mgrachev/dotenv-linter/commits?author=qelphybox" title="Code">💻</a> <a href="#example-qelphybox" title="Examples">💡</a> <a href="#ideas-qelphybox" title="Ideas, Planning, & Feedback">🤔</a> <a href="https://github.com/mgrachev/dotenv-linter/commits?author=qelphybox" title="Tests">⚠️</a></td>
  </tr>
</table>

<!-- markdownlint-enable -->
<!-- prettier-ignore-end -->
<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!

## ♥️ Sponsor

<p>
  <a href="https://evrone.com/?utm_source=dotenv-linter">
    <img src="https://www.mgrachev.com/assets/static/evrone-sponsored-300.png"
      alt="Sponsored by Evrone" width="210">
  </a>
</p>


## 📃 License

[MIT](https://choosealicense.com/licenses/mit)
