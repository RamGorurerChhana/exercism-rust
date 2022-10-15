use std::ops::Add;

pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T: Copy + Default + PartialEq + PartialOrd + Add<Output = T>> Triangle<T> {
    fn is_valid(sides: [T; 3]) -> Option<()> {
        if sides[0] == T::default()
            || sides[1] == T::default()
            || sides[2] == T::default()
            || sides[0] + sides[1] < sides[2]
            || sides[0] + sides[2] < sides[1]
            || sides[1] + sides[2] < sides[0]
        {
            None
        } else {
            Some(())
        }
    }

    pub fn build(sides: [T; 3]) -> Option<Self> {
        Self::is_valid(sides)?;
        Some(Self { sides })
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] && self.sides[1] == self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1]
            || self.sides[1] == self.sides[2]
            || self.sides[0] == self.sides[2]
    }
}
