use std::cmp::max;
use std::fs;
use regex::Regex;

#[derive(Debug)]
struct GameSet {
    reds: i32,
    blues: i32,
    greens: i32,
}

#[derive(Debug)]
struct Game {
    id: i32,
    sets: Vec<GameSet>,
}

impl Game {
    fn is_possible(&self, reds: i32, blues: i32, greens: i32) -> Option<i32> {
        for game_set in &self.sets {
            if game_set.reds > reds {
                return None;
            }
            if game_set.blues > blues {
                return None;
            }
            if game_set.greens > greens {
                return None;
            }
        }
        return Some(self.id);
    }

    fn min_power(&self) -> i32 {
        let mut min_red = 0;
        let mut min_blue = 0;
        let mut min_green = 0;

        for game_set in &self.sets {
            min_red = max(min_red, game_set.reds);
            min_blue = max(min_blue, game_set.blues);
            min_green = max(min_green, game_set.greens);
        }

        return min_red * min_blue * min_green;
    }
}

fn main() {
    let file_path = "./input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("unable to read the file");

    let games = parse_to_games(contents);
    p1(&games);
    p2(&games);
}

fn parse_to_games(contents: String) -> Vec<Game> {
    let mut result = Vec::new();
    let r = Regex::new(r"(: |; )").unwrap();

    contents.lines().for_each(|line| {
        let mut parts: Vec<&str> = r.split(line).collect();

        let id = parts.remove(0)
            .strip_prefix("Game ").unwrap()
            .parse::<i32>().unwrap();

        let mut sets = Vec::new();
        parts.iter().for_each(|s| {
            let mut game_set = GameSet {
                reds: 0,
                blues: 0,
                greens: 0,
            };

            s.split(", ").for_each(|t| {
                if t.ends_with(" blue") {
                    let c = t.strip_suffix(" blue").unwrap().parse::<i32>().unwrap();
                    game_set.blues = c;
                }

                if t.ends_with(" red") {
                    let c = t.strip_suffix(" red").unwrap().parse::<i32>().unwrap();
                    game_set.reds= c;
                }

                if t.ends_with(" green") {
                    let c = t.strip_suffix(" green").unwrap().parse::<i32>().unwrap();
                    game_set.greens = c;
                }
            });
            sets.push(game_set);
        });

        let game = Game {
            id: id,
            sets: sets,
        };

        result.push(game);
    });

    return result;
}

fn p1(games: &Vec<Game>) {
    let possible: Vec<i32> = games.iter().filter_map(|g| g.is_possible(12, 14, 13)).collect();
    let total: i32 = possible.iter().sum();
    println!("P1: {}", total);
}

fn p2(games: &Vec<Game>) {
    let total: i32 = games.iter().map(|g| {
        let p = g.min_power();
        // println!("{:?} => {}", g, p);
        return p;
    }).sum();
    println!("P2: {}", total);
}
