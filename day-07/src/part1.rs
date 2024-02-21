use itertools::Itertools;

use crate::custom_error::AocError;
use std::ops::Deref;

#[derive(Debug, Clone, Copy)]
enum HandType{
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug)]
struct Hand {
    hand: HandType,
    first_card: char,
}

fn score_hand(hand: &str) -> (HandType, (u32, u32, u32, u32, u32)) {
    use HandType::*;

    let counts = hand.chars().counts();
    let values = counts.values().sorted().join("");
    let hand_type = match values.deref() {
        "5" => FiveOfAKind,
        "14" => FourOfAKind,
        "23" => FullHouse,
        "113" => ThreeOfAKind,
        "122" => TwoPair,
        "1112" => OnePair,
        "11111" => HighCard,
        value => panic!("Unknown hand type: {}", value),
    };
    let card_scores = hand.chars().map(|card|
     match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        value => value.to_digit(10).unwrap(),
    }).collect_tuple().unwrap();
    (hand_type, card_scores)
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<u32, AocError> {

    let hands = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(" ").unwrap();
            (hand, bid.parse::<u32>().unwrap(), score_hand(hand))
        })
            .sorted_by_key(|x| (x.2.0 as u8, x.2.1)) 
            .enumerate()
            .map(|(index, (_hand, bid, _))| (index as u32 + 1) * bid)
            .sum::<u32>();

        dbg!(hands);

    Ok(2)
}


// fn determine_rank(hand: &str) -> u32 {
//     let cards: Vec<char> = hand.chars().collect();

//     // Count the occurrences of each card
//     let mut card_counts = [0; 13];
//     for &c in &cards {
//         let index = match c {
//             '2' => 0,
//             '3' => 1,
//             '4' => 2,
//             '5' => 3,
//             '6' => 4,
//             '7' => 5,
//             '8' => 6,
//             '9' => 7,
//             'T' => 8,
//             'J' => 9,
//             'Q' => 10,
//             'K' => 11,
//             'A' => 12,
//             _ => unreachable!(), // This should never happen given the input format
//         };
//         card_counts[index] += 1;
//     }

//     // Determine the rank based on the counts of each card
//     let max_count = *card_counts.iter().max().unwrap();
//     match max_count {
//         5 => 7, // Five of a kind
//         4 => 6, // Four of a kind
//         3 => {
//             let mut has_pair = false;
//             for &count in &card_counts {
//                 if count == 2 {
//                     has_pair = true;
//                     break;
//                 }
//             }
//             if has_pair {
//                 5 // Full house
//             } else {
//                 4 // Three of a kind
//             }
//         }
//         2 => {
//             let num_pairs = card_counts.iter().filter(|&&count| count == 2).count();
//             match num_pairs {
//                 2 => 3, // Two pair
//                 1 => 2, // One pair
//                 _ => 1, // High card
//             }
//         }
//         _ => 1, // High card
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        // todo!("haven't built test yet");
        let input = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";
        assert_eq!("6440", process(input)?);
        Ok(())
    }
}
