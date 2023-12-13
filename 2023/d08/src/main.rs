use std::{fs, collections::HashMap};

type NavMap = HashMap<String, (String, String)>;

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("unable to read file");
    let (pattern, nav_map) = parse_contents(&contents);

    let p1 = navigate_p1(&pattern, &nav_map);
    println!("P1: {}", p1);

    let p2 = navigate_p2(&pattern, &nav_map);
    println!("P2: {}", p2);
}

fn parse_contents(contents: &String) -> (String, NavMap) {
    let mut pattern = String::new();
    let mut nav_map: NavMap = HashMap::new();
    
    for (idx, line) in contents.lines().enumerate() {
        if idx.eq(&0) {
            pattern.push_str(line);
        } else if !line.is_empty() {
            let parts: Vec<&str> = line.split(&['=', '(', ',', ')', ' ']).filter(|s| !s.is_empty()).collect();
            let key = parts[0];
            let value = (parts[1].to_string(), parts[2].to_string());
            nav_map.insert(key.to_string(), value);
        }
    }

    return (pattern, nav_map);
}

fn navigate_p1(pattern: &String, nav_map: &NavMap) -> usize {
    let dst = String::from("ZZZ");
    let direction: Vec<char> = pattern.chars().collect();
    let mut dir_pos = 0;
    let mut curr_loc = String::from("AAA");
    
    while curr_loc.ne(&dst) {
        let lr_choice = nav_map.get(&curr_loc).expect("no location choice found");
        let pos = dir_pos % direction.len();
        
        curr_loc = match direction[pos] {
            'L' => lr_choice.0.clone(),
            'R' => lr_choice.1.clone(),
            _ => String::new(),
        };

        dir_pos += 1;
    }

    return dir_pos;
}

fn navigate_p2(pattern: &String, nav_map: &NavMap) -> usize {
    let direction: Vec<char> = pattern.chars().collect();

    let start: Vec<usize> = nav_map.keys()
        .filter(|k| k.ends_with("A"))
        .map(|s| s.clone())
        .map(|s| {
            let mut dir_pos = 0;
            let mut curr = s;
            
            while !curr.ends_with("Z"){
                let pos = dir_pos % direction.len();
                let choice = nav_map.get(&curr).expect("no location choice found");
                curr = match direction[pos] {
                    'L' => choice.0.clone(),
                    'R' => choice.1.clone(),
                    _ => String::new(),
                };
                dir_pos += 1;
            }

            dir_pos
        })
        .collect();

    return start.iter().skip(1).fold(start[0], |a, b| lcm(a, b.clone()));
}

fn lcm(a: usize, b: usize) -> usize {
    return a / gcd(a, b) * b;
}

fn gcd(a: usize, b: usize) -> usize {
    let mut big = if a.gt(&b) { a } else { b };
    let mut small = if a.lt(&b) { a } else { b };

    loop {
        let res = big % small;
        if res.eq(&0) {
            return small;
        }
        big = small;
        small = res;
    }
}