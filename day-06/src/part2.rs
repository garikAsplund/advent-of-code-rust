use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<u128, AocError> {
    // todo!("day XX - part 1");
    let time: Vec<&str> = input.lines().nth(0).unwrap().strip_prefix("Time: ").unwrap().split_ascii_whitespace().collect();
    let distance: Vec<&str> = input.lines().nth(1).unwrap().strip_prefix("Distance: ").unwrap().split_ascii_whitespace().collect();

    let mut real_time = String::new();
    let mut real_distance = String::new();

    for num in time.iter() {
        real_time.push_str(num);
    }

    for distance in distance.iter() {
        real_distance.push_str(distance);
    }
    
    println!("{}", input);
    println!("time: {}", real_time);
    println!("distance: {:?}", distance);

    // calculate all distances

    let distance: u128 = real_distance.parse().unwrap();
    let time: u128 = real_time.parse().unwrap();
    let mut win_count: u128 = 0;

    for j in 1..=time {

        if j * (time - j) > distance {
            win_count += 1;
        }
    }

    // how many ways can you win for each race?

    // multiply all of them together

    Ok(win_count)
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
