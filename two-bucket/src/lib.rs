#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let mut moves = 0;
    let mut content_1 = 0;
    let mut content_2 = 0;
    if *start_bucket == Bucket::One {
        content_1 = capacity_1;
        moves += 1;
    } else {
        content_2 = capacity_2;
        moves += 1;
    }

    loop {
        if content_1 == goal {
            let bucket_stats = BucketStats {
                moves,
                goal_bucket: Bucket::One,
                other_bucket: content_2,
            };
            return Some(bucket_stats);
        }
        if content_2 == goal {
            let bucket_stats = BucketStats {
                moves,
                goal_bucket: Bucket::Two,
                other_bucket: content_1,
            };
            return Some(bucket_stats);
        }
        if content_1 == 0 && content_2 == 0 {
            return None;
        }
        if *start_bucket == Bucket::One {
            // when other bucket has the exact measure
            if capacity_2 == goal {
                content_2 = capacity_2;
                moves += 1;
                continue;
            }

            if content_2 == capacity_2 {
                // empty bucket 2
                content_2 = 0;
                moves += 1;
                continue;
            }
            if content_2 < capacity_2 && content_1 > 0 {
                // pour from bucket 1 to bucket 2
                let space_left = capacity_2 - content_2;
                if space_left >= content_1 {
                    content_2 += content_1;
                    content_1 = 0;
                    moves += 1;
                } else {
                    content_2 += space_left;
                    content_1 -= space_left;
                    moves += 1;
                }
                continue;
            }
            if content_1 == 0 {
                // fill up bucket 1
                content_1 = capacity_1;
                moves += 1;
                continue;
            }

            return None;
        } else {
            // when other bucket has the exact measure
            if capacity_1 == goal {
                content_1 = capacity_1;
                moves += 1;
                continue;
            }

            if content_1 == capacity_1 {
                // empty bucket 1
                content_1 = 0;
                moves += 1;
                continue;
            }
            if content_1 < capacity_1 && content_2 > 0 {
                // pour from bucket 2 to bucket 1
                let space_left = capacity_1 - content_1;
                if space_left >= content_2 {
                    content_1 += content_2;
                    content_2 = 0;
                    moves += 1;
                } else {
                    content_1 += space_left;
                    content_2 -= space_left;
                    moves += 1;
                }
                continue;
            }
            if content_2 == 0 {
                // fill up bucket 2
                content_2 = capacity_2;
                moves += 1;
                continue;
            }

            return None;
        }
    }
}
