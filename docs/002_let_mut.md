# Rust Getting started

# let

변수에 값을 바인딩하기 위해 사용됩니다.

`let` 문은 패턴에 의해 주어진 현재 범위에 새로운 변수 세트를 도입하는 데 사용됩니다.

```rust
let thing1: i32 = 100;
let thing2 = 200 + thing1;

let mut changing_thing = true;
changing_thing = false;
```

# mut

변경 가능한 변수, 참조, 또는 포인터.

`mut`은 여러 상황에서 사용될 수 있습니다. 첫 번째는 변경 가능한 변수로, 변수 이름에 값을 바인딩할 수 있는 어디에서나 사용할 수 있습니다. 몇 가지 예시는 다음과 같습니다:

```rust
// A mutable variable in the parameter list of a function.
fn foo(mut x: u8, y: u8) -> u8 {
    x += y;
    x
}

// Modifying a mutable variable.
let mut a = 5;
a = 6;

assert_eq!(foo(3, 4), 7);
assert_eq!(a, 6);
```

The second is mutable references.

두 번째는 가변 참조입니다. 이들은 `mut` 변수에서 생성될 수 있으며 고유해야 합니다: 다른 변수는 가변 참조를 가질 수 없고, 공유 참조도 가질 수 없습니다.

```rust
// Taking a mutable reference.
fn push_two(v: &mut Vec<u8>) {
    v.push(2);
}

fn main() {
    // A mutable reference cannot be taken to a non-mutable variable.
    let mut v = vec![0, 1];
    // Passing a mutable reference.
    push_two(&mut v);

    assert_eq!(v, vec![0, 1, 2]);
    let mut v = vec![0, 1];
    let mut_ref_v = &mut v;
    #[allow(unused)]
    let ref_v = &v;
    mut_ref_v.push(2);
}
```

Mutable raw pointers work much like mutable references, with the added possibility of not pointing to a valid object. The syntax is `*mut Type`.

가변 원시 포인터는 유효한 객체를 가리키지 않는 추가 가능성과 함께 가변 참조와 많이 유사하게 작동합니다. 문법은 `*mut Type`입니다.

## mut TrobleShooting

Rust에서는 값에 대해 하나의 변경 가능한 참조 또는 여러 개의 불변 참조를 가질 수 있지만 
동시에 변경 가능한 참조와 불변 참조를 모두 가질 수는 없습니다.

```rust
let mut v = vec![0, 1];

let mut_ref_v = &mut v;  // mutable reference
let ref_v = &v; // immutable reference -> error occured

mut_ref_v.push(2);
```

따라서 이를 해결하려면 immutable reference를 삭제해야 합니다.

```rust
let mut v = vec![0, 1];
let mut_ref_v = &mut v;  // mutable reference
mut_ref_v.push(2);
```

## how to Println object in rust

vector 객체를 프린트하려 했지만 일반적인 방법으로는 바로 출력할 수 없다.

`Vec<i32>` doesn't implement `std::fmt::Display` the trait `std::fmt::Display` is not implemented for `Vec<i32>` in format strings

std::fmt 함수를 사용하면 객체를 쉽게 출력할 수 있다

println! 매크로의 인자값으로는 결국 String을 넘겨주어야 하기 때문에 vector → string 변환 과정을 거쳐야 하는데 이를 fmt 함수로 포맷팅하여 바꿔준다

```rust
format!("{:?}", (3, 4));  // => "(3, 4)"

let mut v = vec![0, 1];
let s: String = format!("{:?}", &v)

// 이런 식으로 Print 가능
println!("{}", &s);
println!("{s}");

// 혹은 바로 fmt
println!("{:?}", &v)

// 읽기만 할 경우 읽기 엑세스를 따로 만들어주는 것이 좋다고 한다
let cloned_v = v.clone();
println!("{:?}", &cloned_v);
```
