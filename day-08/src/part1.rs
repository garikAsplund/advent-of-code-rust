use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<u32, AocError> {
    // todo!("day XX - part 1");

    let mut steps: u32 = 0;

    // follow l/r instructions

    // repeat until 'zzz' adding 1

    let directions = input.lines().nth(0).unwrap();
    // dbg!(directions);

    let nodes: Vec<(&str, Vec<&str>)> = input.lines().skip(2).map(|line| {
        let (node, children) = line.split_once(" = ").unwrap();
        let children = children
            .strip_prefix("(")
            .unwrap()
            .strip_suffix(")")
            .unwrap()
            .split(", ")
            .collect::<Vec<_>>();
        dbg!(&children);
        (node, children)
    }).collect();

    // dbg!(nodes);

    let mut current_node = "AAA";

    loop {
        for char in directions.chars() {
            match char {
                'L' => {
                    steps += 1;
                    current_node = move_node(current_node, 0, &nodes);
                    if current_node == "ZZZ" {
                        break;
                    }
                }
                'R' => {
                    steps += 1;
                    current_node = move_node(current_node, 1, &nodes);
                    if current_node == "ZZZ" {
                        break;
                    }
                }
                _ => {
                    panic!("Unknown direction: {}", char);
                }
            }
        }
        if current_node == "ZZZ" {
            break;
        }
    }
    

    Ok(steps)
}

fn move_node<'a> (start: &'a str, direction: u8, nodes:  &Vec<(&'a str, Vec<&'a str>)>) -> &'a str {    // if left, move to left child
    // if right, move to right child

    let result = nodes.iter().find(|(node, _)| node == &start).unwrap().1[direction as usize];
    println!("start: {}, direction: {}, result: {}", start, direction, result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        // todo!("haven't built test yet");
        let input = "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)";
        assert_eq!(2, process(input)?);
        Ok(())
    }
}
