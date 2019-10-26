#![allow(dead_code)]

use pest::error::ErrorVariant::CustomError as CustomPestError;
use pest::Parser;

use super::super::program::Program;
use super::tokens::{Instruction, Instructions};

#[derive(Parser)]
#[grammar = "metatape/parser/grammar.pest"]
struct Grammar;

type ParseError = pest::error::Error<Rule>;
type TokenPair<'a> = pest::iterators::Pair<'a, Rule>;
type TokenPairs<'a> = pest::iterators::Pairs<'a, Rule>;

/// Take a string input and produce a program containing a sequence of
/// instructions and a hashmap of suborutines. This program will still contain
/// unresolved jumps; it requires syntactic processing before it can be execute.
pub(super) fn tokenize(program_string: &str) -> Result<Program, ParseError> {
    let main_pair = Grammar::parse(Rule::main, program_string)?
        .next()
        .expect("No main token");
    let mut program = Program::new();
    let instructions = &mut program.instructions;
    let subroutines = &mut program.subroutines;
    for pair in main_pair.into_inner() {
        match pair.as_rule() {
            Rule::EOI => (),
            Rule::instruction => {
                instructions.push((pair.as_span().start(), tokenize_instruction(pair)?))
            }
            Rule::subroutine_def => {
                let (name, instructions) = tokenize_subroutine(pair)?;
                subroutines.insert(name, instructions);
            }
            _ => panic!("Invalid token inside main: {:?}", pair.as_rule()),
        }
    }
    Ok(program)
}

fn tokenize_subroutine(pair: TokenPair) -> Result<(String, Instructions), ParseError> {
    let mut parts = pair.into_inner();
    let name = parts
        .next()
        .expect("Subroutine definition contains no name")
        .as_str();
    let block_arg = tokenize_block(
        parts
            .next()
            .expect("Subroutine definition contains no body"),
    );
    return Ok((name.to_owned(), block_arg?));
}

fn tokenize_block(pair: TokenPair) -> Result<Instructions, ParseError> {
    let mut ret: Instructions = vec![];
    for inner_pair in pair.into_inner() {
        let span = inner_pair.as_span();
        ret.push((
            span.start(),
            match inner_pair.as_rule() {
                Rule::instruction => tokenize_instruction(inner_pair),
                _ => parse_error(
                    span,
                    format!("Invalid token inside block: {:?}", inner_pair.as_str()),
                ),
            }?,
        ));
    }
    Ok(ret)
}

fn tokenize_string(pair: TokenPair) -> String {
    let mut ret = String::new();
    for word in pair.into_inner() {
        ret.push_str(word.as_str());
        ret.push(' ');
    }
    ret.pop(); // Remove trailing space
    ret
}

fn tokenize_instruction(pair: TokenPair) -> Result<Instruction, ParseError> {
    let inner_pair = pair
        .into_inner()
        .next()
        .expect("Instruction token contained no inner token");
    let span = inner_pair.as_span();
    match inner_pair.as_rule() {
        Rule::block => Ok(Instruction::Block(Box::new(tokenize_block(inner_pair)?))),
        Rule::block_instruction => Ok(tokenize_block_instruction(inner_pair)?),
        Rule::string_instruction => tokenize_string_instruction(inner_pair),
        Rule::basic_instruction => tokenize_basic_instruction(inner_pair),
        _ => Err(format!(
            "Invalid token inside instruction: {:?}",
            inner_pair.as_rule()
        )),
    }
    .or_else(|error_message| parse_error(span, error_message))
}

fn tokenize_block_instruction(pair: TokenPair) -> Result<Instruction, ParseError> {
    let instruction_char = pair
        .as_str()
        .chars()
        .next()
        .expect("Block instruction contains no instruction");
    let block_arg = tokenize_block(
        pair.into_inner()
            .next()
            .expect("Block instruction contains no argument"),
    );
    let block_arg = Box::new(block_arg?);
    Ok(match instruction_char {
        'f' => Instruction::Fork(block_arg),
        _ => panic!("Unrecognized block instruction: {:#?}", instruction_char),
    })
}

fn tokenize_string_instruction(pair: TokenPair) -> Result<Instruction, String> {
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
        'l' => Instruction::Load,
        _ => panic!("Unrecognized string instruction: {:#?}", instruction_char),
    }(string_arg.to_owned()))
}

fn tokenize_basic_instruction(pair: TokenPair) -> Result<Instruction, String> {
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

fn parse_error<T>(span: pest::Span, message: String) -> Result<T, ParseError> {
    Err(pest::error::Error::new_from_span(
        CustomPestError { message },
        span,
    ))
}
