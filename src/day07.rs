use std::cmp::Ordering;

#[derive(PartialEq, Eq, PartialOrd, Clone, Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub bid: u32,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum Card {
    A(u8),   // 14
    K(u8),   // 13
    Q(u8),   // 12
    J(u8),   // 11
    T(u8),   // 10
    Num(u8), // 2..9
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Combination {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn check_combination(cards: &mut Vec<u32>) -> Combination {
    cards.sort();
    let most = *cards.last().unwrap();
    if most == 5 {
        return Combination::FiveOfAKind;
    }
    if most == 4 {
        return Combination::FourOfAKind;
    }
    if most == 3 {
        if *cards.get(cards.len() - 2).unwrap() == 2 {
            return Combination::FullHouse;
        }
        return Combination::ThreeOfAKind;
    }
    if most == 2 {
        if *cards.get(cards.len() - 2).unwrap() == 2 {
            return Combination::TwoPair;
        }
        return Combination::OnePair;
    }
    Combination::HighCard
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut self_occurences: Vec<u32> = (2..14)
            .map(|value| {
                self.cards
                    .iter()
                    .filter(|card| {
                        let c_value = card.get_value();
                        value == c_value
                    })
                    .count() as u32
            })
            .collect();
        let mut other_occurences: Vec<u32> = (2..14)
            .map(|value| {
                other
                    .cards
                    .iter()
                    .filter(|card| {
                        let c_value = card.get_value();
                        value == c_value
                    })
                    .count() as u32
            })
            .collect();
        let self_combination = check_combination(&mut self_occurences);
        let other_combination = check_combination(&mut other_occurences);
        match self_combination.cmp(&other_combination) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            Ordering::Equal => {
                for (self_card, other_card) in std::iter::zip(&self.cards, &other.cards) {
                    if self_card == other_card {
                        continue;
                    }
                    return self_card.cmp(&other_card);
                }
            }
        }
        unreachable!()
    }
}

impl Card {
    pub fn get_value(&self) -> u8 {
        match self {
            Card::A(n) => *n,
            Card::K(n) => *n,
            Card::Q(n) => *n,
            Card::J(n) => *n,
            Card::T(n) => *n,
            Card::Num(n) => *n,
        }
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        if let Some(n) = value.to_digit(10) {
            return Card::Num(n as u8);
        }
        match value {
            'A' => Card::A(14),
            'K' => Card::K(13),
            'Q' => Card::Q(12),
            'J' => Card::J(11),
            'T' => Card::T(10),
            _ => unreachable!(),
        }
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|l| {
            let mut spl = l.split(' ');
            let cards: Vec<Card> = spl.next().unwrap().chars().map(|c| Card::from(c)).collect();
            let bid: u32 = spl.next().unwrap().parse().unwrap();
            Hand { cards, bid }
        })
        .collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &[Hand]) -> u32 {
    let mut input = input.to_vec();
    input.sort_by(|a, b| b.cmp(a));
    input
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(6440, part1(&input_generator(input)));
    }
}
