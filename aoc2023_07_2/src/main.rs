use std::cmp::max;
use std::io::stdin;

use anyhow::{Context, Error};

struct Hand {
    cards: [u8; 5],
    bid: u64,
    score: u8,
}

fn best_wild_combo(cards: &[u8]) -> u8 {
    let mut num_wildcards = 0;
    for card in cards {
        if *card == b'J' {
            num_wildcards += 1;
        }
    }

    // check what is available without using wildcards
    let combo = calculate_combo(cards);

    match num_wildcards {
        5 => 6,
        4 => 6,
        3 => {
            match combo {
                1 => 6, // 2 of a kind => 5 of a kind
                0 => 5, // high card => 4 of a kind
                _ => panic!("impossible 3 wildcard"),
            }
        }
        2 => {
            match combo {
                3 => 6, // 3 of a kind => 5 of a kind
                1 => 5, // one pair => 4 of a kind
                0 => 2, // high card => two pair
                _ => panic!("impossible 2 wildcard"),
            }
        }
        1 => {
            match combo {
                5 => 6, // 4 of a kind => 5 of a kind
                3 => 5, // 3 of a kind => 4 of a kind
                2 => 4, // two pair => full house
                1 => 3, // one pair => 3 of a kind
                0 => 1, // high card => one pair
                _ => panic!("impossible 1 wildcard"),
            }
        }
        0 => combo,
        _ => panic!("impossible num_wildcards"),
    }
}

fn convert_card(text: u8) -> u8 {
    match text {
        b'J' => 1,
        b'2' => 2,
        b'3' => 3,
        b'4' => 4,
        b'5' => 5,
        b'6' => 6,
        b'7' => 7,
        b'8' => 8,
        b'9' => 9,
        b'T' => 10,
        b'Q' => 12,
        b'K' => 13,
        b'A' => 14,
        _ => panic!("invalid card"),
    }
}

fn calculate_combo(cards: &[u8]) -> u8 {
    let mut counters = [0u8; 256];
    let mut five_card = 0u8;
    let mut four_card = 0u8;
    let mut three_card = 0u8;
    let mut two_card_1 = 0u8;
    let mut two_card_2 = 0u8;
    let mut combo: u8 = 0;

    for card in cards {
        let card = *card;

        if card == b'J' {
            continue;
        }

        counters[card as usize] += 1;
        let counter = counters[card as usize];
        if counter == 2 {
            if two_card_1 != 0 {
                two_card_2 = card;
            } else {
                two_card_1 = card;
            }

            if three_card != 0 {
                combo = max(combo, 4);
            } else if two_card_2 != 0 {
                combo = max(combo, 2);
            } else {
                combo = max(combo, 1);
            }
        } else if counter == 3 {
            if two_card_1 == card {
                two_card_1 = two_card_2;
                two_card_2 = 0;
            } else if two_card_2 == card {
                two_card_2 = 0;
            }
            three_card = card;
            if two_card_1 != 0 {
                combo = max(combo, 4);
            } else {
                combo = max(combo, 3);
            }
        } else if counter == 4 {
            three_card = 0;
            four_card = card;
            combo = max(combo, 5);
        } else if counter == 5 {
            four_card = 0;
            five_card = card;
            combo = max(combo, 6);
        }
    }

    combo
}

fn main() -> Result<(), Error> {
    let mut hands = Vec::<Hand>::new();

    for line in stdin().lines() {
        let line = line?;
        let (cards, bid_str) = line.split_once(" ").context("missing part of line")?;

        let bid: u64 = bid_str.parse()?;

        let combo = best_wild_combo(cards.as_bytes());

        eprintln!("{} {} {}", cards, bid, combo);

        let hand = Hand {
            bid,
            cards: [
                convert_card(cards.as_bytes()[0]),
                convert_card(cards.as_bytes()[1]),
                convert_card(cards.as_bytes()[2]),
                convert_card(cards.as_bytes()[3]),
                convert_card(cards.as_bytes()[4]),
            ],
            score: combo,
        };
        hands.push(hand);
    }

    eprintln!("");

    hands.sort_by_key(|hand| (hand.score, hand.cards));

    let mut winnings = 0;

    let mut rank = 1;
    for hand in hands.iter() {
        let winning = hand.bid * rank;
        eprintln!(
            "{} {} {:?} {} {}",
            hand.bid, hand.score, hand.cards, rank, winning
        );
        winnings += winning;
        rank += 1;
    }

    println!("{}", winnings);

    Ok(())
}
