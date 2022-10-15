use std::collections::HashMap;

// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
pub struct School {
    students: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            students: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.students
            .entry(grade)
            .and_modify(|e| e.push(student.to_string()))
            .or_insert(vec![student.to_string()]);
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut v = self
            .students
            .keys()
            .into_iter()
            .cloned()
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut v = self.students.get(&grade).map_or(vec![], |v| v.clone());
        v.sort_unstable();
        v
    }
}
