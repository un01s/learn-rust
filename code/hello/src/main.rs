// function to call function
fn add_one(x: i32) -> i32 {
  x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
  f(arg) + f(arg)
}

fn hi_5<F>(func: F) where F: Fn(u32) -> u32, {
    println!("Hi, {}", func(5));
}
fn square(x: u32) -> u32 {
    x*x
}

struct Point {
  x: f64,
  y: f64,
}

impl Point {
  fn origin() -> Point {
    Point{x: 0.0, y: 0.0 }
  }
  fn new(x:f64, y:f64) -> Point {
    Point{x: x, y: y}
  }
}

struct Rectangle {
  p1: Point,
  p2: Point,
}

impl Rectangle {
  fn area(&self) -> f64 {
    let Point { x: x1, y: y1 } = self.p1;
    let Point { x: x2, y: y2 } = self.p2;
    ((x1-x2)*(y1-y2)).abs()
  }

  fn perimeter(&self) -> f64 {
    let Point { x: x1, y: y1 } = self.p1;
    let Point { x: x2, y: y2 } = self.p2;
    2.0*((x1-x2).abs() + (y1-y2).abs())
  }

  fn translate(&mut self, x:f64, y:f64) {
    self.p1.x += x;
    self.p2.x += x;
    self.p1.y += y;
    self.p2.y += y;
  }
}

fn main() {
  let answer = do_twice(add_one, 5);
  println!("the answer is: {answer}");
  // another example
  hi_5(square);

  // closures
  // closures are functions that can capture the enclosing environment.
  // closures are anonymous
  let outer_var = 42;
  let closure_annotated = |i: i32| -> i32 { i + outer_var };
  let closure_inferred  = |i     |          i + outer_var  ;
  let one = || 1;

  println!("closure_annotated: {}", closure_annotated(1));
  println!("closure_inferred: {}", closure_inferred(1));
  println!("closure returning one: {}", one());
  
  // methods
  let rectangle = Rectangle {
    // Associated functions are called using double colons
    p1: Point::origin(),
    p2: Point::new(3.0, 4.0),
  };
  println!("Rectangle perimeter: {}", rectangle.perimeter());
  println!("Rectangle area: {}", rectangle.area());
}
