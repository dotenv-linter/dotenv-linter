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

## Roadmap
- [ ] Add more checks:
  - [x] Leading Space;
  - [x] Keys without values;
  - [x] Incorrect delimiter;
  - [ ] [Unordered keys](https://github.com/mgrachev/dotenv-linter/issues/4);
  - [ ] [Duplicated keys](https://github.com/mgrachev/dotenv-linter/issues/5);
  - [ ] [Lowercase keys](https://github.com/mgrachev/dotenv-linter/issues/6);
  - [ ] [Spaces before or after the character `=`](https://github.com/mgrachev/dotenv-linter/issues/9);
  - [ ] Other checks.
- [ ] Support [reviewdog](https://github.com/reviewdog/reviewdog);
- [ ] Create a GitHub Action for easily using `dotenv-linter`.

## How to add a new check
1. Create a new file in the `src/checks` directory. The file name should contain the name of the check, for example: `src/checks/example.rs`
2. Add a new struct for this check, for example:

```rust
pub(crate) struct ExampleChecker {
    template: String,
}
```

3. Implement 2 methods for this struct: `default` and `run`, for example:

```rust
impl Default for ExampleChecker {
    fn default() -> Self {
        Self {
            template: String::from("Example detected"),
        }
    }
}

impl Check for ExampleChecker {
    fn run(&self, line: &LineEntry) -> Option<Warning> {
        // Write your check logic here...
        if line.raw_string.starts_with("EXAMPLE") {
            Some(Warning::new(self.template.clone()))
        } else {
            None
        }
    }
}
```

4. Write tests for this check, for example:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_checker_run() {
        let checker = ExampleChecker::default();
        let line = &LineEntry {
            number: 1,
            raw_string: String::from("DEBUG_HTTP=true"),
        };
        assert_eq!(None, checker.run(line));

        let expected = Some(Warning::from("Example detected"));
        let line = &LineEntry {
            number: 1,
            raw_string: String::from("EXAMPLE=true"),
        };
        assert_eq!(expected, checker.run(line));
    }
}
```

5. Add a new check to the file `src/checks.rs`, for example:

```rust
mod example;
//...
fn checklist() -> Vec<Box<dyn Check>> {
    vec![
        Box::new(leading_space::LeadingSpaceChecker::default()),
        Box::new(example::ExampleChecker::default()),
    ]
}
```

6. That's all! You are awesome! ❤️

## Similar projects
* [wemake-services/dotenv-linter](https://github.com/wemake-services/dotenv-linter) (Python)

## Sponsor

<p>
  <a href="https://evrone.com/?utm_source=dotenv-linter">
    <img src="https://www.mgrachev.com/assets/static/evrone-sponsored-300.png" 
      alt="Sponsored by Evrone" width="210">
  </a>
</p>
