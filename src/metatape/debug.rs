use super::runtime::RuntimeError;
use super::Runtime;

impl Runtime {
    pub fn debug_step(&mut self) -> Result<String, RuntimeError> {
        let mut ret = String::new();
        let (current_instruction_str_idx, current_instruction) = self.fetch_instruction()?;
        let (row, col) =
            pest::Position::new(&self.get_program().source, *current_instruction_str_idx)
                .unwrap()
                .line_col();
        ret.push_str(&format!(
            "{row:>5}:{col:<6}{ip:<4}{instruction:<14?}",
            row = row,
            col = col,
            ip = self.get_instruction_pointer(),
            instruction = current_instruction,
        ));
        let exec_debug = self.step()?;
        ret.push_str(&format!(
            "{bit:<2}{head}",
            bit = match exec_debug.bit {
                Some(false) => "0",
                Some(true) => "1",
                None => "",
            },
            head = self.get_head()
        ));
        Ok(ret)
    }
}
