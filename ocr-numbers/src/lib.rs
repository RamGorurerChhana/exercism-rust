// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

type OCRNum<'a> = [&'a str; 4];

enum OCROutput {
    Comma,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    QuestionMark,
}

impl<'a> From<OCRNum<'a>> for OCROutput {
    fn from(num: OCRNum) -> Self {
        match (num[0], num[1], num[2], num[3]) {
            ("", "", "", "") => Self::Comma,
            (" _ ", "| |", "|_|", "   ") => Self::Zero,
            ("   ", "  |", "  |", "   ") => Self::One,
            (" _ ", " _|", "|_ ", "   ") => Self::Two,
            (" _ ", " _|", " _|", "   ") => Self::Three,
            ("   ", "|_|", "  |", "   ") => Self::Four,
            (" _ ", "|_ ", " _|", "   ") => Self::Five,
            (" _ ", "|_ ", "|_|", "   ") => Self::Six,
            (" _ ", "  |", "  |", "   ") => Self::Seven,
            (" _ ", "|_|", "|_|", "   ") => Self::Eight,
            (" _ ", "|_|", " _|", "   ") => Self::Nine,
            _ => Self::QuestionMark,
        }
    }
}

impl From<OCROutput> for char {
    fn from(num: OCROutput) -> Self {
        match num {
            OCROutput::Comma => ',',
            OCROutput::Zero => '0',
            OCROutput::One => '1',
            OCROutput::Two => '2',
            OCROutput::Three => '3',
            OCROutput::Four => '4',
            OCROutput::Five => '5',
            OCROutput::Six => '6',
            OCROutput::Seven => '7',
            OCROutput::Eight => '8',
            OCROutput::Nine => '9',
            OCROutput::QuestionMark => '?',
        }
    }
}

struct OCRInput<'a> {
    input: Vec<&'a str>,
    curr_row: usize,
    curr_col: usize,
}

impl<'a> OCRInput<'a> {
    fn new(input: &'a str) -> Result<Self, Error> {
        let input = input
            .split('\n')
            .map(|s| match s.len() % 3 == 0 {
                true => Ok(s),
                false => Err(Error::InvalidColumnCount(s.len())),
            })
            .collect::<Result<Vec<_>, _>>()?;

        match input.len() % 4 == 0 {
            true => Ok(Self {
                input,
                curr_col: 0,
                curr_row: 0,
            }),
            false => Err(Error::InvalidRowCount(input.len())),
        }
    }
}

impl<'a> Iterator for OCRInput<'a> {
    type Item = OCRNum<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut item: OCRNum = [""; 4];
        // reached end of line, jump 4 lines and start at the beginning
        if self.curr_col == self.input.get(self.curr_row)?.len() {
            // when there are no more lines to jump to
            if self.curr_row + 4 >= self.input.len() {
                return None;
            }
            // jump 4 lines and start at the beginning
            self.curr_row += 4;
            self.curr_col = 0;
            // return blanks to indicate end of line
            return Some(item);
        }
        // collect all str from 4 lines each 3 chars wide
        for i in 0..4 {
            let s = self
                .input
                .get(self.curr_row + i)?
                .get(self.curr_col..self.curr_col + 3)?;
            item[i] = s;
        }
        // increment curr_col, indicate where to start next time
        self.curr_col += 3;
        Some(item)
    }
}

pub fn convert(input: &str) -> Result<String, Error> {
    OCRInput::new(input)?
        .into_iter()
        .map(|n| OCROutput::from(n))
        .map(|n| Ok(char::from(n)))
        .collect()
}
