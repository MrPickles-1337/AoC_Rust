use std::cmp::Ordering;

#[derive(PartialEq, Eq, PartialOrd, Clone, Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub bid: u32,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum Card {
    J(u8),   // 1
    Num(u8), // 2..9
    T(u8),   // 10
    Q(u8),   // 12
    K(u8),   // 13
    A(u8),   // 14
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Combination {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

fn check_combination(cards: Vec<u32>) -> Combination {
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
        let j_self_occurences = self
            .cards
            .iter()
            .filter(|c| matches!(c, Card::J(_)))
            .count();
        let j_other_occurences = other
            .cards
            .iter()
            .filter(|c| matches!(c, Card::J(_)))
            .count();
        let mut self_occurences: Vec<u32> = (1..15)
            .map(|value| {
                self.cards
                    .iter()
                    .filter(|card| {
                        value == card.get_value()
                            && if j_self_occurences == 0 {
                                true
                            } else {
                                !matches!(card, Card::J(_))
                            }
                    })
                    .count() as u32
            })
            .collect();
        let mut other_occurences: Vec<u32> = (1..15)
            .map(|value| {
                other
                    .cards
                    .iter()
                    .filter(|card| {
                        value == card.get_value()
                            && if j_other_occurences == 0 {
                                true
                            } else {
                                !matches!(card, Card::J(_))
                            }
                    })
                    .count() as u32
            })
            .collect();
        self_occurences.sort();
        other_occurences.sort();
        *self_occurences.last_mut().unwrap() += j_self_occurences as u32;
        *other_occurences.last_mut().unwrap() += j_other_occurences as u32;

        let self_combination = check_combination(self_occurences);
        let other_combination = check_combination(other_occurences);
        match other_combination.cmp(&self_combination) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            Ordering::Equal => {
                for (self_card, other_card) in std::iter::zip(&self.cards, &other.cards) {
                    if self_card == other_card {
                        continue;
                    }
                    return other_card.cmp(&self_card);
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
            'T' => Card::T(10),
            'J' => Card::J(1),
            _ => unreachable!(),
        }
    }
}

#[aoc_generator(day7, part2)]
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

#[aoc(day7, part2)]
pub fn part2(input: &[Hand]) -> u32 {
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
    fn part2_test() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(5905, part2(&input_generator(input)));
    }
}
