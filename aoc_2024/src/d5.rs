use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

struct PagesInput {
    before: HashMap<i32, HashSet<i32>>,
    after: HashMap<i32, HashSet<i32>>,
    pages: Vec<Vec<i32>>,
}

fn read_input(file_path: &str) -> PagesInput {
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

    return PagesInput { before, after, pages};
}

fn is_valid_ord(input: &PagesInput, pxs: &Vec<i32>) -> bool {
    let empty = HashSet::new();
    let mut is_valid = true;
    
    for idx in 0..pxs.len() {
        let curr = pxs[idx];

        let before_pages: &HashSet<i32> = input.before.get(&curr).unwrap_or(&empty);
        let before_slice = HashSet::from_iter(pxs[0..idx].to_vec());
        if before_slice.difference(before_pages).count() != 0 {
            is_valid = false;
            break;
        }

        if idx < pxs.len() {
            let after_pages = input.after.get(&curr).unwrap_or(&empty);
            let after_slice = HashSet::from_iter(pxs[(idx+1)..pxs.len()].to_vec());
            if after_slice.difference(after_pages).count() != 0 {
                is_valid = false;
                break;
            }
        }
    }

    return is_valid;
}

fn compare(input: &PagesInput, a: &i32, b: &i32) -> std::cmp::Ordering {
    let is_before = input.before.get(a).map(|s| s.contains(b)).unwrap_or(false);
    if is_before {
        return std::cmp::Ordering::Greater;
    }

    let is_after = input.after.get(a).map(|s| s.contains(b)).unwrap_or(false);
    if is_after {
        return std::cmp::Ordering::Less;
    }

    return std::cmp::Ordering::Equal;
}

pub fn day_5_1(file_path: &str) -> i32 {
    let page_input = read_input(file_path);
    let pages: Vec<Vec<i32>> = page_input.pages.clone();

    let mut total = 0;
    for pxs in pages {
        let is_valid = is_valid_ord(&page_input, &pxs);
        if is_valid {
            // find middle and add
            let pos = pxs.len() / 2;
            total += pxs[pos];
        }
    }

    return total;
}

pub fn day_5_2(file_path: &str) -> i32 {
    let page_input = read_input(file_path);
    let mut total = 0;

    page_input.pages.iter().for_each(|page| {
        let is_valid = is_valid_ord(&page_input, &page);
        
        if !is_valid {
            //time to make it ordered
            let mut sorted = page.clone();
            sorted.sort_by(|a, b| compare(&page_input, a, b));

            let pos = sorted.len() / 2;
            total += sorted[pos];
        }
    });

    return total;
}


#[cfg(test)]
mod tests {
    use super::day_5_1;
    use super::day_5_2;

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
    
    #[test]
    fn day_5_2_sample() {
        let result = day_5_2("inputs/d5_1_sample.txt");
        assert_eq!(result, 123);
    }
    
    #[test]
    fn day_5_2_run() {
        let result = day_5_2("inputs/d5_1.txt");
        assert_eq!(result, 3062);
    }
}
