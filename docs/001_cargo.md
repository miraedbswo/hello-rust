# Cargo: 새 프로젝트 생성

```shell
$ cargo new myproject1
$ cd myproject1
```
cargo new {프로젝트명} 명령을 사용하면 프로젝트명에 해당하는 폴더가 생성됩니다.  

그 안에 Cargo.toml 파일과 src/main.rs 파일, local git repository가 생성됩니다.

```shell
project1
├── .git
│   ├── ...{git_repository_files}
├── .gitignore
├── Cargo.toml
└── src
    └── main.rs
```

## Cargo.toml과 dependency 추가
Cargo.toml 파일은 Rust 프로젝트의 기본 정보와 종속 패키지들의 목록을 관리하는 파일이다.
```shell
$ cat Cargo.tml
[package]
# project 이름
name = "myproject1"

# project 버전
version = "0.1.0"   

# 컴파일러 edition 설정. 
# 만약 edition이 지정되지 않으면 디폴트로 2015 edition이 사용된다.
edition = "2021"    

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```
필요한 디펜던시는 `cargo add package_name`을 이용해 설치할 수 있습니다.  
패키지의 feature를 입력하기 위해서는 `--features`를 이용하면 됩니다.

```shell
$ cargo add tokio --features full
```

in Cargo.toml:
```toml
[dependencies]
tokio = { version = "1.11.0", features = ["full"] }
```

--------
<br></br>
특정 버전의 패키지를 설치하기 위해서는 패키지 이름 뒤에 @version을 붙여줍니다.

```shell
$ cargo add rocket@0.5.0-rc.1
```
in Cargo.toml:
```toml
[dependencies]
rocket = "0.5.0-rc.1"
```
<br></br>

# Cargo: 빌드와 실행
Cargo 프로젝트에서 소스코드는 항상 src 폴더 내에 위치해야 합니다.
Cargo 빌드를 하면 실행파일은 target 폴더 안에 만들어집니다.
+ 항상 main.rs 파일을 먼저 실행합니다.

```shell
$ cargo build
   Compiling hello-rust v0.1.0 (/Users/yunjae/rust/hello-rust)
    Finished dev [unoptimized + debuginfo] target(s) in 0.34s
$ ./target/debug/hello-rust
Hello Rust!
```
  
또는 `cargo run` 명령을 통해 빌드 & 실행까지 할 수 있다.
```shell
$ cargo run
Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/hello-rust`
Hello Rust!
```

### 알쓸신잡
+ `cargo check` 명령을 통해 실행 파일 (target)을 만들지 않고 컴파일이 되는지만 검증할 수 있다.
+ `cargo update` 명령을 통해 dependency 패키지들의 버전을 업데이트 할 수 있다.
  - Cargo.toml 파일은 기본적으로 버전을 `rand = "0.8.3"`처럼 표기함
  - 이는 버전이 0.8.3 이상이거나 0.9.0 미만인 버전을 나타낸다.
    ```shell
    $ cargo build
      Compiling rand v0.8.5
    ```
  - 버전을 "0.8.3"으로 지정했음에도 0.8.5 버전이 설치되는 것을 볼 수 있다.
  - 이미 로컬에 0.8.3 버전이 깔려있을 때 새로운 버전의 패키지를 다운받기 위해 사용하는 명령임을 알 수 있다.
  - 업데이트 한다고 해서 toml 파일에 명시된 버전을 올려주진 않는다.
+ `cargo doc --open` 기능을 통해 프로젝트에 정의된 함수에 대한 도큐먼트를 생성할 수 있다.
