enum Coin1 {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin1) -> u32 {
    match coin {
        Coin1::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin1::Nickel => 5,
        Coin1::Dime => 10,
        Coin1::Quarter => 25,
    }
}

#[derive(Debug)] // So we can inspect the state in a minute
                 // derive: Debug
                 //   - Debug 트레잇을 사용하면 디버그 포매팅을 사용할 수 있습니다. {:?}
                 //   - 디버깅 목적으로서 출력 가능
                 //   - assert_eq 매크로 사용 가능

// derive: PartialEq, Eq
//   - PartialEq 트레잇을 사용하면 ==, != 연산자를 사용할 수 있습니다.
//   - eq 메소드를 구현해주는 역할을 합니다.
//   - 서로 다른 필드가 하나라도 있다면 동일하지 않다고 판단함.
//   - Eq는 NaN과 같은 특수한 값에 대해서 동일하지 않다고 판단함.
//   - Eq가 필수적인 예는 HashMap<K, V>의 키 값으로 사용될 경우.

// derive: PartialOrd, Ord
//   - PartialOrd 트레잇을 사용하면 <, <=, >, >= 연산자를 사용할 수 있습니다.
//   - patrial_cmp 메소드를 구현해주는 역할을 합니다.
//   - 열거형의 경우 먼저 선언한 variant가 더 작은 값으로 판단됩니다.
//   - Ord는 NaN과 같은 특수한 값에 대해서 정렬할 수 없다고 판단됩니다.
//   - Ord가 필수적인 예는 BTreeMap<T>의 키 값으로 사용될 경우.

// derive: Clone
//   - Clone 트레잇을 사용하면 clone 메소드를 사용할 수 있습니다.
//   - clone 메소드를 구현해주는 역할을 합니다.
//   - 값의 깊은 복사를 생성할 수 있게 해줍니다.
//   - to_vec 메서드를 호출할 경우 Clone 트레잇이 구현되어 있어야 합니다.

// derive: Copy
//   - Copy 트레잇을 사용하면 값이 스택에 저장되는 경우 복사가 발생합니다.
//   - Copy 트레잇은 Clone을 구현하고 있어야 합니다.
//   - 어떠한 메소드도 정의하지 않음으로써 프로그래머가 메소드를 오버로딩해 임의의 코드를 실행시키는 경우를 방지합니다.
//   - 따라서 값의 복사가 느려질 것을 염려하지 않아도 됩니다.

// derive: Default
//   - Default 트레잇을 사용하면 기본값을 생성할 수 있습니다.
//   - ..Default::default() 메소드를 구현해주는 역할을 합니다.
//   - Option<T> 인스턴스에 unwrap_or_default 메소드를 호출할 경우 Default 트레잇이 구현되어 있어야 합니다.
//   - Option<T>가 None일 경우 T::Default::default()를 반환합니다.
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Querter varient 내에 UsState enum을 넣을 수 있다.
}

fn values_in_cents(coin: Coin2) -> u32 {
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    value_in_cents(Coin1::Penny);
    values_in_cents(Coin2::Quarter(UsState::Alaska));
}
