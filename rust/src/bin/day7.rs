use std::{cmp::Ordering, collections::HashMap, fmt::Debug, str::FromStr};

const CARD_REVERSE_ORDER: &str = "AKQJT98765432";
const CARD_REVERSE_ORDER_JOKER_RULE: &str = "AKQT98765432J";

fn main() {
    // dbg!("99JAA".parse::<Hand>());
    // return;

    let data = include_str!("../../../input/day7.txt");
    let mut games: Vec<(Hand, u32)> = data
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(h, b)| (h.parse().unwrap(), b.parse().unwrap()))
        .collect();

    games.sort_by(|g1, g2| g1.0.cmp(&g2.0));
    dbg!(&games
        .iter()
        .filter(|g| g.0 .0.contains(&Card('J')))
        .map(|g| &g.0)
        .collect::<Vec<_>>());

    let games_ranked: Vec<_> = games.iter().zip(1u32..).collect();
    // dbg!(&games_ranked);

    let total_score: u64 = games_ranked
        .iter()
        .map(|((_, bid), rank)| u64::from(*bid) * u64::from(*rank))
        .sum();

    println!("{total_score}");
}

#[derive(Clone)]
struct Hand([Card; 5]);

impl Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = String::from_iter(self.0.map(|c| c.0));
        write!(f, "Hand(\"{s}\", kind={:?})", self.kind())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl Hand {
    fn kind(&self) -> HandType {
        let mut chars = self.0.into_iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

        // joker upgrade: turn every joker into a card we have the biggest
        // number of, if we dont have any other card, turn all jokers into aces
        let num_jokers = *chars.get(&Card('J')).unwrap_or(&0);
        chars.remove(&Card('J'));
        let most_frequent_card = chars
            .iter()
            .max_by(|c1, c2| c1.1.cmp(c2.1).then(c1.0.cmp(c2.0)))
            .map(|c| c.0)
            .unwrap_or(&Card('A'))
            .clone();

        *chars.entry(most_frequent_card).or_default() += num_jokers;

        let mut chars: Vec<(Card, usize)> = chars.into_iter().collect();
        // dbg!(&chars);

        chars.sort_unstable_by_key(|n| std::cmp::Reverse(n.1));

        let kind = match chars[..] {
            [(_, n), ..] if n == 5 => HandType::FiveOfAKind,
            [(_, n), ..] if n == 4 => HandType::FourOfAKind,
            [(_, n1), (_, n2)] if n1 == 3 && n2 == 2 => HandType::FullHouse,
            [(_, n), ..] if n == 3 => HandType::ThreeOfAKind,
            [(_, n1), (_, n2), ..] if n1 == 2 && n2 == 2 => HandType::TwoPair,
            [(_, n), ..] if n == 2 => HandType::OnePair,
            [(_, n), ..] if n == 1 => HandType::HighCard,

            _ => {
                unreachable!("unexpected pattern: {chars:?}");
            }
        };

        kind
    }

    fn cmp_value(&self, other: &Self) -> Ordering {
        let h1 = self.0;

        let h2 = other.0;

        h1.cmp(&h2)
    }

    fn _cmp(&self, other: &Self) -> Ordering {
        let type_order = self.kind().cmp(&other.kind());
        match type_order {
            Ordering::Less | Ordering::Greater => type_order,
            // default lexicographical comparison does what we want
            Ordering::Equal => self.cmp_value(other),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self._cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self._cmp(other)
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hand: [Card; 5] = s
            .chars()
            .map(|c| Card(c))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Ok(Self(hand))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Card(char);

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let pos = CARD_REVERSE_ORDER_JOKER_RULE.find(self.0).unwrap();
        let other_pos = CARD_REVERSE_ORDER_JOKER_RULE.find(other.0).unwrap();
        other_pos.partial_cmp(&pos)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
