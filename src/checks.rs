mod leading_space;

trait Check {
    fn run(&self, line: &str) -> Result<(), String>;
}

fn checklist() -> Vec<impl Check> {
    vec![leading_space::LeadingSpaceChecker]
}

pub fn run(line: &str) -> Vec<String> {
    checklist()
        .iter()
        .filter_map(|c| c.run(line).err())
        .collect()
}
