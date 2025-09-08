use dotenv_core::{LineEntry, is_escaped};

use super::Fix;
use crate::LintKind;

#[derive(Default)]
pub(crate) struct SubstitutionKeyFixer {}

impl Fix for SubstitutionKeyFixer {
    fn name(&self) -> LintKind {
        LintKind::SubstitutionKey
    }

    // TODO: refactor
    fn fix_line(&self, line: &mut LineEntry) -> Option<()> {
        let mut value = line
            .get_value()
            .map(str::trim)
            .filter(|val| !val.starts_with('\''))?;

        let mut result = String::with_capacity(value.len());

        // Checks if keys used in value have both '{' '}' or neither
        while let Some((prefix, raw_key)) = value.split_once('$') {
            result.push_str(prefix);
            result.push('$');

            // Separate initial key from the rest
            let (initial_key, rest) = raw_key
                .find('$')
                .map(|i| raw_key.split_at(i))
                .unwrap_or_else(|| (raw_key, ""));

            value = rest;

            let stripped_key = initial_key.strip_prefix('{').unwrap_or(initial_key);

            let correct_end_index = stripped_key
                .find(|c: char| !c.is_ascii_alphanumeric() && c != '_')
                .unwrap_or(stripped_key.len());

            if is_escaped(prefix) || correct_end_index == 0 {
                result.push_str(stripped_key);
                continue;
            }

            result.push('{');
            result.push_str(&stripped_key[..correct_end_index]);

            let rest_part = &stripped_key[correct_end_index..];
            if !rest_part.starts_with('}') {
                result.push('}')
            }

            result.push_str(rest_part);
        }

        line.raw_string = format!("{}={}", line.get_key()?, result);
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    #[test]
    fn fix_line_test() {
        let fixer = SubstitutionKeyFixer::default();
        let mut line = line_entry(1, 1, "FOO=${BAR");

        assert_eq!(Some(()), fixer.fix_line(&mut line));
        assert_eq!("FOO=${BAR}", line.raw_string);
    }

    #[test]
    fn fix_warnings_test() {
        let fixer = SubstitutionKeyFixer::default();
        let mut lines = vec![
            line_entry(1, 7, "FOO=${BAR-$ABC_ROOT}"),
            line_entry(2, 7, "Z=$Y"),
            line_entry(3, 7, "BAR=$Y}-$OPTS"),
            line_entry(4, 7, "ABC=${BAR$XYZ}"),
            line_entry(5, 7, "FOO=\"\\$BAR}"),
            line_entry(6, 7, "GOD=${BAR!}"),
            blank_line_entry(7, 7),
        ];
        let warning_lines = [1, 3, 4, 6];

        assert_eq!(Some(4), fixer.fix_warnings(&warning_lines, &mut lines));
        assert_eq!("FOO=${BAR}-${ABC_ROOT}", lines[0].raw_string);
        assert_eq!("BAR=${Y}-${OPTS}", lines[2].raw_string);
        assert_eq!("ABC=${BAR}${XYZ}", lines[3].raw_string);
        assert_eq!("GOD=${BAR}!}", lines[5].raw_string);
    }
}
