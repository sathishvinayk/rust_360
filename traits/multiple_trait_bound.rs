use std::ops::Mul;
use std::fmt::Display;

#[derive(Copy, Clone)]
struct Context<T> {
    x: T
}

impl<T> Mul for Context<T> 
    where T: Mul<Output = T> + Display
{
    type Output = Context<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Context {
            x: self.x * rhs.x
        }
    }
}

impl<T: Display> Display for Context<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.x)
    }
}

fn multiply_values<K>(x: K) -> K 
    where K: Mul<Output = K> + Copy + Display
{
    println!("The x value is {}", x);
    x * x
}
fn main() {
    println!("{}", multiply_values(4));
    let context = Context { x: 10 };
    multiply_values(context);
}