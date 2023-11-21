fn main() {
    let arr: [i32; 100] = [1; 100];
    println!("{:?}", arr);
    // [1, 1, 1, 1, 1, 1, 1, 1, 1, ..., 1] 100 times

    let dat: (i32, char, bool) = (1, 'A', true);
    let usr = ("Tom", 'b');  // 문자열과 문자 타입을 갖는 튜플

    let dat: (i32, char, bool) = (1, 'A', true);
    let (a, b, c) = dat;
    // let (a: i32, b: char, c: bool) = dat;
    // 언패킹 할 때 타입 지정은 안 된다.. 위에서 해줬기 때문인 듯
}
