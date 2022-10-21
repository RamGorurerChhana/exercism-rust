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

#[derive(Debug, Clone)]
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
            _ => match s.parse::<i32>() {
                Ok(v) => Some(Self::Int(v)),
                _ => None,
            },
        }
    }
}

#[derive(Debug)]
pub struct Forth {
    stack: Vec<Value>,
    custom_words: HashMap<String, Vec<Token>>,
}

impl Forth {
    pub fn new() -> Forth {
        Self {
            stack: vec![],
            custom_words: HashMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        self.stack.as_slice()
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let input = input.to_ascii_lowercase();
        let mut iter = input.split_ascii_whitespace();
        while let Some(t) = iter.next() {
            // try execute the custom word if present
            let tokens = self.custom_words.get(t);
            if tokens.is_some() {
                let tokens = tokens.unwrap();
                // TODO: How to do it without clone of tokens vector?
                let tokens = tokens.iter().cloned().collect::<Vec<_>>();
                for token in tokens {
                    let _ = self.execute_token(&mut iter, &token)?;
                }
                continue;
            }
            // tokenize
            let token = Token::tokenize(&t).ok_or(Error::UnknownWord)?;
            // execute token
            let _ = self.execute_token(&mut iter, &token)?;
        }

        Ok(())
    }

    fn execute_token<'a>(
        &mut self,
        iter: &mut impl Iterator<Item = &'a str>,
        token: &Token,
    ) -> Result {
        match token {
            Token::Add => {
                let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let b = self.stack.pop().ok_or(Error::StackUnderflow)?;
                self.stack.push(a + b);
                Ok(())
            }
            Token::Sub => {
                let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let b = self.stack.pop().ok_or(Error::StackUnderflow)?;
                self.stack.push(b - a);
                Ok(())
            }
            Token::Mul => {
                let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let b = self.stack.pop().ok_or(Error::StackUnderflow)?;
                self.stack.push(b * a);
                Ok(())
            }
            Token::Div => {
                let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let b = self.stack.pop().ok_or(Error::StackUnderflow)?;
                if a == 0 {
                    Err(Error::DivisionByZero)
                } else {
                    self.stack.push(b / a);
                    Ok(())
                }
            }
            Token::Dup => {
                let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
                self.stack.push(a);
                self.stack.push(a);
                Ok(())
            }
            Token::Drop => {
                let _ = self.stack.pop().ok_or(Error::StackUnderflow)?;
                Ok(())
            }
            Token::Swap => {
                let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let b = self.stack.pop().ok_or(Error::StackUnderflow)?;
                self.stack.push(a);
                self.stack.push(b);
                Ok(())
            }
            Token::Over => {
                let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let b = self.stack.pop().ok_or(Error::StackUnderflow)?;
                self.stack.push(b);
                self.stack.push(a);
                self.stack.push(b);
                Ok(())
            }
            Token::Int(v) => {
                self.stack.push(*v);
                Ok(())
            }
            Token::Colon => {
                let custom_word = iter.next().ok_or(Error::InvalidWord)?.to_string();
                if custom_word.parse::<i32>().is_ok() {
                    return Err(Error::InvalidWord);
                }
                let mut tokens = vec![];
                while let Some(t) = iter.next() {
                    if let Some(v) = self.custom_words.get(t) {
                        for t in v {
                            tokens.push(t.clone());
                        }
                    } else {
                        let token = Token::tokenize(t).ok_or(Error::InvalidWord)?;
                        match token {
                            Token::SemiColon => {
                                tokens.push(token);
                                break;
                            },
                            _ => tokens.push(token),
                        }
                    }
                }
                match tokens.pop().ok_or(Error::InvalidWord)? {
                    Token::SemiColon => {},
                    _ => return Err(Error::InvalidWord)
                };
                self.custom_words.insert(custom_word, tokens);
                Ok(())
            }
            Token::SemiColon => Err(Error::UnknownWord),
        }
    }
}
