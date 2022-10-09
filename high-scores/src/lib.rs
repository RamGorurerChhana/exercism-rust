#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().map(|x| *x)
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores().iter().max().map(|x| *x)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = [0_u32; 3];
        for &score in self.scores() {
            if score >= scores[0] {
                // if score is greater eq than scores[0]
                scores[2] = scores[1];
                scores[1] = scores[0];
                scores[0] = score;
            } else if score >= scores[1] {
                // if score is greater eq than scores[1]
                scores[2] = scores[1];
                scores[1] = score;
            } else if score >= scores[2] {
                // if score is greater eq than scores[1]
                scores[2] = score;
            }
        }

        match self.scores().len() {
            0 => vec![],
            1 => vec![scores[0]],
            2 => vec![scores[0], scores[1]],
            _ => scores.to_vec(),
        }
    }
}
