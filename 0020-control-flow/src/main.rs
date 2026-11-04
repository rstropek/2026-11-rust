use std::ops::RangeInclusive;

fn main() {
    let dice_roll = rand::random_range::<u8, RangeInclusive<_>>(1..=6);

    // Our goal is to roll at least a 4
    let msg = if dice_roll < 3 {
        "You lost"
    } else if dice_roll == 3 {
        "So close!"
    } else {
        "You won!"
    };
    println!("{msg}");

    // Match must be EXHAUSTIVE!!!!

    // let msg = match dice_roll {
    //     n if n < 3 => "You lost",
    //     3 => "So close!",
    //     _ => "You won!",
    // };
    println!(
        "{}",
        match dice_roll {
            1 | 2 => "You lost",
            3 => "So close!",
            4 | 5 | 6 => "You won!",
            _ => panic!("Invalid roll"),
        }
    );

    // let possible_dice_values = vec!(1, 2, 3, 4, 5, 6);
    let possible_dice_values = (1..=6).collect::<Vec<u8>>();
    println!("{possible_dice_values:?}");

    // Do five dice rolls, we win if all five are >= 4
    let msg = {
        let mut rolls = 0;
        loop {
            let dice_roll = rand::random_range(1..=6);
            if dice_roll < 4 {
                break "We lost!";
            }
            rolls += 1;
            if rolls == 5 {
                break "We won!";
            }
        }
    };

    println!("{msg}");
}
