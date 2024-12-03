use std::fs;

fn is_sorted(vec: &Vec<i32>, tolerate: bool) -> bool {
    let mut diffs: Vec<i32> = Vec::new();

    let mut positives = 0;
    let mut negatives = 0;
    
    for idx in 1..vec.len() {
        let diff = vec[idx - 1] - vec[idx];
        diffs.push(diff);
        
        if diff > 0 {
            positives += 1;
        }
        if diff < 0 {
            negatives += 1;
        }
    }

    let bad_pos;
    if positives > negatives {
        bad_pos = diffs.iter().position(|x| !(x >= &1 && x <= &3));
    } else {
        bad_pos = diffs.iter().position(|x| !(x >= &-3 && x <= &-1));
    }

    if bad_pos.is_some() && tolerate {
        let mut p1 = vec.clone();
        p1.remove(bad_pos.unwrap());
        let mut p2 = vec.clone();
        p2.remove(bad_pos.unwrap() + 1);
        return is_sorted(&p1, false) || is_sorted(&p2, false);
    }

    return bad_pos.is_none();
}

fn get_levels(file_path: &str) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(file_path).expect("unable to read {file_path}");
    let lines: Vec<&str> = contents.split("\n").filter(|line| !line.is_empty()).collect();
    return lines.iter().map(|line| line.split_whitespace().map(|level| level.parse::<i32>().unwrap()).collect()).collect();
}

pub fn day_2_1(file_path: &str) -> i32 {
    let levels = get_levels(file_path);

    let mut count: i32 = 0;
    levels.iter().for_each(|lxs| {
        let is_sorted = is_sorted(&lxs, false);
        
        if is_sorted {
            count += 1;
        }
    });

    return count;
}

pub fn day_2_2(file_path: &str) -> i32 {
    let levels = get_levels(file_path);
    
    let mut count: i32 = 0;
    levels.iter().for_each(|lxs| {
        let is_sorted = is_sorted(&lxs, true);
        
        if is_sorted {
            count += 1;
        }
    });

    return count;
}


#[cfg(test)]
mod tests {
    use super::day_2_1;
    use super::day_2_2;

    #[test]
    fn day_2_1_sample() {
        let result = day_2_1("inputs/d2_1_sample.txt");
        assert_eq!(result, 2);
    }
    
    #[test]
    fn day_2_1_run() {
        let result = day_2_1("inputs/d2_1.txt");
        println!("{result}");
        assert_eq!(result, 390);
    }

    #[test]
    fn day_2_2_sample() {
        let result = day_2_2("inputs/d2_1_sample.txt");
        assert_eq!(result, 6);
    }
    
    #[test]
    fn day_2_2_run() {
        let result = day_2_2("inputs/d2_1.txt");
        println!("{result}");
        assert_eq!(result, 439);
    }
}
