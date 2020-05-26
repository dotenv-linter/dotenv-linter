use super::Fix;
use crate::common::*;

//use std::collections::HashMap;

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

// When we sort the keys, we handle a significant line (with key) with all previous blank lines and
// comments as a whole.
// E. g.
// ```
// B=C
//
// # Comment
// A=B
// ```
// will be fixed to:
// ```
//
// # Comment
// A=B
// B=C
// ```
impl Fix for UnorderedKeyFixer<'_> {
    fn name(&self) -> &str {
        self.name
    }

    fn fix_warnings(
        &self,
        warnings: Vec<&mut Warning>,
        lines: &mut Vec<LineEntry>,
    ) -> Option<usize> {
        let sorted_len = Self::get_sorted_len(lines)?;

        let mut start_index = 0;
        let mut slices = Vec::with_capacity(sorted_len);
        for (i, line) in lines[..sorted_len].iter().enumerate() {
            if !line.is_empty_or_comment() {
                slices.push(&lines[start_index..=i]);
                start_index = i + 1;
            }
        }

        slices.sort_by_cached_key(|slice| {
            // I think, that we should modify get_key() so it will return Option<&str> instead of
            // Option<String>.
            slice.last().unwrap().get_key()
        });

        let mut sorted_lines = Vec::with_capacity(lines.len());
        for slice in slices {
            sorted_lines.extend_from_slice(slice);
        }
        if sorted_len < lines.len() {
            sorted_lines.extend_from_slice(&lines[sorted_len..lines.len()]);
        }

        lines.clear();
        lines.append(&mut sorted_lines);

        self.set_fixed_on_all(warnings)
    }

    // This is the alternative implementation. It sorts lines in place without separate
    // Vec<LineEntry> (though allocates HashMap<usize, usize>), but it is more complicated and
    // supposedly takes more time (because of the second sorting)

    // fn fix_warnings(
    //     &self,
    //     warnings: Vec<&mut Warning>,
    //     lines: &mut Vec<LineEntry>,
    // ) -> Option<usize> {
    //     let sorted_len = Self::get_sorted_len(lines)?;
    //
    //     let mut anchor_index = sorted_len - 1;
    //     let mut sort_data: Vec<_> = lines[..sorted_len]
    //         .iter()
    //         .enumerate()
    //         .rev()
    //         .map(|(i, line)| {
    //             if !line.is_empty_or_comment() {
    //                 anchor_index = i;
    //             }
    //             (anchor_index, line.number)
    //         })
    //         .collect();
    //
    //     sort_data.sort_by_cached_key(|&(anchor_index, line_number)| {
    //         let anchor_line = &lines[anchor_index];
    //         (anchor_line.get_key(), line_number)
    //     });
    //
    //     let mut map = HashMap::with_capacity(sorted_len);
    //     for (i, (_, line_number)) in sort_data.iter().enumerate() {
    //         map.insert(line_number, i);
    //     }
    //
    //     lines[..sorted_len].sort_by_key(|line| map[&line.number]);
    //
    //     self.set_fixed_on_all(warnings)
    // }
}

impl UnorderedKeyFixer<'_> {
    fn get_sorted_len(lines: &[LineEntry]) -> Option<usize> {
        for (i, line) in lines.iter().enumerate().rev() {
            if !line.is_empty_or_comment() {
                return Some(i + 1);
            }
        }

        None
    }
}
