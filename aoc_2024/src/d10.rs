use std::fs;
use std::collections::HashSet;

fn calc_adjacents(grid: &Vec<Vec<i8>>, coord: (usize, usize)) -> Vec<(usize, usize)> {
    let mut adjacents: Vec<(usize, usize)> = vec![];

    // north
    if coord.0 > 0 {
        adjacents.push((coord.0 - 1, coord.1));
    }

    // south
    if coord.0 + 1 < grid.len() {
        adjacents.push((coord.0 + 1, coord.1));
    }

    // east
    if coord.1 + 1 < grid[coord.0].len() {
        adjacents.push((coord.0, coord.1 + 1));
    }

    // west
    if coord.1 > 0 {
        adjacents.push((coord.0, coord.1 - 1));
    }

    return adjacents;
}

fn calc_trailhead_score(grid: &Vec<Vec<i8>>, trailhead: &(usize, usize)) -> (i32, i32) {
    let mut summits: HashSet<(usize, usize)> = HashSet::new();
    let mut path_count = 0;

    let mut todo_stack: Vec<(usize, usize)> = vec![*trailhead];
    while !todo_stack.is_empty() {
        let curr_coord = todo_stack.pop().unwrap();
        let curr_height = grid[curr_coord.0][curr_coord.1];
        let target_height = curr_height + 1;
        
        if curr_height == 9 {
            summits.insert(curr_coord);
            path_count += 1;
        } else {
            calc_adjacents(grid, curr_coord)
                .iter()
                .filter(|next_coord| grid[next_coord.0][next_coord.1] == target_height)
                .for_each(|next_coord| todo_stack.push(*next_coord));
        }
    }

    return (summits.len() as i32, path_count);
}

pub fn day_10_1(file_path: &str) -> i32 {
    let grid: Vec<Vec<i8>> = fs::read_to_string(file_path).expect("unable to read file")
        .lines()
        .map(|line| line.chars().map(|c| c.to_string().parse::<i8>().unwrap()).collect())
        .collect();

    let mut trailheads: Vec<(usize, usize)> = vec![];
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] == 0 {
                trailheads.push((x, y));
            }
        }
    }

    let total = trailheads.iter().fold(0, |total, trailhead| {
        total + calc_trailhead_score(&grid, trailhead).0
    });

    return total;
}

pub fn day_10_2(file_path: &str) -> i32 {
    let grid: Vec<Vec<i8>> = fs::read_to_string(file_path).expect("unable to read file")
        .lines()
        .map(|line| line.chars().map(|c| c.to_string().parse::<i8>().unwrap()).collect())
        .collect();

    let mut trailheads: Vec<(usize, usize)> = vec![];
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] == 0 {
                trailheads.push((x, y));
            }
        }
    }

    let total = trailheads.iter().fold(0, |total, trailhead| {
        total + calc_trailhead_score(&grid, trailhead).1
    });

    return total;
}

#[cfg(test)]
mod tests {
    use super::day_10_1;
    use super::day_10_2;
    
    #[test]
    fn day_10_1_sample() {
        let result = day_10_1("inputs/d10_1_sample.txt");
        assert_eq!(result, 36);
    }
    
    #[test]
    fn day_10_1_run() {
        let result = day_10_1("inputs/d10_1.txt");
        println!("{result}");
        assert_eq!(result, 550);
    }
    
    #[test]
    fn day_10_2_sample() {
        let result = day_10_2("inputs/d10_1_sample.txt");
        assert_eq!(result, 81);
    }
    
    #[test]
    fn day_10_2_run() {
        let result = day_10_2("inputs/d10_1.txt");
        println!("{result}");
        assert_eq!(result, 1255);
    }
}

