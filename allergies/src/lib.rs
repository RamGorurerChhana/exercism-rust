pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies().contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut score = self.score;
        let mut allergen = vec![];
        let mut counter = 0;
        // represent score in binary
        // 1 indicate allergen of that index
        // E.g. Eggs = 1 at 0 index
        // Peanuts = 1 at 1 index
        while score > 0 && counter < 8 {
            if score % 2 == 1 {
                match counter {
                    0 => allergen.push(Allergen::Eggs),
                    1 => allergen.push(Allergen::Peanuts),
                    2 => allergen.push(Allergen::Shellfish),
                    3 => allergen.push(Allergen::Strawberries),
                    4 => allergen.push(Allergen::Tomatoes),
                    5 => allergen.push(Allergen::Chocolate),
                    6 => allergen.push(Allergen::Pollen),
                    7 => allergen.push(Allergen::Cats),
                    _ => {}
                }
            }

            counter += 1;
            score = score / 2;
        }

        allergen
    }
}
