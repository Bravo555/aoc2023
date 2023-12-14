use std::str::FromStr;

struct Card {
    winning: Vec<u8>,
    have: Vec<u8>,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (_, numbers) = line.split_once(": ").unwrap();
        let (winning, have) = numbers.split_once(" | ").unwrap();

        let winning = winning
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let have = have
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        Ok(Self { winning, have })
    }
}

impl Card {
    fn score(&self) -> u32 {
        let matches = self.num_matches();
        if matches == 0 {
            0
        } else {
            2u32.pow(u32::from(matches) - 1)
        }
    }

    fn num_matches(&self) -> u8 {
        self.winning
            .iter()
            .filter(|w| self.have.contains(&w))
            .count()
            .try_into()
            .unwrap()
    }
}

fn main() {
    let data = include_str!("../../../input/day4.txt");

    let cards: Vec<Card> = data.lines().map(|l| l.parse().unwrap()).collect();

    let total_points: u32 = cards.iter().map(Card::score).sum();

    println!("{total_points}");

    // part 2
    let mut card_quantities: Vec<u64> = vec![1; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        let quantity = card_quantities[i];
        let num_next_cards = card.num_matches();
        let num_card_copies: u64 = quantity;

        card_quantities[(i + 1)..]
            .iter_mut()
            .take(num_next_cards.try_into().unwrap())
            .for_each(|q| *q += num_card_copies)
    }
    dbg!(&card_quantities);

    let total_cards: u64 = card_quantities.iter().sum();
    println!("{total_cards}");
}
