use std::fs;

pub fn day_4_1(file_path: &str) -> i32 {
    let lines: Vec<Vec<char>> = fs::read_to_string(file_path)
        .expect("unable to open file {file_path}")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut count = 0;
    for x in 0..lines.len() {
        for y in 0..lines[x].len() {
            if lines[x][y] == 'X' {
                // north
                if y >= 3 {
                    let north = vec![
                        lines[x][y].to_string(), 
                        lines[x][y-1].to_string(), 
                        lines[x][y-2].to_string(), 
                        lines[x][y-3].to_string(),
                    ].join("");
                    if north == "XMAS" {
                        count += 1;
                    }
                }

                // south
                if lines[x].len() > y + 3 {
                    let south = vec![
                        lines[x][y].to_string(), 
                        lines[x][y+1].to_string(), 
                        lines[x][y+2].to_string(), 
                        lines[x][y+3].to_string(),
                    ].join("");
                    if south == "XMAS" {
                        count += 1;
                    }
                }

                // east
                if x >= 3 {
                    let east = vec![
                        lines[x][y].to_string(),
                        lines[x-1][y].to_string(),
                        lines[x-2][y].to_string(),
                        lines[x-3][y].to_string(),
                    ].join("");
                    if east == "XMAS" {
                        count += 1;
                    }
                }

                // west
                if lines.len() > x + 3 {
                    let west = vec![
                        lines[x][y].to_string(),
                        lines[x+1][y].to_string(),
                        lines[x+2][y].to_string(),
                        lines[x+3][y].to_string(),
                    ].join("");
                    if west == "XMAS" {
                        count += 1;
                    }
                }

                // north east
                if x >=3 && y >= 3 {
                    let north_east = vec![
                        lines[x][y].to_string(),
                        lines[x-1][y-1].to_string(),
                        lines[x-2][y-2].to_string(),
                        lines[x-3][y-3].to_string(),
                    ].join("");
                    if north_east == "XMAS" {
                        count += 1;
                    }
                }
                
                // north west
                if x >= 3 && lines[x].len() > y + 3 {
                    let north_west = vec![
                        lines[x][y].to_string(),
                        lines[x-1][y+1].to_string(),
                        lines[x-2][y+2].to_string(),
                        lines[x-3][y+3].to_string(),
                    ].join("");
                    if north_west == "XMAS" {
                        count += 1;
                    }
                }

                // south east
                if lines.len() > x + 3 && y >= 3 {
                    let south_east = vec![
                        lines[x][y].to_string(),
                        lines[x+1][y-1].to_string(),
                        lines[x+2][y-2].to_string(),
                        lines[x+3][y-3].to_string(),
                    ].join("");
                    if south_east == "XMAS" {
                        count += 1;
                    }
                }
                
                // south west
                if lines.len() > x + 3 && lines[x].len() > y + 3 {
                    let south_west = vec![
                        lines[x][y].to_string(),
                        lines[x+1][y+1].to_string(),
                        lines[x+2][y+2].to_string(),
                        lines[x+3][y+3].to_string(),
                    ].join("");
                    if south_west == "XMAS" {
                        count += 1;
                    }
                }
            }
        }
    }

    return count;
}

pub fn day_4_2(file_path: &str) -> i32 {
    let lines: Vec<Vec<char>> = fs::read_to_string(file_path)
        .expect("unable to open file {file_path}")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut count = 0;
    for x in 1..(lines.len() - 1) {
        for y in 1..(lines[x].len() - 1) {
            if lines[x][y] == 'A' {
                let mut local_count = 0;
                
                if lines[x-1][y-1] == 'M' && lines[x+1][y+1] == 'S' {
                    local_count += 1;
                }

                if lines[x-1][y+1] == 'M' && lines[x+1][y-1] == 'S' {
                    local_count += 1;
                }

                if lines[x+1][y-1] == 'M' && lines[x-1][y+1] == 'S' {
                    local_count += 1;
                }

                if lines[x+1][y+1] == 'M' && lines[x-1][y-1] == 'S' {
                    local_count += 1;
                }

                if local_count >= 2 {
                    count += 1;
                }
            }
        }
    }
    return count;
}


#[cfg(test)]
mod tests {
    use super::day_4_1;
    use super::day_4_2;

    #[test]
    fn day_4_1_sample() {
        let result = day_4_1("inputs/d4_1_sample.txt");
        assert_eq!(result, 18);
    }
    
    #[test]
    fn day_4_1_run() {
        let result = day_4_1("inputs/d4_1.txt");
        println!("{result}");
        assert_eq!(result, 2543);
    }
    
    #[test]
    fn day_4_2_sample() {
        let result = day_4_2("inputs/d4_1_sample.txt");
        assert_eq!(result, 9);
    }
    
    #[test]
    fn day_4_2_run() {
        let result = day_4_2("inputs/d4_1.txt");
        println!("{result}");
        assert_eq!(result, 1930);
    }
}
