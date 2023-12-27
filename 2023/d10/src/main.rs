use std::fs;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn north(&self) -> Option<Coord> {
        if self.row == 0 {
            None
        } else {
            Some(Coord { row: self.row - 1, col: self.col })
        }
    }

    fn south(&self) -> Option<Coord> {
        Some(Coord { row: self.row + 1, col: self.col })
    }

    fn east(&self) -> Option<Coord> {
        Some(Coord { row: self.row, col: self.col + 1 })
    }

    fn west(&self) -> Option<Coord> {
        if self.col == 0 {
            None
        } else {
            Some(Coord { row: self.row, col: self.col - 1 })
        }
    }

    fn neighbors(&self) -> Vec<Coord> {
        let cxs = vec![
            self.north(),
            self.south(),
            self.east(),
            self.west(),
        ];
        cxs.iter().flatten().map(|c| *c).collect()
    }
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    sym: char,
    coord: Coord,
}

impl Tile {
    fn neighbors(&self) -> Vec<Coord> {
        let nxs = match self.sym {
            '|' => vec![self.coord.north(), self.coord.south()],
            '-' => vec![self.coord.east(), self.coord.west()],
            'L' => vec![self.coord.north(), self.coord.east()],
            'J' => vec![self.coord.north(), self.coord.west()],
            '7' => vec![self.coord.south(), self.coord.west()],
            'F' => vec![self.coord.south(), self.coord.east()],
            _ => vec![]
        };
        nxs.iter().flatten().map(|c| *c).collect()
    }

    fn is_neighbor(&self, coord: &Coord) -> bool {
        for n in self.neighbors() {
            if n.row == coord.row && n.col == coord.col {
                return true;
            }
        }

        return false;
    }
}

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("cannot read to string");

    let mut start: Option<Coord> = None;
    let mut grid_p1: Vec<Vec<Tile>> = Vec::new();
    let mut grid_p2: Vec<Vec<char>> = Vec:: new();

    for (row, line) in contents.lines().enumerate() {
        let mut buff_p1 = Vec::new();
        let mut buff_p2 = Vec::new();

        for (col, sym) in line.chars().enumerate() {
            let tile = Tile { sym, coord: Coord { row, col } };

            if sym == 'S' {
                start = Some(Coord { row, col });
            }

            buff_p1.push(tile);
            buff_p2.push('.');
        }

        grid_p1.push(buff_p1);
        grid_p2.push(buff_p2);
    }

    let start_coord = start.expect("no starting coordingate found");

    let starts: Vec<Tile> = start_coord
        .neighbors()
        .iter()
        .map(|c| grid_p1[c.row][c.col])
        .filter(|t| t.is_neighbor(&start_coord))
        .collect();

    let mut prev = start_coord.clone();
    let mut next = starts[0].coord;
    let mut steps = 1;

    while next != start_coord {
        grid_p2[next.row][next.col] = grid_p1[next.row][next.col].sym;

        let next_tile = grid_p1[next.row][next.col];

        let nxs: Vec<Coord> = next_tile.neighbors().iter()
            .map(|c| *c)
            .filter(|c| *c != prev)
            .collect();

        prev = next.clone();
        next = nxs[0];
        steps += 1;
    }

    println!("P1: {}", steps / 2);

    let mut q: Vec<Coord> = Vec::new();
    let max_row = grid_p2.len();
    let max_col = grid_p2[0].len();
    for (col, sym) in grid_p2[0].iter().enumerate() {
        q.push(Coord { row: 0, col  });
    }
    for (col, sym) in grid_p2[max_row - 1].iter().enumerate() {
        q.push(Coord { row: max_row - 1, col  });
    }
    for (row, v) in grid_p2.iter().enumerate() {
        q.push(Coord { row, col: 0  });
        q.push(Coord { row, col: max_col - 1 });
    }

    let mut next = q.pop();
    while next.is_some() {
        let coord = next.unwrap();

        if coord.row < max_row && coord.col < max_col {
            let sym = grid_p2[coord.row][coord.col];
            if sym == '.' {
                grid_p2[coord.row][coord.col] = '+';
                coord.north().iter().for_each(|c| q.push(*c));
                coord.south().iter().for_each(|c| q.push(*c));
                coord.east().iter().for_each(|c| q.push(*c));
                coord.west().iter().for_each(|c| q.push(*c));
            }
        }

        next = q.pop();
    }

    for (row_num, row) in grid_p2.clone().iter().enumerate() {
        for (col_num, char) in row.iter().enumerate() {
            if *char == '.' {
                let mut east_walls = 0;
                for c in 0..col_num {
                    let wall_char = grid_p2[row_num][c];

                    if wall_char == '|' || wall_char == 'S' || wall_char == 'J' || wall_char == 'L' {
                        east_walls += 1;
                    }
                }

                let is_outside = east_walls % 2 == 0;

                if is_outside {
                    grid_p2[row_num][col_num] = '+';
                }
            }
        }
    }

    let mut p2 = 0;
    for row in grid_p2.iter() {
        for c in row.iter() {
            if *c == '.' {
                p2 += 1;
            }
        }
        let s: String = row.iter().collect();
        // println!("{}", s);
    }

    println!("P2: {}", p2);
}
