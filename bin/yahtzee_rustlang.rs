use yahtzee_rustlang::input::*;
use yahtzee_rustlang::player::Player;
use yahtzee_rustlang::score::Score;

const NUM_ROUNDS: u8 = 13;

fn introduction() {
    println!("Hello and welcome to YAHTZEE!!!")
}

fn main() {
    introduction();

    // let player_count: u32 = loop {
    //     println!("How many players?");
    //     match read_value() {
    //         Ok(num) => break num,
    //         Err(err) => {
    //             println!("{}", err);
    //             continue;
    //         }
    //     };
    // };

    // let mut players: Vec<Player> = Vec::new();
    // for i in 0..player_count {
    //     println!("Player: {}", i);
    //     let player = Player::new(get_player_name());
    //     players.push(player);
    // }

    let mut player = Player::new(get_player_name());
    for round_incr in 1..=NUM_ROUNDS {
        player.roll_dice();

        let mut possible_scores: Vec<Score> = Vec::new();
        for rolls in 1..4 {
            print!("\n{}'s Round {}", player.name, round_incr);
            print!("  |  Current Score: {}", player.score);
            println!("  |  Roll: {}", rolls);

            let scores = player.possible_scores();
            println!("{}\n", player);

            println!("Possible Scores:\n");
            for possible_score in scores.iter() {
                println!("\t{} points", possible_score)
            }

            let is_reroll: bool = loop {
                println!("\nDo you want to reroll? true/false");
                match read_value() {
                    Ok(is_reroll) => break is_reroll,
                    Err(err) => {
                        println!("{}", err);
                        continue;
                    }
                }
            };

            if is_reroll {
                player.reroll();
            } else {
                // could ask the user to score here
                possible_scores.extend(scores);
                break;
            }
        }

        println!("function to score here {:?}", possible_scores);
    }
}