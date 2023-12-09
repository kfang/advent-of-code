use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let file_path = "./input.txt";
    
    let contents = fs::read_to_string(file_path).expect("unable to read the file");

    let scratchcards = parse_contents(contents);
    let mut card_counts = init_card_counts(&scratchcards); 

    let mut p1 = 0; 
    for (card_no, winning_nos, owned_nos) in scratchcards {

        let mut match_count = 0;
        let mut score = 0;
        
        for owned in owned_nos {
            if winning_nos.contains(&owned) {
                match_count += 1;
                
                if score == 0 {
                    score = 1;
                } else {
                    score = score * 2;
                }
            }
        }

        for n in (card_no + 1)..(card_no + match_count + 1) {
            let card_count = card_counts.get(&card_no).unwrap();
            let next_n = card_counts.get(&n).unwrap() + card_count;
            card_counts.insert(n, next_n);
        }

        p1 += score;
    }
    
    println!("P1: {}", p1);
    
    let p2: i32 = card_counts.values().sum();
    println!("P2: {}", p2);
}

fn parse_contents(contents: String) -> Vec<(u16, HashSet<String>, Vec<String>)> {
    contents.lines().map(|line| {
        let parts: Vec<&str> = line.split(&[':', '|']).collect();
        
        let card_no = parts[0].strip_prefix("Card").unwrap().trim().parse::<u16>().unwrap();
        let winning_nos: HashSet<String> = HashSet::from_iter(parts[1].split_whitespace().map(str::to_string));
        let owned_nos: Vec<String> = parts[2].split_whitespace().map(str::to_string).collect();

        return (card_no, winning_nos, owned_nos)
    }).collect()
}

fn init_card_counts(scratchcards: &Vec<(u16, HashSet<String>, Vec<String>)>) -> HashMap<u16, i32> {
    let mut card_counts = HashMap::new();

    for (card_no, ..) in scratchcards {
        card_counts.insert(card_no.clone(), 1);
    }
    
    card_counts
}
