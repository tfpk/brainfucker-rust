use std::io::{self, Read};

struct BracketPair {
    start: usize,
    end: usize,
}

fn get_bracket_pairs(command: &String) -> Vec<BracketPair>{
    let mut bracket_pairs = Vec::new();
    let mut unclosed_stack: Vec<usize> = Vec::new();
    for (i, c) in command.chars().enumerate() {
        if c == '[' {
            unclosed_stack.push(bracket_pairs.len());
            bracket_pairs.push(BracketPair{start: i, end:0});
        } else if c == ']' {
            if unclosed_stack.len() == 0 {panic!("Could not match to open brace")};
            bracket_pairs[unclosed_stack.pop().unwrap()].end = i;
        }
    }
    if unclosed_stack.len() != 0 {panic!("Did not match everything")};
    bracket_pairs

}

fn find_pair(pair_list: &Vec<BracketPair>, index: usize) -> &BracketPair {
    for p in pair_list.iter() {
        if p.start == index || p.end == index {return p}
    }
    panic!("Can't find pair as asked")
}

fn main() {
    println!("Welcome to brainfucker, a simple interpreter for brainfuck.");
    let mut memory_pos = Vec::new();
    let mut memory_neg = Vec::new();
    memory_pos.push(0); // push negative one to ignore.
    memory_neg.push(0); // push negative one to ignore.
    let mut command = String::new();
    io::stdin().read_to_string(&mut command).expect("Could not read string from stdio.");
    let bracket_pairs = get_bracket_pairs(&command);
    let command: Vec<char> = command.chars().collect();
    let mut mem_index: i32 = 0;
    let mut cmd_index: usize = 0;

    while cmd_index < command.len() {
        match command[cmd_index]{
            '>' => {
                mem_index += 1;
                if mem_index >= 0 {
                    if mem_index as usize == memory_pos.len() {memory_pos.push(0)}
                }
            },
            '<' => {
                mem_index -= 1;
                if mem_index < 0 {
                    if (-mem_index) as usize == memory_neg.len() {memory_neg.push(0)}
                }
            },
            '+' => if mem_index >= 0 {memory_pos[mem_index as usize] += 1} else {memory_neg[-mem_index as usize] += 1},
            '-' => if mem_index >= 0 {memory_pos[mem_index as usize] -= 1} else {memory_neg[-mem_index as usize] -= 1},
            '.' => println!("{}", ((if mem_index >= 0 {memory_pos[mem_index as usize]} else {memory_neg[-mem_index as usize]}) % 255) as u8 as char),
            ',' => {
                let input: Option<i32> = std::io::stdin()
                    .bytes() 
                    .next()
                    .and_then(|result| result.ok())
                    .map(|byte| byte as i32);
                    if mem_index >= 0 { memory_pos[mem_index as usize] = input.unwrap(); }
            },
            '[' =>  {
                let pair = find_pair(&bracket_pairs, cmd_index);
                let mem_val = if mem_index >= 0 {memory_pos[mem_index as usize]} else {memory_neg[-mem_index as usize]};
                if mem_val == 0 {cmd_index = pair.end};
            },
            ']' => {
                let pair = find_pair(&bracket_pairs, cmd_index);
                let mem_val = if mem_index >= 0 {memory_pos[mem_index as usize]} else {memory_neg[-mem_index as usize]};
                if mem_val != 0 {cmd_index = pair.start};

            },
            _ => ()
        }
        cmd_index += 1;
    }
}
