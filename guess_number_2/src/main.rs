use std::io::Write;

use rand::{thread_rng, Rng};

struct Game {
    number: u32,
    tries: usize,
}

impl Game {
    fn new(max_number: u32, tries: usize) -> Self {
        let mut rand = thread_rng();
        let number = rand.gen_range(0..max_number);

        Self { number, tries }
    }

    fn play(&self) -> bool {
        println!("Я загадал число! Угадывай!");

        for i in 0..self.tries {
            print!("Попытка {i} > ");
            let guess = get_user_input().parse::<u32>();

            if guess.is_err() {
                println!("Это вообще не число! Попробуй снова")
            } else if guess == Ok(self.number) {
                println!("Правильно! Я и правда загадал {}!", self.number);
                return true;
            } else {
                println!("Неправильно, попробуй снова :(");
            }
        }

        println!("Попытки кончились, ты проиграл");
        false
    }
}

fn main() {
    const TRIES: usize = 10;
    const MAX_NUMBER: u32 = 10;

    let game = Game::new(MAX_NUMBER, TRIES);
    game.play();
}

fn get_user_input() -> String {
    use std::io::{stdin, stdout};

    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    input.trim_end().to_string()
}
