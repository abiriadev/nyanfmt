struct Root;

#[derive(Clone, Debug, PartialEq)]
pub enum HeadTok {
	Inc,
	Dec,
	Debug,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Head(pub Vec<HeadTok>);

#[derive(Clone, Debug, PartialEq)]
pub enum BodyTok {
	Out,
	In,
	JumpRight,
	JumpLeft,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Body(pub Vec<BodyTok>);

#[derive(Clone, Debug, PartialEq)]
pub enum TailTok {
	Right,
	Left,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tail(pub Vec<TailTok>);

#[derive(Clone, Debug, PartialEq)]
pub struct Word {
	pub head: Option<Head>,
	pub body: Option<Body>,
	pub tail: Option<Tail>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Sentence(pub Vec<Word>);

#[derive(Clone, Debug, PartialEq)]
pub struct Comment(pub String);
