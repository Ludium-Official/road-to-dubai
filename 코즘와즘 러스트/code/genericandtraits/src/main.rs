use core::str;
use std::io::Write;
fn main() {
    println!("Hello, world!");

    let mut buf = vec![];
    
    let writer : &mut dyn Write = &mut buf;

    buf.write_all(b"hello");



    /*
    동적 디스패치
    -> 가상 테이블을 호출해서
    런타임에 처리
    가상 테이블은 
    어떤 메서드를 호출할지에 대한 참조들을 다 담고 있음
    그래서 시간은 오래 걸림 하지만 메모리는 덜 잡아먹을 수 있다  */


    /*
    제네릭처럼 정적 디스패치를 한다면
    시간이 덜 걸림
    컴파일러가 타입에 대해서 어떤 메서드를 호출할지 미리 정하기 떄문이다
    대신에 함수객체가 계속 생성되기 때문에 memory가 넘쳐날 수 있다!  
    결론은 그냥 제네릭 쓰기 */



}


trait Vegatable {
    fn delicious(&self);

}

struct Salad <T: Vegatable> {
    veg: Vec<T>
} // 이런 식으로 하면 하나의 채소밖에 못담아

struct Salads {
    veg: Vec<Box<dyn Vegatable>> //항상 dyn이랑 같이 다닌다
}
// 이런 식으로하면 여러가지 채소를 자유롭게 담아서 만들 수 있다! 