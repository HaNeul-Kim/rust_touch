# rust_touch

rust 로 만든 file last modified time 변경. 다음의 이유로 rust 로 개발해봤음

* java, scala 는 실행이 오래 걸림
* bash 는 milliseconds 설정이 불가능

# Build

```shell
cargo clean
cargo build
```

# Usage

```shell
./target/debug/touch
```
