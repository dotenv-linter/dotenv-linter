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
                // the substitution variables used by the current line present earlier in the
                // current sort group
                let substitutions_in_group: Vec<_> = line
                    .get_substitution_keys()
                    .into_iter()
                    .filter(|key| {
                        lines[start_index..i]
                            .iter()
                            .flat_map(|line| line.get_key())
                            .any(|k| &k == key)
                    })
                    .collect();
                let has_substitution_variables = !substitutions_in_group.is_empty();

                if !line.is_empty_or_comment() && !has_substitution_variables {
                    end.replace(i + 1);
                }

                if line.is_empty()
                    || line.is_last_line()
                    || is_control_comment
                    || has_substitution_variables
                {
                    if has_substitution_variables {
                        lines[i].raw_string =
                            Self::with_unordered_comment(&line, substitutions_in_group)?;
                    }
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
        let mut slices = Vec::with_capacity(part.len());
        part.iter().enumerate().fold(0, |acc, (i, line)| {
            if !line.is_comment() {
                slices.push(&part[acc..=i]);
                i + 1
            } else {
                acc
            }
        });

        slices.sort_by_cached_key(|slice| {
            // I think, that we should modify get_key() so it will return Option<&str> instead of
            // Option<String>.
            slice.last()?.get_key()
        });

        let sorted_lines: Vec<_> = slices.into_iter().flat_map(|s| s.iter().cloned()).collect();

        part.clone_from_slice(sorted_lines.as_slice());
    }

    fn with_unordered_comment(line: &LineEntry, used_keys: Vec<&str>) -> Option<String> {
        Some(match used_keys.len() {
            0 => line.raw_string.clone(),
            1 => format!(
                "{} # Unordered because {} uses {}",
                line.raw_string,
                line.get_key()?,
                used_keys[0],
            ),
            2 => format!(
                "{} # Unordered because {} uses {} and {}",
                line.raw_string,
                line.get_key()?,
                used_keys[0],
                used_keys[1],
            ),
            _ => format!(
                "{} # Unordered because {} uses {}, and {}",
                line.raw_string,
                line.get_key()?,
                used_keys[0..used_keys.len() - 1].join(", "),
                used_keys[used_keys.len() - 1],
            ),
        })
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
            .map(|(i, line)| Warning::new(lines[i].clone(), "UnorderedKey", line))
            .collect()
    }

    fn run_fixer(warnings: &mut [Warning], lines: &mut Vec<LineEntry>) -> Option<usize> {
        let mut fixer = UnorderedKeyFixer::default();
        let warning_refs = warnings.iter_mut().collect();

        fixer.fix_warnings(warning_refs, lines)
    }

    fn assert_lines(result: &[LineEntry], lines: Vec<&str>) {
        result
            .iter()
            .zip(lines)
            .for_each(|(result, line)| assert_eq!(line, result.raw_string.as_str()));
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
    fn key_order_substitution_variable_test() {
        let mut lines = get_lines(vec![
            "KEY=VALUE",
            "ABC=XYZ",
            "FOO=$KEY",
            "BOO=$FOO",
            "XYZ=ABC",
            "BAR=FOO",
        ]);

        let mut warnings = get_warnings(
            &lines,
            vec![
                (1, "The ABC key should go before KEY key"),
                (5, "The BAR key should go before BOO key"),
            ],
        );

        assert_eq!(Some(2), run_fixer(&mut warnings, &mut lines));

        assert_lines(
            &lines,
            vec![
                "ABC=XYZ",
                "KEY=VALUE",
                "FOO=$KEY # Unordered because FOO uses KEY",
                "BAR=FOO",
                "BOO=$FOO",
                "XYZ=ABC",
            ],
        );
    }

    #[test]
    fn key_order_substitution_variable_multiple_groups_test() {
        let mut lines = get_lines(vec![
            "KEY=VALUE",
            "ABC=XYZ",
            "FOO=$KEY",
            "BOO=$FOO",
            "XYZ=ABC",
            "BAR=FOO",
            "",
            "M=1",
            "N=2",
            "A=$M",
            "B=3",
        ]);

        let mut warnings = get_warnings(
            &lines,
            vec![
                (1, "The ABC key should go before KEY key"),
                (5, "The BAR key should go before BOO key"),
                (9, "The A key should go before N key"),
            ],
        );

        assert_eq!(Some(3), run_fixer(&mut warnings, &mut lines));

        assert_lines(
            &lines,
            vec![
                "ABC=XYZ",
                "KEY=VALUE",
                "FOO=$KEY # Unordered because FOO uses KEY",
                "BAR=FOO",
                "BOO=$FOO",
                "XYZ=ABC",
                "",
                "M=1",
                "N=2",
                "A=$M # Unordered because A uses M",
                "B=3",
            ],
        );
    }

    #[test]
    fn key_order_multiple_substitution_variable_together_test() {
        let mut lines = get_lines(vec!["FOO=1", "BAR=2", "A=$FOO$BAR", "B=3", "AA=4"]);

        let mut warnings = get_warnings(
            &lines,
            vec![
                (1, "The BAR key should go before FOO key"),
                (4, "The AA key should go before B key"),
            ],
        );

        assert_eq!(Some(2), run_fixer(&mut warnings, &mut lines));

        assert_lines(
            &lines,
            vec![
                "BAR=2",
                "FOO=1",
                "A=$FOO$BAR # Unordered because A uses FOO and BAR",
                "AA=4",
                "B=3",
            ],
        );
    }

    #[test]
    fn key_order_four_substitution_variable_together_test() {
        let mut lines = get_lines(vec![
            "BBB=1",
            "CCC=1",
            "DDD=1",
            "EEE=1",
            "AAA=$EEE$CCC$BBB$DDD$FFF",
        ]);

        let mut warnings = get_warnings(&lines, vec![]);

        assert_eq!(Some(0), run_fixer(&mut warnings, &mut lines));

        assert_lines(
            &lines,
            vec![
                "BBB=1",
                "CCC=1",
                "DDD=1",
                "EEE=1",
                "AAA=$EEE$CCC$BBB$DDD$FFF # Unordered because AAA uses EEE, CCC, BBB, and DDD",
            ],
        );
    }

    #[test]
    fn key_order_substitution_variable_in_different_group_test() {
        let mut lines = get_lines(vec!["FOO=1", "BAR=2", "", "B=3", "A=$FOO"]);

        let mut warnings = get_warnings(
            &lines,
            vec![
                (1, "The BAR key should go before FOO key"),
                (4, "The A key should go before B key"),
            ],
        );

        assert_eq!(Some(2), run_fixer(&mut warnings, &mut lines));

        assert_lines(&lines, vec!["BAR=2", "FOO=1", "", "A=$FOO", "B=3"]);
    }

    #[test]
    fn key_order_many_substitution_variable_test() {
        let mut lines = get_lines(vec!["Z=1", "Y=2", "X=$Y", "W=$Y", "V=4", "U=5", "T=$V"]);

        let mut warnings = get_warnings(
            &lines,
            vec![
                (1, "The Y key should go before Z key"),
                (4, "The V key should go before W key"),
                (5, "The U key should go before W key"),
            ],
        );

        assert_eq!(Some(3), run_fixer(&mut warnings, &mut lines));

        assert_lines(
            &lines,
            vec![
                "Y=2",
                "Z=1",
                "X=$Y # Unordered because X uses Y",
                "U=5",
                "V=4",
                "W=$Y",
                "T=$V # Unordered because T uses V",
            ],
        );
    }

    #[test]
    fn key_order_substitution_variable_big_test() {
        let mut lines = get_lines(vec![
            "FOO=1",
            "BAZ=2",
            "BAR=$BAZ",
            "AAA=$FOO",
            "AAC=3",
            "AAB=\\$AAC",
            "",
            "B=$AAA$BAZ",
            "C=12",
            "A=$B",
            "AA=$AAA$B",
            "",
            "CCC=$B",
            "CAB=$FOO",
            "CAA=$CCC",
        ]);

        let mut warnings = get_warnings(
            &lines,
            vec![
                (1, "The BAZ key should go before FOO key"),
                (5, "The AAB key should go before AAC key"),
                (12, "The CAB key should go before CCC key"),
            ],
        );

        assert_eq!(Some(3), run_fixer(&mut warnings, &mut lines));

        assert_lines(
            &lines,
            vec![
                "BAZ=2",
                "FOO=1",
                "BAR=$BAZ # Unordered because BAR uses BAZ",
                "AAA=$FOO",
                "AAB=\\$AAC",
                "AAC=3",
                "",
                "B=$AAA$BAZ",
                "C=12",
                "A=$B # Unordered because A uses B",
                "AA=$AAA$B",
                "",
                "CAB=$FOO",
                "CCC=$B",
                "CAA=$CCC # Unordered because CAA uses CCC",
            ],
        );
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
