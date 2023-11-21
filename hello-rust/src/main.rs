fn main() {
    let dat: (i32, char, bool) = (1, 'A', true);
    let (a, b, ..) = dat;
    println!("a = {}, b = {}", a, b);
}
