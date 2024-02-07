use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    // parse numbers left of | into vec
    // parse numbers right of | into vec
    // foreach number in right vec see if it's in left vec
    // if it is, start count at 1
    // if another is, multiply count by 2
    // add total to sum
    // return sum
    // todo!("day XX - part 1");

    let output = input
        .lines()
        .map(|line| {
            let mut sum = 0;
            let mut is_first = true;

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
                if winning_numbers.contains(&number) && is_first {
                    sum += 1;
                    is_first = false;
                } else if winning_numbers.contains(&number) {
                    sum *= 2;
                }
            }

            // println!("winning numbers: {:?}", winning_numbers);
            // println!("my numbers: {:?}", my_numbers);
            // println!("{:?}", sum);

            sum
        })
        .sum::<u32>();

    // println!("{:?}", output);

    Ok(output.to_string())
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
        assert_eq!("13", process(input)?);
        Ok(())
    }
}
