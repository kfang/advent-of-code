use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn day_5_1(file_path: &str) -> i32 {
    let mut before: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut after: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut pages: Vec<Vec<i32>> = Vec::new();

    let mut mode = "rules";
    fs::read_to_string(file_path)
        .expect("unable to open file {file_path}")
        .lines()
        .for_each(|line| {
            if line.is_empty() {
                mode = "pages"
            } else if mode == "rules" {
                let pair: Vec<&str> = line.split('|').collect();
                let first = pair[0].parse::<i32>().unwrap();
                let second = pair[1].parse::<i32>().unwrap();

                if before.contains_key(&second) { 
                    before.get_mut(&second).unwrap().insert(first);
                } else { 
                    before.insert(second, HashSet::from_iter([first]));
                };

                if after.contains_key(&first) {
                    after.get_mut(&first).unwrap().insert(second);
                } else {
                    after.insert(first, HashSet::from_iter([second]));
                }
            } else if mode == "pages" {
                // do something else
                let tmp: Vec<i32> = line.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
                pages.push(tmp);
            }
        });

    let empty = HashSet::new();
    let mut total = 0;
    for pxs in pages {
        let mut is_valid = true;
        
        for idx in 0..pxs.len() {
            let curr = pxs[idx];

            let before_pages: &HashSet<i32> = before.get(&curr).unwrap_or(&empty);
            let before_slice = HashSet::from_iter(pxs[0..idx].to_vec());
            if before_slice.difference(before_pages).count() != 0 {
                is_valid = false;
                break;
            }

            if idx < pxs.len() {
                let after_pages = after.get(&curr).unwrap_or(&empty);
                let after_slice = HashSet::from_iter(pxs[(idx+1)..pxs.len()].to_vec());
                if after_slice.difference(after_pages).count() != 0 {
                    is_valid = false;
                    break;
                }
            }
        }

        if is_valid {
            // find middle and add
            let pos = pxs.len() / 2;
            total += pxs[pos];
        }
    }

    return total;
}


#[cfg(test)]
mod tests {
    use super::day_5_1;

    #[test]
    fn day_5_1_sample() {
        let result = day_5_1("inputs/d5_1_sample.txt");
        assert_eq!(result, 143);
    }
    
    #[test]
    fn day_5_1_run() {
        let result = day_5_1("inputs/d5_1.txt");
        println!("{result}");
        assert_eq!(result, 5948);
    }
}
