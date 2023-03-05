use super::*;
use nom::{
	combinator::eof,
	error::{Error, ErrorKind},
	sequence::terminated,
	Finish,
};
use pretty_assertions::assert_eq;

type HT = HeadTok;
type BT = BodyTok;
type TT = TailTok;

#[test]
fn parse_head_tokens() {
	let code = ts![Inc, Debug, Inc, Dec];

	assert_eq!(
		parse_head(code),
		Ok((
			TokenStream::new(),
			Head(vec![
				HT::Inc,
				HT::Debug,
				HT::Inc,
				HT::Dec,
			])
		))
	)
}

#[test]
fn parse_head_tokens2() {
	let code = ts![Inc, Debug, Right, Inc, Dec];

	assert_eq!(
		parse_head(code),
		Ok((
			ts![Right, Inc, Dec],
			Head(vec![HT::Inc, HT::Debug])
		))
	)
}

#[test]
fn must_fail_to_parse_head_if_there_are_no_matched_tokens() {
	let code = ts![Out, Inc, Debug, Right, Inc, Dec];

	assert_eq!(
		parse_head(code.clone()).finish(),
		Err(Error::new(code, ErrorKind::Tag))
	);
}

#[test]
fn parse_body_tokens() {
	let code = ts![Out, JumpRight, JumpRight, Dec, In, JumpLeft, Out];

	assert_eq!(
		parse_body(code),
		Ok((
			ts![Dec, In, JumpLeft, Out],
			Body(vec![
				BT::Out,
				BT::JumpRight,
				BT::JumpRight,
			])
		))
	)
}

#[test]
fn parse_body_tokens2() {
	let code = ts![Out, JumpRight, JumpRight, In, JumpLeft, Out];

	assert_eq!(
		parse_body(code),
		Ok((
			TokenStream::new(),
			Body(vec![
				BT::Out,
				BT::JumpRight,
				BT::JumpRight,
				BT::In,
				BT::JumpLeft,
				BT::Out,
			])
		))
	)
}

#[test]
fn must_fail_to_parse_body_if_there_are_no_matched_tokens() {
	let code = ts![Debug, Out, JumpRight, JumpRight, In, JumpLeft, Out];

	assert_eq!(
		parse_body(code.clone()).finish(),
		Err(Error::new(code, ErrorKind::Tag))
	);
}

#[test]
fn parse_tail_tokens() {
	let code = ts![Right, Left, Right, Right];

	assert_eq!(
		parse_tail(code),
		Ok((
			TokenStream::new(),
			Tail(vec![
				TT::Right,
				TT::Left,
				TT::Right,
				TT::Right,
			])
		))
	)
}

#[test]
fn parse_tail_tokens2() {
	let code = ts![Right, Left, JumpRight, Right, Right];

	assert_eq!(
		parse_tail(code),
		Ok((
			ts![JumpRight, Right, Right],
			Tail(vec![TT::Right, TT::Left])
		))
	)
}

#[test]
fn must_fail_to_parse_tail_if_there_are_no_matched_tokens() {
	let code = ts![Out, Right, Left, JumpRight, Right, Right];

	assert_eq!(
		parse_tail(code.clone()).finish(),
		Err(Error::new(code, ErrorKind::Tag))
	);
}

#[test]
fn must_parse_word() {
	let code = ts![Out, JumpRight, In, JumpLeft, Left, Out, Debug];

	assert_eq!(
		parse_word(code),
		Ok((
			ts![Out, Debug],
			word!(
				,
				[
					BT::Out,
					BT::JumpRight,
					BT::In,
					BT::JumpLeft,
				],
				[TT::Left],
			)
		))
	)
}

#[test]
fn must_parse_word2() {
	let code = ts![Debug, Inc, JumpRight, In, JumpLeft, Left, Out, Debug];

	assert_eq!(
		parse_word(code),
		Ok((
			ts![Out, Debug],
			word!(
				[HT::Debug, HT::Inc],
				[BT::JumpRight, BT::In, BT::JumpLeft],
				[TT::Left],
			)
		))
	)
}

#[test]
fn parse_word_only_match_head() {
	let code = ts![Inc, Inc];

	assert_eq!(
		parse_word(code),
		Ok((
			TokenStream::new(),
			word!([HT::Inc, HT::Inc],,)
		))
	)
}

#[test]
fn parse_word_only_match_body() {
	let code = ts![Out, JumpLeft, Dec, JumpLeft];

	assert_eq!(
		parse_word(code),
		Ok((
			ts![Dec, JumpLeft],
			word!(, [BT::Out, BT::JumpLeft],),
		))
	)
}

#[test]
fn parse_word_only_match_tail() {
	let code = ts![Left, Left, Debug];

	assert_eq!(
		parse_word(code),
		Ok((
			ts![Debug],
			word!(
				,,
				[TT::Left, TT::Left]
			)
		))
	)
}

#[test]
fn must_fail_to_parse_word_if_input_is_empty() {
	let code = TokenStream::new();

	assert_eq!(
		parse_word(code).finish(),
		Err(Error::new(
			TokenStream::new(),
			ErrorKind::Verify
		))
	);
}

#[test]
fn test_parse_words0() {
	let code = ts![
		Inc, Dec, Out, In, Right, Left, Inc, Inc, Debug, Out, JumpLeft,
		JumpRight, Left, Left, Left,
	];

	assert_eq!(
		parse_words0(code),
		Ok((
			TokenStream::new(),
			vec![
				word!(
					[HT::Inc, HT::Dec],
					[BT::Out, BT::In],
					[TT::Right, TT::Left],
				),
				word!(
					[HT::Inc, HT::Inc, HT::Debug],
					[BT::Out, BT::JumpLeft, BT::JumpRight],
					[TT::Left, TT::Left, TT::Left],
				)
			]
		))
	)
}

#[test]
fn test_parse_words0_with_empty_input() {
	let code = TokenStream::new();

	assert_eq!(
		parse_words0(code),
		Ok((TokenStream::new(), vec![]))
	)
}

#[test]
fn test_parse_words1() {
	let code = ts![
		Inc, Dec, Out, In, Right, Left, Inc, Inc, Debug, Out, JumpLeft,
		JumpRight, Left, Left, Left,
	];

	assert_eq!(
		parse_words1(code),
		Ok((
			TokenStream::new(),
			vec![
				word!(
					[HT::Inc, HT::Dec],
					[BT::Out, BT::In],
					[TT::Right, TT::Left],
				),
				word!(
					[HT::Inc, HT::Inc, HT::Debug],
					[BT::Out, BT::JumpLeft, BT::JumpRight],
					[TT::Left, TT::Left, TT::Left],
				),
			]
		))
	)
}

#[test]
fn test_parse_words1_with_empty_input() {
	let code = TokenStream::new();

	assert_eq!(
		parse_words1(code).finish(),
		Err(Error::new(
			TokenStream::new(),
			ErrorKind::Verify
		))
	);
}

#[test]
fn must_match_without_any_newlines() {
	let code = ts![Debug, Left];

	assert_eq!(
		pad_newline(tag(&Debug))(code),
		Ok((ts![Left], ts![Debug])),
	);
}

#[test]
fn must_match_with_newline_at_the_left() {
	let code = ts![NewLine, Debug, Left];

	assert_eq!(
		pad_newline(tag(&Debug))(code),
		Ok((ts![Left], ts![Debug])),
	);
}

#[test]
fn must_match_with_newline_at_the_right() {
	let code = ts![Debug, NewLine, Left];

	assert_eq!(
		pad_newline(tag(&Debug))(code),
		Ok((ts![Left], ts![Debug])),
	);
}

#[test]
fn must_match_newlines_at_both_side() {
	let code = ts![NewLine, Debug, NewLine, Left];

	assert_eq!(
		pad_newline(tag(&Debug))(code),
		Ok((ts![Left], ts![Debug])),
	);
}

#[test]
fn must_fail_if_inner_parser_fails() {
	let code = ts![NewLine, NewLine, Left];

	assert_eq!(
		pad_newline(tag(&Debug))(code).finish(),
		Err(Error::new(
			ts![NewLine, Left],
			ErrorKind::Tag
		)),
	);
}

#[test]
fn newlines_should_not_be_repeated() {
	let code = ts![
		NewLine, NewLine, In, NewLine, NewLine, NewLine, NewLine, In, NewLine
	];

	assert_eq!(
		terminated(many0(pad_newline(tag(&In))), eof)(code.clone()).finish(),
		Err(Error::new(code, ErrorKind::Eof)),
	);
}

#[test]
fn must_match_with_newline_separated_tokens() {
	let code = ts![NewLine, In, NewLine, NewLine, In, NewLine];

	assert_eq!(
		terminated(many0(pad_newline(tag(&In))), eof)(code.clone()).finish(),
		Ok((ts![], vec![ts![In], ts![In]]))
	);
}

#[test]
fn sentences0_must_match_with_empty_token_stream() {
	let code = ts![];

	assert_eq!(
		parse_sentences0(code),
		Ok((ts![], vec![]))
	);
}

#[test]
fn sentences0_must_recognize_single_word_as_single_sentence() {
	let code = ts![In];

	assert_eq!(
		parse_sentences0(code),
		Ok((ts![], vec![vec![word!(, [BT::In],)]]))
	);
}

#[test]
fn sentences0_must_recognize_many_words_as_single_sentence() {
	let code = ts![In, JumpLeft, Debug, Out];

	assert_eq!(
		parse_sentences0(code),
		Ok((
			ts![],
			vec![vec![
				word!(, [BT::In, BT::JumpLeft],),
				word!([HT::Debug], [BT::Out],),
			]]
		))
	);
}

#[test]
fn sentences0_must_recognize_many_words_separated_with_newlines_as_multiple_sentence(
) {
	let code = ts![In, NewLine, JumpLeft, In, Debug, NewLine, Out];

	assert_eq!(
		parse_sentences0(code),
		Ok((
			ts![],
			vec![
				vec![word!(, [BT::In, ],),],
				vec![word!(, [BT::JumpLeft, BT::In],), word!([HT::Debug],,),],
				vec![word!(, [BT::Out],)],
			]
		))
	);
}

#[test]
fn sentences0_must_ignore_trailing_newline() {
	let code = ts![In, NewLine, JumpLeft, In, Debug, NewLine, Out, NewLine];

	assert_eq!(
		parse_sentences0(code),
		Ok((
			ts![],
			vec![
				vec![word!(, [BT::In, ],),],
				vec![word!(, [BT::JumpLeft, BT::In],), word!([HT::Debug],,),],
				vec![word!(, [BT::Out],)],
			]
		))
	);
}

#[test]
fn sentences0_must_ignore_leading_newline() {
	let code = ts![NewLine, In, NewLine, JumpLeft, In, Debug, NewLine, Out];

	assert_eq!(
		parse_sentences0(code),
		Ok((
			ts![],
			vec![
				vec![word!(, [BT::In, ],),],
				vec![word!(, [BT::JumpLeft, BT::In],), word!([HT::Debug],,),],
				vec![word!(, [BT::Out],)],
			]
		))
	);
}

#[test]
fn sentences1_must_fail_to_match_with_empty_token_stream() {
	let code = ts![];

	assert_eq!(
		parse_sentences1(code).finish(),
		Err(Error::new(ts![], ErrorKind::Verify))
	);
}

#[test]
fn sentences1_must_fail_to_match_with_only_newlines() {
	let code = ts![NewLine];

	assert_eq!(
		parse_sentences1(code).finish(),
		Err(Error::new(ts![], ErrorKind::Verify))
	);
}
