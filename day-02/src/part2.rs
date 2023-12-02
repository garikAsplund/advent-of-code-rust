use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let output = input
        .lines()
        .map(|line| {
            let game = line.split(": ").nth(0).expect("E").split(" ").nth(1).expect("Should be a number string").parse::<u32>().expect("should be a number");
            let line = line.split(": ").nth(1).expect("should be a line with just values and colors");
            let pulls: Vec<&str> = line.split("; ").collect();

            let mut sum = true;

            let mut red_min = 0;
            let mut green_min = 0;
            let mut blue_min = 0;

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
                if red > red_min && red > 0 {
                    red_min = red;
                }
                if green > green_min && green > 0 {
                    green_min = green;
                }
                if blue > blue_min && blue > 0 {
                    blue_min = blue;
                }  
            }
            
            // powers
            println!("red: {}, green: {}, blue: {}", red_min, green_min, blue_min);
            red_min * green_min * blue_min

        }).sum::<u32>();

        println!("{:?}", output);

    Ok("ok".to_string())
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
        assert_eq!("2286", process(input)?);
        Ok(())
    }
}