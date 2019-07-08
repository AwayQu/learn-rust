use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    

    let secret_number = rand::thread_rng().gen_range(1, 101);
     println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        // mut mutable
        // let foo = 5; immutable
        let mut guess = String::new();

        io::stdin().read_line(&mut guess) // & indicate is a reference
            .expect("Failed to read line");
        // expect will cause the program to crash and display the message that you passed as an argument to expect
        /**
         * If the read_line method returns an Err, 
         * it would likely be the result of an error coming from the underlying operating system. 
         * If this instance of io::Result is an Ok value,
         *  expect will take the return value that Ok is holding and return just that value to you so you can use it.
         *  In this case, that value is the number of bytes in what the user entered into standard input.
         */
        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");


        


        //  Result Type, the variants are Ok or Err.
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
