# Contributing

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
