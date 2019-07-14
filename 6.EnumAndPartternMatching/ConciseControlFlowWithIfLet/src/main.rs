enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(u32),
}
fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    if let Some(3) = some_u8_value {
        println!("three");
    }
    println!("Hello, world!");
    let mut count = 0;
    let coin = Coin::Quarter(3);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
