use dotenv_core::{LineEntry, is_escaped};

use super::Fix;
use crate::LintKind;

#[derive(Default)]
pub(crate) struct SubstitutionKeyFixer {}

/// Returns the position of the `}` that closes the leading `{`.
///
/// Nested braces are counted, so `{A:-${B}}` is closed by the last brace,
/// while `{A${B}` has no closing brace at all.
fn closing_brace(raw: &str) -> Option<usize> {
    let mut depth = 0;

    for (index, c) in raw.char_indices() {
        match c {
            '{' => depth += 1,
            '}' if depth > 0 => {
                depth -= 1;
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
    }

    None
}

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

            if raw_key.starts_with('{')
                && let Some(closing_pos) = closing_brace(raw_key)
            {
                let whole_sub = &raw_key[..=closing_pos];
                result.push_str(whole_sub);
                value = &raw_key[closing_pos + 1..];
                continue;
            }

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
                result.push_str(initial_key);
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

        result.push_str(value);

        let new_raw_string = format!("{}={}", line.get_key()?, result);
        if new_raw_string == line.raw_string {
            return None;
        }

        line.raw_string = new_raw_string;
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
    fn fix_line_keeps_text_after_braced_substitution() {
        let fixer = SubstitutionKeyFixer::default();

        for raw_string in ["FOO=${BAR-BAZ}-suffix", "FOO=x${BAR-BAZ}y"] {
            let mut line = line_entry(1, 1, raw_string);

            assert_eq!(None, fixer.fix_line(&mut line), "input: {raw_string}");
            assert_eq!(raw_string, line.raw_string, "input: {raw_string}");
        }
    }

    #[test]
    fn fix_line_closes_the_outer_brace_of_a_nested_substitution() {
        let fixer = SubstitutionKeyFixer::default();

        for (input, expected) in [
            ("FOO=${ENVS${BAR}", "FOO=${ENVS}${BAR}"),
            ("FOO=$ENVS}${BAR}", "FOO=${ENVS}${BAR}"),
        ] {
            let mut line = line_entry(1, 1, input);

            assert_eq!(Some(()), fixer.fix_line(&mut line), "input: {input}");
            assert_eq!(expected, line.raw_string, "input: {input}");
        }
    }

    #[test]
    fn fix_line_keeps_closed_nested_substitutions() {
        let fixer = SubstitutionKeyFixer::default();

        for raw_string in ["FOO=${A:-${B}}", "FOO=${A${B}}"] {
            let mut line = line_entry(1, 1, raw_string);

            assert_eq!(None, fixer.fix_line(&mut line), "input: {raw_string}");
            assert_eq!(raw_string, line.raw_string, "input: {raw_string}");
        }
    }

    #[test]
    fn fix_line_keeps_balanced_braced_substitutions() {
        let fixer = SubstitutionKeyFixer::default();

        for raw_string in [
            "FOO=${BAR:-baz}",
            "FOO=${MY.VAR}",
            "FOO=${BAR}suffix",
            "FOO='${BAR'",
        ] {
            let mut line = line_entry(1, 1, raw_string);

            assert_eq!(None, fixer.fix_line(&mut line), "input: {raw_string}");
            assert_eq!(raw_string, line.raw_string, "input: {raw_string}");
        }
    }

    #[test]
    fn fix_line_keeps_escaped_and_empty_substitutions() {
        let fixer = SubstitutionKeyFixer::default();

        for raw_string in ["FOO=\\${BAR", "FOO=${"] {
            let mut line = line_entry(1, 1, raw_string);

            assert_eq!(None, fixer.fix_line(&mut line), "input: {raw_string}");
            assert_eq!(raw_string, line.raw_string, "input: {raw_string}");
        }

        // The escaped substitution stays as is, the next one is fixed.
        let mut line = line_entry(1, 1, "FOO=\\${BAR${BAZ");
        assert_eq!(Some(()), fixer.fix_line(&mut line));
        assert_eq!("FOO=\\${BAR${BAZ}", line.raw_string);
    }

    #[test]
    fn fix_malformed_substitution_with_braces() {
        let fixer = SubstitutionKeyFixer::default();
        let mut line = line_entry(1, 1, "URL=${FOO-BAR}://$HOST-db");
        assert_eq!(Some(()), fixer.fix_line(&mut line));
        assert_eq!("URL=${FOO-BAR}://${HOST}-db", line.raw_string);
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
        let warning_lines = [1, 2, 3, 4, 6];

        // Only lines that actually change are counted as fixed. Braced
        // substitutions whose braces are balanced are left as is, so they must
        // not be counted.
        assert_eq!(Some(2), fixer.fix_warnings(&warning_lines, &mut lines));

        assert_eq!("FOO=${BAR-$ABC_ROOT}", lines[0].raw_string);
        assert_eq!("Z=${Y}", lines[1].raw_string);
        assert_eq!("BAR=${Y}-${OPTS}", lines[2].raw_string);
        assert_eq!("ABC=${BAR$XYZ}", lines[3].raw_string);
        assert_eq!("FOO=\"\\$BAR}", lines[4].raw_string);
        assert_eq!("GOD=${BAR!}", lines[5].raw_string);
    }
}
