// Bookmark: https://doc.rust-lang.org/book/ch06-00-enums.html

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    // println!("rect1 is {:#?}", rect1);
    dbg!(&rect1);
    println!("rect1 area is {}", rect1.area());
    let mut user1 = User {
        email: String::from("test@thing.com"),
        username: String::from("thisisausername"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = User{
        email: String::from("test2@thing.com"),
        ..user1
    };
    println!("user2 email is {}", user2.email);
    println!("The 10th fibonacci number is {}", nth_fibonacci_number(10));
    println!("80f is {}c", fahrenheit_to_celsius(80.0));
    println!("26.6c is {}f", celsius_to_fahrenheit(26.6));
    println!("Initiating countdown from 10");
    for i in (0..11).rev() {
        println!("{}", i)
    }
    println!("LIFTOFF!");
    loop {
        println!("What would you like to do?");
        println!("Enter your choice number");
        println!("1: Number Guessing Game");
        println!("2: Quit");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Not a valid choice.");
        let choice: i8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match choice {
            1 => guessing_game(),
            2 => break,
            _ => {
                println!("Not a valid choice. Try again.");
                continue;
            }
        }
    }

}


fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn nth_fibonacci_number(n: i64) -> i64 {
    match n {
        0 => 1,
        1 => 1,
        _ => nth_fibonacci_number(n - 1) + nth_fibonacci_number(n - 2),
    }
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0/5.0) + 32.0
}

fn guessing_game() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("The secret number is: {}", secret_number);
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32
    {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}