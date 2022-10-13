use std::collections::HashMap;

/// count no of occurreneces of a given nucleotides in a given dna string
/// if the nucleotide is invalid of there is any invalid character in the
/// dna string then it returns Err(X)
pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    is_valid(nucleotide)?;
    let mut count = 0;
    for c in dna.chars() {
        if is_valid(c)? && c == nucleotide {
            count += 1;
        }
    }

    Ok(count)
}

/// given the dna string it counts the no of occurrences of each nucleotides
/// by default each nucleotides have count 0
/// if a ny invalid character encountered then Err(X) is returned
pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    ['A', 'C', 'G', 'T']
        .into_iter()
        .map(|nucleotide| count(nucleotide, dna).map(|count| (nucleotide, count)))
        .collect()
}

/// check if the given char is a valid nucleotide
fn is_valid(nucleotide: char) -> Result<bool, char> {
    match nucleotide {
        'A' | 'C' | 'G' | 'T' => Ok(true),
        _ => Err(nucleotide),
    }
}
