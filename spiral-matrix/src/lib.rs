enum Direction {
    Right,
    Down,
    Left,
    Up,
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut vec:Vec<Vec<u32>> = (0..size).map(|_| (0..size).map(|_| 0).collect()).collect();
    let mut i = 0;
    let mut j = 0;
    let mut d = Direction::Right;
    (0..size * size).for_each(|n| {
        vec[i][j] = n + 1;
        let size = size as usize;
        match d {
            Direction::Right => {
                if j + 1 < size && vec[i][j + 1] == 0 {
                    j += 1;
                } else {
                    i += 1;
                    d = Direction::Down;
                }
            }
            Direction::Down => {
                if i + 1 < size && vec[i + 1][j] == 0 {
                    i += 1;
                } else {
                    j -= 1;
                    d = Direction::Left;
                }
            }
            Direction::Left => {
                if j > 0 && vec[i][j - 1] == 0 {
                    j -= 1;
                } else {
                    i -= 1;
                    d = Direction::Up;
                }
            }
            Direction::Up => {
                if i > 0 && vec[i - 1][j] == 0 {
                    i -= 1;
                } else {
                    j += 1;
                    d = Direction::Right;
                }
            }
        }
    });

    vec
}
