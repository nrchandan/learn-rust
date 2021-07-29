mod guess;
mod tweet;
mod summary;

use std::{cmp::Ordering, io};
use rand::Rng;
use guess::Guess;
use tweet::tweet_play;

fn main() {
    vector_play();
    tweet_play();
    let msg = "Enter 1 for guessing game, 2 for fibonacci, 3 for first word, 4 to quit";
    let option = _read_num(Option::Some(msg));
    match option {
        1 => guessing_game(),
        2 => nth_fibonacci(),
        3 => first_word(),
        _ => println!("bye")
    }
}

fn guessing_game() {
    let upper = 100;
    println!("Guess the number from 1 to {}!", upper);
    let secret_number = rand::thread_rng().gen_range(1..upper+1);
    loop {
        let guess = Guess::new(_read_num(Option::Some("Please input your guess.")));
        println!("You guessed: {}", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn nth_fibonacci() {
    println!("Enter 'n' for fibonacci sequence.");
    let num = _read_num(Option::None);
    let suffix = match num {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th"
    };
    println!("{}{} fibonacci number is {}", num, suffix, _nth_fibonacci(num));
}

fn _nth_fibonacci(mut n: i32) -> u32 {
    if n <= 2 {
        return 1;
    }
    n -= 2;
    let mut i: u32 = 1;
    let mut j: u32 = 1;
    while n > 0 {
        let tmp = i + j;
        i = j;
        j = tmp;
        n -= 1;
    }
    j
}

fn first_word() {
    let msg = String::from("Enter some text");
    let string = _read_text(&msg);
    println!("First word is {}", _first_word(&string));
}

// assumes no leading spaces
fn _first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    s
}

fn _read_num(some_msg: Option<&str>) -> i32 {
    let msg = some_msg.unwrap_or("Please enter a number");
    loop {
        let n: i32 = match _read_text(&msg).trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        return n;
    }
}

fn _read_text(msg: &str) -> String {
    println!("{}", msg);
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    n
}

fn vector_play() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", &v);
}
