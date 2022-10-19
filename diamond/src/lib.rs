struct Diamond {
    letter: char,
    total_row: usize,
    curr_row: usize,
    stay_same_row: bool,
}

impl Diamond {
    fn new(ch: char) -> Self {
        let size = (ch as u8 - 'A' as u8 + 1) as usize;
        let total_row = 2 * size - 1;
        Self {
            letter: 'A',
            total_row,
            curr_row: 0,
            stay_same_row: false,
        }
    }

    fn get_next_letter(&self) -> char {
        let mid = (self.total_row - 1) / 2;
        if self.curr_row > mid {
            (self.letter as u8 - 1) as char
        } else {
            (self.letter as u8 + 1) as char
        }
    }

    fn to_vec(&mut self) -> Vec<String> {
        let mut vec = vec![vec![' '; self.total_row]; self.total_row];
        for (i, j, ch) in self {
            vec[i][j] = ch;
        }

        vec.iter().map(|v| v.iter().collect::<String>()).collect()
    }
}

impl Iterator for Diamond {
    type Item = (usize, usize, char);
    fn next(&mut self) -> Option<Self::Item> {
        let mid = (self.total_row - 1) / 2;
        if self.curr_row >= self.total_row {
            return None;
        }
        let i = self.curr_row;
        let j: usize;
        let letter = self.letter;

        if self.curr_row == 0 {
            j = mid;
            self.curr_row += 1;
            self.stay_same_row = false;
            self.letter = self.get_next_letter();
        } else if self.curr_row == self.total_row - 1 {
            j = mid;
            self.curr_row += 1;
        } else if self.curr_row <= mid {
            j = if self.stay_same_row {
                self.curr_row += 1;
                self.letter = self.get_next_letter();
                mid + i
            } else {
                mid - i
            };
            self.stay_same_row = !self.stay_same_row;
        } else {
            j = if self.stay_same_row {
                self.curr_row += 1;
                self.letter = self.get_next_letter();
                mid + (self.total_row - 1 - i)
            } else {
                mid - (self.total_row - 1 - i)
            };
            self.stay_same_row = !self.stay_same_row;
        }


        Some((i, j, letter))
    }
}

pub fn get_diamond(c: char) -> Vec<String> {
    Diamond::new(c).to_vec()
}
