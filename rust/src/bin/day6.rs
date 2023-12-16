fn main() {
    let data = include_str!("../../../input/day6.txt");
    let mut lines = data.lines();

    let times: Vec<u64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse().unwrap())
        .collect();

    let distances: Vec<u64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse().unwrap())
        .collect();

    let num_solutions: Vec<_> = times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| get_num_solutions(*t, *d))
        .collect();

    let margin: u64 = num_solutions.iter().product();

    println!("{margin}");

    let single_race_time: Vec<_> = times.iter().map(|t| t.to_string()).collect();
    let single_race_time: u64 = single_race_time.join("").parse().unwrap();

    let single_race_distance: Vec<_> = distances.iter().map(|t| t.to_string()).collect();
    let single_race_distance: u64 = single_race_distance.join("").parse().unwrap();

    let single_race_num_solutions = get_num_solutions(single_race_time, single_race_distance);
    println!("{single_race_num_solutions}");
}

fn get_num_solutions(time: u64, distance_to_beat: u64) -> u64 {
    let mut a = time / 2;
    let mut b = time - a;

    let mut num_solutions = if time % 2 == 0 { 1 } else { 2 };
    loop {
        a -= 1;
        b += 1;
        let distance = a * b;

        if distance <= distance_to_beat {
            break;
        }

        num_solutions += 2;
    }

    num_solutions
}
