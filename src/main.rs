use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    loop {
        /*
        The :: syntax in the ::new line indicates
        that new is an associated function of the String type.
        An associated function is a function that’s implemented on a type
        */
        let mut guess = String::new(); //mutable
        let read_error = String::from("Failed to read line"); //not mutable

        /*
        The & indicates that this argument is a reference,
        which gives you a way to let multiple parts of your code access
        one piece of data without needing to copy that data
        into memory multiple times.

        like variables, references are immutable by default.
        Hence, you need to write &mut guess rather than &guess to make it mutable.
        */
        io::stdin()
            .read_line(&mut guess)
            .expect(read_error.as_str());

        /*
        The underscore, _, is a catchall value;
        in this example, we’re saying we want to match all Err values,
        no matter what information they have inside them.
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess: {}", guess);

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
