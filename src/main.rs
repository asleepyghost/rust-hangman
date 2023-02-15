mod game;

use std::io::stdin;

use game::GameState;
use rand_word;

fn print_greeting(game_state: &GameState) {
    println!("Current guess: {}", game_state.get_current_guess());
    println!("Please guess a letter: ");
}

fn get_user_guess() -> String {
    let mut guess = String::from("");

    stdin()
        .read_line(&mut guess)
        .expect("Something went wrong reading your guess.");

    guess
}

fn next_round(game_state: &mut GameState) -> bool {
    if String::from(game_state.get_current_guess()) == game_state.get_generated_word_as_str() {
        return true;
    }

    if game_state.out_of_guesses() {
        return false;
    }

    print_greeting(game_state);

    let guess = get_user_guess();

    if guess.trim().len() != 1 {
        println!("Please provide a single character as a guess.");
        next_round(game_state);
    }

    game_state.submit_guess(&guess);
    game_state.increment_guess_count();
    next_round(game_state)
}

fn main() {
    let word = rand_word::new(1);
    let mut game_state = GameState::new(&word);

    println!("Welcome to Hangman! :3");
    println!("----------------------------");

    let success = next_round(&mut game_state);

    if success {
        println!("You win!");
        return;
    }

    println!(
        "Game over! The word was: {} & you guessed: {}",
        word,
        game_state.get_current_guess()
    )
}
