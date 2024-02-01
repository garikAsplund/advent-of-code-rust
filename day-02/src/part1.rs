use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    // The Elf would first like to know which games would have been possible 
    // if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?

    // parse each game by ; and see if red > 12, green > 13, blue > 14
    // if not, then add to total
    // return total

    // "NUMBER color"

    let output = input
        .lines()
        .map(|line| {
            let game = line.split(": ").nth(0).expect("E").split(" ").nth(1).expect("Should be a number string").parse::<u32>().expect("should be a number");
            let line = line.split(": ").nth(1).expect("should be a line with just values and colors");
            let pulls: Vec<&str> = line.split("; ").collect();

            let mut sum = true;

            for pull in pulls {
                let pull = pull.split(", ").collect::<Vec<&str>>();
                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;
                for color in pull {
                    let color = color.split(" ").collect::<Vec<&str>>();
                    let number = color[0].parse::<u32>().expect("should be a number");
                    let color = color[1];
                    match color {
                        "red" => red += number,
                        "green" => green += number,
                        "blue" => blue += number,
                        _ => panic!("should be a valid color"),
                    }
                }
                if red > 12 || green > 13 || blue > 14 {
                    // println!("red: {}, green: {}, blue: {}", red, green, blue);
                    sum = false;
                }
            }
            if sum {
                game
            } else {
                0
            }
            // println!("{:?}", pull);
        }).sum::<u32>();

        println!("{:?}", output);

    Ok(output.to_string())

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("8", process(input)?);
        Ok(())
    }
}
