use std::cmp::max;
use std::io::stdin;

use anyhow::{Context, Error};

struct Hand {
    cards: [u8; 5],
    bid: u64,
    score: u8,
}

fn convert_card(text: u8) -> u8 {
    match text {
        b'2' => 2,
        b'3' => 3,
        b'4' => 4,
        b'5' => 5,
        b'6' => 6,
        b'7' => 7,
        b'8' => 8,
        b'9' => 9,
        b'T' => 10,
        b'J' => 11,
        b'Q' => 12,
        b'K' => 13,
        b'A' => 14,
        _ => panic!("invalid card"),
    }
}

fn main() -> Result<(), Error> {
    let mut hands = Vec::<Hand>::new();

    for line in stdin().lines() {
        let line = line?;
        let (cards, bid_str) = line.split_once(" ").context("missing part of line")?;

        let bid: u64 = bid_str.parse()?;

        let mut counters = [0u8; 256];
        let mut first_card = 0u8;
        let mut five_card = 0u8;
        let mut four_card = 0u8;
        let mut three_card = 0u8;
        let mut two_card_1 = 0u8;
        let mut two_card_2 = 0u8;
        let mut combo: u8 = 0;

        for card in cards.as_bytes() {
            let card = *card;

            if first_card == 0 {
                first_card = card;
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

        eprintln!("{} {} {} {}", cards, bid, combo, convert_card(first_card));

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
