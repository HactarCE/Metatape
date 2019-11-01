use super::{parse_error, ParseError, SemanticParser};
use crate::metatape::program::{Instruction, InstructionSeq};

impl SemanticParser {
    /// Resolve jump instructions (If, Else, and EndLoop) to their matching
    /// destinations, throwing a ParseError if one does not have a matching
    /// counterpart.
    pub(super) fn resolve_jumps(
        &self,
        instructions: &mut InstructionSeq,
    ) -> Result<(), ParseError> {
        self.resolve_conditions(instructions)?;
        self.resolve_loops(instructions)?;
        Ok(())
    }

    fn resolve_conditions(&self, instructions: &mut InstructionSeq) -> Result<(), ParseError> {
        // Track a stack of IF and ELSE instructions. For each instruction, store a
        // tuple (source_idx: usize, jump_destination: &mut usize).
        let mut source_indices: Vec<usize> = vec![];
        let mut destination_refs: Vec<&mut usize> = vec![];
        for (idx, (source_idx, instruction)) in instructions.iter_mut().enumerate() {
            if let Instruction::Else(_) | Instruction::EndIf = instruction {
                source_indices.pop();
                if let Some(jump_destination) = destination_refs.pop() {
                    *jump_destination = idx;
                } else {
                    return parse_error(
                        pest::Span::new(&self.source_string, *source_idx, *source_idx + 1).unwrap(),
                        "No matching 'if' instruction".to_owned(),
                    );
                }
            }
            if let Instruction::If(ref mut destination) | Instruction::Else(ref mut destination) =
                instruction
            {
                source_indices.push(*source_idx);
                destination_refs.push(destination);
            }
        }
        if let Some(source_idx) = source_indices.pop() {
            parse_error(
                pest::Span::new(&self.source_string, source_idx, source_idx + 1).unwrap(),
                "No matching 'endif' instruction".to_owned(),
            )
        } else {
            Ok(())
        }
    }

    fn resolve_loops(&self, instructions: &mut InstructionSeq) -> Result<(), ParseError> {
        let mut source_indices: Vec<usize> = vec![];
        let mut destinations: Vec<usize> = vec![];
        for (idx, (source_idx, instruction)) in instructions.iter_mut().enumerate() {
            if let Instruction::EndLoop(ref mut destination) = instruction {
                source_indices.pop();
                if let Some(dest) = destinations.pop() {
                    *destination = dest;
                } else {
                    return parse_error(
                        pest::Span::new(&self.source_string, *source_idx, *source_idx + 1).unwrap(),
                        "No matching 'loop' instruction".to_owned(),
                    );
                }
            }
            if let Instruction::Loop = instruction {
                source_indices.push(*source_idx);
                destinations.push(idx);
            }
        }
        if let Some(source_idx) = destinations.pop() {
            parse_error(
                pest::Span::new(&self.source_string, source_idx, source_idx + 1).unwrap(),
                "No matching 'endloop' instruction".to_owned(),
            )
        } else {
            Ok(())
        }
    }
}
