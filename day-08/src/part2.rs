// use crate::custom_error::AocError;

// #[tracing::instrument]
// pub fn process(
//     input: &str,
// ) -> miette::Result<u32, AocError> {
//     // todo!("day XX - part 1");

//     let mut steps: u32 = 0;

//     // follow l/r instructions

//     // repeat until 'zzz' adding 1

//     let directions = input.lines().nth(0).unwrap();
//     // dbg!(directions);

//     let nodes: Vec<(&str, Vec<&str>)> = input.lines().skip(2).map(|line| {
//         let (node, children) = line.split_once(" = ").unwrap();
//         let children = children
//             .strip_prefix("(")
//             .unwrap()
//             .strip_suffix(")")
//             .unwrap()
//             .split(", ")
//             .collect::<Vec<_>>();
//         // dbg!(&children);
//         (node, children)
//     }).collect();

//     // dbg!(nodes);

//     let mut current_node: Vec<&str> = nodes.iter().filter_map(|(node, _)| if node.ends_with("A") { Some(*node) } else { None }).collect();

//     // dbg!(current_node);
// loop {
//         for char in directions.chars() {
//             match char {
//                 'L' => {
//                     steps += 1;
//                     current_node = move_node(current_node, 0, &nodes);
//                 }
//                 'R' => {
//                     steps += 1;
//                     current_node = move_node(current_node, 1, &nodes);
//                 }
//                 _ => {
//                     panic!("Unknown direction: {}", char);
//                 }
//             }
//         }
//         if current_node.iter().all(|last: &&str| last.ends_with("Z")) {
//             break;
//         }
//     }

//     Ok(steps)
// }

// fn move_node<'a> (start: Vec<&str>, direction: u8, nodes:  &Vec<(&'a str, Vec<&'a str>)>) -> Vec<&'a str> {    // if left, move to left child
//     // if right, move to right child

//     let result: Vec<&str> = start.iter().map(|node| {
//         nodes.iter().find(|(n, _)| n == node).unwrap().1[direction as usize]
//     }).collect();

//     // println!("start: {:?}, direction: {}, result: {:?}", start, direction, result);
//     result
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_process() -> miette::Result<()> {
//         // todo!("haven't built test yet");
//         let input = "LR

//         11A = (11B, XXX)
//         11B = (XXX, 11Z)
//         11Z = (11B, XXX)
//         22A = (22B, XXX)
//         22B = (22C, 22C)
//         22C = (22Z, 22Z)
//         22Z = (22B, 22B)
//         XXX = (XXX, XXX)";
//         assert_eq!(6, process(input)?);
//         Ok(())
//     }
// }

use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        self, alphanumeric1, line_ending, multispace1,
    },
    combinator::eof,
    multi::{fold_many1, many1},
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};

use crate::custom_error::AocError;

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
}

#[allow(clippy::type_complexity)]
fn parser(
    input: &str,
) -> IResult<
    &str,
    (
        Vec<Direction>,
        BTreeMap<&str, (&str, &str)>,
    ),
> {
    let (input, instructions) = many1(alt((
        complete::char('R').map(|_| Direction::Right),
        complete::char('L').map(|_| Direction::Left),
    )))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, map) = fold_many1(
        terminated(
            separated_pair(
                alphanumeric1,
                tag(" = "),
                delimited(
                    complete::char('('),
                    separated_pair(
                        alphanumeric1,
                        tag(", "),
                        alphanumeric1,
                    ),
                    complete::char(')'),
                ),
            ),
            alt((line_ending, eof)),
        ),
        BTreeMap::new,
        |mut acc: BTreeMap<&str, (&str, &str)>,
         (key, value)| {
            acc.insert(key, value);
            acc
        },
    )(input)?;

    Ok((input, (instructions, map)))
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let (input, (instructions, map)) =
        parser(input).expect("should validly parse");

    debug_assert_eq!(input, "");

    let starting_nodes: Vec<&str> = map
        .keys()
        .filter(|key| key.ends_with('A'))
        .cloned()
        .collect();

    let results = starting_nodes
        .iter()
        .map(|node| {
            let mut visited_nodes = vec![*node];
            let mut current_node = *node;
            // cycle is magically "the first Z",
            // and other assorted conditions due
            // to how the input is constructed.
            instructions
                .iter()
                .cycle()
                .enumerate()
                .find_map(|(index, instruction)| {
                    let options =
                        map.get(current_node).expect(
                            "always exist at a valid node",
                        );
                    let next_node = match instruction {
                        Direction::Left => options.0,
                        Direction::Right => options.1,
                    };
                    if next_node.ends_with('Z') {
                        Some(index + 1)
                    } else {
                        visited_nodes.push(next_node);
                        current_node = next_node;
                        None
                    }
                })
                .expect("should find a cycle")
        })
        .collect::<Vec<usize>>();

    let min_cycle = lcm(&results);

    Ok(min_cycle.to_string())
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        "6"
    )]
    fn test_process(
        #[case] input: &str,
        #[case] expected: &str,
    ) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}