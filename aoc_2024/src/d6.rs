use std::fs;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct GuardPos {
    dir: Direction,
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct GameState {
    grid: Vec<Vec<char>>,
    visited: HashSet<(usize, usize)>,
    guard_pos: GuardPos,
    x_len: usize,
    y_len: usize,
}

impl GameState {
    fn is_out_bounds(&self, coord: (usize, usize)) -> bool {
        let (x, y) = coord;
        return x >= self.x_len || y >= self.y_len;
    }

    fn is_open(&self, coord: (usize, usize)) -> bool {
        let (x, y) = coord;
        let ch = self.grid[x][y];
        return ch == '.' || ch == '^';
    }
}

fn read_grid(file_path: &str) -> GameState {
    let mut grid: Vec<Vec<char>> = fs::read_to_string(file_path)
        .expect("cannot read {file_path}")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut guard_pos: Option<GuardPos> = None;
    for x in 0..grid.len() {
        if guard_pos.is_some() {
            break;
        }
        for y in 0..grid[x].len() {
            if grid[x][y] == '^' {
                guard_pos = Some(GuardPos { dir: Direction::North, x, y });
                grid[x][y] = '.';
                break;
            };
        }
    }

    let visited = HashSet::new();
    let x_len = grid.len();
    let y_len = grid[0].len();
    return GameState { grid, guard_pos: guard_pos.unwrap(), visited, x_len, y_len };
}

fn next_pos(game_state: &GameState, order: &Vec<Direction>, coord: (usize, usize)) -> Option<GuardPos> {
    let (x, y) = coord;

    for ord in order {
        match ord {
            Direction::North => {
                if x == 0 {
                    return None
                }
                let next_coord = (x-1, y);
                if game_state.is_out_bounds(next_coord) {
                    return None;
                }
                if game_state.is_open(next_coord) {
                    return Some(GuardPos { dir: Direction::North, x: next_coord.0, y: next_coord.1 });
                }
            }
            Direction::South => {
                let next_coord = (x+1, y);
                if game_state.is_out_bounds(next_coord) {
                    return None;
                }
                if game_state.is_open(next_coord) {
                    return Some(GuardPos { dir: Direction::South , x: next_coord.0, y: next_coord.1 });
                }
            }
            Direction::East => {
                let next_coord = (x, y+1);
                if game_state.is_out_bounds(next_coord) {
                    return None;
                }
                if game_state.is_open(next_coord) {
                    return Some(GuardPos { dir: Direction::East, x: next_coord.0, y: next_coord.1 });
                }
            }
            Direction::West => {
                if y == 0 {
                    return None
                }
                let next_coord = (x, y-1);
                if game_state.is_out_bounds(next_coord) {
                    return None;
                }
                if game_state.is_open(next_coord) {
                    return Some(GuardPos { dir: Direction::West, x: next_coord.0, y: next_coord.1 });
                }
            }
        }
    }

    return None;
}

fn calc_order(direction: Direction) -> Vec<Direction> {
    match direction {
        Direction::North => vec![Direction::North, Direction::East, Direction::South, Direction::West],
        Direction::East  => vec![Direction::East, Direction::South, Direction::West, Direction::North],
        Direction::South => vec![Direction::South, Direction::West, Direction::North, Direction::East],
        Direction::West  => vec![Direction::West, Direction::North, Direction::East, Direction::South],
    }
}

fn tick(game_state: &mut GameState) -> bool {
    let GuardPos { dir, x, y } = game_state.guard_pos;
    game_state.visited.insert((x, y));
    
    let order = calc_order(dir);
    let next = next_pos(game_state, &order, (x, y));
    
    if next.is_some() {
        game_state.guard_pos = next.unwrap();
    }

    return next.is_none();
}

fn will_loop(game_state: &mut GameState) -> bool {
    let mut is_done = false;
    let mut positions: HashSet<GuardPos> = HashSet::new();
    
    while !is_done {
        if !positions.insert(game_state.guard_pos.clone()) {
            return true;
        }
        is_done = tick(game_state);
    }

    return false;
}

pub fn day_6_1(file_path: &str) -> usize {
    let mut game: GameState = read_grid(file_path);

    let mut is_done = false;
    while !is_done {
        is_done = tick(&mut game);
    }

    return game.visited.len();
}

pub fn day_6_2(file_path: &str) -> usize {
    let mut game: GameState = read_grid(file_path);

    let mut is_done = false;
    let mut loop_positions: HashSet<(usize, usize)> = HashSet::new();
    while !is_done {
        // if we introduce a barrier on this iteration, will it loop?
        let mut alternative_game = game.clone();
        let GuardPos { dir, x, y } = alternative_game.guard_pos;
        let order = calc_order(dir);
        let next_barrier_pos = next_pos(&alternative_game, &order, (x, y));
        if next_barrier_pos.is_some() {
            let GuardPos { x, y, .. } = next_barrier_pos.unwrap();
            
            // barriers can't be in places we've already pathed through
            if !alternative_game.visited.contains(&(x, y)) {
                alternative_game.grid[x][y] = 'X';
                let is_loop = will_loop(&mut alternative_game);
                if is_loop {
                    loop_positions.insert((x, y));
                } 
            }
        }
        
        // the normal game
        is_done = tick(&mut game);
    }

    return loop_positions.len();
}

#[cfg(test)]
mod tests {
    use super::day_6_1;
    use super::day_6_2;

    #[test]
    fn day_6_1_sample() {
        let result = day_6_1("inputs/d6_1_sample.txt");
        assert_eq!(result, 41);
    }
    
    #[test]
    fn day_6_1_run() {
        let result = day_6_1("inputs/d6_1.txt");
        println!("{result}");
        assert_eq!(result, 5404);
    }
    
    #[test]
    fn day_6_2_sample() {
        let result = day_6_2("inputs/d6_1_sample.txt");
        assert_eq!(result, 6);
    }
    
    #[test]
    fn day_6_2_run() {
        let result = day_6_2("inputs/d6_1.txt");
        println!("{result}");
        assert_eq!(result, 1984);
    }
}
