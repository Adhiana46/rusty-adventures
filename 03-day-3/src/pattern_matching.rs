/*
    The match keyword lets you match a value against one or more patterns. The comparisons are done from top to bottom and the first match wins.
*/
#[test]
fn pattern_matching_str_test() {
    let input = "xxx";
    match input {
        "bca" => println!("Bank Cape Antri"),
        "raya" | "jenius" | "jago" => println!("Bank Digital"),
        "xxx" => println!("Backup Site!"),
        _ => println!("Something Else"),
    }
}

#[test]
fn pattern_matching_number_test() {
    let input = 3;
    match input {
        1 => println!("One"),
        2 | 3 | 4 | 5 => println!("Below five but not one"),
        6..=10 => println!("Between six and ten"),
        _ => println!("Greater than 10"),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[test]
fn pattern_matching_enum_test() {
    let msg = Message::Move { x: 10, y: 20 };

    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
    }
}

/* Struct Match
    - The patterns Point { x, y: 0 } and Point { x: 0, y } match points on the x and y axes, respectively
    - The pattern Point { x, y } matches any other point and destructures the Point struct.
*/
struct Point {
    x: i32,
    y: i32,
}

#[test]
fn pattern_matching_struct_test() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

// Using if Guards
#[test]
fn pattern_matching_if_guard_test() {
    let num: Option<i32> = Some(5);
    match num {
        Some(x) if x < 5 => println!("Less than five: {}", x),
        Some(x) => println!("{}", x),
        None => println!("No value"),
    }
}
