use nom::{
	branch::alt,
	bytes::complete::take_until,
	character::complete::{char, line_ending, space0},
	combinator::{cut, eof, map, value},
	error::Error,
	multi::{many0, many1},
	sequence::{delimited, terminated},
	Finish, IResult,
};
use str_macro::str as s;

use super::Token;

char_token! { lex_right: '?' -> Token::Right }
char_token! { lex_left: '!' -> Token::Left }
char_token! { lex_inc: '냥' -> Token::Inc }
char_token! { lex_dec: '냐' -> Token::Dec }
char_token! { lex_out: '.' -> Token::Out }
char_token! { lex_in: ',' -> Token::In }
char_token! { lex_jump_right: '~' -> Token::JumpRight }
char_token! { lex_jump_left: '-' -> Token::JumpLeft }
char_token! { lex_debug: '뀨' -> Token::Debug }

fn lex_comment(input: &str) -> IResult<&str, Token> {
	map(
		delimited(char('"'), take_until(r#"""#), char('"')),
		|o: &str| Token::Comment(s!(o)),
	)(input)
}

fn lex_newline(input: &str) -> IResult<&str, Token> {
	value(Token::NewLine, many1(line_ending))(input)
}

fn lex_token(input: &str) -> IResult<&str, Token> {
	alt((
		lex_right,
		lex_left,
		lex_inc,
		lex_dec,
		lex_out,
		lex_in,
		lex_jump_right,
		lex_jump_left,
		lex_debug,
		lex_comment,
		lex_newline,
	))(input)
}

fn lex_tokenstream(input: &str) -> IResult<&str, Vec<Token>> {
	many0(delimited(space0, lex_token, space0))(input)
}

pub fn lex_code(input: &str) -> Result<Vec<Token>, Error<&str>> {
	terminated(lex_tokenstream, cut(eof))(input)
		.finish()
		.map(|(_, o)| o)
}

#[cfg(test)]
#[path = "lexer.spec.rs"]
mod lexer_tests;
