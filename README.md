# [WIP] dotenv-linter

Linter for files with prefix `.env`. For example: `.env`, `.env.test`, `.env.docker`.

## Installation

`$ cargo install dotenv-linter`

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

## Plans
- [ ] Add more checks:
  - [x] Leading Space
  - [ ] [Unordered keys](https://github.com/mgrachev/dotenv-linter/issues/4);
  - [ ] [Duplicated keys](https://github.com/mgrachev/dotenv-linter/issues/5);
  - [ ] [Lowercase keys](https://github.com/mgrachev/dotenv-linter/issues/6);
  - [ ] [Keys without values](https://github.com/mgrachev/dotenv-linter/issues/7);
  - [ ] [Incorrect delimiter](https://github.com/mgrachev/dotenv-linter/issues/8);
  - [ ] [Spaces before or after the character `=`](https://github.com/mgrachev/dotenv-linter/issues/9);
  - [ ] Other checks.
- [ ] Support [reviewdog](https://github.com/reviewdog/reviewdog);
- [ ] Create a GitHub Action for easily using `dotenv-linter`.

## Similar projects
* [wemake-services/dotenv-linter](https://github.com/wemake-services/dotenv-linter) (Python)

## Sponsor

<p>
  <a href="https://evrone.com/?utm_source=action-rubocop">
    <img src="https://solovev.one/static/evrone-sponsored-300.png" 
      alt="Sponsored by Evrone" width="210">
  </a>
</p>
