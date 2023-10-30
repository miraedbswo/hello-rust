# 복합 타입 (Compound Type)

복합 타입은 여러 개의 값들로 구성된 타입을 의미하는데, `Array`와 `Tuple`이 있다.

## Array
배열은 동일한 데이터 타입을 갖는 요소의 집합이며, 고정된 길이를 갖는다.

튜플은 각 요소마다 데이터 타입이 다를 수 있지만 배열은 같은 타입만 허용된다.

Rust에서 배열은 스택(`Stack`)에 할당된다.

```rust
// 3개의 정수 배열
let arr: [i32; 3] = [1, 2, 3]; 

// 배열 첫 번째 요소 출력
println!("{}", arr[0]);

// 배열 전체를 출력
println!("{:?}", arr);
```
좀 더 자세한 출력은 [002_let_mut 챕터: how to Println object in rust](https://github.com/miraedbswo/hello-rust/blob/main/docs/002_let_mut.md#tm%E2%84%B9-how-to-println-object-in-rust)에서도 정리되어 있다!

배열값을 지정할 때 `[1; 100]`과 같이 세미콜론을 사용하면 이는 [1, 1, ..., 1]와 같이 1이 100개 초기화된 배열이 된다.
```rust
let arr: [i32; 100] = [1; 100];
```

## Tuple
튜플은 여러 데이터 타입의 값을 하나로 묶은 것.
() 괄호 기호를 사용하고 각 타입이나 값들을 콤마로 분리한다.

튜플이 한 번 정의되면 새로운 요소를 추가하거나 기존 요소를 삭제할 수 없다.
```rust
let dat: (i32, char, bool) = (1, 'A', true);
let usr = ("Tom", 'b');  // (&str, char)
```

```rust
let dat: (i32, char, bool) = (1, 'A', true, true, true, true);

/* 만약 요소를 더 넣으면?

에러가 발생한다.

 mismatched types
 expected tuple `(i32, char, bool)`
   found tuple `(i32, char, bool, bool, bool, bool)`rustcClick for full compiler diagnostic
*/
```

### 튜플에서 값 읽어오기
튜플에서 값을 읽어 내기 위해서는, 첫번째 값은 {변수명}.0 으로, 두번째 값은 {변수명}.1 와 같은 방식으로 읽어 낼 수 있다.

```rust
let dat: (i32, char, bool) = (1, 'A', true);
let a = dat.0;
let b: char = dat.1;
let c: bool = dat.2;
```

### Destructuring
튜플의 값을 읽는 또 다른 방식으로, 튜플 전체를 읽어 여러 변수에 나누어 할당하는 방식이 있는데 이를 Destructuring 이라고 불린다.
Python에서 Unpacking 같은 문법이다.
```rust
let dat: (i32, char, bool) = (1, 'A', true);
let (a, b, c) = dat;
// let (a: i32, b: char, c: bool) = dat;
// 언패킹 할 때 타입 지정은 안 된다.. 위에서 해줬기 때문인 듯
```