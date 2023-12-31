#![allow(dead_code)]

#[derive(Debug)]
struct Person {
  name: String,
  age: u8,
}

struct Pair(i32, i32);

#[derive(Debug)]
struct Point {
  x: f32,
  y: f32,
}

#[derive(Debug)]
struct Rectangle {
  p1: Point,
  p2: Point,
}

struct Unit;


fn rect_area(rect: Rectangle) -> f32 {
  let Rectangle { p1: Point { x: x1, y: y1 }, p2: Point { x: x2, y: y2 } } = rect;
  (x1 - x2).abs() * (y1 - y2).abs()
}

fn square(point: Point, width: f32) -> Rectangle {
  let Point { x, y } = point;
  Rectangle {
    p1: Point { x, y },
    p2: Point { x: x + width, y: y + width },
  }
}

fn main() {
    let name = String::from("Peter");
    let age = 27; 
    let peter = Person { name, age };
    println!("{:?}", peter);


    let point: Point = Point { x: 10.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);

  let bottom_right = Point { x: 5.2, ..point };

    println!("{:?}", bottom_right);

    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
      p1: Point { x: left_edge, y: top_edge },
      p2: bottom_right,
    };

    println!("{:?}", _rectangle);
}
