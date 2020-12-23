#[allow(dead_code)]
use aoc_runner_derive::{aoc, aoc_generator};
#[allow(unused_imports)]
use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::hash::Hasher;

#[aoc_generator(day22)]
fn parse_input(input: &str) -> (Deck, Deck) {
    let mut player1 = Deck::new();
    let mut player2 = Deck::new();

    let mut p1 = true;
    for line in input.lines() {
        if let Ok(n) = line.parse::<usize>() {
            match p1 {
                true => player1.push_back(n),
                false => player2.push_back(n),
            }
        } else if line == "Player 2:" {
            p1 = false;
        }
    }
    (player1, player2)
}

#[aoc(day22, part1)]
fn part1((player1, player2): &(Deck, Deck)) -> usize {
    let mut player1 = player1.clone();
    let mut player2 = player2.clone();

    loop {
        if player1.len() == 0 || player2.len() == 0 {
            break;
        }
        let n1 = player1.pop_front().unwrap();
        let n2 = player2.pop_front().unwrap();

        if n1 > n2 {
            player1.push_back(n1);
            player1.push_back(n2);
        } else {
            player2.push_back(n2);
            player2.push_back(n1);
        }
    }
    if player1.len() != 0 {
        calculate_score(&mut player1)
    } else {
        calculate_score(&mut player2)
    }
}
type Cache = HashMap<u64, HashSet<u64>>;
type Deck = VecDeque<usize>;

#[aoc(day22, part2)]
fn part2((player1, player2): &(Deck, Deck)) -> usize {
    let mut player1 = player1.clone();
    let mut player2 = player2.clone();

    let p1_win = play_game(&mut player1, &mut player2, 1);
    //println!("P1:{:?}", player1);
    //println!("P2:{:?}", player2);
    //println!("WINNER P1:{:?}", p1_win);

    if p1_win {
        calculate_score(&mut player1)
    } else {
        calculate_score(&mut player2)
    }
}
fn play_game(player1: &mut Deck, player2: &mut Deck, game: usize) -> bool {
    let mut played_rounds = Cache::new();

    let mut round = 0;
    loop {
        if player1.len() == 0 || player2.len() == 0 {
            break;
        }
        round = round + 1;
        //println!("\n-- Round {:?} (Game {:?}) --", round, game);
        //println!("Player 1's deck:{:?}", player1);
        //println!("Player 2's deck:{:?}", player2);
        {
            let h1 = calculate_hash(&player1);
            let h2 = calculate_hash(&player2);

            let h2rounds = played_rounds.entry(h1).or_insert(HashSet::new());
            if h2rounds.contains(&h2) {
                //println!("...already played ... player 1 wins");
                return true; //player1 wins
            } else {
                h2rounds.insert(h2);
            }
        }
        //play
        let n1 = player1.pop_front().unwrap();
        let n2 = player2.pop_front().unwrap();
        //println!("Player 1's plays:{:?}", n1);
        //println!("Player 2's plays:{:?}", n2);

        #[allow(unused_assignments)]
        let mut won_by_player1 = false;
        if player1.len() < n1 || player2.len() < n2 {
            //Winner determined by highest number
            won_by_player1 = n1 > n2;
        } else {
            //Recursive Game
            let mut p1sub = player1.iter().take(n1).fold(VecDeque::new(), |mut acc, v| {
                acc.push_back(*v);
                acc
            });
            let mut p2sub = player2.iter().take(n2).fold(VecDeque::new(), |mut acc, v| {
                acc.push_back(*v);
                acc
            });
            //println!("------ going into sub game -------",);

            won_by_player1 = play_game(&mut p1sub, &mut p2sub, game + 1);
            //println!("------ leaving sub game -------",);
        }
        if won_by_player1 {
            player1.push_back(n1);
            player1.push_back(n2);
        //println!("Player 1 wins game {:?} round {:?}", game, round);
        } else {
            player2.push_back(n2);
            player2.push_back(n1);
            //println!("Player 2 wins game {:?} round {:?}", game, round);
        }
    }
    /*match player1.len() != 0 {
        true => //println!("The winner of game {:?} is player 1!", game),
        false => //println!("The winner of game {:?} is player 2!", game),
    }*/
    //println!("");
    player1.len() != 0
}

fn calculate_score(player: &mut Deck) -> usize {
    let mut result = 0;
    while let Some(x) = player.pop_front() {
        result += x * (player.len() + 1);
    }
    result
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let data = parse_input(&SAMPLE_DATA);
        assert_eq!(291, part2(&data));
    }

    #[test]
    fn test_part2_stalemate() {
        let data = parse_input(&STALEMATE_DATA);
        assert_eq!(105, part2(&data));
    }

    #[test]
    fn test_part1() {
        let data = parse_input(&SAMPLE_DATA);

        assert_eq!(306, part1(&data));
    }

    #[test]
    fn test_parse_input() {
        let (player1, player2) = parse_input(&SAMPLE_DATA);

        assert_eq!(5, player1.len());
        assert_eq!(player1, [9, 2, 6, 3, 1]);

        assert_eq!(5, player2.len());
        assert_eq!(player2, [5, 8, 4, 7, 10]);
    }

    lazy_static! {
        static ref STALEMATE_DATA: String =
            ["Player 1:", "43", "19", "", "Player 2:", "2", "29", "14",].join("\n");
        static ref SAMPLE_DATA: String = [
            "Player 1",
            "9",
            "2",
            "6",
            "3",
            "1",
            "",
            "Player 2:",
            "5",
            "8",
            "4",
            "7",
            "10",
        ]
        .join("\n");
    }
}
