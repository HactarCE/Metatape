use super::{Runtime, RuntimeError};

impl Runtime {
    pub fn debug_step(&mut self) -> Result<(), RuntimeError> {
        let (current_instruction_str_idx, current_instruction) = self.fetch_instruction()?;
        let (row, col) =
            pest::Position::new(&self.get_program().source, *current_instruction_str_idx)
                .unwrap()
                .line_col();
        let s = format!(
            "{row:>5}:{col:<5}{ip:>3} {instruction:<14}",
            row = row,
            col = col,
            ip = self.get_instruction_pointer(),
            instruction = format!("{}", current_instruction),
        );
        let step_result = self.step();
        print!("{}", s);
        print!(
            "{bit:<2}",
            bit = if let Ok(exec_debug) = &step_result {
                match exec_debug.bit {
                    Some(false) => "0",
                    Some(true) => "1",
                    None => "",
                }
            } else {
                ""
            },
        );
        println!("{:#}", self.get_head());
        step_result.map(|_| ())
    }
}
