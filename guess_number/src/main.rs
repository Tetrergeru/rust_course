use std::io::Write;

use rand::{thread_rng, Rng};

fn main() {
    const TRIES: usize = 10;
    const MAX_NUMBER: u32 = 10;

    let mut rand = thread_rng();
    let number = rand.gen_range(0..MAX_NUMBER);

    println!("Я загадал число! Угадывай!");

    for i in 0..TRIES {
        print!("Попытка {i} > ");
        
        let guess = get_user_input();
        let guess = guess.parse::<u32>();

        if guess.is_err() {
            println!("Это вообще не число! Попробуй снова")
        } else if guess == Ok(number) {
            println!("Правильно! Я и правда загадал {number}!");
            return;
        } else {
            println!("Неправильно, попробуй снова :(");
        }
    }

    println!("Попытки кончились, ты проиграл");
}

fn get_user_input() -> String {
    use std::io::{stdin, stdout};

    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    input.trim_end().to_string()
}
