use itertools::Itertools;

const PART1: [char; 13] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
const PART2: [char; 13] = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
	FullHouse,
	ThreeOfAKind,
	TwoPair,
	OnePair,
	HighCard,
}
#[derive(Debug)]
struct Hand {
    comb: Vec<char>,
    hand_type: HandType,
    bid: u32,
    rank: u32,
}
#[derive(Debug)]
struct Hands (Vec<Hand>);

impl Hands {
    fn is_stronger(&self, idx_self: usize, idx_other: usize) -> bool {
        if self.0[idx_self].hand_type < self.0[idx_other].hand_type {
            return true;
        } else if self.0[idx_self].hand_type > self.0[idx_other].hand_type {
            return false;
        } else {
            for idx in 0..5 {
                let self_pos = PART2.iter().position(|&c| c == self.0[idx_self].comb[idx]).unwrap();
                let other_pos = PART2.iter().position(|&c| c == self.0[idx_other].comb[idx]).unwrap();

                if self_pos < other_pos {
                    return true;
                } else if self_pos > other_pos {
                    return false;
                }
            }
        }
        false
    }
    fn sort_ranks(&mut self) {
        let len = self.0.len();
        for idx1 in 0..len {
            for idx2 in 0..len {
                if idx1 == idx2 {continue;}
                if self.is_stronger(idx1, idx2) {
                    self.0[idx1].rank += 1
                } else {}
            }
        }
    }
    fn get_total_winnings(&self) -> u32 {
        self.0.iter().map(|hand| {
            hand.bid * hand.rank
        }).sum::<u32>()
    }
}
fn define_type_part1(comb: Vec<char>) -> HandType {
    let unique: Vec<char> = comb.clone().into_iter().unique().collect();
    let duplicates: Vec<char> = comb.clone().into_iter().duplicates().collect();
    if unique.len() == 1 {
        HandType::FiveOfAKind
    } else if duplicates.len() == 1 && unique.len() == 2 {
        HandType::FourOfAKind
    } else if unique.len() == 2 {
        HandType::FullHouse
    } else if duplicates.len() == 1 && unique.len() == 3 {
        HandType::ThreeOfAKind
    } else if unique.len() == 3 {
        HandType::TwoPair
    } else if unique.len() == 4 {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}
fn define_type_part2(comb: Vec<char>) -> HandType {
    let jokers = comb.iter().filter(|&&c| c == 'J').count();
    let unique: Vec<char> = comb.clone().into_iter().unique().collect();
    let duplicates: Vec<char> = comb.clone().into_iter().duplicates().collect();
    match jokers {
        1 => {
            if unique.len() == 2 {
                HandType::FiveOfAKind
            } else if unique.len() == 3 && duplicates.len() == 2 {
                HandType::FullHouse
            } else if unique.len() == 3 && duplicates.len() == 1 {
                HandType::FourOfAKind
            } else if unique.len() == 4 {
                HandType::ThreeOfAKind
            } else {
                HandType::OnePair
            }
        },
        2 => {
            if unique.len() == 2 {
                HandType::FiveOfAKind
            } else if unique.len() == 3 {
                HandType::FourOfAKind
            } else {
                HandType::ThreeOfAKind
            }
        },
        3 => {
            if unique.len() == 2 {
                HandType::FiveOfAKind
            } else {
                HandType::FourOfAKind
            }
        },
        4 => HandType::FiveOfAKind,
        5 => HandType::FiveOfAKind,
        _ => {
            if unique.len() == 1 {
                HandType::FiveOfAKind
            } else if duplicates.len() == 1 && unique.len() == 2 {
                HandType::FourOfAKind
            } else if unique.len() == 2 {
                HandType::FullHouse
            } else if duplicates.len() == 1 && unique.len() == 3 {
                HandType::ThreeOfAKind
            } else if unique.len() == 3 {
                HandType::TwoPair
            } else if unique.len() == 4 {
                HandType::OnePair
            } else {
                HandType::HighCard
            }
        },
    }

}
fn part1(input: &str) -> u32 {
    let hands: Vec<Hand> = input.lines().map(|line| {
        let (card_type, bid) = line.split_once(" ").unwrap();
        let bid = bid.parse::<u32>().unwrap();
        let card_type: Vec<char> = card_type.chars().collect();
        Hand {
            comb: card_type.clone(),
            hand_type: define_type_part1(card_type),
            bid: bid,
            rank: 1,
        }
    }).collect();
    let mut all_hands = Hands(hands);
    all_hands.sort_ranks();
    all_hands.get_total_winnings()
}

fn part2(input: &str) -> u32 {
    let hands: Vec<Hand> = input.lines().map(|line| {
        let (card_type, bid) = line.split_once(" ").unwrap();
        let bid = bid.parse::<u32>().unwrap();
        let card_type: Vec<char> = card_type.chars().collect();
        Hand {
            comb: card_type.clone(),
            hand_type: define_type_part2(card_type),
            bid: bid,
            rank: 1,
        }
    }).collect();
    let mut all_hands = Hands(hands);
    all_hands.sort_ranks();
    all_hands.get_total_winnings()
}

fn main() {
    let input = include_str!("input7.txt");
    println!("{}, {}", part1(input), part2(input));
}