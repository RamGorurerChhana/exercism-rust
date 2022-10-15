pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let get_count = |i: usize, j: usize| {
        let mut count = 0_u32;
        let len = minefield.len();
        if i > 0 {
            if let Some(42) = minefield[i - 1].bytes().nth(j) {
                count += 1;
            }
            if let Some(42) = minefield[i - 1].bytes().nth(j + 1) {
                count += 1;
            }
            if j > 0 {
                if let Some(42) = minefield[i - 1].bytes().nth(j - 1) {
                    count += 1;
                }
            }
        }
        if let Some(42) = minefield[i].bytes().nth(j + 1) {
            count += 1;
        }
        if j > 0 {
            if let Some(42) = minefield[i].bytes().nth(j - 1) {
                count += 1;
            }
        }
        if i < len - 1 {
            if let Some(42) = minefield[i + 1].bytes().nth(j) {
                count += 1;
            }
            if let Some(42) = minefield[i + 1].bytes().nth(j + 1) {
                count += 1;
            }
            if j > 0 {
                if let Some(42) = minefield[i + 1].bytes().nth(j - 1) {
                    count += 1;
                }
            }
        }

        if count == 0 {
            ' '
        } else {
            char::from_digit(count, 10).unwrap()
        }
    };

    minefield
        .into_iter()
        .enumerate()
        .map(|(i, s)| {
            s.bytes()
                .enumerate()
                .map(|(j, b)| match b {
                    32 => get_count(i, j),
                    _ => b as char,
                })
                .collect::<String>()
        })
        .collect()
}
