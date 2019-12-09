use crate::LineEntry;
use std::fmt;

mod leading_space;

#[derive(Debug, Clone)]
pub struct Warning {
    message: String,
}

impl Warning {
    fn new(msg: &str) -> Warning {
        Warning {
            message: msg.to_string(),
        }
    }
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

trait Check {
    fn run(&self, line: &LineEntry) -> Result<(), Warning>;
}

fn checklist() -> Vec<impl Check> {
    vec![leading_space::LeadingSpaceChecker::new()]
}

pub fn run(line: &LineEntry) -> Vec<Warning> {
    checklist()
        .iter()
        .filter_map(|c| c.run(line).err())
        .collect()
}
