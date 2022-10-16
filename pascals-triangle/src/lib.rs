pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: usize) -> Self {
        let rows = (0..row_count).fold(vec![], |mut acc, row| {
            let v = (0..row + 1)
                .map(|col| match col {
                    0 => 1u32,
                    _ => {
                        let last: &Vec<u32> = acc.last().unwrap();
                        last.get(col - 1).map_or(0, |&n| n as u32)
                            + last.get(col).map_or(0, |&n| n as u32)
                    }
                })
                .collect::<Vec<_>>();
            acc.push(v);
            acc
        });

        Self { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
