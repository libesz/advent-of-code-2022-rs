fn main() {
    day1();
    day2();
    day3();
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
