fn main() {
    let a = 1;
    println!("{}", a);

    let a = "hello";
    println!("{}", a);

    // ----------
    #[allow(unused)]
    let a = 1;
    println!("{:p}", &a); // 0x7ff7b5eaf59c
    let a = 2;
    println!("{:p}", &a); // 0x7ff7b5eaf5ec != 이전 변수의 주소와 다름 (새로 할당됨)
    {
        let a = a + 1;
        println!("{}", a); // output(출력): 3
        println!("{:p}", &a); // 0x7ff7b5eaf63c != 이전 변수의 주소와 다름 (새로 할당됨)
    }
    println!("{}", a); // output(출력): 2
    println!("{:p}", &a); // 0x7ff7b5eaf5ec == 13번째 줄 변수의 주소와 같음
}

