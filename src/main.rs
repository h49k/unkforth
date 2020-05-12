// use std::collections::HashMap;

enum Word {
    Plus,
    Dot,
    Drop,
    Dup,
    Swap,
    Over,
    Quit,
    Push(u8),
    Nop(u8),
}

fn main() {
    // Data stack
    let mut d_stack: Vec<u8> = Vec::with_capacity(10);
    // Return stack
    let mut r_stack: Vec<u8> = Vec::with_capacity(10);
    // Accumulator
    let mut acc: u8 = 0;
    // Dictionary
    // let mut dictionary: HashMap<&str, Vec<Word>> = HashMap::new();

    loop {
        //  Command line
        let mut command_string = String::new();
        std::io::stdin().read_line(&mut command_string).ok();
        let command_line: Vec<&str> = command_string.split_whitespace().collect();
        let command_ws = command_line.iter();

        for &word in command_ws {
            // if parse
            let val: Option<u8> = word.parse().ok();

            let op = match word {
                "quit" => Word::Quit,
                "+" => Word::Plus,
                "." => Word::Dot,
                "drop" => Word::Drop,
                "over" => Word::Over,
                "swap" => Word::Swap,
                "dup" => Word::Dup,
                _ => match val {
                    Some(v) => Word::Push(v),
                    None => Word::Nop(0),
                },
            };

            match op {
                Word::Quit => {
                    println!("bye!");
                    std::process::exit(0);
                }
                Word::Plus => {
                    plus(val, &mut d_stack, &mut r_stack, &mut acc);
                    println!("{:<5} {:?}", &word, d_stack);
                }
                Word::Dot => {
                    pop(val, &mut d_stack, &mut r_stack, &mut acc);
                    println!("{:<5} {:?}\n{} ok", &word, d_stack, acc);
                }
                Word::Drop => {
                    drop(val, &mut d_stack, &mut r_stack, &mut acc);
                    println!("{:<5} {:?}", &word, d_stack);
                }
                Word::Over => {
                    over(val, &mut d_stack, &mut r_stack, &mut acc);
                    println!("{:<5} {:?}", &word, d_stack);
                }
                Word::Swap => {
                    swap(val, &mut d_stack, &mut r_stack, &mut acc);
                    println!("{:<5} {:?}", &word, d_stack);
                }
                Word::Dup => {
                    dup(val, &mut d_stack, &mut r_stack, &mut acc);
                    println!("{:<5} {:?}", &word, d_stack);
                }
                Word::Push(_v) => {
                    push(val, &mut d_stack, &mut r_stack, &mut acc);
                    println!("{:<5} {:?}", &word, d_stack);
                }
                _ => {
                    println!("unrecognized word {}.\n{:<5}{:?}", &word, "", d_stack);
                    nop(val, &mut d_stack, &mut r_stack, &mut acc);
                }
            }
        }
    }
}

/// When Data stack underflow
fn stack_underflow() {
    println!("stack_underflow!");
}

/// drop ( a -- )
/// drop stack top value, if stack isn't empty.
fn drop(_input: Option<u8>, d_stack: &mut Vec<u8>, _r_stack: &mut Vec<u8>, _acc: &mut u8) {
    if d_stack.is_empty() {
        stack_underflow();
    } else {
        d_stack.pop();
    }
}

/// dup ( a -- a a )
/// Duplicate top stack.
fn dup(_input: Option<u8>, d_stack: &mut Vec<u8>, _r_stack: &mut Vec<u8>, acc: &mut u8) {
    if d_stack.is_empty() {
        stack_underflow();
        return;
    }

    if let Some(r) = d_stack.last_mut() {
        *acc = *r;
        d_stack.push(*acc);
    }
}

/// swap ( a b -- b a )
/// Swap between top and scond
fn swap(_input: Option<u8>, d_stack: &mut Vec<u8>, r_stack: &mut Vec<u8>, acc: &mut u8) {
    if d_stack.len() > 1 {
        if let Some(r) = d_stack.pop() {
            r_stack.push(r)
        }
        if let Some(r) = d_stack.pop() {
            *acc = r;
        }
        if let Some(r) = r_stack.pop() {
            d_stack.push(r)
        }

        d_stack.push(*acc);
    } else {
        stack_underflow();
    }
}

/// over ( a b -- a b a )
///
fn over(_input: Option<u8>, d_stack: &mut Vec<u8>, r_stack: &mut Vec<u8>, acc: &mut u8) {
    if d_stack.len() > 1 {
        if let Some(r) = d_stack.pop() {
            r_stack.push(r)
        }
        if let Some(r) = d_stack.last_mut() {
            *acc = *r;
        }
        if let Some(r) = r_stack.pop() {
            d_stack.push(r)
        }
        d_stack.push(*acc);
    } else {
        stack_underflow();
    }
}

/// push ( -- a )
fn push(input: Option<u8>, d_stack: &mut Vec<u8>, _r_stack: &mut Vec<u8>, _acc: &mut u8) {
    if let Some(r) = input {
        d_stack.push(r);
    }
}

/// nop ( -- )
/// non operation.
fn nop(_input: Option<u8>, _d_stack: &mut Vec<u8>, _r_stack: &mut Vec<u8>, _acc: &mut u8) {}

/// pop ( -- )
/// pop to accumurator
fn pop(_input: Option<u8>, d_stack: &mut Vec<u8>, _r_stack: &mut Vec<u8>, acc: &mut u8) {
    if d_stack.is_empty() {
        stack_underflow();
        return;
    }

    if let Some(r) = d_stack.pop() {
        *acc = r;
    }
}

/// + ( a b -- a+b )
fn plus(_input: Option<u8>, d_stack: &mut Vec<u8>, _r_stack: &mut Vec<u8>, acc: &mut u8) {
    if d_stack.len() > 1 {
        if let Some(r) = d_stack.pop() {
            *acc = r;
        }
        if let Some(r) = d_stack.pop() {
            *acc = *acc + r;
        }
        d_stack.push(*acc);
    } else {
        stack_underflow();
    }
}
