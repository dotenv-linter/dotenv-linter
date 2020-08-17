use super::Fix;
use crate::common::*;

pub(crate) struct ExtraBlankLineFixer<'a> {
    name: &'a str,
}

impl Default for ExtraBlankLineFixer<'_> {
    fn default() -> Self {
        Self {
            name: "ExtraBlankLine",
        }
    }
}

impl Fix for ExtraBlankLineFixer<'_> {
    fn name(&self) -> &str {
        self.name
    }

    fn fix_warnings(
        &self,
        warnings: Vec<&mut Warning>,
        lines: &mut Vec<LineEntry>,
    ) -> Option<usize> {
        let mut count: usize = 0;

        // check and remove all blank lines.
        let mut is_preview_line_blank = false;
        lines.clone().iter().enumerate().for_each(|(i, line)| {
            let is_empty = line.is_empty();
            if is_empty && is_preview_line_blank {
                lines.remove(i - count);
                count += 1;
            }

            is_preview_line_blank = is_empty;
        });

        // mark as fixed
        for warnig in warnings {
            warnig.mark_as_fixed();
        }

        Some(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn fix_multiple_lines_test() {
        let fixer = ExtraBlankLineFixer::default();
        let file = FileEntry {
            path: PathBuf::from(".env"),
            file_name: ".env".to_string(),
            total_lines: 3,
        };

        let warnings = vec![];
        let lines = vec![
            LineEntry {
                number: 1,
                file: file.clone(),
                raw_string: String::from("FOO=BAR"),
            },
            LineEntry {
                number: 2,
                file: file.clone(),
                raw_string: String::from(""),
            },
            LineEntry {
                number: 3,
                file: file.clone(),
                raw_string: String::from("HOGE=HUGA"),
            },
        ];
        let mut fixing_lines = lines.clone();

        assert_eq!(
            Some(0 as usize),
            fixer.fix_warnings(warnings, &mut fixing_lines)
        );
        assert_eq!(lines, fixing_lines);
    }

    #[test]
    fn fix_multiple_lines_test_double() {
        let fixer = ExtraBlankLineFixer::default();
        let file = FileEntry {
            path: PathBuf::from(".env"),
            file_name: ".env".to_string(),
            total_lines: 4,
        };

        let line1 = LineEntry {
            number: 1,
            file: file.clone(),
            raw_string: String::from("FOO=BAR"),
        };
        let line2 = LineEntry {
            number: 2,
            file: file.clone(),
            raw_string: String::from(""),
        };
        let line3 = LineEntry {
            number: 3,
            file: file.clone(),
            raw_string: String::from(""),
        };
        let line4 = LineEntry {
            number: 4,
            file: file.clone(),
            raw_string: String::from("HOGE=HUGA"),
        };
        let mut warning = Warning::new(
            line3.clone(),
            "ExtraBlankLine",
            "Extra blank line detected".to_string(),
        );
        let warnings = vec![&mut warning];
        let mut lines = vec![line1.clone(), line2.clone(), line3.clone(), line4.clone()];
        assert_eq!(Some(1 as usize), fixer.fix_warnings(warnings, &mut lines));
        assert!(warning.is_fixed);
        assert_eq!(vec!(line1.clone(), line2.clone(), line4.clone(),), lines);
    }

    #[test]
    fn fix_multiple_lines_test_triple() {
        let fixer = ExtraBlankLineFixer::default();
        let file = FileEntry {
            path: PathBuf::from(".env"),
            file_name: ".env".to_string(),
            total_lines: 4,
        };

        let line1 = LineEntry {
            number: 1,
            file: file.clone(),
            raw_string: String::from("FOO=BAR"),
        };
        let line2 = LineEntry {
            number: 2,
            file: file.clone(),
            raw_string: String::from(""),
        };
        let line3 = LineEntry {
            number: 3,
            file: file.clone(),
            raw_string: String::from(""),
        };
        let line4 = LineEntry {
            number: 4,
            file: file.clone(),
            raw_string: String::from(""),
        };
        let line5 = LineEntry {
            number: 5,
            file: file.clone(),
            raw_string: String::from("HOGE=HUGA"),
        };
        let mut warning1 = Warning::new(
            line3.clone(),
            "ExtraBlankLine",
            "Extra blank line detected".to_string(),
        );
        let mut warning2 = Warning::new(
            line4.clone(),
            "ExtraBlankLine",
            "Extra blank line detected".to_string(),
        );
        let warnings = vec![&mut warning1, &mut warning2];
        let mut lines = vec![
            line1.clone(),
            line2.clone(),
            line3.clone(),
            line4.clone(),
            line5.clone(),
        ];
        assert_eq!(Some(2 as usize), fixer.fix_warnings(warnings, &mut lines));
        assert!(warning1.is_fixed);
        assert!(warning2.is_fixed);
        assert_eq!(vec!(line1.clone(), line2.clone(), line5.clone(),), lines);
    }
}
