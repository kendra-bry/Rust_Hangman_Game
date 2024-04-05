use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{self, BufRead};
use std::io::Write;

/// This function takes a reference to a secret word and a slice of guessed
/// letters and returns a string representing the displayed word in the Hangman
/// game.
fn display_word(word: &str, guessed_letters: &[char]) -> String {
    // Iterate over each character in the secret word.
    word.chars()
        // Map each character to either itself if it has been guessed or '_' if not.
        .map(|c| if guessed_letters.contains(&c) { c } else { '_' })
        // Collect the mapped characters into a new String.
        .collect::<String>()
}

/// This function checks if the game has been won by comparing the secret word
/// with the guessed letters. It returns true if all the letters in the word
/// have been guessed correctly, otherwise false.
fn is_game_won(word: &str, guessed_letters: &[char]) -> bool {
    // Check if all characters in the secret word are present in the guessed letters.
    word.chars().all(|c| guessed_letters.contains(&c))
}

/// This function prints the letters that have been guessed so far in the Hangman game.
fn print_guessed_letters(guessed_letters: &[char]) {
    // Print the guessed letters as a string, separated by commas.
    println!("Guessed letters: {}", guessed_letters.iter().collect::<String>());
}

/// This function prints the Hangman ASCII art corresponding to the number of wrong guesses.
/// The ASCII art represents the hangman figure, which progressively appears as more wrong
/// guesses are made.
fn print_hangman(num_wrong_guesses: i32) {
    // Match the number of wrong guesses to determine which Hangman stage to print.
    match num_wrong_guesses {
        0 => {
            // Print the Hangman figure for 0 wrong guesses.
            println!("  _______");
            println!(" |       |");
            println!(" |");
            println!(" |");
            println!(" |");
            println!(" |");
            println!("_|___");
        }
        1 => {
            // Print the Hangman figure for 1 wrong guess.
            println!("  _______");
            println!(" |       |");
            println!(" |       O");
            println!(" |");
            println!(" |");
            println!(" |");
            println!("_|___");
        }
        2 => {
            // Print the Hangman figure for 2 wrong guesses.
            println!("  _______");
            println!(" |       |");
            println!(" |       O");
            println!(" |       |");
            println!(" |");
            println!(" |");
            println!("_|___");
        }
        3 => {
            // Print the Hangman figure for 3 wrong guesses.
            println!("  _______");
            println!(" |       |");
            println!(" |       O");
            println!(" |      /|");
            println!(" |");
            println!(" |");
            println!("_|___");
        }
        4 => {
            // Print the Hangman figure for 4 wrong guesses.
            println!("  _______");
            println!(" |       |");
            println!(" |       O");
            println!(" |      /|\\");
            println!(" |");
            println!(" |");
            println!("_|___");
        }
        5 => {
            // Print the Hangman figure for 5 wrong guesses.
            println!("  _______");
            println!(" |       |");
            println!(" |       O");
            println!(" |      /|\\");
            println!(" |      /");
            println!(" |");
            println!("_|___");
        }
        _ => {
            // Print the Hangman figure for 6 or more wrong guesses (game over).
            println!("  _______");
            println!(" |       |");
            println!(" |       O");
            println!(" |      /|\\");
            println!(" |      / \\");
            println!(" |");
            println!("_|___");
        }
    }
}


/// This is the main function of the Hangman game. It initializes the game,
/// reads a list of words from a file, selects a random word from the list,
/// and then allows the player to guess letters or the full word until they win
/// or run out of attempts.
fn main() {
    // Initialize an empty vector to store the words read from the file.
    let mut words: Vec<String> = Vec::new();

    // Open the file containing the list of words.
    if let Ok(file) = File::open("words.txt") {
        // Read the file line by line using a buffered reader.
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            // For each line in the file, attempt to parse it as a word and add it to the vector.
            if let Ok(word) = line {
                words.push(word);
            }
        }
    } else {
        // If there was an error opening the file, print an error message and exit the program.
        println!("Failed to open file 'words.txt'. Exiting...");
        return;
    }

    // Choose a random word from the list of words.
    let secret_word = match words.choose(&mut rand::thread_rng()) {
        Some(word) => word,
        None => {
            // If there are no words in the file, print an error message and exit the program.
            println!("No words found in the file. Exiting...");
            return;
        }
    };

    // Define the maximum number of attempts allowed for the game.
    const MAX_ATTEMPTS: i32 = 6;
    // Initialize an empty vector to store the guessed letters.
    let mut guessed_letters: Vec<char> = Vec::new();
    // Initialize the number of attempts remaining to the maximum allowed.
    let mut attempts = MAX_ATTEMPTS;

    // Print a welcome message to the player.
    println!("\n---------------------------------");
    println!("Welcome to Hangman!");

    // Start the game loop, which continues until the player wins or runs out of attempts.
    while attempts > 0 {
        // Print the number of attempts remaining.
        println!("Attempts left: {}", attempts);
        println!("---------------------------------\n");
        // Display the current state of the word with guessed letters revealed.
        let display = display_word(secret_word, &guessed_letters);
        println!("Word: {}", display);
        // Display the letters that have been guessed so far.
        print_guessed_letters(&guessed_letters);

        // Prompt the player to enter a letter or a full word.
        print!("Guess a letter or the full word: ");
        io::stdout().flush().unwrap();

        // Read the player's input from the standard input.
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // Convert the input to lowercase for case-insensitive matching.
        let guess = guess.trim().to_lowercase();

        // Check if the input is a single letter and contains only alphabetic characters.
        if guess.len() == 1 && guess.chars().all(|c| c.is_alphabetic()) {
            // Guessing a single letter
            let letter = guess.chars().next().unwrap();
            if guessed_letters.contains(&letter) {
                // If the letter has already been guessed, inform the player and continue to the next iteration.
                println!("\n---------------------------------");
                println!("You've already guessed that letter!");
                continue;
            }
            // Add the guessed letter to the vector of guessed letters.
            guessed_letters.push(letter);
            // If the secret word does not contain the guessed letter, decrement the attempts and print the Hangman figure.
            if !secret_word.contains(letter) {
                println!("\n---------------------------------");
                println!("Incorrect guess!");
                attempts -= 1;
                print_hangman(MAX_ATTEMPTS - attempts);
            }
        } else if guess.len() > 1 && guess.chars().all(|c| c.is_alphabetic()) {
            // Guessing the full word
            if guess == *secret_word {
                // If the guess matches the secret word, print a congratulatory message and break out of the loop.
                println!("\n------------------------------------------------------");
                println!("Congratulations! You've won! The word was '{}'", secret_word);
                println!("------------------------------------------------------\n");
                break;
            } else {
                // If the guess is incorrect, decrement the attempts and print the Hangman figure.
                println!("\n---------------------------------");
                println!("Incorrect guess!");
                attempts -= 1;
                print_hangman(MAX_ATTEMPTS - attempts);
            }
        } else {
            // If the input is neither a single letter nor the full word, inform the player and continue to the next iteration.
            println!("\n---------------------------------");
            println!("Please enter only alphabetic characters!");
            continue;
        }

        // Check if the game has been won by comparing the secret word with the guessed letters.
        if is_game_won(secret_word, &guessed_letters) {
            // If the game is won, print a congratulatory message with the secret word and break out of the loop.
            println!("\n------------------------------------------------------");
            println!("Congratulations! You've won! The word was '{}'", secret_word);
            println!("------------------------------------------------------\n");
            break;
        }
    }

    // If the player runs out of attempts, print a message indicating that the game is over and reveal the secret word.
    if attempts == 0 {
        println!("\n----------------------------------------------------------");
        println!("Sorry, you've run out of attempts. The word was '{}'", secret_word);
        println!("----------------------------------------------------------\n");
    }
}

