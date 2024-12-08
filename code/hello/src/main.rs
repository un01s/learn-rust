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
}
