#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T> {
    elements: Vec<T>,
}

impl<T: Copy + PartialEq + Ord> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut elements = input.into_iter().map(|&e| e).collect::<Vec<_>>();
        elements.sort_unstable();
        Self { elements }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.elements.contains(element)
    }

    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            self.elements.push(element);
            self.elements.sort_unstable();
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.elements.iter().all(|e| other.contains(e))
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.elements.iter().all(|e| !other.contains(e))
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        let mut elements = self
            .elements
            .iter()
            .filter(|&e| other.contains(e))
            .map(|&e| e)
            .collect::<Vec<_>>();
        elements.sort_unstable();
        Self { elements }
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        let mut elements = self
            .elements
            .iter()
            .filter(|&e| !other.contains(e))
            .map(|&e| e)
            .collect::<Vec<_>>();
        elements.sort_unstable();
        Self { elements }
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut elements = self
            .elements
            .iter()
            .filter(|&e| !other.contains(e))
            .chain(other.elements.iter())
            .map(|&e| e)
            .collect::<Vec<_>>();
        elements.sort_unstable();
        Self { elements }
    }
}
