use std::fs;

#[derive(Debug)]
struct Calibration {
    result: i64,
    elems: Vec<i64>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Operators {
    Add,
    Multiply,
    Concat
}

fn read_file(file_path: &str) -> Vec<Calibration> {
    fs::read_to_string(file_path)
        .expect("unable to read {file_path}")
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            let result = parts[0].parse::<i64>().unwrap();
            let elems: Vec<i64> = parts[1].split_whitespace().map(|e| e.parse::<i64>().unwrap()).collect();
            return Calibration { result, elems }
        })
        .collect()
}

fn find_operators(ops: &Vec<Operators>, target: i64, curr_total: i64, elems: &Vec<i64>) -> bool {
    // operators can only increase values, cut early if total is already too big
    if curr_total > target {
        return false;
    }

    // no more elements, check the running total
    if elems.is_empty() {
        if curr_total == target {
            return true
        }
        return false
    }

    let mut next_elems = elems.clone();
    let curr = next_elems.pop().unwrap();

    for op in ops {
        match op {
            Operators::Add => {
                let res = find_operators(ops, target, curr_total + curr, &next_elems);
                if res {
                    return true;
                }
            }
            Operators::Multiply => {
                let res = find_operators(ops,target, curr_total * curr, &next_elems);
                if res {
                    return true;
                }
            }
            Operators::Concat => {
                let mut next_str = String::new();
                next_str += &curr_total.to_string() ;
                next_str += &curr.to_string();
                let next_total = next_str.parse::<i64>().unwrap();
                let res = find_operators(ops, target, next_total, &next_elems);
                if res {
                    return true;
                }
            }
        }
    }

    return false;
}

 pub fn day_7_1(file_path: &str) -> i64 {
    let calibrations = read_file(file_path);

    let ops = vec![Operators::Add, Operators::Multiply];
    let mut total = 0;
    for calibration in calibrations {
        let mut elems = calibration.elems.clone();
        elems.reverse();
        let curr_total = elems.pop().unwrap();

        let is_valid = find_operators(
            &ops,
            calibration.result, 
            curr_total,
            &mut elems,
        );

        if is_valid {
            total += calibration.result;
        }
    };

    return total;
}
 
pub fn day_7_2(file_path: &str) -> i64 {
    let calibrations = read_file(file_path);

    let ops = vec![Operators::Add, Operators::Multiply, Operators::Concat];
    let mut total = 0;
    for calibration in calibrations {
        let mut elems = calibration.elems.clone();
        elems.reverse();
        let curr_total = elems.pop().unwrap();

        let is_valid = find_operators(
            &ops,
            calibration.result, 
            curr_total,
            &mut elems,
        );

        if is_valid {
            total += calibration.result;
        }
    };

    return total;
}

#[cfg(test)]
mod tests {
    use super::day_7_1;
    use super::day_7_2;

    #[test]
    fn day_7_1_sample() {
        let result = day_7_1("inputs/d7_1_sample.txt");
        assert_eq!(result, 3749);
    }
    
    #[test]
    fn day_7_1_run() {
        let result = day_7_1("inputs/d7_1.txt");
        println!("{result}");
        assert_eq!(result, 5030892084481);
    }
    
    #[test]
    fn day_7_2_sample() {
        let result = day_7_2("inputs/d7_1_sample.txt");
        assert_eq!(result, 11387);
    }
    
    #[test]
    fn day_7_2_run() {
        let result = day_7_2("inputs/d7_1.txt");
        println!("{result}");
        assert_eq!(result, 91377448644679);
    }
}
