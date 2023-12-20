use std::cmp::Ordering;

const INPUT: &str = include_str!("input");

fn main() {
    part2();
}

#[derive(Debug)]
struct Line {
    hand: Hand,
    bid: u64,
}

fn part2() {
    let mut lines: Vec<Line> = INPUT
        .lines()
        .map(|line| {
            let (hand, bid) = line
                .split_once(' ')
                .expect("every line contains whitespace");

            Line {
                hand: parse_hand(hand),
                bid: u64::from_str_radix(bid, 10).expect("valid integer"),
            }
        })
        .collect();

    lines.sort_unstable_by(|line1, line2| line1.hand.cmp(&line2.hand));

    let solution: u64 = lines
        .into_iter()
        .enumerate()
        .map(|(index, line)| (index + 1) as u64 * line.bid)
        .sum();

    println!("part 2: {}", solution);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Hand {
    hand: [u8; 5],
}

impl Hand {
    fn ty(&self) -> u8 {
        let mut ranks: Vec<_> = (2..=14)
            .map(|card| {
                let rank = self
                    .hand
                    .into_iter()
                    .filter(|hand_card| *hand_card == card)
                    .count();
                (card, rank)
            })
            .collect();

        let js = ranks.swap_remove(usize::from(card_to_u8(b'J') - 2));
        assert_eq!(js.0, card_to_u8(b'J'));
        let num_js = js.1;

        ranks.sort_by(|(card1, rank1), (card2, rank2)| {
            rank1.cmp(rank2).then(card1.cmp(card2)).reverse()
        });

        let max_count = ranks[0].1 + num_js;
        let max_count_2 = ranks[1].1;

        match (max_count, max_count_2) {
            (5, _) => FIVE_OF_A_KIND,
            (4, _) => FOUR_OF_A_KIND,
            (3, 2) => FULL_HOUSE,
            (3, _) => THREE_OF_A_KIND,
            (2, 2) => TWO_PAIR,
            (2, _) => ONE_PAIR,
            _ => HIGH_CARD,
        }
    }
}

const FIVE_OF_A_KIND: u8 = 6;
const FOUR_OF_A_KIND: u8 = 5;
const FULL_HOUSE: u8 = 4;
const THREE_OF_A_KIND: u8 = 3;
const TWO_PAIR: u8 = 2;
const ONE_PAIR: u8 = 1;
const HIGH_CARD: u8 = 0;

fn hand_values(hand: &Hand) -> [u8; 5] {
    hand.hand.map(|card| if card == 11 { 1 } else { card })
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ty().cmp(&other.ty()).then_with(|| {
            hand_values(&self).cmp(&hand_values(&other))
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_hand(hand: &str) -> Hand {
    Hand {
        hand: hand
            .bytes()
            .map(card_to_u8)
            .collect::<Vec<_>>()
            .try_into()
            .expect("every hand has length 5"),
    }
}

fn card_to_u8(card: u8) -> u8 {
    match card {
        b'A' => 14,
        b'K' => 13,
        b'Q' => 12,
        b'J' => 11,
        b'T' => 10,
        c if c.is_ascii_digit() => (c as char).to_digit(10).expect("is a digit") as u8,
        _ => panic!(),
    }
}
