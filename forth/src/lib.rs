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
    Custom(u32),
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
struct CustomWords {
    word_ids: HashMap<u32, String>,
    custom_words: HashMap<String, Vec<Token>>
}

impl CustomWords {
    fn new() -> Self {
        Self { word_ids: HashMap::new(), custom_words: HashMap::new() }
    }

    fn is_known_word(&self, k: &str) -> bool {
        self.custom_words.contains_key(k)
    }

    fn get_by_id(&self, id: &u32) -> Option<&String> {
        self.word_ids.get(id)
    }

    fn get_tokens(&self, k: &str) -> Option<&Vec<Token>>{
        self.custom_words.get(k)
    }
}


#[derive(Debug)]
pub struct Forth {
    stack: Vec<Value>,
    custom_words: CustomWords
}

impl Forth  {
    pub fn new() -> Self {
        Self { stack: vec![], custom_words: CustomWords::new()}
    }

    pub fn stack(&self) -> &[Value] {
        self.stack.as_slice()
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let input = input.to_ascii_lowercase();
        let mut iter = input.split_ascii_whitespace();
        while let Some(t) = iter.next() {
            if self.custom_words.is_known_word(t) {
                let _ = Self::execute_custom_word(t, &self.custom_words, &mut self.stack)?;
                continue;
            }

            let token = Token::tokenize(t).ok_or(Error::UnknownWord)?;
            match token {
                Token::Colon => self.execute_definition(&mut iter)?,
                _ => Self::execute_token(&mut self.stack, &token)?
            };
        }


        Ok(())
    }

    fn execute_custom_word(token: &str, custom_words: &CustomWords, stack: &mut Vec<Value>) -> Result {
        let tokens = custom_words.get_tokens(token).ok_or(Error::UnknownWord)?;
        for token in tokens {
            match token {
                Token::Custom(id) => Self::execute_custom_token(id, custom_words, stack)?,
                _ =>Self::execute_token(stack, token)?
            }
        }

        Ok(())
    }


    // fn execute_custom_word(&mut self, token: &str) -> Result{
    //     let tokens = self.custom_words.get_tokens(token).ok_or(Error::UnknownWord)?;
    //     for token in tokens {
    //         match token {
    //             Token::Custom(id) => {},
    //             _ =>Self::execute_token(&mut self.stack, token)?
    //         }
    //     }
    //     Ok(())
    // }

    fn execute_token(stack: &mut Vec<Value>, token: &Token) -> Result{
        match token {
            Token::Add => Self::execute_add(stack),
            Token::Sub => Self::execute_sub(stack),
            Token::Mul => Self::execute_mul(stack),
            Token::Div => Self::execute_div(stack),
            Token::Dup => Self::execute_dup(stack),
            Token::Drop => Self::execute_drop(stack),
            Token::Swap => Self::execute_swap(stack),
            Token::Over => Self::execute_over(stack),
            Token::Int(n) => Self::execute_int(stack, *n),
            // Token::Custom(id) => self.execute_custom_token(id),
            _ => Err(Error::UnknownWord)
        }
        
    }

    fn execute_add(stack: &mut Vec<Value>) -> Result {
        let a = stack.pop().ok_or(Error::StackUnderflow)?;
        let b = stack.pop().ok_or(Error::StackUnderflow)?;
        stack.push(a + b);
        Ok(())
    }

    fn execute_sub(stack: &mut Vec<Value>) -> Result {
        let a = stack.pop().ok_or(Error::StackUnderflow)?;
        let b = stack.pop().ok_or(Error::StackUnderflow)?;
        stack.push(b - a);
        Ok(())
    }

    fn execute_mul(stack: &mut Vec<Value>) -> Result {
        let a = stack.pop().ok_or(Error::StackUnderflow)?;
        let b = stack.pop().ok_or(Error::StackUnderflow)?;
        stack.push(b * a);
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
        let _ = stack.pop().ok_or(Error::StackUnderflow)?;
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

    fn execute_custom_token(id: &u32, custom_words: &CustomWords, stack: &mut Vec<Value>) -> Result {
        let token = custom_words.get_by_id(id).ok_or(Error::UnknownWord)?;
        Self::execute_custom_word(token, custom_words, stack)?;
        Ok(())
    }

    fn execute_definition<'a>(&mut self, iter: &mut impl Iterator<Item = &'a str>) -> Result {
        self.stack.push(56);

        Ok(())
    }

}
