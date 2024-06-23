fn main() {
    println!("Welcome to black jack game!");
    let mut main_game = Game::new();
    let mut user_input = String::new();
    loop {
        println!("What do you want to do:");
        println!("1) Take another card");
        println!("2) Stop");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let input_number: u32 = input.trim().parse().expect("Please type a number!");
        if (input_number == 2) {
            break;
        } else if (input_number == 1) {
            main_game.hand.add(main_game.deck.draw())
        } else {
            println!("Wrong option input!")
        }
    }
}
