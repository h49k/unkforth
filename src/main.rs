use std::collections::HashMap;

enum Word {
    Plus,
    Dot,
    Push(u8),
    Nop(u8),
}

 
fn main() {
    // スタックの本体
    let mut stack:Vec<u8> = Vec::with_capacity(10);
    // アキュームレータ
    let mut acc: u8 = 0;

    // コマンドライン
    let command_string = "7 8 + 8 - . .";
    println!("Input=>{}<", command_string);
    let command_line: Vec<&str> = command_string.split_whitespace().collect();
    let command_ws = command_line.iter();

    let mut dictionary: HashMap<&str, Vec<Word>> = HashMap::new();


    for &word in command_ws {

        println!("WORD==>{}", &word);

        let val: Option<u8> = word.parse().ok();

        let op = match word {
            "+" => Word::Plus,
            "." => Word::Dot,
            _ => match val {
                Some(v) => Word::Push(v),
                None => Word::Nop(0),
            }
        };

        match op {
            Word::Plus => plus(val, &mut stack, &mut acc),
            Word::Dot => dot(val, &mut stack, &mut acc),
            Word::Push(_v) => push(val, &mut stack, &mut acc),
            _ => nop(val, &mut stack, &mut acc),      
        }
    
    }

}

fn plus(input: Option<u8>, stack:&mut Vec<u8>, acc: &mut u8){
    let top = stack.pop().unwrap() + stack.pop().unwrap();
    stack.push(top);
    println!("Stack{:?}",stack);
}

fn dot(input: Option<u8>, stack:&mut Vec<u8>, acc: &mut u8){
    let val = stack.pop().unwrap();
    print!("Stack{:?}",stack);
    println!("{}", val);
}

fn push(input: Option<u8>, stack:&mut Vec<u8>, acc: &mut u8){
    let val = input.unwrap();
    stack.push(val);
    println!("Stack{:?}",stack);
}

fn nop(input: Option<u8>, stack:&mut Vec<u8>, acc: &mut u8){
    println!("Stack{:?}",stack);

}

fn pop(input: Option<u8>, stack:&mut Vec<u8>, acc: &mut u8) {
    Vec
}

