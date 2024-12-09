use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Grid {
    nodes: HashMap<char, Vec<(i32, i32)>>,
    x_len: i32,
    y_len: i32,
}

fn read_grid(file_path: &str) -> Grid {
    let lines: Vec<Vec<char>> = fs::read_to_string(file_path).expect("unable to read file").lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut nodes: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for x in 0..lines.len() {
        for y in 0..lines[x].len() {
            let node = lines[x][y];

            if node == '.' {
                continue;
            }

            let coord = (x as i32, y as i32);
            let v = nodes.get_mut(&node);
            if v.is_some() {
                v.unwrap().push(coord);
            } else {
                nodes.insert(node, vec![coord]);
            }
        }
    };

    let x_len = lines.len() as i32;
    let y_len = lines[0].len() as i32;

    return Grid { nodes, x_len, y_len };
}

fn is_valid_coord(grid: &Grid, coord: &(i32, i32)) -> bool {
    return coord.0 >= 0 
        && coord.0 < grid.x_len
        && coord.1 >= 0
        && coord.1 < grid.y_len
}

fn calc_antis(grid: &Grid, a: &(i32, i32), b: &(i32, i32), repeat: bool) -> Vec<(i32, i32)> {
    let x_diff = a.0 - b.0;
    let y_diff = a.1 - b.1;

    let mut res = vec![];

    let mut c_curr: Option<(i32, i32)> = Some((a.0 + x_diff, a.1 + y_diff));
    while c_curr.is_some() {
        let c = c_curr.unwrap();
        let is_valid = is_valid_coord(grid, &c);

        if is_valid {
            res.push(c);
        } else {
            break;
        }

        c_curr = if repeat && is_valid { Some((c.0 + x_diff, c.1 + y_diff)) } else { None };
    }
    
    let mut d_curr: Option<(i32, i32)> = Some((b.0 - x_diff, b.1 - y_diff));
    while d_curr.is_some() {
        let d = d_curr.unwrap();
        let is_valid = is_valid_coord(grid, &d);

        if is_valid {
            res.push(d);
        } else {
            break;
        }

        d_curr = if repeat && is_valid { Some((d.0 - x_diff, d.1 - y_diff)) } else { None };
    }

    return res;
}

pub fn day_8_1(file_path: &str) -> i32 {
    let grid = read_grid(file_path);


    let mut positions: HashSet<(i32, i32)> = HashSet::new();
    for (_node, coords) in grid.nodes.iter() {
        for i in 0..coords.len() {
            for j in i+1..coords.len() {
                calc_antis(&grid, &coords[i], &coords[j], false)
                    .iter()
                    .for_each(|coord| { positions.insert(*coord); });
            }
        }
    };

    return positions.len() as i32;
}

pub fn day_8_2(file_path: &str) -> i32 {
    let grid = read_grid(file_path);


    let mut positions: HashSet<(i32, i32)> = HashSet::new();

    for (_node, coords) in grid.nodes.iter() {
        for i in 0..coords.len() {
            for j in i+1..coords.len() {
                positions.insert(coords[i]);
                positions.insert(coords[j]);
                calc_antis(&grid, &coords[i], &coords[j], true)
                    .iter()
                    .for_each(|coord| { positions.insert(*coord); });
            }
        }
    };

    return positions.len() as i32;
}

#[cfg(test)]
mod tests {
    use super::day_8_1;
    use super::day_8_2;

    #[test]
    fn day_8_1_sample() {
        let result = day_8_1("inputs/d8_1_sample.txt");
        assert_eq!(result, 14);
    }
    
    #[test]
    fn day_8_1_run() {
        let result = day_8_1("inputs/d8_1.txt");
        println!("{result}");
        assert_eq!(result, 273);
    }
    
    #[test]
    fn day_8_2_sample() {
        let result = day_8_2("inputs/d8_1_sample.txt");
        assert_eq!(result, 34);
    }
    
    #[test]
    fn day_8_2_run() {
        let result = day_8_2("inputs/d8_1.txt");
        println!("{result}");
        assert_eq!(result, 34);
    }
}
