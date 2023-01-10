use std::io;
use std::collections::HashSet;

struct HangmanGame {
    word: String,
    guessed: HashSet<char>,
    wrong: u8,
}

impl HangmanGame {
    fn new(word: &str) -> HangmanGame {
        HangmanGame {
            word: word.to_string(),
            guessed: HashSet::new(),
            wrong: 0,
        }
    }

    fn guess(&mut self, c: char) -> bool {
        if self.word.contains(c) {
            self.guessed.insert(c);
            if self.guessed.len() == self.word.len() {
                return true;
            }
            return false;
        } else {
            self.wrong += 1;
            return false;
        }
    }

    fn display(&self) {
        for c in self.word.chars() {
            if self.guessed.contains(&c) {
                print!("{} ", c);
            } else {
                print!("_ ");
            }
        }
        println!("");

        println!("Guessed: {:?}", self.guessed);
        println!("Wrong: {}", self.wrong);
    }
}

fn main() {
    let mut game = HangmanGame::new("rust");

    loop {
        game.display();

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim().chars().next().unwrap();
        if game.guess(guess) {
            game.display();
            println!("You win!");
            break;
        }
    }
}

