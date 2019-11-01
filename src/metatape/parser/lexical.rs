#![allow(dead_code)]

use pest::Parser;
use std::collections::HashMap;
use std::rc::Rc;

use super::{parse_error, Grammar, ParseError, Rule, SemanticParser, TokenPair};
use crate::metatape::program::{Instruction, InstructionSeq, Program, Subroutines};

impl SemanticParser {
    pub(super) fn parse_semantics(self) -> Result<Program, ParseError> {
        let main_pair = Grammar::parse(Rule::main, &self.source_string)?
            .next()
            .expect("No main token");
        let mut instructions: InstructionSeq = vec![];
        let mut subroutines: Subroutines = HashMap::new();
        for pair in main_pair.into_inner() {
            match pair.as_rule() {
                Rule::EOI => (),
                Rule::instruction => {
                    instructions.push((pair.as_span().start(), self.tokenize_instruction(pair)?))
                }
                Rule::subroutine_def => {
                    let (name, mut sub_instructions) = self.tokenize_subroutine(pair)?;
                    self.resolve_jumps(&mut sub_instructions)?;
                    subroutines.insert(name, Rc::new(sub_instructions));
                }
                _ => panic!("Invalid token inside main: {:?}", pair.as_rule()),
            }
        }
        self.resolve_jumps(&mut instructions)?;
        Ok(Program {
            source: self.source_string,
            instructions: Rc::new(instructions),
            subroutines,
        })
    }

    fn tokenize_subroutine(&self, pair: TokenPair) -> Result<(String, InstructionSeq), ParseError> {
        let mut parts = pair.into_inner();
        let name = parts
            .next()
            .expect("Subroutine definition contains no name")
            .as_str();
        let block_arg = self.tokenize_block(
            parts
                .next()
                .expect("Subroutine definition contains no body"),
        );
        return Ok((name.to_owned(), block_arg?));
    }

    fn tokenize_block(&self, pair: TokenPair) -> Result<InstructionSeq, ParseError> {
        let mut ret: InstructionSeq = vec![];
        for inner_pair in pair.into_inner() {
            let span = inner_pair.as_span();
            ret.push((
                span.start(),
                match inner_pair.as_rule() {
                    Rule::instruction => self.tokenize_instruction(inner_pair),
                    _ => parse_error(
                        span,
                        format!("Invalid token inside block: {:?}", inner_pair.as_str()),
                    ),
                }?,
            ));
        }
        self.resolve_jumps(&mut ret)?;
        Ok(ret)
    }

    fn tokenize_string(&self, pair: TokenPair) -> String {
        let mut ret = String::new();
        for word in pair.into_inner() {
            ret.push_str(word.as_str());
            ret.push(' ');
        }
        ret.pop(); // Remove trailing space
        ret
    }

    fn tokenize_instruction(&self, pair: TokenPair) -> Result<Instruction, ParseError> {
        let inner_pair = pair
            .into_inner()
            .next()
            .expect("Instruction token contained no inner token");
        let span = inner_pair.as_span();
        match inner_pair.as_rule() {
            Rule::block => Ok(Instruction::Block(Rc::new(
                self.tokenize_block(inner_pair)?,
            ))),
            Rule::block_instruction => Ok(self.tokenize_block_instruction(inner_pair)?),
            Rule::string_instruction => self.tokenize_string_instruction(inner_pair),
            Rule::basic_instruction => self.tokenize_basic_instruction(inner_pair),
            _ => Err(format!(
                "Invalid token inside instruction: {:?}",
                inner_pair.as_rule()
            )),
        }
        .or_else(|error_message| parse_error(span, error_message))
    }

    fn tokenize_block_instruction(&self, pair: TokenPair) -> Result<Instruction, ParseError> {
        let instruction_char = pair
            .as_str()
            .chars()
            .next()
            .expect("Block instruction contains no instruction");
        let block_arg = self.tokenize_block(
            pair.into_inner()
                .next()
                .expect("Block instruction contains no argument"),
        );
        Ok(match instruction_char {
            'f' => Instruction::Fork(Rc::new(block_arg?)),
            _ => panic!("Unrecognized block instruction: {:#?}", instruction_char),
        })
    }

    fn tokenize_string_instruction(&self, pair: TokenPair) -> Result<Instruction, String> {
        let instruction_char = pair
            .as_str()
            .chars()
            .next()
            .expect("String instruction contains no instruction");
        let string_arg = pair
            .into_inner()
            .next()
            .expect("String instruction contains no argument")
            .as_str();
        Ok(match instruction_char {
            '!' => Instruction::Call,
            _ => panic!("Unrecognized string instruction: {:#?}", instruction_char),
        }(string_arg.to_owned()))
    }

    fn tokenize_basic_instruction(&self, pair: TokenPair) -> Result<Instruction, String> {
        match pair.as_str() {
            "." => Ok(Instruction::Nop),
            "<" => Ok(Instruction::Left),
            ">" => Ok(Instruction::Right),
            "e" => Ok(Instruction::Enter),
            "x" => Ok(Instruction::Exit),
            "n" => Ok(Instruction::Null),
            "(" => Ok(Instruction::If(0)),
            "|" => Ok(Instruction::Else(0)),
            ")" => Ok(Instruction::EndIf),
            "[" => Ok(Instruction::Loop),
            "]" => Ok(Instruction::EndLoop(0)),
            "?" => Ok(Instruction::Random),
            "i" => Ok(Instruction::Input),
            "o" => Ok(Instruction::Output),
            "h" => Ok(Instruction::Halt),
            "@" => Err("Subroutines cannot be defined inside a subroutine or block".to_owned()),
            _ => Err(format!("Unrecognized instruction: {:?}", pair.as_str())),
        }
    }
}
