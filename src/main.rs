use std::{path::PathBuf, collections::HashMap, hash::Hash};

fn main() {
/*     day1();
    day2();
    day3();
    day4();
    day5();
    day6(); */
    day7();
}

fn day7() {
    #[derive(Debug)]
    enum Mode{
        Command,
        Output
    }

    #[derive(Debug)]
    enum Command {
        ChDir(String),
        List(String)
    }

    #[derive(Debug)]
    enum Output {
        SubDir(String),
        File(String, u64)
    }

    let lines: Vec<_> = include_str!("input_temp.txt").lines().collect();
    let mut current_mode = Mode::Command;
    let mut current_command = Command::ChDir("".into());
    let mut current_output = Output::File("".into(), 0);
    let mut current_directory = PathBuf::new();

    let mut dir_sizes = HashMap::<String, u64>::new();
    for line in lines {
        let mut line_pieces = line.split_whitespace().into_iter();
        let mut first_token = "";
        match line_pieces.next() {
            Some(token) => {
                if token == "$" {
                    current_mode = Mode::Command;
                } else {
                    current_mode = Mode::Output;
                    first_token = token;
                }
            },
            None => todo!(),
        }
        match current_mode {
            Mode::Command => {
                match line_pieces.next() {
                    Some(token) => {
                        match token {
                            "cd" => {
                                match line_pieces.next().unwrap() {
                                    ".." => {
                                        current_directory = current_directory.parent().unwrap().to_path_buf();
                                    }
                                    sub_dir => {
                                        current_directory = current_directory.join(sub_dir);
                                    }
                                }
                                current_command = Command::ChDir(current_directory.to_str().unwrap().into());
                            },
                            "ls" => {
                                current_command = Command::List(current_directory.to_str().unwrap().into());
                            },
                            _ => todo!()
                        }
                    }
                    None => todo!(),
                }
            },
            Mode::Output => {
                match first_token {
                    "dir" => {
                        current_output = Output::SubDir(line_pieces.next().unwrap().into());
                    },
                    size_str => {
                        let file_size = size_str.parse::<u64>().unwrap();
                        let file_name = line_pieces.next().unwrap();
                        current_output = Output::File(file_name.into(), file_size);

                        let mut temp = current_directory.clone();
                        loop {
                            match temp.parent() {
                                Some(new_temp) => {
                                    temp = new_temp.to_path_buf();
                                    let current_directory_string = String::from(temp.as_path().to_str().unwrap());
                                    let mut size = file_size;
                                    match dir_sizes.get(current_directory.clone().as_path().to_str().unwrap()) {
                                        Some(prev_size) => {size += prev_size},
                                        None => {},
                                    }
                                    _ = dir_sizes.insert(current_directory_string, size);

                                },
                                None => {break;},
                            }
                        }
                    }
                }
            },
        }
        println!("line '{}' decoded as: {:?} {:?} {:?}", line, current_mode, current_command, current_output);
    }
    for (key, value) in &dir_sizes {
        println!("{}: {}", key, value);
    }
}

/* fn day7_add(map: &mut HashMap<&str, u64>, key: &str, value: u64) {
    let mut size = value;
    match dir_sizes.get(current_directory.clone().as_path().to_str().unwrap()) {
        Some(prev_size) => {size += prev_size},
        None => {},
    }
    _ = dir_sizes.insert(current_directory_string, size);
} */

fn day6() {
    let lines: Vec<_> = include_str!("input_day6.txt").lines().collect();
    if lines.len() != 1 {
        panic!("single line expected in input, but {}", lines.len());
    }
    let input = lines.get(0).unwrap();
    println!("{:?}", day6_find(4, input.clone()));
    println!("{:?}", day6_find(14, input));
}

fn day6_find(x: usize, input: &str) -> (u64, Vec<char>) {
    let mut last_x = Vec::<char>::new();
    let mut i: usize= 0;
    for next_char in input.chars() {
        if last_x.len() >= x {
            last_x.drain(0..1);
        }
        last_x.push(next_char);
        i += 1;
        if last_x.len() == x {
            let mut not_found_the_same = true;
            for char_inner in last_x.clone() {
                if last_x.clone().iter().filter(|c| { **c == char_inner }).count() > 1 {
                    not_found_the_same = false;
                    break;
                }
            }
            if not_found_the_same {
                //println!("{}, {:?}", i, last_x);
                return (i as u64, last_x);
            }
        }
    }
    (0, last_x)
}

fn day5() {
    #[derive(Clone)]
    struct Stacks {
        state: Vec<Vec<char>>
    }
    impl Stacks {
        fn new() -> Self {
            return Self{state: Vec::<Vec<char>>::new()};
        }
        fn init_stacks(&mut self, input: &Vec<&str>) {
            for j in 0..8 {
                let line = 7 - j;
                for i in 0..9 {
                    let data = input.get(line).unwrap().as_bytes()[1+i*4];
                    if !data.is_ascii_uppercase() {
                        continue
                    }
                    match self.state.get_mut(i) {
                        Some(stack) => stack.push(data as char),
                        None => {
                            self.state.push(vec![data as char]);
                        },
                    }
                }
            }
        }
        fn dump_state(&self) {
            for stack in self.state.clone() {
                println!("{:?}", stack);
            }
            println!();
        }
        fn dump_top(&self) {
            for stack in self.state.clone().iter_mut() {
                print!("{}", stack.pop().unwrap());
            }
            println!();
        }
        fn parse_move(input: &str) -> Option<(u8, u8, u8)> {
            if !input.starts_with("move") {
                return None
            }
            let pieces = input.split_whitespace();
            let number_pieces: Vec<_> = pieces.filter(|c| {
                match c.parse::<u8>() {
                    Ok(_) => true,
                    Err(_) => false,
                }
            }).collect();
            if number_pieces.len() != 3 {
                panic!("incorrect move parse");
            }
            Some((number_pieces[0].parse::<u8>().unwrap(), number_pieces[1].parse::<u8>().unwrap(), number_pieces[2].parse::<u8>().unwrap()))
        }
        fn move_item(&mut self, from: usize, to: usize) {
            //println!("moving from {} (size: {}) to {} (size: {})", from, self.state[from].len(), to, self.state[to].len());
            let item = self.state[from].pop().unwrap();
            self.state[to].push(item);
        }
        fn move_items(&mut self, amount: usize, from: usize, to: usize) {
            //println!("moving {} from {} (size: {}) to {} (size: {})", amount, from, self.state[from].len(), to, self.state[to].len());
            let start_pos = self.state[from].len();
            let mut items = self.state[from].split_off((start_pos - amount) as usize);
            //let item = self.state[from].pop().unwrap();
            self.state[to].append(&mut items);
        }
    }
    let lines: Vec<_> = include_str!("input_day5.txt").lines().collect();
    let mut stacks = Stacks::new();
    stacks.init_stacks(&lines);
    let mut stacks2 = stacks.clone();
    for moves in lines {
        match Stacks::parse_move(moves) {
            Some(one_move) => {
                //println!("{:?}", one_move);
                for _ in 0..one_move.0 {
                    stacks.move_items(1 as usize,(one_move.1 - 1) as usize, (one_move.2 - 1) as usize);
                }
                stacks2.move_items(one_move.0 as usize,(one_move.1 - 1) as usize, (one_move.2 - 1) as usize);
            },
            None => {},
        }
        //stacks.dump_state();
    }
    stacks.dump_top();
    stacks2.dump_top();

}

fn day4() {
    day4_1st();
    day4_2nd();
}

fn day4_1st() {
    let lines: Vec<_> = include_str!("input_day4.txt").lines().collect();
    let mut pairs: Vec<((i32, i32), (i32, i32))> = vec![];

    for line in lines {
        let parsed_line = day4_get_ranges(line);
        pairs.push(parsed_line);
    }
    let result = pairs.iter().filter(|&ranges| {
        (ranges.0.0 <= ranges.1.0 && ranges.0.1 >= ranges.1.1) ||
        (ranges.1.0 <= ranges.0.0 && ranges.1.1 >= ranges.0.1)
    }).count();
    println!("{}", result);
}
    
fn day4_2nd() {
    let lines: Vec<_> = include_str!("input_day4.txt").lines().collect();
    let mut pairs: Vec<((i32, i32), (i32, i32))> = vec![];

    for line in lines {
        let parsed_line = day4_get_ranges(line);
        pairs.push(parsed_line);
    }
    let result = pairs.iter().filter(|&ranges| {
        (ranges.0.0 <= ranges.1.1 && ranges.0.0 >= ranges.1.0) ||
        (ranges.0.1 <= ranges.1.1 && ranges.0.1 >= ranges.1.0) ||
        ((ranges.1.0 <= ranges.0.1 && ranges.1.0 >= ranges.0.0) ||
        (ranges.1.1 <= ranges.0.1 && ranges.1.1 >= ranges.0.0) )
    });
    println!("{}", result.count());
}
    
fn day4_get_ranges(line: &str) -> ((i32, i32), (i32, i32)) {
    let chunks: Vec<_> = line.split(',').collect();
    if chunks.len() != 2 {
        panic!("unexpected chunks length ({}) in line: {}", chunks.len(), line)
    }
    let nums0: Vec<_> = chunks[0].split('-').collect();
    if nums0.len() != 2 {
        panic!("unexpected nums length ({}) in line: {}", chunks.len(), line)
    }
    let ints00 = nums0[0].parse::<i32>().unwrap();
    let ints01 = nums0[1].parse::<i32>().unwrap();

    let nums1: Vec<_> = chunks[1].split('-').collect();
    if nums1.len() != 2 {
        panic!("unexpected nums length ({}) in line: {}", chunks.len(), line)
    }
    let ints10 = nums1[0].parse::<i32>().unwrap();
    let ints11 = nums1[1].parse::<i32>().unwrap();

    ((ints00, ints01), (ints10, ints11))
}


fn day3() {
    day3_1st();
    day3_2nd();
}

fn day3_1st() {
    let lines: Vec<_> = include_str!("input_day3.txt").lines().collect();
    let mut result = 0u32;
    for line in lines {
        if line.len() % 2 != 0 {
            panic!("odd line length ({}) in line: {}", line.len(), line)
        }
        let (first_half, second_half) = line.split_at(line.len() / 2);
        let shared = day3_shared_char(first_half, second_half, second_half).unwrap();
        //println!("{} {} -> {} {}", first_half, second_half, shared, shared as u8);
        result += day3_char_to_point(shared) as u32;
    }
    println!("{}", result);
}

fn day3_2nd() {
    let lines: Vec<_> = include_str!("input_day3.txt").lines().collect();
    let mut result = 0u32;
    let chunks_of_3: Vec<Vec<&str>> = lines.chunks(3).map(|s| s.into()).collect();
    for chunk_of_3 in chunks_of_3 {
        let shared = day3_shared_char(chunk_of_3[0], chunk_of_3[1], chunk_of_3[2]).unwrap();
        //println!("{} {} {} -> {} {}", chunk_of_3[0], chunk_of_3[1], chunk_of_3[2], shared, shared as u8);
        result += (day3_char_to_point(shared)) as u32;
    }
    println!("{}", result)
}

fn day3_char_to_point(shared: char) -> u8 {
    if shared.is_lowercase() {
        //println!("{}", shared as u8 - 'a' as u8 + 1);
        shared as u8 - 'a' as u8 + 1
    } else if shared.is_uppercase() {
        //println!("{}", shared as u8 - 'A' as u8 + 26 + 1);
        shared as u8 - 'A' as u8 + 26 + 1
    } else {
        0
    }
}

// Thanks to: https://stackoverflow.com/questions/52882267/how-to-find-if-two-strings-have-common-characters-in-rust
fn day3_shared_char(a: &str, b: &str, c: &str) -> Option<char> {
    // get which one is shorter
    let (shorter, mid, longer) = if a.len() > b.len() {
        if a.len() > c.len() {
            (b, c, a)
        } else {
            (b, a, c)
        }
    } else if b.len() > c.len() {
        (a, b, c)
    } else {
        (a, c, b)
    };

    // fill the set with the characters from the shorter string
    let set: std::collections::HashSet<char> = mid.chars().collect();
    let set2: std::collections::HashSet<char> = shorter.chars().collect();

    //longer.chars().any(|c| set.contains(&c));
    for longer_char in longer.chars() {
        if set.contains(&longer_char) && set2.contains(&longer_char) {
            return Some(longer_char)
        }
    }
    None
}

fn day2() {
    let lines: Vec<_> = include_str!("input_day2.txt").lines().collect();
    let mut result = 0;
    let mut result2 = 0;
    for line in lines {
        match line {
            "A X" => {
                result += 1 + 3;
                result2 += 3 + 0;
            },
            "A Y" => {
                result += 2 + 6;
                result2 += 1 + 3;
            },
            "A Z" => {
                result += 3 + 0;
                result2 += 2 + 6;
            },
            "B X" => {
                result += 1 + 0;
                result2 += 1 + 0;
            },
            "B Y" => {
                result += 2 + 3;
                result2 += 2 + 3;
            },
            "B Z" => {
                result += 3 + 6;
                result2 += 3 + 6;
            },
            "C X" => {
                result += 1 + 6;
                result2 += 2 + 0;
            },
            "C Y" => {
                result += 2 + 0;
                result2 += 3 + 3;
            },
            "C Z" => {
                result += 3 + 3;
                result2 += 1 + 6;
            },
            _ => {
                panic!("unknown pattern");
            }
        }
    }
    println!("{} {}", result, result2);
}

fn day1() {
    let input = include_str!("input_day1.txt");
    let lines: Vec<_> = input.lines().collect();
    let mut chunks: Vec<u64> = vec![];
    let mut current_sum = 0;
    for line in lines {
        match line.parse::<u64>() {
            Ok(line_u64) => current_sum += line_u64,
            Err(_) => {
                chunks.push(current_sum);
                current_sum = 0;
            }
        }
    }
    chunks.sort();
    let mut sum_top3 = 0u64;
    print!("top3: ");
    for _ in 0 .. 3 {
        let popped = chunks.pop().unwrap();
        sum_top3 += popped;
        print!("{} ", popped);
    }
    println!();
    println!("sum: {}", sum_top3);
}
