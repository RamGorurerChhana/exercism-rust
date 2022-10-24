use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(Debug)]
enum Token {
    Int(Value),
    Add,
    Sub,
    Mul,
    Div,
    Dup,
    Drop,
    Swap,
    Over,
    Colon,
    SemiColon,
    Custom(usize),
}

impl Token {
    fn tokenize(s: &str) -> Option<Self> {
        match s {
            ":" => Some(Self::Colon),
            ";" => Some(Self::SemiColon),
            "+" => Some(Self::Add),
            "-" => Some(Self::Sub),
            "*" => Some(Self::Mul),
            "/" => Some(Self::Div),
            "dup" => Some(Self::Dup),
            "drop" => Some(Self::Drop),
            "swap" => Some(Self::Swap),
            "over" => Some(Self::Over),
            _ => s.parse::<i32>().map(Self::Int).ok(),
        }
    }
}

#[derive(Debug, Default)]
struct CustomWords {
    word_ids: HashMap<String, usize>,
    custom_words: Vec<Vec<Token>>,
}

impl CustomWords {
    fn new() -> Self {
        Self {
            word_ids: HashMap::new(),
            custom_words: vec![],
        }
    }

    fn is_known_word(&self, k: &str) -> bool {
        self.word_ids.contains_key(k)
    }

    fn get_tokens(&self, k: &str) -> Option<&Vec<Token>> {
        let id = self.word_ids.get(k)?;
        self.custom_words.get(*id)
    }

    fn get_by_id(&self, id: usize) -> Option<&Vec<Token>> {
        self.custom_words.get(id)
    }

    fn get_custom_token(&self, k: &str) -> Option<Token> {
        let id = self.word_ids.get(k)?;
        Some(Token::Custom(*id))
    }

    fn insert_word(&mut self, k: &str, tokens: Vec<Token>) -> Result {
        // custom word name cannot be a number
        if k.parse::<i32>().is_ok() {
            return Err(Error::InvalidWord);
        }
        let id = self.custom_words.len();
        self.word_ids.insert(k.to_string(), id);
        self.custom_words.push(tokens);

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct Forth {
    stack: Vec<Value>,
    custom_words: CustomWords,
}

impl Forth {
    pub fn new() -> Self {
        Self {
            stack: vec![],
            custom_words: CustomWords::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        self.stack.as_slice()
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let input = input.to_ascii_lowercase();
        let mut iter = input.split_ascii_whitespace();
        while let Some(t) = iter.next() {
            if self.custom_words.is_known_word(t) {
                execute_custom_word(t, &self.custom_words, &mut self.stack)?;
                continue;
            }

            let token = Token::tokenize(t).ok_or(Error::UnknownWord)?;
            match token {
                Token::Colon => self.create_definition(&mut iter)?,
                _ => execute_token(&token, &self.custom_words, &mut self.stack)?,
            };
        }

        Ok(())
    }

    fn create_definition<'a>(&mut self, iter: &mut impl Iterator<Item = &'a str>) -> Result {
        // expect the custom word name
        let word = iter.next().ok_or(Error::InvalidWord)?;

        // collect all tokens in the vector
        let mut tokens = vec![];
        for t in iter.by_ref() {
            if self.custom_words.is_known_word(t) {
                // create Custom token if it is a known word
                let token = self
                    .custom_words
                    .get_custom_token(t)
                    .ok_or(Error::InvalidWord)?;
                // collect the Custom token in the token vec
                tokens.push(token);
                continue;
            }

            // parse the string to a valid token
            let token = Token::tokenize(t).ok_or(Error::InvalidWord)?;
            match token {
                // if semi colon is received then we must break the loop
                Token::SemiColon => {
                    tokens.push(token);
                    break;
                }
                // if another colon is received then it is error
                Token::Colon => return Err(Error::InvalidWord),
                // push all other valid tokens in the vec
                _ => tokens.push(token),
            }
        }
        // pop the last inserted semi colon and also check the token vec is not empty
        // last token in the vec must always be semi colon
        match tokens.pop().ok_or(Error::InvalidWord)? {
            Token::SemiColon => {}
            _ => return Err(Error::InvalidWord),
        };

        // insert the new custom word to known_word list
        self.custom_words.insert_word(word, tokens)
    }
}

fn execute_custom_word(token: &str, custom_words: &CustomWords, stack: &mut Vec<Value>) -> Result {
    let tokens = custom_words.get_tokens(token).ok_or(Error::UnknownWord)?;
    for token in tokens {
        execute_token(token, custom_words, stack)?;
    }

    Ok(())
}

fn execute_token(token: &Token, custom_words: &CustomWords, stack: &mut Vec<Value>) -> Result {
    match token {
        Token::Add => execute_binary_op(stack, |a, b| a + b),
        Token::Sub => execute_binary_op(stack, |a, b| a - b),
        Token::Mul => execute_binary_op(stack, |a, b| a * b),
        Token::Div => execute_div(stack),
        Token::Dup => execute_dup(stack),
        Token::Drop => execute_drop(stack),
        Token::Swap => execute_swap(stack),
        Token::Over => execute_over(stack),
        Token::Int(n) => execute_int(stack, *n),
        Token::Custom(id) => execute_custom_token(*id, custom_words, stack),
        _ => Err(Error::UnknownWord),
    }
}

fn execute_binary_op(stack: &mut Vec<Value>, f: fn(i32, i32) -> i32) -> Result {
    let a = stack.pop().ok_or(Error::StackUnderflow)?;
    let b = stack.pop().ok_or(Error::StackUnderflow)?;
    stack.push(f(b, a));
    Ok(())
}

fn execute_div(stack: &mut Vec<Value>) -> Result {
    let a = stack.pop().ok_or(Error::StackUnderflow)?;
    let b = stack.pop().ok_or(Error::StackUnderflow)?;
    if a == 0 {
        Err(Error::DivisionByZero)
    } else {
        stack.push(b / a);
        Ok(())
    }
}

fn execute_dup(stack: &mut Vec<Value>) -> Result {
    let a = stack.pop().ok_or(Error::StackUnderflow)?;
    stack.push(a);
    stack.push(a);
    Ok(())
}

fn execute_drop(stack: &mut Vec<Value>) -> Result {
    stack.pop().ok_or(Error::StackUnderflow)?;
    Ok(())
}

fn execute_swap(stack: &mut Vec<Value>) -> Result {
    let a = stack.pop().ok_or(Error::StackUnderflow)?;
    let b = stack.pop().ok_or(Error::StackUnderflow)?;
    stack.push(a);
    stack.push(b);
    Ok(())
}

fn execute_over(stack: &mut Vec<Value>) -> Result {
    let a = stack.pop().ok_or(Error::StackUnderflow)?;
    let b = stack.pop().ok_or(Error::StackUnderflow)?;
    stack.push(b);
    stack.push(a);
    stack.push(b);
    Ok(())
}

fn execute_int(stack: &mut Vec<Value>, n: Value) -> Result {
    stack.push(n);
    Ok(())
}

fn execute_custom_token(id: usize, custom_words: &CustomWords, stack: &mut Vec<Value>) -> Result {
    let tokens = custom_words.get_by_id(id).ok_or(Error::UnknownWord)?;
    for token in tokens {
        execute_token(token, custom_words, stack)?;
    }
    Ok(())
}
