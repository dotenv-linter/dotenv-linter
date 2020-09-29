use super::Fix;
use crate::common::*;

pub(crate) struct UnorderedKeyFixer<'a> {
    name: &'a str,
}

impl Default for UnorderedKeyFixer<'_> {
    fn default() -> Self {
        Self {
            name: "UnorderedKey",
        }
    }
}

// When we sort the keys, we handle a significant line (with key) with all previous comments
// as a whole. E. g.:
// ```
// B=C
// # Comment
// A=B
// ```
// will be fixed to:
// ```
// # Comment
// A=B
// B=C
// ```
// We check the order separately in each group of entries (groups are separated by blank lines or
// control comments).
impl Fix for UnorderedKeyFixer<'_> {
    fn name(&self) -> &str {
        self.name
    }

    fn fix_warnings(
        &mut self,
        warnings: Vec<&mut Warning>,
        lines: &mut Vec<LineEntry>,
    ) -> Option<usize> {
        // We find all sorting groups and sort them
        let mut start_index = 0;
        let mut end = None;
        let mut is_disabled = false;

        for i in 0..lines.len() {
            let line = &lines[i];

            let mut is_control_comment = false;
            let mut controls_this_check = false;
            let mut is_off = false;

            if let Some(comment) = line.get_control_comment() {
                is_control_comment = true;
                controls_this_check = comment.checks.contains(&self.name);
                is_off = comment.is_disabled();
            }

            if !is_disabled {
                if !line.is_empty_or_comment() {
                    end.replace(i + 1);
                }

                if line.is_empty() || line.is_last_line() || is_control_comment {
                    if let Some(end_index) = end {
                        Self::sort_part(&mut lines[start_index..end_index]);
                        end = None;
                    }
                    start_index = i + 1;
                }
            }

            if controls_this_check {
                is_disabled = is_off;
                start_index = i + 1;
            }
        }

        Some(warnings.len())
    }
}

impl UnorderedKeyFixer<'_> {
    fn sort_part(part: &mut [LineEntry]) {
        // Each slice includes a significant line (with key) and previous comments (if present)
        let mut slice_start = 0;
        let mut slices = Vec::with_capacity(part.len());
        for (i, line) in part.iter().enumerate() {
            if !line.is_comment() {
                slices.push(&part[slice_start..=i]);
                slice_start = i + 1;
            }
        }

        slices.sort_by_cached_key(|slice| {
            // I think, that we should modify get_key() so it will return Option<&str> instead of
            // Option<String>.
            slice.last()?.get_key()
        });

        let mut sorted_lines = Vec::with_capacity(part.len());
        for slice in slices {
            sorted_lines.extend_from_slice(slice);
        }

        part.clone_from_slice(sorted_lines.as_slice());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::line_entry;

    fn get_lines(lines: Vec<&str>) -> Vec<LineEntry> {
        let total = lines.len();

        lines
            .iter()
            .enumerate()
            .map(|(i, &line)| line_entry(i + 1, total, line))
            .collect()
    }

    fn get_warnings(lines: &[LineEntry], warnings: Vec<(usize, &str)>) -> Vec<Warning> {
        warnings
            .into_iter()
            .map(|(i, line)| Warning::new(lines[i].clone(), "UnorderedKey", String::from(line)))
            .collect()
    }

    fn run_fixer(warnings: &mut [Warning], lines: &mut Vec<LineEntry>) -> Option<usize> {
        let mut fixer = UnorderedKeyFixer::default();
        let warning_refs = warnings.iter_mut().collect();

        fixer.fix_warnings(warning_refs, lines)
    }

    fn assert_lines(result: &[LineEntry], lines: Vec<&str>) {
        for (i, &line) in lines.iter().enumerate() {
            assert_eq!(line, result[i].raw_string.as_str());
        }
    }

    #[test]
    fn fix_warnings_test() {
        let mut lines = get_lines(vec!["B=C", "A=B", "D=E", "\n"]);
        let mut warnings = get_warnings(&lines, vec![(1, "The A key should go before B key")]);

        assert_eq!(Some(1), run_fixer(&mut warnings, &mut lines));

        assert_lines(&lines, vec!["A=B", "B=C", "D=E", "\n"]);
    }

    #[test]
    fn fix_when_no_warnings_test() {
        let mut lines = get_lines(vec!["B=C", "A=B", "D=E", "\n"]);
        let mut warnings = get_warnings(&lines, vec![]);

        assert_eq!(Some(0), run_fixer(&mut warnings, &mut lines));

        assert_lines(&lines, vec!["A=B", "B=C", "D=E", "\n"]);
    }

    #[test]
    fn many_moves_test() {
        let mut lines = get_lines(vec!["X=X", "A=A", "D=D", "Z=Z", "Y=Y", "KLM=123", "\n"]);
        let mut warnings = get_warnings(
            &lines,
            vec![
                (1, "The A key should go before B key"),
                (4, "The Y key should go before Z key"),
                (5, "The KLM key should go before Y key"),
            ],
        );

        assert_eq!(Some(3), run_fixer(&mut warnings, &mut lines));

        assert_lines(
            &lines,
            vec!["A=A", "D=D", "KLM=123", "X=X", "Y=Y", "Z=Z", "\n"],
        );
    }

    #[test]
    fn previous_comments_test() {
        let mut lines = get_lines(vec![
            "# X comment 1",
            "# X comment 2",
            "X=X",
            "# A comment",
            "A=A",
            "B=B",
            "Z=Z",
            "\n",
        ]);
        let mut warnings = get_warnings(&lines, vec![(4, "The A key should go before X key")]);

        assert_eq!(Some(1), run_fixer(&mut warnings, &mut lines));

        assert_lines(
            &lines,
            vec![
                "# A comment",
                "A=A",
                "B=B",
                "# X comment 1",
                "# X comment 2",
                "X=X",
                "Z=Z",
                "\n",
            ],
        );
    }

    #[test]
    fn several_groups_test() {
        let mut lines = get_lines(vec![
            "\n",
            "# start comment",
            "\n",
            "A=1",
            "E=3",
            "C=2",
            "\n",
            "# middle comment",
            "\n",
            "D=2",
            "# B comment",
            "B=1",
            "\n",
            "AB=22",
            "CD=33",
            "\n",
            "# end comment",
            "\n",
        ]);
        let mut warnings = get_warnings(
            &lines,
            vec![
                (6, "The C key should go before E key"),
                (12, "The B key should go before D key"),
            ],
        );

        assert_eq!(Some(2), run_fixer(&mut warnings, &mut lines));

        assert_lines(
            &lines,
            vec![
                "\n",
                "# start comment",
                "\n",
                "A=1",
                "C=2",
                "E=3",
                "\n",
                "# middle comment",
                "\n",
                "# B comment",
                "B=1",
                "D=2",
                "\n",
                "AB=22",
                "CD=33",
                "\n",
                "# end comment",
                "\n",
            ],
        );
    }

    #[test]
    fn no_ending_blank_line_test() {
        let mut lines = get_lines(vec!["B=C", "A=B", "D=E"]);
        let mut warnings = get_warnings(&lines, vec![(2, "The A key should go before B key")]);

        assert_eq!(Some(1), run_fixer(&mut warnings, &mut lines));

        assert_lines(&lines, vec!["A=B", "B=C", "D=E"]);
    }

    #[test]
    fn all_file_control_comments_test() {
        let mut lines = get_lines(vec![
            "# dotenv-linter:off UnorderedKey",
            "B=C",
            "A=B",
            "L=L",
            "K=K",
            "D=E",
            "\n",
        ]);
        let mut warnings = vec![];

        assert_eq!(Some(0), run_fixer(&mut warnings, &mut lines));

        assert_lines(
            &lines,
            vec![
                "# dotenv-linter:off UnorderedKey",
                "B=C",
                "A=B",
                "L=L",
                "K=K",
                "D=E",
                "\n",
            ],
        );
    }

    #[test]
    fn some_lines_control_comments_test() {
        let mut lines = get_lines(vec![
            "C=C",
            "B=B",
            "# dotenv-linter:off UnorderedKey",
            "A2=2",
            "A1=1",
            "# dotenv-linter:on UnorderedKey",
            "X=X",
            "A=A",
            "\n",
        ]);
        let mut warnings = get_warnings(
            &lines,
            vec![
                (2, "The B key should go before C key"),
                (8, "The A key should go before X key"),
            ],
        );

        assert_eq!(Some(2), run_fixer(&mut warnings, &mut lines));

        assert_lines(
            &lines,
            vec![
                "B=B",
                "C=C",
                "# dotenv-linter:off UnorderedKey",
                "A2=2",
                "A1=1",
                "# dotenv-linter:on UnorderedKey",
                "A=A",
                "X=X",
                "\n",
            ],
        );
    }

    #[test]
    fn other_checker_control_comments_test() {
        let mut lines = get_lines(vec![
            "C=C",
            "B=B",
            "# dotenv-linter:off LowercaseKey",
            "A2=2",
            "A1=1",
            "# dotenv-linter:on LowercaseKey",
            "X=X",
            "A=A",
            "\n",
        ]);
        let mut warnings = get_warnings(
            &lines,
            vec![
                (2, "The B key should go before C key"),
                (5, "The A1 key should go before A2 key"),
                (8, "The A key should go before X key"),
            ],
        );

        assert_eq!(Some(3), run_fixer(&mut warnings, &mut lines));

        assert_lines(
            &lines,
            vec![
                "B=B",
                "C=C",
                "# dotenv-linter:off LowercaseKey",
                "A1=1",
                "A2=2",
                "# dotenv-linter:on LowercaseKey",
                "A=A",
                "X=X",
                "\n",
            ],
        );
    }

    #[test]
    fn different_control_comments_test() {
        let mut lines = get_lines(vec![
            "C=C",
            "B=B",
            "# dotenv-linter:off UnorderedKey",
            "A2=2",
            "A1=1",
            "# dotenv-linter:on UnorderedKey",
            "# dotenv-linter:off LowercaseKey",
            "B2=2",
            "B1=1",
            "# dotenv-linter:on LowercaseKey",
            "# some comment",
            "\n",
        ]);
        let mut warnings = get_warnings(
            &lines,
            vec![
                (2, "The B key should go before C key"),
                (9, "The B1 key should go before B2 key"),
            ],
        );

        assert_eq!(Some(2), run_fixer(&mut warnings, &mut lines));

        assert_lines(
            &lines,
            vec![
                "B=B",
                "C=C",
                "# dotenv-linter:off UnorderedKey",
                "A2=2",
                "A1=1",
                "# dotenv-linter:on UnorderedKey",
                "# dotenv-linter:off LowercaseKey",
                "B1=1",
                "B2=2",
                "# dotenv-linter:on LowercaseKey",
                "# some comment",
                "\n",
            ],
        );
    }

    #[test]
    fn comment_between_control_comments_test() {
        let mut lines = get_lines(vec![
            "# dotenv-linter:off UnorderedKey",
            "A2=2",
            "A1=1",
            "# dotenv-linter:on UnorderedKey",
            "# comment",
            "# dotenv-linter:off LowercaseKey",
            "B2=2",
            "B1=1",
            "# dotenv-linter:on LowercaseKey",
            "\n",
        ]);
        let mut warnings = get_warnings(&lines, vec![(8, "The B1 key should go before B2 key")]);

        assert_eq!(Some(1), run_fixer(&mut warnings, &mut lines));

        assert_lines(
            &lines,
            vec![
                "# dotenv-linter:off UnorderedKey",
                "A2=2",
                "A1=1",
                "# dotenv-linter:on UnorderedKey",
                "# comment",
                "# dotenv-linter:off LowercaseKey",
                "B1=1",
                "B2=2",
                "# dotenv-linter:on LowercaseKey",
                "\n",
            ],
        );
    }

    #[test]
    fn crossed_control_comments_test() {
        let mut lines = get_lines(vec![
            "# dotenv-linter:off LowercaseKey",
            "A3=3",
            "A2=2",
            "# dotenv-linter:off UnorderedKey",
            "A1=1",
            "# dotenv-linter:on LowercaseKey",
            "B2=2",
            "B1=1",
            "# dotenv-linter:on UnorderedKey",
            "C2=2",
            "C1=1",
            "\n",
        ]);
        let mut warnings = get_warnings(
            &lines,
            vec![
                (3, "The A2 key should go before A3 key"),
                (11, "The C1 key should go before C2 key"),
            ],
        );

        assert_eq!(Some(2), run_fixer(&mut warnings, &mut lines));

        assert_lines(
            &lines,
            vec![
                "# dotenv-linter:off LowercaseKey",
                "A2=2",
                "A3=3",
                "# dotenv-linter:off UnorderedKey",
                "A1=1",
                "# dotenv-linter:on LowercaseKey",
                "B2=2",
                "B1=1",
                "# dotenv-linter:on UnorderedKey",
                "C1=1",
                "C2=2",
                "\n",
            ],
        );
    }

    #[test]
    fn nested_control_comments_test() {
        let mut lines = get_lines(vec![
            "# dotenv-linter:off LowercaseKey",
            "A4=4",
            "# comment",
            "A3=3",
            "# dotenv-linter:off UnorderedKey",
            "A2=2",
            "A1=1",
            "# dotenv-linter:on UnorderedKey",
            "C2=2",
            "C1=1",
            "# dotenv-linter:on LowercaseKey",
            "B2=2",
            "B1=1",
            "\n",
        ]);
        let mut warnings = get_warnings(
            &lines,
            vec![
                (4, "The A3 key should go before A4 key"),
                (10, "The C1 key should go before C2 key"),
                (13, "The B1 key should go before B2 key"),
            ],
        );

        assert_eq!(Some(3), run_fixer(&mut warnings, &mut lines));

        assert_lines(
            &lines,
            vec![
                "# dotenv-linter:off LowercaseKey",
                "# comment",
                "A3=3",
                "A4=4",
                "# dotenv-linter:off UnorderedKey",
                "A2=2",
                "A1=1",
                "# dotenv-linter:on UnorderedKey",
                "C1=1",
                "C2=2",
                "# dotenv-linter:on LowercaseKey",
                "B1=1",
                "B2=2",
                "\n",
            ],
        );
    }

    #[test]
    fn empty_control_comments_test() {
        let mut lines = get_lines(vec![
            "Z=Z",
            "# dotenv-linter:off UnorderedKey",
            "# dotenv-linter:on UnorderedKey",
            "A4=4",
            "# comment",
            "A2=2",
            "A3=3",
            "A1=1",
            "\n",
        ]);
        let mut warnings = get_warnings(
            &lines,
            vec![
                (6, "The A2 key should go before A4 key"),
                (8, "The A1 key should go before A3 key"),
            ],
        );

        assert_eq!(Some(2), run_fixer(&mut warnings, &mut lines));

        assert_lines(
            &lines,
            vec![
                "Z=Z",
                "# dotenv-linter:off UnorderedKey",
                "# dotenv-linter:on UnorderedKey",
                "A1=1",
                "# comment",
                "A2=2",
                "A3=3",
                "A4=4",
                "\n",
            ],
        );
    }

    #[test]
    fn control_comments_and_blank_lines_test() {
        let mut lines = get_lines(vec![
            "\n",
            "# start comment",
            "\n",
            "A=1",
            "E=3",
            "# C comment",
            "# C comment",
            "C=2",
            "\n",
            "# middle comment",
            "# dotenv-linter:off UnorderedKey",
            "A201=201",
            "A200=200",
            "\n",
            "A100=100",
            "A101=101",
            "# dotenv-linter:on UnorderedKey",
            "W=2",
            "D=2",
            "# dotenv-linter:off KeyWithoutValue",
            "# B comment",
            "B=1",
            "# dotenv-linter:on KeyWithoutValue",
            "# dotenv-linter:off UnorderedKey",
            "\n",
            "AB=22",
            "CD=33",
            "\n",
            "# end comment",
            "\n",
        ]);
        let mut warnings = get_warnings(
            &lines,
            vec![
                (8, "The C key should go before E key"),
                (19, "The D key should go before W key"),
            ],
        );

        assert_eq!(Some(2), run_fixer(&mut warnings, &mut lines));

        assert_lines(
            &lines,
            vec![
                "\n",
                "# start comment",
                "\n",
                "A=1",
                "# C comment",
                "# C comment",
                "C=2",
                "E=3",
                "\n",
                "# middle comment",
                "# dotenv-linter:off UnorderedKey",
                "A201=201",
                "A200=200",
                "\n",
                "A100=100",
                "A101=101",
                "# dotenv-linter:on UnorderedKey",
                "D=2",
                "W=2",
                "# dotenv-linter:off KeyWithoutValue",
                "# B comment",
                "B=1",
                "# dotenv-linter:on KeyWithoutValue",
                "# dotenv-linter:off UnorderedKey",
                "\n",
                "AB=22",
                "CD=33",
                "\n",
                "# end comment",
                "\n",
            ],
        );
    }
}
