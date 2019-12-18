mod stack;
// fn main() {
//     stack::hello();
//     println!("Hello, world!");
// }
fn f<T>(g: T, x: u32) -> u32
where
    T: Fn(u32) -> u32,
{
    g(x + 1) * g(x + 2)
}
fn main() {
    let a = f(|x| x * x, 2);
    println!("{}", a);
}

