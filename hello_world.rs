#[inline(never)]
fn uniquely_named_function() {
  println!("Hello world");
}

fn main() {
  uniquely_named_function();
}