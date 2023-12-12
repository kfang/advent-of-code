use std::{fs, collections::HashMap, ops::{Mul, Add}};

#[derive(Debug)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: i32,
}

fn main() {
    let file_path = "./input.txt";
    let contents = fs::read_to_string(file_path).expect("unable to read file");
    let hands = parse_contents(&contents);

    let p1 = solve(&hands, false);
    println!("P1: {}", p1);
    
    let p2 = solve(&hands, true);
    println!("P2: {}", p2);
}

fn parse_contents(contents: &String) -> Vec<Hand> {
    let mut hands = Vec::new();
    for line in contents.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        let cards = parts[0].to_string();
        let bid = parts[1].parse::<i32>().expect("unable to parse bid");
        hands.push(Hand { cards, bid })
    }
    return hands;
}

fn solve(hands: &Vec<Hand>, wildcard: bool) -> i32 {
    let mut val_bid: Vec<(String, i32, String)> = Vec::new();

    for hand in hands {
        let mut card_ord = String::new();
        let mut card_map : HashMap<char, i8> = HashMap::new();

        for card in hand.cards.chars() {
            let curr = card_map.get(&card);
            let next = curr.map(|i| i + 1).unwrap_or(1);
            card_map.insert(card, next);

            let s = match card {
                '2' => "01",
                '3' => "02",
                '4' => "03",
                '5' => "04",
                '6' => "05",
                '7' => "06",
                '8' => "07",
                '9' => "08",
                'T' => "09",
                'J' => if wildcard { "00" } else { "10" },
                'Q' => "11",
                'K' => "12",
                'A' => "13",
                _ => ""
            };
            card_ord.push_str(s);
        }

        let wildcard_count = (if wildcard { card_map.remove(&'J') } else { None }).unwrap_or(0);
        let mut card_map_values: Vec<&i8> = card_map.values().collect();
        card_map_values.sort();

        let mut hand_type = HandType::HighCard;
        if card_map_values.contains(&&5) {
            hand_type = HandType::FiveKind;
        } else if card_map_values.contains(&&4) {
            hand_type = HandType::FourKind;
        } else if card_map_values.contains(&&3) {
            if card_map_values.contains(&&2) {
                hand_type = HandType::FullHouse;
            } else {
                hand_type = HandType::ThreeKind;
            }
        } else {
            let pair_count = card_map_values.iter().fold(0, |c, i| {
                if i.eq(&&2) { c + 1 } else { c }
            });

            if pair_count.eq(&1) {
                hand_type = HandType::OnePair;
            } else if pair_count.eq(&2) {
                hand_type = HandType::TwoPair;
            }
        }

        if wildcard_count.gt(&0) {
            hand_type = match hand_type {
                HandType::FiveKind  => HandType::FiveKind,
                
                HandType::FourKind if wildcard_count.eq(&1)  => HandType::FiveKind,
                HandType::FourKind => HandType::FourKind,
                
                HandType::FullHouse => HandType::FullHouse,
                
                HandType::ThreeKind if wildcard_count.eq(&2) => HandType::FiveKind,
                HandType::ThreeKind if wildcard_count.eq(&1) => HandType::FourKind,
                HandType::ThreeKind => HandType::ThreeKind,
                
                HandType::TwoPair if wildcard_count.eq(&1) => HandType::FullHouse,
                HandType::TwoPair => HandType::TwoPair,

                HandType::OnePair if wildcard_count.eq(&3) => HandType::FiveKind,
                HandType::OnePair if wildcard_count.eq(&2) => HandType::FourKind,
                HandType::OnePair if wildcard_count.eq(&1) => HandType::ThreeKind,
                HandType::OnePair => HandType::OnePair,

                HandType::HighCard if wildcard_count.eq(&5) => HandType::FiveKind,
                HandType::HighCard if wildcard_count.eq(&4) => HandType::FiveKind,
                HandType::HighCard if wildcard_count.eq(&3) => HandType::FourKind,
                HandType::HighCard if wildcard_count.eq(&2) => HandType::ThreeKind,
                HandType::HighCard if wildcard_count.eq(&1) => HandType::OnePair,
                HandType::HighCard => HandType::HighCard
            };
        }

        let ht = match hand_type {
            HandType::HighCard  => 10,
            HandType::OnePair   => 20,
            HandType::TwoPair   => 30,
            HandType::ThreeKind => 40,
            HandType::FullHouse => 50,
            HandType::FourKind  => 60,
            HandType::FiveKind  => 70,
        };

        let hand_val = format!("{}{}", ht, card_ord);
        val_bid.push((hand_val, hand.bid, hand.cards.clone()));
    }
    
    val_bid.sort_by_key(|h| h.0.clone());

    let mut score = 0; 
    for (i, (_val, bid, cards)) in val_bid.iter().enumerate() {
        let rank = i.add(1) as i32;
        score += rank.mul(bid);
    }

    return score;
}
