use std::fs;

fn main() {
    let file_path = "./input.txt";
    let contents = fs::read_to_string(file_path).expect("unable to load file");
    let races = parse_contents(&contents);

    let mut p1: Vec<f32> = Vec::new();
    for (time, dist) in races {
        let disc = (time.powi(2) - (4.0 * dist)).sqrt();

        let i = ((time - disc) / 2.0).ceil();
        let j = ((time + disc) / 2.0).floor();
        let diff = j - i + 1.0;
        // println!("{}, {} => {}, {} => {}", time, dist, i, j, diff);
        p1.push(diff);
    }
    println!("P1: {}", p1.iter().product::<f32>());

    let p2 = solve_p2(&contents);
    println!("P2: {}", p2);
}

fn solve_p2(contents: &String) -> f64 {
    let lines: Vec<&str> = contents.lines().collect();
    let time = lines[0].strip_prefix("Time:").unwrap()
        .replace(" ", "")
        .parse::<f64>()
        .unwrap();
    let dist = lines[1].strip_prefix("Distance:").unwrap()
        .replace(" ", "")
        .parse::<f64>()
        .unwrap();

    let disc = (time.powi(2) - (4.0 * dist)).sqrt();
    let i = ((time - disc) / 2.0).ceil();
    let j = ((time + disc) / 2.0).floor();
    let diff = j - i + 1.0;
    diff
}

fn parse_contents(contents: &String) -> Vec<(f32, f32)> {
    let lines: Vec<&str> = contents.lines().collect();
    let times: Vec<f32> = lines[0].strip_prefix("Time:").unwrap()
        .split_whitespace()
        .map(str::parse::<f32>)
        .map(Result::unwrap)
        .collect();
    let distances: Vec<f32> = lines[1].strip_prefix("Distance:").unwrap()
        .split_whitespace()
        .map(str::parse::<f32>)
        .map(Result::unwrap)
        .map(|f| f + 1.0)
        .collect();

    let mut races = Vec::new();
    for (idx, time) in times.iter().enumerate() {
        races.push((*time, distances[idx]));
    }

    return races;
}
