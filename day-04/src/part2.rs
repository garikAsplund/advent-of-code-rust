use std::collections::HashMap;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    // parse numbers left of | into vec
    // parse numbers right of | into vec
    // foreach number in right vec see if it's in left vec
    // if it is, add to count by 1
    // copy the next count cards and repeat
    // add total to sum
    // return sum of cards

    let mut row = 0;
    let mut cards: HashMap<u32, u32> = (1..=204).map(|i| (i, 1)).collect();
    let mut total = 0;

    let output = input
        .lines()
        .map(|line| {
            row += 1;
            let mut sum = 0;

            let numbers_str = line
                .split("|")
                .nth(0)
                .expect("numbers")
                .split(":")
                .nth(1)
                .expect("Should be numbers");

            let my_str = line.split("|").nth(1).expect("numbers");

            let winning_numbers: Vec<u32> = numbers_str
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect();

            let my_numbers: Vec<u32> = my_str
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect();

            for number in &my_numbers {
                if winning_numbers.contains(&number) {
                    sum += 1;
                }
            }

            let current_value = *cards.get(&row).unwrap() as u32;

            for n in 1..=sum {
                cards
                    .entry(row + n)
                    .and_modify(|count| *count += current_value)
                    .or_insert(1);
            }

            // println!("winning numbers: {:?}", winning_numbers);
            // println!("my numbers: {:?}", my_numbers);
            println!("{:?}", sum);
            println!("{:?}", cards);

            if let Some(&value) = cards.get(&row) {
                if sum != 0 {
                    2
                } else {
                    1
                }
            } else {
                0
            }
        })
        .sum::<u32>();

    println!("{:?}", output);

    for (_, val) in cards.iter() {
        println!("{}", val);
        total += val;
    }

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("30", process(input)?);
        Ok(())
    }
}
