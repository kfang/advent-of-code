use std::fs;

pub fn day_3_1(file_path: &str) -> i32 {
    let contents = fs::read_to_string(file_path).expect("could not read file");
    let mut sum = 0;
    let mut iter = contents.chars().peekable();
    let mut tok = "".to_owned();
    let mut left = "".to_owned();
    let mut right = "".to_owned();
    while let Some(&c) = iter.peek() {
        match c {
            'm' if tok == "" => {
                tok.push(c);
            }
            'u' if tok == "m" => {
                tok.push(c);
            }
            'l' if tok == "mu" => {
                tok.push(c);
            }
            '(' if tok == "mul" => {
                tok.push(c);
            }
            '0'..='9' if tok == "mul(" => {
                left.push(c);
            }
            '0'..='9' if tok == "mul(," => {
                right.push(c);
            }
            ',' if tok == "mul(" && !left.is_empty() => {
                tok.push(c);
            }
            ')' if tok == "mul(," && !left.is_empty() && !right.is_empty() => {
                let left_int = left.parse::<i32>().unwrap();
                let right_int = right.parse::<i32>().unwrap();

                sum += left_int * right_int;

                tok.truncate(0);
                left.truncate(0);
                right.truncate(0);
            }
            _ => {
                tok.truncate(0);
                left.truncate(0);
                right.truncate(0);
            }
        }
        iter.next();

    }

    return sum;
}

pub fn day_3_2(file_path: &str) -> i32 {
    let contents = fs::read_to_string(file_path).expect("could not read file");
    
    let mut sum = 0;

    let mut is_enabled = true;
    let mut iter = contents.chars().peekable();
    let mut tok = "".to_owned();
    let mut left = "".to_owned();
    let mut right = "".to_owned();
    while let Some(&c) = iter.peek() {
        match c {
            'd' if tok == "" => {
                tok.push(c);
            }
            'o' if tok == "d" => {
                tok.push(c);
            }
            'n' if tok == "do" => {
                tok.push(c);
            }
            '\'' if tok == "don" => {
                tok.push(c);
            }
            't' if tok == "don\'" => {
                tok.push(c);
            }
            'm' if tok == "" => {
                tok.push(c);
            }
            'u' if tok == "m" => {
                tok.push(c);
            }
            'l' if tok == "mu" => {
                tok.push(c);
            }
            '(' if tok == "mul" => {
                tok.push(c);
            }
            '(' if tok == "do" => {
                tok.push(c);
            }
            '(' if tok == "don\'t" => {
                tok.push(c);
            }
            '0'..='9' if tok == "mul(" => {
                left.push(c);
            }
            '0'..='9' if tok == "mul(," => {
                right.push(c);
            }
            ',' if tok == "mul(" && !left.is_empty() => {
                tok.push(c);
            }
            ')' if tok == "mul(," && !left.is_empty() && !right.is_empty() => {
                let left_int = left.parse::<i32>().unwrap();
                let right_int = right.parse::<i32>().unwrap();

                if is_enabled {
                    sum += left_int * right_int;
                }

                tok.truncate(0);
                left.truncate(0);
                right.truncate(0);
            }
            ')' if tok == "do(" => {
                is_enabled = true;
                tok.truncate(0);
                left.truncate(0);
                right.truncate(0);
            }
            ')' if tok == "don\'t(" => {
                is_enabled = false;
                tok.truncate(0);
                left.truncate(0);
                right.truncate(0);
            }
            _ => {
                tok.truncate(0);
                left.truncate(0);
                right.truncate(0);
            }
        }
        iter.next();

    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::day_3_1;
    use super::day_3_2;

    #[test]
    fn day_3_1_sample() {
        let result = day_3_1("inputs/d3_1_sample.txt");
        assert_eq!(result, 161);
    }
    
    #[test]
    fn day_3_1_run() {
        let result = day_3_1("inputs/d3_1.txt");
        println!("{result}");
        assert_eq!(result, 188116424);
    }
    
    #[test]
    fn day_3_2_sample() {
        let result = day_3_2("inputs/d3_2_sample.txt");
        assert_eq!(result, 48);
    }
    
    #[test]
    fn day_3_2_run() {
        let result = day_3_2("inputs/d3_1.txt");
        println!("{result}");
        assert_eq!(result, 104245808);
    }
}
