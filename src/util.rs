pub fn num_to_alphabet(i: usize) -> char {
    match i {
        0 => 'A',
        1 => 'B',
        2 => 'C',
        3 => 'D',
        4 => 'E',
        5 => 'F',
        6 => 'G',
        7 => 'H',
        8 => 'I',
        9 => 'J',
        10 => 'K',
        11 => 'L',
        12 => 'M',
        13 => 'N',
        14 => 'O',
        15 => 'P',
        16 => 'Q',
        17 => 'R',
        18 => 'S',
        19 => 'T',
        20 => 'U',
        21 => 'V',
        22 => 'W',
        23 => 'X',
        24 => 'Y',
        25 => 'Z',
        _ => ' ',
    }
}

pub fn alphabet_to_num(c: char) -> usize {
    match c {
        'a' | 'A' => 0,
        'b' | 'B' => 1,
        'c' | 'C' => 2,
        'd' | 'D' => 3,
        'e' | 'E' => 4,
        'f' | 'F' => 5,
        'g' | 'G' => 6,
        'h' | 'H' => 7,
        'i' | 'I' => 8,
        'j' | 'J' => 9,
        'k' | 'K' => 10,
        'l' | 'L' => 11,
        'm' | 'M' => 12,
        'n' | 'N' => 13,
        'o' | 'O' => 14,
        'p' | 'P' => 15,
        'q' | 'Q' => 16,
        'r' | 'R' => 17,
        's' | 'S' => 18,
        't' | 'T' => 19,
        'u' | 'U' => 20,
        'v' | 'V' => 21,
        'w' | 'W' => 22,
        'x' | 'X' => 23,
        'y' | 'Y' => 24,
        'z' | 'Z' => 25,
        _ => 26,
    }
}

pub fn ask_yes_or_no() -> bool {
    let mut input_line = String::new();
    std::io::stdin().read_line(&mut input_line).expect("Failed to read from stdin");
    match input_line.split_whitespace().take(1).last().unwrap() {
        "yes" | "y" => true,
        "no" | "n" => false,
        _ => {
            println!("Invalid input. Type yes/y/no/n");
            ask_yes_or_no()
        },
    }
}

pub fn ask_matrix_option(min_x: usize, max_x: usize, min_y: usize, max_y: usize) -> (usize, usize) {
    let mut input_line = String::new();
    std::io::stdin().read_line(&mut input_line).expect("Failed to read from stdin");

    let y = match input_line.trim().chars().next() {
        Some(c) => {
            let y = alphabet_to_num(c);
            if y >= min_y && y <= max_y {
                y
            } else {
                panic!("aaaaaaaaa too big/small");
            }
        },
        None => {
            panic!("aaaaaaaaa no letter found");
        }
    };

    let x = match input_line.trim()[1..].parse::<usize>() {
        Ok(digit) if digit <= max_x && digit >= min_x => digit as usize,
        _ => {
            panic!("aaaaaaaaa no letter found or too small/big");
        }
    };

    (x, y)
}
