# Hangaman Game

This code is an implementation of a Hangman game in the Rust programming language. The game is played by guessing letters in a word and receiving feedback about whether the guessed letter is contained in the word. If the player guesses the word correctly, they win the game. Otherwise, the player loses if they make too many incorrect guesses.

In this code, there is a HangmanGame struct that stores the word that the player is trying to guess, a HashSet of the letters that the player has guessed, and a count of the number of incorrect guesses that the player has made. The HangmanGame struct has an implementation block with three methods:

The new method creates and returns a new HangmanGame instance with the given word, an empty HashSet of guessed letters, and a count of incorrect guesses initialized to 0.
The guess method takes a character as input and returns a boolean indicating whether the guess was correct. It also updates the guessed set and the wrong count as necessary.
The display method prints the current state of the game to the console, including the word with unguessed letters replaced by underscores, the set of guessed letters, and the number of incorrect guesses.
The main function is the entry point of the program. It creates a new HangmanGame instance with the word "rust" and enters a loop to play the game. Inside the loop, the display method is called to show the current state of the game, and then the player is prompted to enter their guess. The guess is checked using the guess method, and if it is correct, the game ends and the player is declared the winner. If the guess is incorrect, the loop continues and the player can make another guess.
