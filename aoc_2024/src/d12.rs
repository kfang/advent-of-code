use std::fs;
use std::collections::HashSet;

#[derive(Debug)]
struct Region {
    area: usize,
    fences: usize,
}

pub fn day_12_1(file_path: &str) -> usize {
    let grid: Vec<Vec<char>> = fs::read_to_string(file_path).expect("unable to read file")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut regions: Vec<Region> = vec![];

    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            let plant_id = grid[x][y];
           
            if visited.contains(&(x, y)) {
                continue;
            }

            let mut queue: Vec<(usize, usize)> = vec![(x, y)];

            // calculate region by finding adjacent plots with the same id
            let mut area = 0;
            let mut fences = 0;
            while !queue.is_empty() {
                let coord = queue.pop().unwrap();

                if visited.contains(&coord) {
                    continue;
                }

                let (curr_x, curr_y) = coord;
                let mut curr_fences = 4;

                // north
                if curr_x > 0 && grid[curr_x-1][curr_y] == plant_id {
                    queue.push((curr_x-1, curr_y));
                    curr_fences -= 1;
                }

                // south
                if curr_x + 1 < grid.len() && grid[curr_x+1][curr_y] == plant_id {
                    queue.push((curr_x+1, curr_y));
                    curr_fences -= 1;
                }

                // east
                if curr_y + 1 < grid[curr_x].len() && grid[curr_x][curr_y+1] == plant_id {
                    queue.push((curr_x, curr_y+1));
                    curr_fences -= 1;
                }

                // west
                if curr_y > 0 && grid[curr_x][curr_y-1] == plant_id {
                    queue.push((curr_x, curr_y-1));
                    curr_fences -= 1;
                }
                
                visited.insert(coord);

                area += 1;
                fences += curr_fences;
            }

            regions.push(Region { area, fences });
        }
    }

    let total = regions.iter().fold(0, |price, region| {
        price + (region.area * region.fences)
    });

    return total;
}

#[cfg(test)]
mod tests {
    use super::day_12_1;

    #[test]
    fn day_12_1_sample() {
        let result = day_12_1("inputs/d12_1_sample.txt");
        assert_eq!(result, 1930);
    }
    
    #[test]
    fn day_12_1_run() {
        let result = day_12_1("inputs/d12_1.txt");
        println!("{result}");
        assert_eq!(result, 1930);
    }
}
