use std::fs;
use regex::Regex;

fn main() {
    let file_path = "./input.txt";
    println!("File Path: {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Unable to read the file");

    p1(contents.clone());
    p2(contents.clone());
}

fn p1(contents: String) {
    let re = Regex::new(r"\d").unwrap();
    let mut tot = 0;
    contents.lines().for_each(|line| {
        let nums: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();
        let first = nums.first().unwrap();
        let last = nums.last().unwrap();
        let calibration = format!("{first}{last}");
        tot += calibration.parse::<u32>().unwrap();
    });

    println!("Part 1: {}", tot);
}

fn p2(contents: String) {
    let mut tot = 0;
    contents.lines().for_each(|line| {
        let first = find_first_digit(line);
        let last = find_last_digit(line);
        let cal = format!("{first}{last}");
        tot += cal.parse::<i32>().unwrap();
    });
    println!("Part 2: {}", tot);
}

fn find_last_digit(line: &str) -> &str{
    for n in 1..(line.len() + 1) {
        let end = line.len() - n + 1;
        let buf = &line[..end];

        if buf.ends_with(char::is_numeric) {
            let ch = &buf[end - 1..end];
            return ch;
        }
        if buf.ends_with("one") {
            return "1";
        }
        if buf.ends_with("two") {
            return "2";
        }
        if buf.ends_with("three") {
            return "3";
        }
        if buf.ends_with("four") {
            return "4";
        }
        if buf.ends_with("five") {
            return "5";
        }
        if buf.ends_with("six") {
            return "6";
        }
        if buf.ends_with("seven") {
            return "7";
        }
        if buf.ends_with("eight") {
            return "8";
        }
        if buf.ends_with("nine") {
            return "9";
        }
    }

    return "0";
}

fn find_first_digit(line: &str) -> &str {
    for n in 0..line.len() {
        let buf = &line[n..];
        if buf.starts_with(char::is_numeric) {
            let i = &buf[0..1];
            return i;
        }
        if buf.starts_with("one") {
            return "1";
        }
        if buf.starts_with("two") {
            return "2";
        }
        if buf.starts_with("three") {
            return "3";
        }
        if buf.starts_with("four") {
            return "4";
        }
        if buf.starts_with("five") {
            return "5";
        }
        if buf.starts_with("six") {
            return "6";
        }
        if buf.starts_with("seven") {
            return "7";
        }
        if buf.starts_with("eight") {
            return "8";
        }
        if buf.starts_with("nine") {
            return "9";
        }
    }

    return "0";
}
