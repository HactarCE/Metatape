use super::{parse_error, ParseError};
use crate::metatape::program::{Instruction, Instructions, Program};

/// Resolve jump instructions (If, Else, and EndLoop) to their matching
/// destinations, throwing a ParseError if one does not have a matching
/// counterpart.
pub(super) fn resolve_jumps(mut program: Program) -> Result<Program, ParseError> {
    resolve_conditions(program.source, &mut program.instructions)?;
    resolve_loops(program.source, &mut program.instructions)?;
    for subroutine in program.subroutines.values_mut() {
        resolve_conditions(program.source, subroutine)?;
        resolve_loops(program.source, subroutine)?;
    }
    Ok(program)
}

fn resolve_conditions(source_str: &str, instructions: &mut Instructions) -> Result<(), ParseError> {
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
                    pest::Span::new(source_str, *source_idx, *source_idx + 1).unwrap(),
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
            pest::Span::new(source_str, source_idx, source_idx + 1).unwrap(),
            "No matching 'endif' instruction".to_owned(),
        )
    } else {
        Ok(())
    }
}

fn resolve_loops(source_str: &str, instructions: &mut Instructions) -> Result<(), ParseError> {
    let mut source_indices: Vec<usize> = vec![];
    let mut destinations: Vec<usize> = vec![];
    for (idx, (source_idx, instruction)) in instructions.iter_mut().enumerate() {
        if let Instruction::EndLoop(ref mut destination) = instruction {
            source_indices.pop();
            if let Some(dest) = destinations.pop() {
                *destination = dest;
            } else {
                return parse_error(
                    pest::Span::new(source_str, *source_idx, *source_idx + 1).unwrap(),
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
            pest::Span::new(source_str, source_idx, source_idx + 1).unwrap(),
            "No matching 'endloop' instruction".to_owned(),
        )
    } else {
        Ok(())
    }
}
