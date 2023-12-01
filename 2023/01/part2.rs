use std::io::stdin;

fn word_to_value(word: &str) -> Option<char> {
    match word {
        "zero" => Some('0'),
        "one" => Some('1'),
        "two" => Some('2'),
        "three" => Some('3'),
        "four" => Some('4'),
        "five" => Some('5'),
        "six" => Some('6'),
        "seven" => Some('7'),
        "eight" => Some('8'),
        "nine" => Some('9'),
        _ -> None
    }
}


fn numify(line: &str) -> String {
    let mut numbers = String::from("");
    let mut ptr = 0;
    for i in 1..(numbers.len()) {
        
        let subword = &lines[ptr..i];
        if let Some(digit) = word_to_value(subword) {
            numbers.push(digit);
            ptr = i;
        }
    }

    numbers
}


fn main() {
    let lines = stdin().lines();
    let mut total = 0;
    for line in lines {
        let line = line.unwrap();
        
    }
    println!("{}", total);
}
