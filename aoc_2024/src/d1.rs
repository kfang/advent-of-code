use std::fs;
use std::collections::HashMap;

fn read_left_right(file_path: &str) -> (Vec<i32>, Vec<i32>) {
    let contents = fs::read_to_string(file_path).expect("could not read file");

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    contents
        .split('\n')
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let pair: Vec<&str> = line.split_whitespace().collect();
            left.push(pair[0].parse::<i32>().unwrap());
            right.push(pair[1].parse::<i32>().unwrap());
        });

    left.sort();
    right.sort();

    return (left, right);
}

pub fn day_1_1(file_path: &str) -> i32 {
    let (left, right) = read_left_right(file_path);

    let mut sum: i32 = 0;
    for idx in 0..left.len() {
        let diff = (left[idx] - right[idx]).abs();
        sum += diff;
    }

    return sum;
}

pub fn day_1_2(file_path: &str) -> i32 {
    let (left, right) = read_left_right(file_path);

    let mut right_count: HashMap<&i32, i32> = HashMap::new();
    right.iter().for_each(|key| { 
        let val: i32 = right_count.get(key).unwrap_or(&0).clone();
        right_count.insert(key, val + 1);
    });

    let mut sum: i32 = 0;
    left.iter().for_each(|key| {
        let count = right_count.get(&key).unwrap_or(&0);
        sum += key * count
    });

    return sum;
}

#[cfg(test)]
mod tests {
    use super::day_1_1;
    use super::day_1_2;

    #[test]
    fn day_1_1_sample() {
        let sample_result = day_1_1("inputs/d1_1_sample.txt");
        assert_eq!(sample_result, 11);
    }
    
    #[test]
    fn day_1_1_run() {
        let result = day_1_1("inputs/d1_1.txt");
        println!("D1_1: {result}");
    }
    
    #[test]
    fn day_1_2_sample() {
        let sample_result = day_1_2("inputs/d1_1_sample.txt");
        assert_eq!(sample_result, 31);
    }
    
    #[test]
    fn day_1_2_run() {
        let result = day_1_2("inputs/d1_1.txt");
        println!("D1_2: {result}");
    }
}
