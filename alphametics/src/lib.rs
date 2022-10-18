//! Approach:
//! To solve this alphametic puzzle we will adopt recusrsive baacktracking approach.
//! For each of the variable we will try fit a number starting from 0 - 9 .
//! If the numbers fits for the variable then we assume that to be potential solution.
//! we move on the next variable and try fitting numbers again.
//! When a number is not fit for a variable. we try fitting the next available number.
//! Two scenarios to think about here -
//! 1. We manage to fit some number to all variables then
//!     - try out the equation
//!     - if the equation holds that means we found a solution
//!     - if not that same as we do not manage to fit a number to the last variable
//!
//! 2. For some variable we have checked all numbers 0 - 9 and not manage to fit any number.
//! that means we need to backtrack. return Err in this case.
//!
//! When Err is received that means no solution present for the current choice of number.
//! try the next choice.
//!
//! Conditions for a number not fitting for a variable -
//!     - same number is already taken for other variable
//!     - creating a number with leading zero
//!     - all variables are assigned some number and the euation does not hold
//!
//!
//! Special Note: For some equation there could be more than one solution present.
//! Upon finding a solution we will not immediately return to caller.
//! Instead continue with the next number to see if there is more choices available.
//! However our target is not to find all possible solutions. Whenever we find more than one solution
//! we backtrack we Ok. Indicating we found a solution to the caller.
//!
//! Bounderies:
//!     - We assume the lhs only contains plus(+) operators.
//!     - RHS only contain one variable and other other operations.
//!     - All variables are uppercase letters only and no sanity checks are done around that.
//!     - Tracking down kind of error is not important. So generic Err type used everywhere.
//!
//!
//!
//!
//!
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

/// represents a number consists of variables
#[derive(Debug)]
struct AlphameticNumber {
    digits: Vec<char>,
}

impl AlphameticNumber {
    /// construct the whole number using the variables
    /// TODO: refactor to consider possible error scenarios and handle error
    fn construct_number(&self, possible_solution: &HashMap<char, u8>) -> u64 {
        let num = self
            .digits
            .iter()
            .map(|ch| char::from_digit(*possible_solution.get(ch).unwrap() as u32, 10).unwrap())
            .collect::<String>();

        num.parse().unwrap()
    }

    /// check if the given character can constitute leading zero
    fn is_leading_zero(&self, ch: char) -> bool {
        if let Some(c) = self.digits.get(0) {
            if *c == ch {
                return true;
            }
        }

        false
    }
}

/// implement FromStr trait for AlphameticNumber
/// so that string to AlphameticNumber conversion can be done easily
impl FromStr for AlphameticNumber {
    // since tracking error kind is not important use generic type
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // skip validation of invalid chars so always returns Ok
        let digits = s.trim().chars().collect::<Vec<_>>();
        Ok(Self { digits })
    }
}

#[derive(Debug)]
struct Puzzle {
    // holds the actual eqatuion provided as strings
    #[allow(dead_code)]
    equation: String,
    // holds all the variables from the equation and possible solution currently under trail
    variables: Vec<(char, Option<u8>)>,
    // holds AlphameticNumbers from LHS
    lhs: Vec<AlphameticNumber>,
    // holds the AlphameticNumber from RHS
    rhs: AlphameticNumber,
    // holds all verified solution
    solutions: Vec<HashMap<char, u8>>,
    iteration_count: u64
}

impl Puzzle {
    /// create new instance of the puzzle
    /// converts the input string into lhs and rhs AlphmeticNumber
    /// extracts all variables
    fn new(input: &str) -> Self {
        // skip validation on the structure of the equation
        // so fearlessly calling unwrap whenever required
        // get equation string
        let equation = input.to_string();
        // get lhs and rhs
        let splitted = input.split("==").collect::<Vec<_>>();
        let rhs = AlphameticNumber::from_str(splitted[1]).unwrap();
        let lhs = splitted[0]
            .split("+")
            .map(|s| AlphameticNumber::from_str(s).unwrap())
            .collect::<Vec<_>>();
        // get all variables
        let variables = input
            .chars()
            .filter(char::is_ascii_alphabetic)
            .collect::<HashSet<char>>();
        let mut variables = variables.iter().map(|&ch| (ch, None)).collect::<Vec<_>>();
        variables.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        // return Self instance
        Self {
            equation,
            variables,
            lhs,
            rhs,
            solutions: vec![],
            iteration_count: 0
        }
    }

    /// check if a particular digit is a fit for a variable
    fn is_possible_candidate(&self, candidate: u8, var_idx: usize) -> bool {
        // check if same canidate is used already
        if self
            .variables
            .iter().any(|e| e.1.is_some() && e.1 == Some(candidate))
        {
            return false;
        }
        // check for leading zero
        if candidate == 0 {
            let var = self.variables[var_idx].0;
            if self.lhs.iter().any(|e| e.is_leading_zero(var)) {
                return false;
            }
            if self.rhs.is_leading_zero(var) {
                return false;
            }
        }

        true
    }

    /// check if equation holds and possible solution is found
    fn check_equation(&self) -> Option<HashMap<char, u8>> {
        let possible_solution = self
            .variables
            .iter()
            .map(|e| (e.0, e.1.unwrap()))
            .collect::<HashMap<char, u8>>();
        let lhs = self
            .lhs
            .iter()
            .map(|n| n.construct_number(&possible_solution))
            .sum::<u64>();
        let rhs = self.rhs.construct_number(&possible_solution);
        if lhs == rhs {
            Some(possible_solution)
        } else {
            None
        }
    }

    /// do some basic test which invalidates an equation without even trying any solution
    /// like if rhs length is less than any number in lhs
    /// return None when basic tests does not hold
    fn basic_test(&self) -> Option<()> {
        if self
            .lhs
            .iter()
            .any(|e| e.digits.len() > self.rhs.digits.len())
        {
            None
        } else {
            Some(())
        }
    }

    /// recursively try to find a solution to the equation
    /// by trying different number for a variable
    /// when no solution found return Err variant
    /// before returning Err variant reset the tuple with None value
    /// if a solution found return Ok variant
    fn try_solve(&mut self, var_idx: usize) -> Result<(), ()> {
        self.iteration_count += 1;
        // get the variable which hasn't yet assigned any value
        // if var_idx is greater than the length of variables that means all variables are assigned value
        // check if the equation holds true at this point
        // if the equation doesn't hold then return Err variant
        if var_idx >= self.variables.len() {
            let possible_solution = self.check_equation().ok_or(())?;
            self.solutions.push(possible_solution);
            return Ok(());
        }
        // start fitting number from zero
        let mut candidate = 0_u8;
        loop {
            if candidate > 9 {
                // when candidate is greater than 9
                // we didn't find any fitting candidate
                // return Err variant
                return Err(());
            }
            // check if the current candidate is fitting
            if self.is_possible_candidate(candidate, var_idx) {
                // add the candidate into variable
                self.variables[var_idx].1 = Some(candidate);
                // call try_solve again
                let result = self.try_solve(var_idx + 1);
                if result.is_ok() {
                    // // when Ok variant is received from the recursive call
                    // // check if more than 1 solution found already
                    // // then return Ok otherwise continue
                    // if self.solutions.len() > 1 {
                    //     return Ok(());
                    // }
                    return Ok(());
                }
                // nullify the current choice in variables
                self.variables[var_idx].1 = None;
            }
            // increment candidate
            candidate += 1;
        }
    }
}

/// In the solve function
/// 1. create a new instance of Puzzle struct.
/// 2. Extract all variables from the equation.
/// 3. Call **basic_test** on the equation if Err is received then return None.
/// 3. call **try_solve** passing mutable reference.
/// 4. if Err is returned by the `try_solve` then return None
/// 5. if Ok is returned by the `try_solve` then check for no of solutions present.
/// 6. if more than 1 solution present then return None.
/// 7. else return the solution
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut puzzle = Puzzle::new(input);
    puzzle.basic_test()?;
    let _ = puzzle.try_solve(0);
    let solution = puzzle.solutions.pop()?;
    println!("Iteration Count is: {}", puzzle.iteration_count);
    match puzzle.solutions.pop() {
        Some(_) => None,
        None => Some(solution),
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_from_str_alphametic_number() {
//         let s = "ABC ";
//         let num = AlphameticNumber::from_str(s).unwrap();
//         assert!(num.digits.len() == s.trim().len());
//         assert!(num.digits.contains(&'A'));
//         assert!(num.digits.contains(&'B'));
//         assert!(num.digits.contains(&'C'));
//         assert!(!num.digits.contains(&' '));
//     }

//     #[test]
//     fn test_puzzle_new() {
//         let s = "ABC + DE == MPR";
//         let puzzle = Puzzle::new(s);
//         assert_eq!(puzzle.equation, s);
//         assert_eq!(puzzle.variables.len(), 8);
//         assert_eq!(puzzle.lhs.len(), 2);
//         assert_eq!(puzzle.solutions.len(), 0);
//     }
// }
