#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

use CalculatorInput::*;

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = vec![];
    for input in inputs {
        match input {
            Value(n) => stack.push(*n),
            Add => {
                let result = stack.pop()? + stack.pop()?;
                stack.push(result);
            }
            Subtract => {
                let e1 = stack.pop()?;
                let e2 = stack.pop()?;
                let result = e2 - e1;
                stack.push(result);
            }
            Multiply => {
                let result = stack.pop()? * stack.pop()?;
                stack.push(result);
            }
            Divide => {
                let e1 = stack.pop()?;
                let e2 = stack.pop()?;
                let result = e2 / e1;
                stack.push(result);
            }
        }
    }
    let result = stack.pop()?;
    match stack.pop() {
        None => Some(result),
        _ => None,
    }
}
