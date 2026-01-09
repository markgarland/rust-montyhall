use rand::prelude::*;
use crate::Item::*;

fn main() {

    let mut rng = rand::rng();

    let mut no_swap_total = 0;
    let mut swap_total = 0;
    let number_of_runs = 1;
    for _ in 1..= number_of_runs {
        run_game(&mut no_swap_total, &mut swap_total, &mut rng);
    };
    println!("Player wins with no swap total = {:?}, player wins with swap total = {:?}", no_swap_total, swap_total);

}

fn run_game (no_swap_total: &mut i32, swap_total: &mut i32, rng: &mut ThreadRng) {
    let mut boxes = vec![Car, Goat, Goat];
    boxes.shuffle(rng);
    println!("{:?}", boxes);

    let players_choice: i8 = rng.random_range(0..=2);
    println!("Player selects box {}", players_choice);

    let goats: Vec<i8> = boxes
        .iter()
        .enumerate()
        .filter_map(|(index, item)| {
            if *item == Goat {
                Some(index as i8)
            } else {
                None
            }
        }
        )
        .collect();
    println!("Goats are in boxes {:?}", goats);

    let montys_options: Vec<i8> = goats.into_iter().filter(|x| *x != players_choice).collect();
    let montys_choice = *montys_options.choose(rng).unwrap();
    println!("Monty's options are {:?} and he chooses {:?}", montys_options, montys_choice);

    let player_options: Vec<i8> = (0..=2).filter(|x| {
        *x != montys_choice && *x != players_choice
    }).collect();
    let player_option: i8 = *player_options.get(0).unwrap();
    println!("Player's options are {:?}", player_options);

    let player_wins_with_no_swap = boxes[players_choice as usize] == Car;
    let player_wins_with_swap = boxes[player_option as usize] == Car;
    println!("Player wins with no swap {:?}, player wins with swap {:?}", player_wins_with_no_swap, player_wins_with_swap);

    if player_wins_with_no_swap {
        *no_swap_total = *no_swap_total + 1;
    } else if player_wins_with_swap {
        *swap_total = *swap_total + 1;
    }
}

#[derive(Debug, PartialEq)]
enum Item {
    Goat,
    Car
}