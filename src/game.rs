use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct GameState {
    // make it a vector so it's easy to find the position of correctly guessed letters
    random_word: Vec<String>,
    no_of_guesses: u8,
    guess: String, // This is the guess output with underscores and found letters included
}

fn get_as_vec_of_strings(word: &String) -> Vec<String> {
    word.chars().map(|x| x.to_string()).collect::<Vec<String>>()
}

// This generates a word of underscores which is the same length as the generated random word
fn initialise_guess(random_word: &Vec<String>) -> String {
    random_word.iter().map(|_| "_").collect::<String>()
}

impl GameState {
    pub fn new(word: &String) -> Self {
        let random_word = get_as_vec_of_strings(&word);
        let guess = initialise_guess(&random_word);

        GameState {
            random_word,
            no_of_guesses: 0,
            guess,
        }
    }

    pub fn get_generated_word_as_str(&self) -> String {
        self.random_word
            .iter()
            .flat_map(|x| x.chars())
            .collect::<String>()
    }

    pub fn submit_guess(&mut self, guess: &String) {
        self.guess = self
            .random_word
            .iter()
            .enumerate()
            .map(|(idx, letter)| {
                if self.guess.chars().nth(idx).unwrap() != '_' {
                    return letter.to_string();
                }

                if String::from(letter).trim() == String::from(guess).trim() {
                    return letter.to_string();
                }

                return String::from("_");
            })
            .collect();
    }

    pub fn increment_guess_count(&mut self) {
        self.no_of_guesses = self.no_of_guesses + 1;
    }

    pub fn out_of_guesses(&self) -> bool {
        self.no_of_guesses as usize >= self.random_word.len() + 3
    }

    pub fn get_current_guess(&self) -> &String {
        &self.guess
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let str_word = self.get_generated_word_as_str();

        write!(
            f,
            "Generated word: {}, length: {} | Num guesses: {} | Current guess: {}, length: {}",
            str_word,
            str_word.len(),
            &self.no_of_guesses,
            &self.guess,
            &self.guess.len()
        )
    }
}
