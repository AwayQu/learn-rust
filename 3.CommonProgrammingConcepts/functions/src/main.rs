fn main() {
    println!("Hello, world!");

    another_function();
    another_function1(5);
    another_function2(5, 6);

    clousure();

    println!("The value of x is: {}", five());

}

fn five() -> i32 {
    5
}

fn clousure() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn another_function() {
    println!("Another function.");
}

fn another_function1(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function2(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
