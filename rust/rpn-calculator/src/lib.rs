#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    if inputs.is_empty() {
        return None;
    }

    let mut stack: Vec<i32> = Vec::new();

    for input in inputs {
        match input {
            CalculatorInput::Value(v) => stack.push(*v),
            _ => {
                if stack.len() < 2 {
                    return None;
                }

                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();

                match input {
                    CalculatorInput::Add => stack.push(left + right),
                    CalculatorInput::Subtract => stack.push(left - right),
                    CalculatorInput::Multiply => stack.push(left * right),
                    CalculatorInput::Divide => stack.push(left / right),
                    _ => return None,
                }
            }
        };
    }

    if stack.len() != 1 {
        return None;
    }

    stack.pop()
}
