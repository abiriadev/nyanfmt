use std::{iter::Peekable, str::Chars, vec::IntoIter};

#[derive(Debug, PartialEq)]
pub enum Token {
	Right,
	Left,
	Inc,
	Dec,
	Out,
	In,
	JumpRight,
	JumpLeft,
	Span,
}

#[derive(Debug)]
struct Lexer<'a> {
	code: Chars<'a>,
}

impl<'a> Lexer<'a> {
	fn new(code: Chars<'a>) -> Self {
		Self { code }
	}

	fn tokenize(ch: char) -> Option<Token> {
		use Token::*;

		match ch {
			'?' => Some(Right),
			'!' => Some(Left),
			'냥' => Some(Inc),
			'냐' => Some(Dec),
			'.' => Some(Out),
			',' => Some(In),
			'~' => Some(JumpRight),
			'-' => Some(JumpLeft),
			' ' | '\t'..='\r' => Some(Span),
			_ => None,
		}
	}
}

impl<'a> From<Lexer<'a>> for Vec<Token> {
	fn from(mut lexer: Lexer<'a>) -> Self {
		let mut v: Vec<Token> = vec![];
		let mut is_span = false;

		while let Some(ch) = lexer.code.next() {
			let Some(token) = Lexer::tokenize(ch) else {
                continue;
            };

			is_span = match token {
				Token::Span => {
					if !is_span {
						v.push(Token::Span)
					}
					true
				},
				_ => {
					v.push(token);
					false
				},
			};
		}

		v
	}
}

struct State {
	counter: u32,
}

struct Formatter {
	token_stream: Peekable<IntoIter<Token>>,
	state: State,
}

impl<'a> From<Lexer<'a>> for Formatter {
	fn from(lexer: Lexer<'a>) -> Self {
		Self {
			token_stream: Vec::from(lexer).into_iter().peekable(),
			state: State { counter: 0 },
		}
	}
}

impl From<Formatter> for Vec<Token> {
	fn from(mut formatter: Formatter) -> Self {
		use Token::*;

		let mut v = vec![];

		while let Some(token) = formatter.token_stream.next() {
			let Some(next) = formatter.token_stream.peek() else {
                v.push(token);
                break;
            };

			match token {
				Right | Left => {
					v.push(token);
					if !matches!(next, Right | Left) {
						v.push(Span);
					}
				},
				Out | In | JumpRight | JumpLeft => {
					v.push(token);
					if matches!(next, Inc | Dec) {
						v.push(Span);
					}
				},
				Span => (),
				_ => v.push(token),
			}
		}

		v
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::assert_eq;

	#[test]
	fn lex() {
		use Token::*;

		let code = "냥냥냥냥냥  냥냥냥~? 냥냥냥냥~? 냥냥?    냥냥냥? 냥냥냥?냥!!  !!  냐-? 냥?냥?  냐??냥~!-!   냐-??.? 냐  냐냐. ";
		let lexer = Lexer::new(code.chars());
		let token_stream: Vec<Token> = lexer.into();

		assert_eq!(
			token_stream,
			[
				Inc, Inc, Inc, Inc, Inc, Span, Inc, Inc, Inc, JumpRight, Right,
				Span, Inc, Inc, Inc, Inc, JumpRight, Right, Span, Inc, Inc,
				Right, Span, Inc, Inc, Inc, Right, Span, Inc, Inc, Inc, Right,
				Inc, Left, Left, Span, Left, Left, Span, Dec, JumpLeft, Right,
				Span, Inc, Right, Inc, Right, Span, Dec, Right, Right, Inc,
				JumpRight, Left, JumpLeft, Left, Span, Dec, JumpLeft, Right,
				Right, Out, Right, Span, Dec, Span, Dec, Dec, Out, Span
			],
		)
	}

	#[test]
	fn format() {
		use Token::*;

		let code = "냥~?냥냥??냥냥-???-!-??.?냐.";
		let rep = Formatter::from(Lexer::new(code.chars()));

		assert_eq!(
			Vec::from(rep),
			[
				Inc, JumpRight, Right, Span, Inc, Inc, Right, Right, Span, Inc,
				Inc, JumpLeft, Right, Right, Right, Span, JumpLeft, Left, Span,
				JumpLeft, Right, Right, Span, Out, Right, Span, Dec, Out,
			],
		)
	}
}
