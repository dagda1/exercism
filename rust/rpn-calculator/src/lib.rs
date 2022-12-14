#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i16),
}

pub enum Operation {
    Add,
    Sub,
    Div,
    Mul
}

pub fn unwrap(stack: &mut Vec<i16>, op: Operation, current: Option<i16>) -> i16 {
    let mut sub_total = if current.is_none() { 0 } else { current.unwrap()};
                
    while let Some(next) = stack.pop() {
        match op {
            Operation::Add => sub_total += next,
            Operation::Sub => sub_total -= next,
            Operation::Mul => sub_total *= next,
            Operation::Div => sub_total /= next
        }
    }

    match current {
        Some(p) => p + &sub_total,
        None => sub_total
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i16> {
    if inputs.is_empty() {
        return None;
    }

    let mut total: Option<i16>= None;
    let mut stack: Vec<i16> = Vec::new();


    for i in inputs.iter() {
        match i {
            CalculatorInput::Value(v) => stack.push(*v),
            CalculatorInput::Add => total = Some(unwrap(&mut stack, Operation::Add, total)),
            CalculatorInput::Subtract => println!("sub"),
            CalculatorInput::Multiply => println!("mul"),
            CalculatorInput::Divide => println!("div"),
        };

    }

    println!("-------------------");
    println!("{:?}", stack);
    println!("{:?}", total);
    println!("-------------------");

    match stack.len() {
        1 => match total {
            Some(_) => total,
            None => Some(stack[0])
        },
        _ => total
    }
}
