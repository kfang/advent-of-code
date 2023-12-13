use std::fs;

fn main() {
    let file_path = "./input.txt";
    let contents = fs::read_to_string(file_path).expect("unable to read file");
    let histories = parse_contents(&contents);

    let p1 = solve_p1(&histories);
    println!("P1: {}", p1);

    let p2 = solve_p2(&histories);
    println!("P2: {}", p2);
}

fn parse_contents(contents: &String) -> Vec<Vec<i32>> {
    return contents.lines()
        .map(|line| {
            line
                .split_whitespace()
                .map(|s| s.parse::<i32>().expect("failed to parse to i32"))
                .collect()
        })
        .collect();
}

fn solve_history(vec: &Vec<i32>, forward: bool) -> i32 {
    if vec.iter().all(|i| *i == vec[0]) {
        return vec[0];
    }

    let mut diff: Vec<i32> = Vec::new();
    for i in 1..vec.len() {
        diff.push(vec[i] - vec[i - 1]);
    }

    let add = solve_history(&diff, forward);

    if forward {
        vec.last().expect("no last element") + add
    } else {
        vec.first().expect("no first element") - add
    }
}

fn solve_p1(histories: &Vec<Vec<i32>>) -> i32 {
    histories.iter().fold(0, |res, h| {
        res + solve_history(&h, true)
    })
}

fn solve_p2(histories: &Vec<Vec<i32>>) -> i32 {
    histories.iter().fold(0, |res, h| {
        let d = solve_history(&h, false);
        res + d
    })
}
