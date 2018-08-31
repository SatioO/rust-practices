// extern crate rand;

// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;

// fn main() {
//     println!("Guess the number!");

//     let secret_number = rand::thread_rng().gen_range(1, 101);
//     loop {
//         let mut guess = String::new();

//         println!("The secret number is: {}", secret_number);

//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Failed to read line");

//         let guess: u32 = guess.trim().parse().expect("Please type a number");

//         println!("You guessed: {}", guess);

//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Too small!"),
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Equal => {
//                 println!("You win!");
//                 break;
//             }
//         }
//     }
// }
// struct User {
//     name: String,
//     age: i32,
//     active: bool,
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// let user1 = User {
//     name: String::from("Vaibhav Satam"),,
//     active: true,
//     age: 1,
// }

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 40,
        height: 50,
    };

    let area1 = rect1.area();
    let area2 = rect2.area();
    let squared = Rectangle::square(3);
    println!("{:?}", squared);
    println!("rect1 area is {}", area1);
    println!("rect2 area is {}", area2);
    println!("rect1 is {:?}", rect1);
}
