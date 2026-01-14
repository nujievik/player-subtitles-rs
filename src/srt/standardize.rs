use super::{SrtLine, SrtLineType, SrtSubtitles};
use crate::{Error, Result};

impl<'a> SrtSubtitles<'a> {
    /// Tries standardizes a subtitles:
    /// 1. Removes start blank lines.
    /// 2. Changes type a block non-blank lines after a [`SrtLineType::TimeRange`] to
    /// [`SrtLineType::Text`].
    /// 3. Adds missing number line to a blocks with [`SrtLineType::TimeRange`] and
    /// [`SrtLineType::Text`] pairs.
    /// 4. Adds missing end blank line to last block.
    /// 5. Removes non-standard blocks.
    /// 6. Sorts block by times.
    ///
    /// Ensures that output subtitles is not empty and all its blocks is standard.
    ///
    /// Does not checks raws.
    ///
    /// # Errors
    /// Returns an error **only if** not received a standard block after 1-6 steps.
    pub fn standardize(self) -> Result<SrtSubtitles<'a>> {
        let mut owned_blocks: Vec<Vec<SrtLine>> = vec![Vec::with_capacity(6)];
        let mut lines = &mut owned_blocks[0];
        let mut was_blank = false;

        for line in self.0.into_iter().skip_while(|l| l.ty.is_blank()) {
            let is_blank = line.ty.is_blank();
            if !is_blank && was_blank {
                if is_keep_lines(lines) {
                    owned_blocks.push(Vec::with_capacity(6));
                    lines = owned_blocks.last_mut().unwrap();
                } else {
                    lines.clear();
                }
            }
            was_blank = is_blank;
            lines.push(line);
        }

        if !is_keep_lines(lines) {
            owned_blocks.pop();
        }

        if owned_blocks.is_empty() {
            return Err(Error::ValueValidation("not received a standard block"));
        }

        owned_blocks.sort_by(|a, b| a[1].ty.unwrap_time_range().cmp(b[1].ty.unwrap_time_range()));

        let lines: Vec<_> = owned_blocks
            .into_iter()
            .flat_map(|block| block.into_iter())
            .collect();
        Ok(SrtSubtitles(lines))
    }
}

fn is_keep_lines(lines: &mut Vec<SrtLine>) -> bool {
    let len = lines.len();
    if len < 2 {
        return false;
    }

    let mut has_text = false;
    if lines[0].ty.is_standard_time_range() && !lines[1].ty.is_blank() {
        has_text = true;
        lines.insert(0, SrtLine::new_with_ty(&[], SrtLineType::Number));
    }

    let mut has_blank = false;
    if has_text || (len > 2 && lines[1].ty.is_standard_time_range() && !lines[2].ty.is_blank()) {
        for l in &mut lines[2..] {
            if l.ty.is_blank() {
                has_blank = true;
                break;
            }
            l.ty = SrtLineType::Text;
        }
    } else {
        return false;
    }

    if !has_blank {
        lines.push(SrtLine::new_with_ty(&[], SrtLineType::Blank));
    }
    true
}
