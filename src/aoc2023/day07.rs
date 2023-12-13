use std::cmp::Ordering;
use std::collections::HashMap;
use std::marker::PhantomData;

const INPUT: &str = include_str!("inputs/day07.txt");

pub fn run() {
    println!("{}", part_a(INPUT));
    println!("{}", part_b(INPUT));
}

fn part_a(input: &str) -> String {
    let mut input: Vec<(Hand<StandardDeck>, usize)> = input
        .lines()
        .map(|line| {
            let x = line.split_at(5);
            (
                Hand::<StandardDeck>::try_from(x.0).unwrap(),
                x.1[1..].parse().unwrap(),
            )
        })
        .collect();

    input.sort_unstable_by_key(|(h, _)| *h);

    input
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (_, x))| acc + (i + 1) * *x)
        .to_string()
}

fn part_b(input: &str) -> String {
    let mut input: Vec<(Hand<JokerDeck>, usize)> = input
        .lines()
        .map(|line| {
            let x = line.split_at(5);
            (
                Hand::<JokerDeck>::try_from(x.0).unwrap(),
                x.1[1..].parse().unwrap(),
            )
        })
        .collect();

    input.sort_unstable_by_key(|(h, _)| *h);

    input
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (_, x))| acc + (i + 1) * *x)
        .to_string()
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct StandardDeck;
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct JokerDeck;
trait DeckType {
    fn deck() -> [char; 13]
    where
        Self: Sized;
}

impl DeckType for StandardDeck {
    fn deck() -> [char; 13] {
        [
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
        ]
    }
}

impl DeckType for JokerDeck {
    fn deck() -> [char; 13] {
        [
            'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Card<DeckType: ?Sized> {
    card_type: char,
    _d: PhantomData<DeckType>,
}

impl<T: DeckType + std::cmp::Eq> PartialOrd for Card<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: DeckType + std::cmp::Eq> Ord for Card<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        T::deck()
            .iter()
            .position(|c| c == &self.card_type)
            .cmp(&T::deck().iter().position(|c| c == &other.card_type))
    }
}

impl<T: DeckType> TryFrom<char> for Card<T> {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        T::deck()
            .contains(&value)
            .then_some(Self {
                card_type: value,
                _d: PhantomData,
            })
            .ok_or("Card creation failed, maybe wrong char?")
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Hand<DeckType> {
    cards: [Card<DeckType>; 5],
}

impl Ord for Hand<StandardDeck> {
    fn cmp(&self, other: &Self) -> Ordering {
        let t1 = self.occurrences();
        let t2 = other.occurrences();

        if t1.0 == t2.0 {
            if t1.1 == t2.1 {
                self.cards
                    .iter()
                    .zip(other.cards.iter())
                    .map(|(a, b)| a.cmp(b))
                    .find(|x| *x != Ordering::Equal)
                    .expect("Equality between the two hands!")
            } else {
                t1.1.cmp(&t2.1)
            }
        } else {
            t1.0.cmp(&t2.0)
        }
    }
}

impl PartialOrd for Hand<StandardDeck> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand<JokerDeck> {
    fn cmp(&self, other: &Self) -> Ordering {
        let t1 = self.occurrences();
        let t2 = other.occurrences();

        if t1.0 == t2.0 {
            if t1.1 == t2.1 {
                self.cards
                    .iter()
                    .zip(other.cards.iter())
                    .map(|(a, b)| a.cmp(b))
                    .find(|x| *x != Ordering::Equal)
                    .expect("Equality between the two hands!")
            } else {
                t1.1.cmp(&t2.1)
            }
        } else {
            t1.0.cmp(&t2.0)
        }
    }
}

impl PartialOrd for Hand<JokerDeck> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: DeckType> TryFrom<&str> for Hand<T> {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // assert_eq!(value.len(), 5);
        value
            .chars()
            .flat_map(Card::try_from)
            .collect::<Vec<_>>()
            .try_into()
            .map_or(Err("Conversion failed, maybe not five cards?"), |x| {
                Ok(Self { cards: x })
            })
    }
}

impl Hand<StandardDeck> {
    fn occurrences(&self) -> (usize, usize) {
        let t: HashMap<_, _> = self.cards.iter().fold(HashMap::new(), |mut acc, c| {
            acc.entry(c).and_modify(|x| *x += 1).or_insert(1);
            acc
        });

        let mut t: Vec<usize> = t.into_values().collect();

        t.sort_unstable();

        match (t.pop(), t.pop()) {
            (Some(5), None) => (5, 0),
            (Some(x), Some(y)) => (x, y),
            _ => unreachable!(),
        }
    }
}

impl Hand<JokerDeck> {
    fn occurrences(&self) -> (usize, usize) {
        let mut t: HashMap<_, _> = self.cards.iter().fold(HashMap::new(), |mut acc, c| {
            acc.entry(c).and_modify(|x| *x += 1).or_insert(1);
            acc
        });

        let j = t
            .remove(&Card {
                card_type: 'J',
                _d: PhantomData,
            })
            .unwrap_or(0);

        let mut t: Vec<usize> = t.into_values().collect();

        if j == 5 {
            return (5, 0);
        }

        t.sort_unstable();

        match (t.pop(), t.pop()) {
            (Some(_), None) => (5, 0),
            (Some(x), Some(y)) => (x + j, y),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const _TEST: &str = include_str!("tests/day07.txt");
    const _TESTRESULT_A: &str = "6440";
    const _TESTRESULT_B: &str = "5905";

    #[test]
    fn test_a() {
        assert_eq!(part_a(_TEST), _TESTRESULT_A);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(_TEST), _TESTRESULT_B);
    }
}
