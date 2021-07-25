use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let msg = "Enter 1 for guessing game, 2 for fibonacci, 3 for first word, 4 to quit";
    let option = _read_num(Option::Some(msg));
    match option {
        1 => guessing_game(),
        2 => {
            println!("Enter 'n' for fibonacci sequence.");
            let num = _read_num(Option::None);
            let suffix = match num {
                1 => "st",
                2 => "nd",
                3 => "rd",
                _ => "th"
            };
            println!("{}{} fibonacci number is {}", num, suffix, nth_fibonacci(num));
        },
        3 => {
            let msg = String::from("Enter some text");
            let string = _read_text(&msg);
            println!("First word is {}", first_word(&string));
        },
        _ => println!("bye")
    }
}

fn guessing_game() {
    let upper = 100;
    println!("Guess the number from 1 to {}!", upper);
    let secret_number = rand::thread_rng().gen_range(1..upper+1);
    loop {
        let guess = _read_num(Option::Some("Please input your guess."));
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn nth_fibonacci(mut n: u32) -> u32 {
    if n <= 2 {
        return 1;
    }
    n = n - 2;
    let mut i: u32 = 1;
    let mut j: u32 = 1;
    while n > 0 {
        let tmp = i + j;
        i = j;
        j = tmp;
        n = n - 1;
    }
    return j;
}

// assumes no leading spaces
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn _read_num(some_msg: Option<&str>) -> u32 {
    let msg = match some_msg {
        None => "Please enter a number.",
        Some(msg) => msg
    };
    loop {
        let n: u32 = match _read_text(&msg).trim().parse() {
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
