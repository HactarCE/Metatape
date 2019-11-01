use pest::error::ErrorVariant::CustomError as CustomPestError;

use super::program::Program;

mod lexical;
mod syntactic;

#[derive(Parser)]
#[grammar = "metatape/parser/grammar.pest"]
struct Grammar;

pub type ParseError = pest::error::Error<Rule>;
type TokenPair<'a> = pest::iterators::Pair<'a, Rule>;

struct SemanticParser {
    source_string: String,
}

fn parse_error<T>(span: pest::Span, message: String) -> Result<T, ParseError> {
    Err(pest::error::Error::new_from_span(
        CustomPestError { message },
        span,
    ))
}

pub(super) fn parse(source_string: String) -> Result<Program, ParseError> {
    SemanticParser { source_string }.parse_semantics()
}
