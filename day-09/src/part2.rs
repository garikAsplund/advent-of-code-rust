use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<i128, AocError> {
    // todo!("day XX - part 1");
    
    let result = input
        .lines()
        .map(|line| {
            let mut numbers: Vec<i128> = line
                .split_whitespace()
                .map(|x| x.parse::<i128>().unwrap())
                .collect();

            let mut depth: Vec<i128> = Vec::new();
            
            numbers.reverse();

            loop {
                if numbers.iter().all(|&x| x == 0) {
                    break;
                }
                dbg!(&numbers);
                depth.push(*numbers.last().unwrap());

                let diff = diff(numbers.clone());
                numbers = diff;
            }

            // depth.reverse();

            println!("Depth: {:?}", depth);

            -depth.iter().fold(0, |acc, x| acc - x)
        }).sum();

    Ok(result)
}

fn diff (input: Vec<i128>) -> Vec<i128>{
    let mut result = Vec::new();

    for i in 0..input.len() - 1 {
        result.push(input[i + 1] - input[i]);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        // todo!("haven't built test yet");
        let input = "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";
        assert_eq!(2, process(input)?);
        Ok(())
    }
}
