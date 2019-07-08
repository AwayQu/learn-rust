use std::io;

fn main() {
    println!("Guess the number!");

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


    //  Result Type, the variants are Ok or Err.
    println!("You guessed: {}", guess);

}
