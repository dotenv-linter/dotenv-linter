use crate::checks::leading_space::LeadingSpaceCheck;

mod leading_space;

trait Lint {
    fn run(&self, line: &str) -> Result<(), String>;
}

fn checklist() -> Vec<impl Lint> {
    vec![LeadingSpaceCheck]
}

pub fn run(line: &str) -> Vec<String> {
    checklist()
        .iter()
        .filter_map(|c| c.run(line).err())
        .collect()
}
