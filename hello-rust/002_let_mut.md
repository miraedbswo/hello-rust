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

## 타입
변수의 데이터 타입이 정의되지 않는 경우 Rust 컴파일러는 타입을 추론하여 결정하게 됩니다.

```Rust
let a = 12345;
let a: i32 = 12345;  // default

let b = 3.14;
let b: f64 = 3.14;  // default

let c = false;
let c: bool = false;  //default
```
변수의 데이터 타입을 직접 지정하는 경우 변수명 뒤에 `:`을 찍고 타입을 지정할 수 있습니다.


# 변수의 불변성 (immutability)
Rust에서 모든 변수는 "디폴트로" 불변성의 성질을 갖습니다.
변수에 값이 한 번 지정된 후에는 그 값을 수정할 수 없습니다.

아래 예제를 보면, 변수 a에 처음 100을 할당한 후, 다시 1을 증가시키려 하였는데, 이 프로그램은 컴파일러에 의해 에러로 처리됩니다. 
런타임시에 에러가 나는 것이 아니라, 컴파일 시점에 에러가 발생합니다.

```rust
let a = 100;
a = a + 1;     // 에러: cannot assign twice to immutable variable `a`
println!("{}", a);
```

## mut
변경 가능한 변수, 참조, 또는 포인터.
변수의 값이 변경되어야 하는 경우라면 변수 앞에 `mut`라는 키워드를 지정할 수 있습니다.

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

(아래 코드는 vscode mut 예제인데 실행 시 에러가 발생합니다. 아래에서 후술합니다.)

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

### mut TrobleShooting

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

# 상수 Const
Rust에서 const는 불변의 값을 갖는다.  
상수는 let이 아닌 const로 선언되며 항상 데이터 타입을 지정해야 한다.  

let으로 선언되는 변수의 경우 컴파일러가 추론 (infer)를 통해 데이터 타입을 알아낸다.
```rust
fn main() {
    const PI: f64 = 3.141592;

    let area = PI * 5.0 * 5.0;
    println!("{}", area);
}
```
  
# Shadowing
Rust 코드에서 `let`은 변수를 선언하는데 사용되는데, 한 Scope에서 동일한 변수명을 `let`으로 여러 번 정의하는 것이 가능하다.

예를 들어 아래 예제에서 변수 a는 정수 타입으로 선언되었는데, 중간에 `let`을 사용하여 다시 문자열 타입으로 변경하여 정의하였다.

```rust
fn main() {
    let a = 1;
    println!("{}", a);

    let a = "hello";
    println!("{}", a);
}
```

아래 코드는 변수 a를 여러 번 할당하면서 기존에 선언했던 값을 잃게 하는 예제이다.

```rust
fn main() {
    let a = 1;
    let a = 2;
    {
        let a = a + 1;
        println!("{}", a); // output(출력): 3
    }
    println!("{}", a); // output(출력): 2
}
```
내부 Scope에서 a는 `a+1 = 3`을 갖는 변수 a를 만들게 되고, 이는 3을 표시하게 된다.  
하지만 내부 Scope를 벗어나게 되면 안의 변수 a는 소멸하게 되고 외부 변수인 2를 갖게 된다.

위 예제를 Shadowing이 아닌 mut을 써서 변경하면 Scope 밖에서도 값이 달라짐을 알 수 있다.
```rust
fn main() {
    let mut a = 1;
    a = 2;
    {
        a = a + 1;
        println!("{}", a); // output (출력): 3
    }
    println!("{}", a); // output (출력) : 3
}
```

## TMℹ: how to Println object in rust

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
