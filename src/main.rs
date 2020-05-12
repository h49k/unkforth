use std::collections::HashMap;

enum Word {
    Plus,
    Dot,
    Push(u8),
    Nop(u8),
}

 
fn main() {
    // Data stack
    let mut d_stack:Vec<u8> = Vec::with_capacity(10);
    // Return stack
    let mut r_stack:Vec<u8> = Vec::with_capacity(10);
    // Accumulator
    let mut acc: u8 = 0;
    // Dictionary
    let mut dictionary: HashMap<&str, Vec<Word>> = HashMap::new();
    // Input value
    let val: Option<u8> = Some(0);

    //  Command line
    let command_string = "7 8 + 8 - . .";
    let command_line: Vec<&str> = command_string.split_whitespace().collect();
    let command_ws = command_line.iter();

    // 
    d_stack.push(35);
    d_stack.push(41);
    d_stack.push(19);
    d_stack.push(9);
    println!("init:{:?}", d_stack);
    dup(val, &mut d_stack, &mut r_stack, &mut acc);
    println!("dup :{:?}", d_stack);
    drop(val, &mut d_stack, &mut r_stack, &mut acc);
    println!("drop:{:?}", d_stack);
    swap(val, &mut d_stack, &mut r_stack, &mut acc);
    println!("swap:{:?}", d_stack);
    over(val, &mut d_stack, &mut r_stack, &mut acc);
    println!("over:{:?}", d_stack);
    // for &word in command_ws {

    //     // println!("WORD==>{}", &word);

    //     let val: Option<u8> = word.parse().ok();

    //     // let op = match word {
    //     //     "+" => Word::Plus,
    //     //     "." => Word::Dot,
    //     //     _ => match val {
    //     //         Some(v) => Word::Push(v),
    //     //         None => Word::Nop(0),
    //     //     }
    //     // };

    //     // match op {
    //     //     Word::Plus => plus(val, &mut d_stack, &mut acc),
    //     //     Word::Dot => dot(val, &mut d_stack, &mut acc),
    //     //     Word::Push(_v) => push(val, &mut d_stack, &mut acc),
    //     //     _ => nop(val, &mut d_stack, &mut acc),      
    //     // }
    
    // }

}

/// When Data stack underflow
fn stack_underflow(){
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


// fn pop(input: Option<u8>, stack:&mut Vec<u8>, acc: &mut u8) {
//     if let Some(r) = stack.pop() {
//         *acc = r;
//     }

// }
// fn plus(input: Option<u8>, stack:&mut Vec<u8>, acc: &mut u8){
//     let top = stack.pop().unwrap() + stack.pop().unwrap();
//     stack.push(top);
//     println!("Stack{:?}",stack);
// }

// fn dot(input: Option<u8>, stack:&mut Vec<u8>, acc: &mut u8){
//     let val = stack.pop().unwrap();
//     print!("Stack{:?}",stack);
//     println!("{}", val);
// }

// fn push(input: Option<u8>, stack:&mut Vec<u8>, acc: &mut u8){
//     let val = input.unwrap();
//     stack.push(val);
//     println!("Stack{:?}",stack);
// }

// fn nop(input: Option<u8>, stack:&mut Vec<u8>, acc: &mut u8){
//     println!("Stack{:?}",stack);

// }

// fn pop(input: Option<u8>, stack:&mut Vec<u8>, acc: &mut u8) {
//     if let Some(r) = stack.pop() {
//         *acc = r;
//     }

// }

 