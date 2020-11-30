#[macro_use]
extern crate roof;

#[spec(
- 10 <= a && a <= 10 &&
- 10 <= b && b <= 10 &&
- 20 <= return_value && return_value <= 20
)]
fn test_function(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {}
