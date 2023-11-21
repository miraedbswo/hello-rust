// A mutable variable in the parameter list of a function.
fn foo(mut x: u8, y: u8) -> u8 {
    x += y;
    x
}

fn main() {
    // Modifying a mutable variable.
    let mut a = 5;
    a += 1;

    assert_eq!(foo(3, 4), 7);
    assert_eq!(a, 6);
}


// Taking a mutable reference.
fn push_two(v: &mut Vec<u8>) {
    v.push(2);
}

fn main() {
    // A mutable reference cannot be taken to a non-mutable variable.
    let mut v: Vec<u8> = vec![0, 1];
    // Passing a mutable reference.
    push_two(&mut v);
    assert_eq!(v, vec![0, 1, 2]);

    let mut v = vec![0, 1];
    let mut_ref_v = &mut v; // mutable reference
    mut_ref_v.push(2);
    assert_eq!(v, vec![0, 1, 2]);
}
