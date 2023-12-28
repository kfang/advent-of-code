use std::collections::HashSet;
use std::fs;

#[derive(Copy, Clone, Debug)]
struct Dist {
    vertical: usize,
    horizontal: usize,
}
type DistGrid = Vec<Vec<Dist>>;

#[derive(Copy, Clone, Debug)]
struct Coord {
    row: usize,
    col: usize,
}

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("unable to read file");

    let (dist_grid, gxs)= create_distance_grid(&contents, 2);
    let p1: usize = find_pairs(&gxs).iter().map(|p| calc_distance(p, &dist_grid)).sum();
    println!("P1: {}", p1);

    let (p2_grid, p2_gxs) = create_distance_grid(&contents, 1_000_000);
    let p2: usize = find_pairs(&p2_gxs).iter().map(|p| calc_distance(p, &p2_grid)).sum();
    println!("P2: {}", p2);
}

fn find_pairs(coords: &Vec<Coord>) -> Vec<(Coord, Coord)> {
    let mut pairs = Vec:: new();

    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            pairs.push((coords[i], coords[j]))
        }
    }

    return pairs;
}

fn calc_distance(pair: &(Coord, Coord), dist_grid: &DistGrid) -> usize {
    let mut start_h  = pair.0;
    let mut end_h = pair.1;
    if end_h.col < start_h.col {
        start_h = pair.1;
        end_h = pair.0;
    }
    let row = &dist_grid[start_h.row];
    let mut total_h = 0;
    for i in start_h.col..end_h.col {
        total_h += row[i + 1].horizontal;
    }

    let mut start_v = pair.0;
    let mut end_v = pair.1;
    if end_v.row < start_v.row {
        start_v = pair.1;
        end_v = pair.0;
    }
    let mut total_v: usize = 0;
    for i in start_v.row..end_v.row {
        total_v += &dist_grid[i + 1][start_v.col].vertical;
    }

    // println!("{:?} => H:{}, V:{}", pair, total_h, total_v);

    return total_v + total_h;
}

fn create_distance_grid(contents: &String, expansion: usize) -> (DistGrid, Vec<Coord>) {
    let mut grid = Vec::new();
    let mut galaxies: Vec<Coord> = Vec::new();

    let mut cols_seen: HashSet<usize> = HashSet::new();

    for (row_num, row) in contents.lines().enumerate() {
        let is_empty = row.chars().all(|c| c == '.');
        let default_dist = if is_empty { expansion } else { 1 };

        let row_buff: Vec<Dist> = row.chars().enumerate().map(|tup| match tup {
            (_, '.') => Dist { vertical: default_dist, horizontal: 1 },
            (i, _)   => {
                cols_seen.insert(i);
                galaxies.push(Coord { row: row_num, col: i });
                Dist { vertical: default_dist, horizontal: 1 }
            },
        }).collect();

        grid.push(row_buff);
    }

    let mut cols_missing: HashSet<usize> = HashSet::from_iter(0..grid[0].len());
    cols_seen.iter().for_each(|c| { cols_missing.remove(c); });

    for row in grid.iter_mut() {
        for &i in cols_missing.iter() {
            let d = row.get_mut(i).expect("missing dist");
            d.horizontal = expansion;
        }
    }

    return (grid, galaxies);
}

fn print_dist_grid(dist_grid: &DistGrid) {
    for row in dist_grid.iter() {
        for dist in row.iter() {
            print!("({}, {})", dist.vertical, dist.horizontal)
        }
        println!("");
    }
}
