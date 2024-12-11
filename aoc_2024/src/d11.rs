use std::fs;
use std::collections::HashMap;

pub fn run_ticks(num: &u64, ticks: u32) -> usize {
    let mut stones: Vec<u64> = vec![*num];

    for _iteration in 0..ticks {
        let mut next: Vec<u64> = vec![];
        stones.iter().for_each(|stone| {
            if stone == &0 {
                next.push(1);
            } else if stone.to_string().len() % 2 == 0 {
                let stone_str = stone.to_string();
                let digits = stone_str.len() / 2;

                let left = stone_str[0..digits].parse::<u64>().expect("unable to parse left: {stone}");
                let right = stone_str[digits..stone_str.len()].parse::<u64>().expect("unable to parse right: {stone}");
                next.push(left);
                next.push(right);
            } else {
                next.push(stone * 2024);
            }
        });
        stones = next;
        println!("{_iteration}");
    }

    //println!("{num}, {ticks} => {:?}", stones);

    return stones.len();
}

pub fn run_ticks_c(stone: &u64, ticks: u32, cache: &mut HashMap<(u64, u32), usize>) -> usize {

    let cached = cache.get(&(*stone, ticks));
    if cached.is_some() {
        return *cached.unwrap();
    };

    if ticks == 0 {
        return 1;
    }
    
    if stone == &0 {
        let res = run_ticks_c(&1, ticks - 1, cache);
        cache.insert((*stone, ticks), res);
        return res;
    } else if stone.to_string().len() % 2 == 0 {
        let stone_str = stone.to_string();
        let digits = stone_str.len() / 2;

        let left = stone_str[0..digits].parse::<u64>().expect("unable to parse left: {stone}");
        let right = stone_str[digits..stone_str.len()].parse::<u64>().expect("unable to parse right: {stone}");
        let res = run_ticks_c(&left, ticks - 1, cache) + run_ticks_c(&right, ticks - 1, cache);
        cache.insert((*stone, ticks), res);
        return res;
    } else {
        let next = stone * 2024;
        let res = run_ticks_c(&next, ticks - 1, cache);
        cache.insert((*stone, ticks), res);
        return res;
    }
}

pub fn day_11_1(file_path: &str) -> usize {
    let stones: Vec<u64> = fs::read_to_string(file_path)
        .expect("unable to read file")
        .split_whitespace()
        .map(|s| s.parse::<u64>().expect("unable to parse {s} to u32"))
        .collect();

    let count = stones.iter().fold(0, |total, stone| {
        total + run_ticks(stone, 25)
    });
    
    return count;
}

pub fn day_11_2(file_path: &str) -> usize {
    let stones: Vec<u64> = fs::read_to_string(file_path)
        .expect("unable to read file")
        .split_whitespace()
        .map(|s| s.parse::<u64>().expect("unable to parse {s} to u32"))
        .collect();

    let mut cache: HashMap<(u64, u32), usize> = HashMap::new();

    let count = stones.iter().fold(0, |total, stone| {
        let next = total + run_ticks_c(stone, 75, &mut cache);
        return next;
    });
    
    return count;
}

#[cfg(test)]
mod tests {
    use super::day_11_1;
    use super::day_11_2;

    #[test]
    fn day_11_1_sample() {
        let result = day_11_1("inputs/d11_1_sample.txt");
        assert_eq!(result, 55312);
    }
    
    #[test]
    fn day_11_1_run() {
        let result = day_11_1("inputs/d11_1.txt");
        println!("{result}");
        assert_eq!(result, 187738);
    }
    
    #[test]
    fn day_11_2_run() {
        let result = day_11_2("inputs/d11_1.txt");
        println!("{result}");
        assert_eq!(result, 223767210249237);
    }
}
