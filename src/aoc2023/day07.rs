use std::cmp::Ordering;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::AddAssign;

const INPUT: &str = include_str!("inputs/day07.txt");

pub fn run() {
    println!("{}", part_a(INPUT));
    println!("{}", part_b(INPUT));
}

fn part_a(input: &str) -> String {
    let mut hands: Vec<(Hand<StandardDeck>, usize)> = read_input(input);

    hands.sort_unstable_by_key(|(h, _)| h.clone());

    count_result(&hands)
}

fn part_b(input: &str) -> String {
    let mut hands: Vec<(Hand<JokerDeck>, usize)> = read_input(input);

    hands.sort_unstable_by_key(|(h, _)| h.clone());

    count_result(&hands)
}

fn read_input<T: DeckType>(input: &str) -> Vec<(Hand<T>, usize)> {
    input
        .lines()
        .map(|line| line.split_at(5))
        .filter_map(|(a, b)| b[1..].parse().map_or(None, |s| Some((Hand::new(a), s))))
        .collect()
}

fn count_result<T: DeckType>(hands: &[(Hand<T>, usize)]) -> String {
    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &(_, x))| acc + (i + 1) * x)
        .to_string()
}

trait DeckType: Eq + Ord {
    fn card_value(x: char) -> Option<u8>;

    fn compare_cards(a: char, b: char) -> Ordering {
        Self::card_value(a).cmp(&Self::card_value(b))
    }

    fn hand_ordering(occ: &HashMap<&u8, u8>) -> [u8; 2];
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct StandardDeck;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct JokerDeck;

impl DeckType for StandardDeck {
    fn card_value(x: char) -> Option<u8> {
        [
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
        ]
        .iter()
        .position(|&t| t == x)
        .and_then(|x| u8::try_from(x).ok())
    }

    fn hand_ordering(occ: &HashMap<&u8, u8>) -> [u8; 2] {
        let mut t: Vec<_> = occ.values().collect();

        t.sort_unstable();

        match (t.pop(), t.pop()) {
            (Some(5), None) => [5, 0],
            (Some(&x), Some(&y)) => [x, y],
            _ => unreachable!(),
        }
    }
}

impl DeckType for JokerDeck {
    fn card_value(x: char) -> Option<u8> {
        [
            'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
        ]
        .iter()
        .position(|&t| t == x)
        .and_then(|x| u8::try_from(x).ok())
    }

    fn hand_ordering(occ: &HashMap<&u8, u8>) -> [u8; 2] {
        let mut occ = occ.clone();

        let j = occ.remove(&Self::card_value('J').unwrap()).unwrap_or(0);
        if j == 5 {
            return [5, 0];
        }

        let mut t: Vec<_> = occ.into_values().collect();

        t.sort_unstable();

        match (t.pop(), t.pop()) {
            (Some(_), None) => [5, 0],
            (Some(x), Some(y)) => [x + j, y],
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Hand<T: DeckType> {
    // cards: [u8; 5],
    values: [u8; 7],
    deck_type: PhantomData<T>,
}

impl<T: DeckType> Ord for Hand<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.values
            .iter()
            .zip(other.values)
            .map(|(a, b)| a.cmp(&b))
            .find(|&x| x != Ordering::Equal)
            .unwrap_or(Ordering::Equal)
    }
}

impl<T: DeckType> PartialOrd for Hand<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: DeckType> Hand<T> {
    fn new(s: &str) -> Self {
        assert_eq!(s.len(), 5);

        let t = s.chars().filter_map(T::card_value).collect::<Vec<u8>>();

        assert_eq!(t.len(), 5);

        Self {
            values: occurrences::<T>(&t)
                .into_iter()
                .chain(t)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            deck_type: PhantomData,
        }
    }
}

fn occurrences<T: DeckType>(cards: &[u8]) -> [u8; 2] {
    let t: HashMap<_, u8> = cards.iter().fold(HashMap::new(), |mut acc, c| {
        acc.entry(c).or_insert(0).add_assign(1);
        acc
    });

    T::hand_ordering(&t)
}

#[cfg(test)]
mod tests {
    use super::*;

    const _TEST: &str = include_str!("tests/day07.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(_TEST), "6440");
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(_TEST), "5905");
    }
}
