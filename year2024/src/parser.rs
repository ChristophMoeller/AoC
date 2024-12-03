pub mod prelude {
    pub use super::AndThen;
    pub use super::Or;
    pub use super::Repeated;
    pub use super::Tokenizer;
}

pub trait Tokenizer<T: Copy>: Sized {
    fn tokenize<'a>(&self, input: &'a str, output: &mut Vec<T>) -> &'a str;

    fn then<Other: Tokenizer<T>>(self, other: Other) -> AndThen<T, Self, Other> {
        AndThen {
            first: self,
            second: other,
            _marker: std::marker::PhantomData,
        }
    }

    fn or<Other: Tokenizer<T>>(self, other: Other) -> Or<T, Self, Other> {
        Or {
            first: self,
            second: other,
            _marker: std::marker::PhantomData,
        }
    }

    fn repeated(self) -> Repeated<T, Self, 0> {
        Repeated {
            token: self,
            _marker: std::marker::PhantomData,
        }
    }

    fn repeated1(self) -> Repeated<T, Self, 1> {
        Repeated {
            token: self,
            _marker: std::marker::PhantomData,
        }
    }

    fn ignore(self) -> Ignore<Self> {
        Ignore { t: self }
    }

    fn process<'a>(&self, input: &'a str) -> (Vec<T>, &'a str) {
        let mut buffer = Vec::new();
        let remainder = self.tokenize(input, &mut buffer);
        (buffer, remainder)
    }
}

#[derive(Clone)]
pub struct Tag<T> {
    tag: String,
    token: T,
}

impl<T> Tag<T> {
    pub fn new(tag: String, token: T) -> Self {
        Self { tag, token }
    }
}

impl<T: Copy> Tokenizer<T> for Tag<T> {
    fn tokenize<'a>(&self, input: &'a str, output: &mut Vec<T>) -> &'a str {
        let mut input_chars = input.chars();
        let mut rem = self.tag.chars();
        loop {
            let Some(next_rem) = rem.next() else {
                output.push(self.token);
                return &input[self.tag.len()..];
            };
            let Some(next_in) = input_chars.next() else {
                return input;
            };
            if next_rem != next_in {
                return input;
            }
        }
    }
}

#[derive(Clone)]
pub struct Conditional<T, C: Fn(usize, char) -> bool, F: Fn(&str) -> T, const EMPTY: bool> {
    condition: C,
    finalize: F,
}

impl<T, C: Fn(usize, char) -> bool, F: Fn(&str) -> T, const EMPTY: bool>
    Conditional<T, C, F, EMPTY>
{
    pub fn new(condition: C, finalize: F) -> Self {
        Self {
            condition,
            finalize,
        }
    }
}

impl<T: Copy, C: Fn(usize, char) -> bool, F: Fn(&str) -> T, const EMPTY: bool> Tokenizer<T>
    for Conditional<T, C, F, EMPTY>
{
    fn tokenize<'a>(&self, input: &'a str, output: &mut Vec<T>) -> &'a str {
        let mut input_chars = input.chars();
        let mut len = 0;
        let mut position = 0;
        loop {
            let Some(next) = input_chars.next() else {
                break;
            };
            if (self.condition)(position, next) {
                len += next.len_utf8();
                position += 1;
                continue;
            }
            break;
        }
        if !EMPTY && len == 0 {
            return input;
        }
        output.push((self.finalize)(&input[0..len]));
        &input[len..]
    }
}

#[derive(Clone)]
pub struct Any {}

impl<T: Copy> Tokenizer<T> for Any {
    fn tokenize<'a>(&self, input: &'a str, _: &mut Vec<T>) -> &'a str {
        &input[input.chars().next().map(char::len_utf8).unwrap_or(0)..]
    }
}

#[derive(Clone)]
pub struct Ignore<A> {
    t: A,
}
impl<T: Copy, A: Tokenizer<T>> Tokenizer<T> for Ignore<A> {
    fn tokenize<'a>(&self, input: &'a str, output: &mut Vec<T>) -> &'a str {
        let start = output.len();
        let rem = self.t.tokenize(input, output);
        output.truncate(start);
        rem
    }
}

#[derive(Clone)]
pub struct Number<T, I, F: Fn(I) -> T, const SIGNED: bool> {
    finalize: F,
    radix: u32,
    _marker_t: std::marker::PhantomData<T>,
    _marker_i: std::marker::PhantomData<I>,
}

impl<T: Copy, I: std::str::FromStr, F: Fn(I) -> T, const SIGNED: bool> Tokenizer<T>
    for Number<T, I, F, SIGNED>
where
    <I as std::str::FromStr>::Err: std::fmt::Debug,
{
    fn tokenize<'a>(&self, input: &'a str, output: &mut Vec<T>) -> &'a str {
        let mut input_chars = input.chars().peekable();
        let mut len = 0;
        if SIGNED {
            let Some(c) = input_chars.peek() else {
                return input;
            };
            if *c != '-' && !c.is_digit(self.radix) {
                return input;
            }
            if *c == '-' {
                input_chars.next();
                len += '-'.len_utf8();
            }
        }
        let Some(c) = input_chars.peek() else {
            return input;
        };
        if !c.is_digit(self.radix) {
            return input;
        }
        len += input_chars
            .take_while(|c| c.is_digit(self.radix))
            .map(|c| c.len_utf8())
            .sum::<usize>();
        output.push((self.finalize)(
            (&input[..len])
                .parse::<I>()
                .expect("Unable to parse number!"),
        ));
        &input[len..]
    }
}

pub fn unsigned<T: Copy, F: Fn(u64) -> T>(radix: u32, finalize: F) -> Number<T, u64, F, false> {
    Number {
        finalize,
        radix,
        _marker_t: Default::default(),
        _marker_i: Default::default(),
    }
}

pub fn signed<T: Copy, F: Fn(i64) -> T>(radix: u32, finalize: F) -> Number<T, i64, F, true> {
    Number {
        finalize,
        radix,
        _marker_t: Default::default(),
        _marker_i: Default::default(),
    }
}

pub fn tag<T>(tag: &str, token: T) -> Tag<T> {
    Tag::<T>::new(tag.to_owned(), token)
}

pub fn linebreak<T>(token: T) -> Tag<T> {
    tag("\n", token)
}

pub fn double_linebreak<T>(token: T) -> Tag<T> {
    tag("\n\n", token)
}

pub fn any() -> Any {
    Any {}
}

pub fn ignore<T>(token: T) -> Ignore<T> {
    Ignore { t: token }
}

pub fn conditional0_pos<T: Copy, C: Fn(usize, char) -> bool, F: Fn(&str) -> T>(
    condition: C,
    finalize: F,
) -> Conditional<T, C, F, true> {
    Conditional::new(condition, finalize)
}

pub fn conditional1_pos<T: Copy, C: Fn(usize, char) -> bool, F: Fn(&str) -> T>(
    condition: C,
    finalize: F,
) -> Conditional<T, C, F, false> {
    Conditional::new(condition, finalize)
}

pub fn conditional0<T: Copy, C: Fn(char) -> bool, F: Fn(&str) -> T>(
    condition: C,
    finalize: F,
) -> Conditional<T, impl Fn(usize, char) -> bool, F, true> {
    Conditional::new(move |_, c| condition(c), finalize)
}

pub fn conditional1<T: Copy, C: Fn(char) -> bool, F: Fn(&str) -> T>(
    condition: C,
    finalize: F,
) -> Conditional<T, impl Fn(usize, char) -> bool, F, false> {
    Conditional::new(move |_, c| condition(c), finalize)
}

pub fn whitespace0<T: Copy>(
    token: T,
) -> Conditional<T, impl Fn(usize, char) -> bool, impl Fn(&str) -> T, false> {
    conditional1(|c| c.is_whitespace(), move |_| token)
}

pub fn whitespace1<T: Copy>(
    token: T,
) -> Conditional<T, impl Fn(usize, char) -> bool, impl Fn(&str) -> T, false> {
    conditional1(|c| c.is_whitespace(), move |_| token)
}

#[derive(Clone)]
pub struct AndThen<T: Copy, A: Tokenizer<T>, B: Tokenizer<T>> {
    first: A,
    second: B,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Copy, A: Tokenizer<T>, B: Tokenizer<T>> Tokenizer<T> for AndThen<T, A, B> {
    fn tokenize<'a>(&self, input: &'a str, output: &mut Vec<T>) -> &'a str {
        let start = output.len();
        let input_a = self.first.tokenize(input, output);
        if input_a == input {
            output.truncate(start);
            return input;
        }
        let input_b = self.second.tokenize(input_a, output);
        if input_b == input_a {
            output.truncate(start);
            return input;
        }
        input_b
    }
}

#[derive(Clone)]
pub struct Or<T: Copy, A: Tokenizer<T>, B: Tokenizer<T>> {
    first: A,
    second: B,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Copy, A: Tokenizer<T>, B: Tokenizer<T>> Tokenizer<T> for Or<T, A, B> {
    fn tokenize<'a>(&self, input: &'a str, output: &mut Vec<T>) -> &'a str {
        let start = output.len();
        let input_a = self.first.tokenize(input, output);
        if input_a != input {
            return input_a;
        }
        let input_b = self.second.tokenize(input, output);
        if input_b != input {
            return input_b;
        }
        output.truncate(start);
        return input;
    }
}

#[derive(Clone)]
pub struct Repeated<T: Copy, A: Tokenizer<T>, const MIN: usize> {
    token: A,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Copy, A: Tokenizer<T>, const MIN: usize> Tokenizer<T> for Repeated<T, A, MIN> {
    fn tokenize<'a>(&self, input: &'a str, output: &mut Vec<T>) -> &'a str {
        let start = output.len();
        let mut counter = 0;
        let mut current_input = input;
        loop {
            let last_input = current_input;
            current_input = self.token.tokenize(last_input, output);
            if current_input != last_input {
                counter += 1;
                continue;
            }
            if counter < MIN {
                output.truncate(start);
                return input;
            }
            return current_input;
        }
    }
}

pub trait Parser<T> {
    type Output;

    fn parse<'a>(&mut self, input: &'a [T]) -> (Option<Self::Output>, &'a [T]);
}

pub struct ParserUntilFn<T, O, C, F> {
    condition: C,
    f: F,
    exhaust: bool,
    remove_end: bool,
    include_end: bool,
    _marker_t: std::marker::PhantomData<T>,
    _marker_o: std::marker::PhantomData<O>,
}

impl<T, O, C: FnMut(&T) -> bool, F: FnMut(&[T]) -> Option<O>> ParserUntilFn<T, O, C, F> {
    pub fn new(condition: C, f: F, exhaust: bool, remove_end: bool, include_end: bool) -> Self {
        Self {
            condition,
            f,
            exhaust,
            remove_end,
            include_end,
            _marker_t: std::marker::PhantomData,
            _marker_o: std::marker::PhantomData,
        }
    }
}

impl<T, O, C: FnMut(&T) -> bool, F: FnMut(&[T]) -> Option<O>> Parser<T>
    for ParserUntilFn<T, O, C, F>
{
    type Output = O;

    fn parse<'a>(&mut self, input: &'a [T]) -> (Option<Self::Output>, &'a [T]) {
        let mut input_iter = input.iter();
        let mut counter = 0;

        while let Some(x) = input_iter.next() {
            if !(self.condition)(x) {
                break;
            }
            counter += 1;
        }

        if !self.exhaust && counter == input.len() {
            return (None, &input[counter..]);
        }

        let output = if self.include_end {
            let end = (counter + 1).min(input.len());
            (self.f)(&input[..end])
        } else {
            (self.f)(&input[..counter])
        };

        let remainder = if self.remove_end {
            let end = (counter + 1).min(input.len());
            &input[end..]
        } else {
            &input[counter..]
        };

        (output, remainder)
    }
}

pub struct ParserRepeat<T, P: Parser<T>> {
    p: P,
    _marker: std::marker::PhantomData<T>,
}

impl<T, P: Parser<T>> ParserRepeat<T, P> {
    pub fn new(p: P) -> Self {
        Self {
            p,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T, P: Parser<T>> Parser<T> for ParserRepeat<T, P> {
    type Output = Vec<P::Output>;

    fn parse<'a>(&mut self, mut input: &'a [T]) -> (Option<Self::Output>, &'a [T]) {
        let mut current;
        let mut result = Vec::new();
        loop {
            (current, input) = self.p.parse(input);
            if let Some(v) = current.take() {
                result.push(v);
            }
            if input.len() == 0 {
                break;
            }
        }
        return (Some(result), input);
    }
}
