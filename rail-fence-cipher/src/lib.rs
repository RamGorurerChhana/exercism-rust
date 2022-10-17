pub struct RailFence {
    rails: u32,
}

enum Direction {
    Up,
    Down,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        Self { rails }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut vec = vec![];
        (0..self.rails).for_each(|_| vec.push(vec![None; text.len()]));
        let mut i = 0;
        let mut direction = Direction::Down;
        for (j, ch) in text.chars().enumerate() {
            vec[i][j] = Some(ch);
            match direction {
                Direction::Down => {
                    if i + 1 < self.rails as usize {
                        i += 1;
                    } else {
                        i -= 1;
                        direction = Direction::Up;
                    }
                }
                Direction::Up => {
                    if i > 0 {
                        i -= 1;
                    } else {
                        i += 1;
                        direction = Direction::Down;
                    }
                }
            }
        }

        vec.iter()
            .flat_map(|v| v.iter().filter_map(|&ch| ch))
            .collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut vec = vec![];
        (0..self.rails).for_each(|_| vec.push(vec![None; cipher.len()]));
        let mut i = 0;
        let mut direction = Direction::Down;

        // Step 1: fill with ? character
        for j in 0..cipher.len() {
            vec[i][j] = Some('?');
            match direction {
                Direction::Down => {
                    if i + 1 < self.rails as usize {
                        i += 1;
                    } else {
                        i -= 1;
                        direction = Direction::Up;
                    }
                }
                Direction::Up => {
                    if i > 0 {
                        i -= 1;
                    } else {
                        i += 1;
                        direction = Direction::Down;
                    }
                }
            }
        }

        // Step 2: Replace all ? characters
        let mut chars = cipher.chars();
        for i in 0..self.rails as usize {
            for j in 0..cipher.len() {
                if vec[i][j] == Some('?') {
                    vec[i][j] = chars.next();
                }
            }
        }

        // Step 3: Read all character in zig zag way
        let mut i = 0;
        let mut direction = Direction::Down;
        let mut decoded = String::new();
        for j in 0..cipher.len() {
            decoded.push(vec[i][j].unwrap());
            match direction {
                Direction::Down => {
                    if i + 1 < self.rails as usize {
                        i += 1;
                    } else {
                        i -= 1;
                        direction = Direction::Up;
                    }
                }
                Direction::Up => {
                    if i > 0 {
                        i -= 1;
                    } else {
                        i += 1;
                        direction = Direction::Down;
                    }
                }
            }
        }

        decoded
    }
}
