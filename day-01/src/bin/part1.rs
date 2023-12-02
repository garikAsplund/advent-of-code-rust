use std::io;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

pub fn process(input: &str) -> Result<String, io::Error> {
    let output = input
        .lines()
        .map(|line| {
            let mut it = line.chars().filter_map(|character| {
                character.to_digit(10)
            });
            let first = it.next().expect("should be a number");
            match it.last() {
                Some(num) => format!("{first}{num}"),
                None => format!("{first}{first}"),
            }
            .parse::<u32>()
            .expect("should be a valid number")
        }).sum::<u32>();

    Ok(output.to_string())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), io::Error>{
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}