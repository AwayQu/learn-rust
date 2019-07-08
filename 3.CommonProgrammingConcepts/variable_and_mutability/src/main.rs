
const MAX_POINTS: u32 = 100_000;
fn shadowing() {
    // shadowing
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}


fn main() {
    // let x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // ^^^^^ cannot assign twice to immutable variable
    // println!("The value of x is: {}", x);
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // You declare constants using the const keyword instead of the let keyword,
    // and the type of the value must be annotated.
    println!("const {}", MAX_POINTS);
    
    shadowing();
    
    shadowinglet();
}

fn shadowinglet() {
    let spaces = "   ";
    let spaces = spaces.len();
}

fn shadoingmut() {
    // let mut spaces = "   ";
    // spaces = spaces.len();
    //         ^^^^^^^^^^^^ expected &str, found usize
}