use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<u64, AocError> {
    // todo!("day XX - part 1");
    let time: Vec<&str> = input.lines().nth(0).unwrap().strip_prefix("Time: ").unwrap().split_ascii_whitespace().collect();
    let distance: Vec<&str> = input.lines().nth(1).unwrap().strip_prefix("Distance: ").unwrap().split_ascii_whitespace().collect();

    println!("{}", input);
    println!("time: {:?}", time);
    println!("distance: {:?}", distance);

    // calculate all distances

    let mut wins: Vec<u64> = Vec::new();

    for i in 0..distance.len() {
        let distance: u64 = distance[i].parse().unwrap();
        let time: u64 = time[i].parse().unwrap();
        let mut win_count = 0;

        for j in 1..=time {

            if j * (time - j) > distance {
                win_count += 1;
            }
        }

        wins.push(win_count);
    }

    // how many ways can you win for each race?
    println!("wins: {:?}", wins);

    // multiply all of them together

    Ok(wins.iter().fold(1, |acc, x| acc * x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
