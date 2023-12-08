fn main() {
    let data = include_str!("../../../input/day3.txt");

    let lines = data.lines();

    let mut numbers: Vec<(&str, (u32, u32))> = Vec::new();
    let mut symbols: Vec<(char, (u32, u32))> = Vec::new();

    for (y, row) in lines.enumerate() {
        // numbers can only appear horizontally on the same line
        let mut current_num_start_idx: Option<usize> = None;

        for (x, c) in row.char_indices() {
            let c_is_number = c.is_numeric();

            match current_num_start_idx {
                Some(start) => {
                    if !c_is_number {
                        // parse and add new number
                        numbers.push((
                            &row[start..x],
                            (y.try_into().unwrap(), start.try_into().unwrap()),
                        ));
                        current_num_start_idx = None;
                    }
                }

                None => {
                    if c_is_number {
                        current_num_start_idx = Some(x);
                    }
                }
            }

            let c_is_symbol = !c_is_number && c != '.';
            if c_is_symbol {
                symbols.push((c, (y.try_into().unwrap(), x.try_into().unwrap())));
            }
        }

        // if we finished the line and we have started the number, add it before continuing to next line
        if let Some(start) = current_num_start_idx {
            numbers.push((
                &row[start..],
                (y.try_into().unwrap(), start.try_into().unwrap()),
            ))
        }
    }

    let numbers_adjecent_to_symbols = numbers
        .iter()
        .filter(|(num, (y, x))| {
            let top: i32 = *y as i32 - 1;
            let bottom: i32 = *y as i32 + 1;
            let left: i32 = *x as i32 - 1;
            let right: i32 = *x as i32 + num.len() as i32;

            symbols.iter().any(|(_, (s_y, s_x))| {
                point_is_adjecent(
                    (i32::try_from(*s_y).unwrap(), i32::try_from(*s_x).unwrap()),
                    (top, left),
                    (bottom, right),
                )
            })
        })
        .collect::<Vec<_>>();

    let numbers_adj_to_symbols_sum: u32 = numbers_adjecent_to_symbols
        .iter()
        .map(|(num, _)| num.parse::<u32>().unwrap())
        .sum();

    println!("{numbers_adj_to_symbols_sum}");

    let gears: Vec<((u32, u32), (u32, u32))> = symbols
        .iter()
        .filter(|(c, _)| *c == '*')
        .filter_map(|(_, (y, x))| {
            let adj_numbers: Vec<_> = numbers
                .iter()
                .filter(|(num, (n_y, n_x))| {
                    let top: i32 = *n_y as i32 - 1;
                    let bottom: i32 = *n_y as i32 + 1;
                    let left: i32 = *n_x as i32 - 1;
                    let right: i32 = *n_x as i32 + num.len() as i32;

                    point_is_adjecent(
                        (i32::try_from(*y).unwrap(), i32::try_from(*x).unwrap()),
                        (top, left),
                        (bottom, right),
                    )
                })
                .collect();

            if adj_numbers.len() == 2 {
                let (num1, _) = adj_numbers[0];
                let num1 = num1.parse::<u32>().unwrap();
                let (num2, _) = adj_numbers[1];
                let num2 = num2.parse::<u32>().unwrap();
                Some(((num1, num2), (*y, *x)))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let gear_ratios = gears.into_iter().map(|((n1, n2), _)| n1 * n2);
    let gear_ratio_sum: u32 = gear_ratios.sum();

    println!("{gear_ratio_sum}");
}

/// Checks if point is in the rectangle denoted by top_left and bottom_right points
fn point_is_adjecent(
    (y, x): (i32, i32),
    (top, left): (i32, i32),
    (bottom, right): (i32, i32),
) -> bool {
    (top..=bottom).contains(&y) && (left..=right).contains(&x)
}
