# ✌️dotenv-linter

![](https://github.com/mgrachev/dotenv-linter/workflows/CI/badge.svg)
![](https://img.shields.io/github/license/mgrachev/dotenv-linter)
![](https://img.shields.io/github/v/release/mgrachev/dotenv-linter)

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

Use [mgrachev/action-dotenv-linter](https://github.com/mgrachev/action-dotenv-linter) to run `dotenv-linter`:

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

## 📋 Roadmap
- [ ] Add more checks:
  - [x] Duplicated keys;
  - [x] Incorrect delimiter;
  - [x] Keys without values;
  - [x] Leading Space;
  - [x] Lowercase keys;
  - [x] Spaces around equal sign;
  - [ ] [Unordered keys](https://github.com/mgrachev/dotenv-linter/issues/4);
  - [ ] Other checks.
- [x] Support [reviewdog](https://github.com/reviewdog/reviewdog);
- [x] Create a GitHub Action for easily using `dotenv-linter`.

## 🤝 Contributing

* [How to set up the project](/CONTRIBUTING.md#how-to-set-up-the-project)
* [How to add a new check](/CONTRIBUTING.md#how-to-add-a-new-check)

## 👍 Similar projects
* [wemake-services/dotenv-linter](https://github.com/wemake-services/dotenv-linter) (Python)

## ♥️ Sponsor

<p>
  <a href="https://evrone.com/?utm_source=dotenv-linter">
    <img src="https://www.mgrachev.com/assets/static/evrone-sponsored-300.png"
      alt="Sponsored by Evrone" width="210">
  </a>
</p>


## 📃 License

[MIT](https://choosealicense.com/licenses/mit)
