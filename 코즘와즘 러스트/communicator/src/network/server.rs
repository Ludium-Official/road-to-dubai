pub fn connect() {
}
/*
이런 식으로 하지 않으면
Rust 컴파일러는 Server가 network의 서브 모듈인지 알 수 없음
mod.rs로 메인 모듈 파고, Server.rs로 서브모듈 따로 파야한다
*/