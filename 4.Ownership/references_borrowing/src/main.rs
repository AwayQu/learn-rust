fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    change(&mut s);


    let mut s = String::from("hello");
    /// mutable reference 只能有一个
    /// you can have only one mutable reference 
    /// to a particular piece of data in a particular scope. 
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    let mut s = String::from("hello");

    {
        let r1 = &mut s;

    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    // println!("{}, {}, and {}", r1, r2, r3);
    
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{}, {}", r1, r2);
    let r3 = &mut s; // BIG PROBLEM
    
    // let reference_dangling = dangle();
    let no_dangle_variable = no_dangle();
}

///  悬垂指针
// fn dangle() -> &String {
//     let s = String::from("hello"); /// will dealloc after return

//     &s
// }

fn no_dangle() -> String { /// move owner
    let s = String::from("hello");

    s
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}