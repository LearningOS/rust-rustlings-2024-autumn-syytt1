// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let d: Option<Point> = Some(Point { x: 100, y: 200 });

    match d {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),//match will move d into p,we use ref here to borrow it
        _ => panic!("no match!"),
    }
    d; // Fix without deleting this line.
}
