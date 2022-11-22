fn main() {
    println!("Guess the number!");
    loop {
        println!("Input your guess: ");

        let mut guess = String::new();

        let random_num = rand::random::<u8>();

        println!("{random_num}");

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("Your Guess: {guess}");

        let guess: u8 = guess.trim()
            .parse::<u8>()
            .expect("Please type a number.");

        match guess.cmp(&random_num) {
            std::cmp::Ordering::Less => println!("HAHA TOO SMALL DING NUT!"),
            std::cmp::Ordering::Greater => println!("HAHA TOO BIG BIGGER DING NUT!"),
            std::cmp::Ordering::Equal =>  {
                println!("okayy you got it. good job.");
                break;
            }
        }
    }
    return;
}