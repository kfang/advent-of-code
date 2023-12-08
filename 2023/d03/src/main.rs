use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

fn is_symbol(ch: &char) -> bool {
    return !ch.is_numeric() && !ch.eq(&'.');
}

fn main() {
    let file_path = "./input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("unable to read the file");

    let mut grid: Vec<Vec<char>> = Vec::new();
    contents.lines().for_each(|line| grid.push(line.chars().collect::<Vec<_>>()));

    let mut total = 0;
    let mut gears: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    for (y, line) in grid.iter().enumerate() {

        let mut buf = String::new();
        let mut symbols: HashSet<(char, usize, usize)> = HashSet::new();
        
        for (x, c) in line.iter().enumerate() {
            if c.is_numeric() {
                buf.push(*c);

                let hasLeft = x > 0;
                let hasRight = x < line.len() - 1;
                let hasUp = y > 0;
                let hasDown = y < grid.len() - 1;
                
                // check left
                if hasLeft {
                    let ch = grid[y][x - 1];
                    if is_symbol(&ch) {
                        symbols.insert((ch, y, x - 1));
                    }
                }

                // check right
                if hasRight {
                    let ch = grid[y][x + 1];
                    if is_symbol(&ch) {
                        symbols.insert((ch, y, x + 1));
                    }
                }

                // check up
                if hasUp {
                    let ch = grid[y - 1][x];
                    if is_symbol(&ch) {
                        symbols.insert((ch, y - 1, x));
                    }
                }

                // check down
                if hasDown {
                    let ch = grid[y + 1][x];
                    if is_symbol(&ch) {
                        symbols.insert((ch, y + 1, x));
                    }
                }

                // check up-left
                if hasUp && hasLeft {
                    let ch = grid[y - 1][x - 1];
                    if is_symbol(&ch) {
                        symbols.insert((ch, y - 1, x - 1));
                    }
                }

                // check up-right
                if hasUp && hasRight {
                    let ch = grid[y - 1][x + 1];
                    if is_symbol(&ch) {
                        symbols.insert((ch, y - 1, x + 1));
                    }
                }

                // check down-left
                if hasDown && hasLeft {
                    let ch = grid[y + 1][x - 1];
                    if is_symbol(&ch) {
                        symbols.insert((ch, y + 1, x - 1));
                    }
                }
                
                // check down-right
                if hasDown && hasRight{
                    let ch = grid[y + 1][x + 1];
                    if is_symbol(&ch) {
                        symbols.insert((ch, y + 1, x + 1));
                    }
                }
            } else if !buf.is_empty() {
                if !symbols.is_empty() {
                    let partNo = buf.parse::<i32>().unwrap();
                    total += partNo;
                    symbols.iter().for_each(|(ch, _x, _y)| {
                        let key = (*_x, *_y);
                        
                        if gears.contains_key(&key) {
                            let vec = gears.get_mut(&key).unwrap();
                            vec.push(partNo);
                        } else {
                            gears.insert(key, vec![partNo]);
                        }
                    });
                }
                buf.clear();
                symbols.clear();
            }
        }

        if !buf.is_empty() {
            if !symbols.is_empty() {
                let partNo = buf.parse::<i32>().unwrap();
                total += partNo;

                symbols.iter().for_each(|(ch, _x, _y)| {
                    let key = (*_x, *_y);
                    
                    if gears.contains_key(&key) {
                        let vec = gears.get_mut(&key).unwrap();
                        vec.push(partNo);
                    } else {
                        gears.insert(key, vec![partNo]);
                    }
                });
            }
            buf.clear();
            symbols.clear();
        }
    }
    println!("P1: {}", total);


    let mut p2 = 0;
    for parts in gears.values() {
        if parts.len() > 1 {
            let product: i32 = parts.iter().product();
            p2 += product;
        }
    }
    println!("P2: {}", p2);
}
