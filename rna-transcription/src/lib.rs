#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    nucleotides: Vec<char>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    nucleotides: Vec<char>,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        dna.chars()
            .into_iter()
            .enumerate()
            .map(|(i, ch)| match ch {
                'A' | 'C' | 'G' | 'T' => Ok(ch),
                _ => Err(i),
            })
            .collect::<Result<Vec<char>, usize>>()
            .map(|nucleotides| Self { nucleotides })
    }

    pub fn into_rna(self) -> Rna {
        let nucleotides = self
            .nucleotides
            .into_iter()
            .map(|ch| match ch {
                'A' => 'U',
                'C' => 'G',
                'G' => 'C',
                'T' => 'A',
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
        Rna { nucleotides }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        rna.chars()
            .into_iter()
            .enumerate()
            .map(|(i, ch)| match ch {
                'A' | 'C' | 'G' | 'U' => Ok(ch),
                _ => Err(i),
            })
            .collect::<Result<Vec<char>, usize>>()
            .map(|nucleotides| Self { nucleotides })
    }
}
