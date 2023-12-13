use std::collections::HashMap;

fn main() {
    let data = include_str!("../../../input/day2.txt");

    let games = Games::parse(data);

    let mut bag = HashMap::new();
    bag.insert("red".to_string(), 12);
    bag.insert("green".to_string(), 13);
    bag.insert("blue".to_string(), 14);

    let possible_games = games
        .possible(bag)
        .into_iter()
        .enumerate()
        .map(|(i, v)| (i + 1, v));

    let possible_games_ids_sum: usize = possible_games
        .filter(|(_, v)| *v == true)
        .map(|(i, _)| i)
        .sum();

    println!("Sum of ids of possible games: {possible_games_ids_sum}");

    let min_number_of_cubes_for_possible_games = games.min_number_of_cubes_for_possible_games();

    let min_bag_powers_sum: u32 = min_number_of_cubes_for_possible_games
        .iter()
        .map(|bag| bag.get("red").unwrap() * bag.get("green").unwrap() * bag.get("blue").unwrap())
        .sum();

    println!("Sum of powers of minimal bags: {min_bag_powers_sum}");
}

type Color = String;
type Game = Vec<HashMap<Color, u32>>;

struct Games(Vec<Game>);

impl Games {
    fn min_number_of_cubes_for_possible_games(&self) -> Vec<HashMap<Color, u32>> {
        self.0
            .iter()
            .map(|game| {
                game.iter()
                    .fold(HashMap::new(), |mut min_possible_bag, round| {
                        round.iter().for_each(|(color, &number)| {
                            let in_bag = min_possible_bag.entry(color.to_string()).or_insert(0);
                            if *in_bag < number {
                                *in_bag = number;
                            }
                        });
                        min_possible_bag
                    })
            })
            .collect()
    }

    fn possible(&self, bag: HashMap<Color, u32>) -> Vec<bool> {
        self.0
            .iter()
            .map(|game| {
                // the game is impossible when any of the game's rounds show more cubes of a color than there are cubes of that color in a bag
                game.iter().all(|round| {
                    round.iter().all(|(color, number_in_round)| {
                        *number_in_round <= *bag.get(color).unwrap_or(&0)
                    })
                })
            })
            .collect()
    }

    fn parse(input: &str) -> Self {
        let games = input
            .lines()
            .map(|line| {
                // cube data starts after first `:` plus a space
                let (_, rounds_data) = line.split_once(": ").unwrap();

                let rounds = {
                    rounds_data
                        .split("; ")
                        .map(|round| {
                            round
                                .split(", ")
                                .map(|cubes| {
                                    let (number, color) = cubes.split_once(' ').unwrap();
                                    (color.to_string(), number.parse::<u32>().unwrap())
                                })
                                .collect::<HashMap<Color, u32>>()
                        })
                        .collect::<Vec<_>>()
                };

                rounds
            })
            .collect::<Vec<_>>();

        Self(games)
    }
}
