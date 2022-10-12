use std::collections::HashMap;

/// Use hashmap to hold the list of codons and corresponding protien names
/// lifetime generic 'a is used since we are going to store string slices
pub struct CodonsInfo<'a> {
    codons: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    /// search with the codon as the key in the hashmap
    /// and return corresponding protien name
    /// when not found return None
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.codons.get(codon).map(|&s| s)
    }

    /// For a given rna string scan taking group of 3 characters
    /// and then search for the codon in the Hashmap
    /// collect all the protien names in the vector
    /// if stop codon found then break the loop an return
    /// if the given rna string is not multiple of 3 chars then return None
    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut names = vec![];
        let mut idx = 0;
        // continue the loop until the whole string is scanned
        while idx < rna.len() {
            // if rna string length is not multiple of 3 then return None
            if idx + 3 > rna.len() {
                return None;
            }
            // take each 3 characters at a time from the staring and
            // use that as the key to search the HashMap
            let k = &rna[idx..idx + 3];
            // get the protien name or if not found then return None
            // ? at the end is for returning None
            let nm = self.codons.get(k).map(|&nm| nm)?;
            // when stop codon found break the loop
            if nm == "stop codon" {
                break;
            }
            // push into the names vector
            names.push(nm);
            // increase the index by 3 to scan next 3 character
            idx += 3;
        }

        // return the names
        Some(names)
    }
}

/// Instantiate the CodonsInfo struct with the given (codon, protien_name) pair
/// Generate the HashMap from the vector
pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let mut codons: HashMap<&'a str, &'a str> = HashMap::new();
    for pair in pairs {
        codons.insert(pair.0, pair.1);
    }
    CodonsInfo { codons }
}
