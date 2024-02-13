// // use crate::custom_error::AocError;
// // use std::collections::HashMap;

// // #[tracing::instrument]
// // pub fn process(
// //     input: &str,
// // ) -> miette::Result<i64, AocError> {
// //     // todo!("day XX - part 1");
// //     // use a hashmap to make a list of all the seeds
// //     // parse input into vectors of 3 numbers for each map
// //     // go back and edit from DESTINATION to SOURCE for RANGE
// //         // 
// //     // return the lowest number at the end

// //     let mut seeds = HashMap::new();

// //     // println!("input: {:?}", input);

// //     let output = input
// //         .lines()
// //         .nth(0)
// //         .map(|line| {
// //             let line: Vec<i64> = line
// //                 .trim_start_matches("seeds: ")
// //                 .split_whitespace()
// //                 .map(|num| num.parse::<i64>().expect("should be a number"))
// //                 .collect();

// //             let line: HashMap<i64, i64> = line.into_iter().map(|x: i64| (x, x)).collect();

// //             seeds = line.clone();

// //             println!("line: {:?}", line);

// //             let min_value = *line.values().min().unwrap();

// //             min_value
// //         }).unwrap();
    
// //     let mut temp = vec![];
// //     let mut map_out = vec![];

// //     let maps: Vec<_> = input
// //         .lines()
// //         .map(|line| {
// //             // let mut numbers: Vec<Vec<u32>> = vec![];
// //             let mut nums: Vec<i64> = vec![];

// //             if let Some(first_char) = line.trim_start().chars().next() {
// //                 if first_char.is_numeric() {
// //                     line.split_ascii_whitespace().for_each(|num| {
// //                         nums.push(num.parse::<i64>().expect("Should be a number"));
// //                     });                        
                    
// //                     // numbers.push(nums);

// //                     // println!("The first character '{}' is a number.", first_char);
// //                 } else {
// //                     // println!("The first character '{}' is not a number.", first_char);
// //                 }
// //             } else {
// //                 // println!("The string is empty.");
// //             }

// //             // println!("nums: {:?}", line);
        
// //             if nums.is_empty() {
// //                 map_out.push(temp.clone());
// //                 temp = vec![];
// //             } else {
// //                 temp.push(nums);
// //             }

// //             1
// //         })
// //         .collect();

// //     let mut counter = 0;
// //     let translate: Vec<_> = map_out.iter()
// //         .map(|category| {
// //             let mut used: Vec<_> = vec![];

// //             let dummy: Vec<_> = category.iter().map(|category| {
// //                 // println!("category: {:?}", category);
// //                 let high = category[1] + category[2] - 1;
// //                 let low = category[1];

// //                 // if any key is in range(high, low)
// //                     // then update it to the new value
// //                 for (_, value) in seeds.iter_mut() {
// //                     if low <= *value && *value <= high && !used.contains(&*value) {
// //                         // println!("{}", category[0] as i64 - category[1] as i64);
// //                         // println!("ins");
// //                         if category[0] > category[1] {
// //                             *value += category[0] - category[1];
// //                             used.push(*value);
// //                          } else {
// //                             *value -= category[1] - category[0];
// //                             // println!("val: {}", category[1] - category[0]);
// //                             used.push(*value);
// //                          } 
// //                     }
// //                 }
// //                 counter += 1;
// //                 // println!("{:?}, {}", seeds, counter);
// //                 // println!("used: {:?}", used);
// //             }).collect();

// //     }).collect();

// //     // println!("translate: {:?}", translate);

// //     println!("maps: {:?}", maps);

// //     println!("seeds: {:?}", seeds);

// //     // return the lowest value of seeds
// //     let lowest_value = *seeds.values().min().unwrap();
// //     Ok(lowest_value)
// // }

// // #[cfg(test)]
// // mod tests {
// //     use super::*;

// //     #[test]
// //     fn test_process() -> miette::Result<()> {
// //         // todo!("haven't built test yet");
// //         let input = "seeds: 79 14 55 13

// //         seed-to-soil map:
// //         50 98 2
// //         52 50 48
        
// //         soil-to-fertilizer map:
// //         0 15 37
// //         37 52 2
// //         39 0 15
        
// //         fertilizer-to-water map:
// //         49 53 8
// //         0 11 42
// //         42 0 7
// //         57 7 4
        
// //         water-to-light map:
// //         88 18 7
// //         18 25 70
        
// //         light-to-temperature map:
// //         45 77 23
// //         81 45 19
// //         68 64 13
        
// //         temperature-to-humidity map:
// //         0 69 1
// //         1 0 69
        
// //         humidity-to-location map:
// //         60 56 37
// //         56 93 4";
// //         assert_eq!(35, process(input)?);
// //         Ok(())
// //     }
// // }

// use std::ops::Range;

// use crate::custom_error::AocError;
// use nom::{
//     bytes::complete::take_until,
//     character::complete::{self, line_ending, space1},
//     multi::{many1, separated_list1},
//     sequence::tuple,
//     IResult, Parser,
// };
// use nom_supreme::{tag::complete::tag, ParserExt};
// use tracing::info;

// // struct SeedId(u64);

// #[derive(Debug)]
// struct SeedMap {
//     mappings: Vec<(Range<u64>, Range<u64>)>,
// }

// impl SeedMap {
//     fn translate(&self, source: u64) -> u64 {
//         let valid_mapping = self.mappings.iter().find(
//             |(source_range, _)| {
//                 source_range.contains(&source)
//             },
//         );

//         let Some((source_range, destination_range)) =
//             valid_mapping
//         else {
//             return source;
//         };

//         let offset = source - source_range.start;

//         destination_range.start + offset
//     }
// }

// #[tracing::instrument(skip(input))]
// fn line(
//     input: &str,
// ) -> IResult<&str, (Range<u64>, Range<u64>)> {
//     let (input, (destination, source, num)) = tuple((
//         complete::u64,
//         complete::u64.preceded_by(tag(" ")),
//         complete::u64.preceded_by(tag(" ")),
//     ))(input)?;

//     // dbg!(destination, num);
//     Ok((
//         input,
//         (
//             source..(source + num),
//             destination..(destination + num),
//         ),
//     ))
// }
// fn seed_map(input: &str) -> IResult<&str, SeedMap> {
//     take_until("map:")
//         .precedes(tag("map:"))
//         .precedes(
//             many1(line_ending.precedes(line))
//                 .map(|mappings| SeedMap { mappings }),
//         )
//         .parse(input)
// }
// #[tracing::instrument(skip(input), fields(input_first_line = input.split('\n').next().unwrap()))]
// fn parse_seedmaps(
//     input: &str,
// ) -> IResult<&str, (Vec<u64>, Vec<SeedMap>)> {
//     let (input, seeds) = tag("seeds: ")
//         .precedes(separated_list1(space1, complete::u64))
//         .parse(input)?;
//     let mut seeds_tuped = vec![];
//     for k in (0..seeds.len()).step_by(2) {
//         seeds_tuped.push((seeds[k], seeds[k + 1]));
//     }    
//     println!("seeds: {:?}", seeds_tuped);
//     let mut new_seeds = Vec::new();
//     for (start, end) in seeds_tuped {
//         for i in start..start + end {
//             new_seeds.push(i);
//         }
//     }
//     println!("length: {}", new_seeds.len());
//     info!(?seeds);
//     let (input, maps) = many1(seed_map)(input)?;

//     Ok((input, (new_seeds, maps)))
// }

// #[tracing::instrument(skip(input))]
// pub fn process(
//     input: &str,
// ) -> miette::Result<String, AocError> {
//     let (_, (seeds, maps)) =
//         parse_seedmaps(input).expect("a valid parse");

//     let locations = seeds
//         .iter()
//         .map(|seed| {
//             maps.iter().fold(*seed, |seed, map| {
//                 map.translate(seed)
//             })
//         })
//         .collect::<Vec<u64>>();

//     Ok(locations
//         .iter()
//         .min()
//         .expect("should have a minimum location value")
//         .to_string())
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test_log::test]
//     fn test_process() -> miette::Result<()> {
//         info!("here");
//         let input = "seeds: 79 14 55 13

// seed-to-soil map:
// 50 98 2
// 52 50 48

// soil-to-fertilizer map:
// 0 15 37
// 37 52 2
// 39 0 15

// fertilizer-to-water map:
// 49 53 8
// 0 11 42
// 42 0 7
// 57 7 4

// water-to-light map:
// 88 18 7
// 18 25 70

// light-to-temperature map:
// 45 77 23
// 81 45 19
// 68 64 13

// temperature-to-humidity map:
// 0 69 1
// 1 0 69

// humidity-to-location map:
// 60 56 37
// 56 93 4";
//         assert_eq!("35", process(input)?);
//         Ok(())
//     }
// }

use std::ops::Range;

use crate::custom_error::AocError;

use nom::{
    bytes::complete::take_until,
    character::complete::{self, line_ending, space1},
    multi::{many1, separated_list1},
    sequence::{separated_pair, tuple},
    IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tracing::info;

// struct SeedId(u64);

#[derive(Debug)]
struct SeedMap {
    mappings: Vec<(Range<u64>, Range<u64>)>,
}

impl SeedMap {
    fn translate(&self, source: u64) -> u64 {
        let valid_mapping = self.mappings.iter().find(
            |(source_range, _)| {
                source_range.contains(&source)
            },
        );

        let Some((source_range, destination_range)) =
            valid_mapping
        else {
            return source;
        };

        let offset = source - source_range.start;

        destination_range.start + offset
    }
}

#[tracing::instrument(skip(input))]
fn line(
    input: &str,
) -> IResult<&str, (Range<u64>, Range<u64>)> {
    let (input, (destination, source, num)) = tuple((
        complete::u64,
        complete::u64.preceded_by(tag(" ")),
        complete::u64.preceded_by(tag(" ")),
    ))(input)?;

    // dbg!(destination, num);
    Ok((
        input,
        (
            source..(source + num),
            destination..(destination + num),
        ),
    ))
}
fn seed_map(input: &str) -> IResult<&str, SeedMap> {
    take_until("map:")
        .precedes(tag("map:"))
        .precedes(
            many1(line_ending.precedes(line))
                .map(|mappings| SeedMap { mappings }),
        )
        .parse(input)
}
#[tracing::instrument(skip(input), fields(input_first_line = input.split('\n').next().unwrap()))]
fn parse_seedmaps(
    input: &str,
) -> IResult<&str, (Vec<Range<u64>>, Vec<SeedMap>)> {
    let (input, seeds) = tag("seeds: ")
        .precedes(separated_list1(
            space1,
            separated_pair(
                complete::u64,
                tag(" "),
                complete::u64,
            )
            .map(|(start, offset)| start..(start + offset)),
        ))
        .parse(input)?;
    info!(?seeds);
    let (input, maps) = many1(seed_map)(input)?;

    Ok((input, (seeds, maps)))
}

#[tracing::instrument(skip(input))]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let (_, (seeds, maps)) =
        parse_seedmaps(input).expect("a valid parse");

    // let count = seeds
    //     .iter()
    //     .map(|range| range.end - range.start)
    //     .sum();
    // let count = seeds.len() as u64;
    let minimum_location = seeds
        .into_par_iter()
        // .progress_count(count)
        .flat_map(|range| range.clone())
        .map(|seed| {
            maps.iter()
                .fold(seed, |seed, map| map.translate(seed))
        })
        .min();

    Ok(minimum_location
        .expect("should have a minimum location value")
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!("46", process(input)?);
        Ok(())
    }
}