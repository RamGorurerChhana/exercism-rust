pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let check_row = |v: &Vec<u64>, e: u64| v.into_iter().all(|&x| e >= x);
    let check_col = |j: usize, e: u64| {
        input
            .into_iter()
            .all(|v| v.into_iter().nth(j).unwrap() >= &e)
    };

    input
        .into_iter()
        .enumerate()
        .flat_map(|(i, v)| {
            v.into_iter()
                .enumerate()
                .map(move |(j, &e)| {
                    if check_row(v, e) && check_col(j, e) {
                        Some((i, j))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .filter(Option::is_some)
        .map(|v| v.unwrap())
        .collect()
}
