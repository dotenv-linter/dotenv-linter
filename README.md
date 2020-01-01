# dotenv-linter ✌️

Linter for files with prefix `.env`. For example: `.env`, `.env.test`, `.env.docker`.

## Installation

### Binary

```bash
# Linux
$ curl https://github.com/mgrachev/dotenv-linter/releases/download/v0.1.24/dotenv-linter-v0.1.24-linux-x86_64.tar.gz -sSfL | tar -xzf - 

# Alpine Linux
$ wget https://github.com/mgrachev/dotenv-linter/releases/download/v0.1.24/dotenv-linter-v0.1.24-alpine-x86_64.tar.gz -O - -q | tar -xzf -

# macOS
$ curl https://github.com/mgrachev/dotenv-linter/releases/download/v0.1.24/dotenv-linter-v0.1.24-darwin-x86_64.tar.gz -sSfL | tar -xzf -
```

## Usage

```bash
$ ./dotenv-linter
.env.test:4 Leading space detected
.env:2 Leading space detected
```

## Checks

### Leading Space

Detects if a line starts with a space or a tab character:
```env
# Wrong
 DEBUG_HTTP=true

# Correct
DEBUG_HTTP=true
```

### Keys Without Values

Detects if a line has a key without a value:
```env
# Wrong
RAILS_ENV

# Correct
RAILS_ENV=

# Correct
RAILS_ENV=development
```

### Incorrect delimiter

Detects if a key does not use an underscore to separate words:
```env
# Wrong
DB-NAME=testing

# Correct
DB_NAME=test
```

### Lowercase key

Detects if a key has lowercase characters:

```env
# Wrong
DEbUG_hTTP=true
debug_http=true

# Correct
DEBUG_HTTP=true
```

## Roadmap
- [ ] Add more checks:
  - [x] Leading Space;
  - [x] Keys without values;
  - [x] Incorrect delimiter;
  - [x] Lowercase keys;
  - [ ] [Unordered keys](https://github.com/mgrachev/dotenv-linter/issues/4);
  - [ ] [Duplicated keys](https://github.com/mgrachev/dotenv-linter/issues/5);
  - [ ] [Spaces before or after the character `=`](https://github.com/mgrachev/dotenv-linter/issues/9);
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
