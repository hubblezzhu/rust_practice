


#![allow(dead_code)]

#[derive(Debug)]

struct Person {
    name: String,
    age: u8,
}


struct Unit;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}


struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let x_point: Point = Point { x: 10.3, y: 0.4 };
    let y_point: Point = Point { x: 5.2,  y: 0.2 };

    println!("se")
}