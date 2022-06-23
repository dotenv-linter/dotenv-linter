# 🤝 Contributing

## How to set up the project

### 🦀 Install Rust via [rustup](https://rustup.rs)

```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 🚀 Install [clippy](https://github.com/rust-lang/rust-clippy) and [rustfmt](https://github.com/rust-lang/rustfmt)

```bash
$ rustup component add clippy
$ rustup component add rustfmt
```

### 🖖 Useful commands

```bash
# Run the project
$ cargo run

# Run tests
$ cargo test

# Run linters
$ cargo clippy

# Run rustfmt
$ cargo fmt
```

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
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        // Write your check logic here...
        if line.raw_string.starts_with("EXAMPLE") {
            Some(Warning::new(line.clone(), self.template.clone()))
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
    use std::path::PathBuf;

    #[test]
    fn working_run() {
        let mut checker = ExampleChecker::default();
        let line = &LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("FOO=BAR"),
        };
        assert_eq!(None, checker.run(line));
    }

    #[test]
    fn failing_run() {
        let mut checker = ExampleChecker::default();
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("EXAMPLE=true"),
        };
        let expected = Some(Warning::new(line.clone(), String::from("Example detected")));
        assert_eq!(expected, checker.run(&line));
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
