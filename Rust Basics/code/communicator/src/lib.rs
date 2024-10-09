pub mod client;
// 여기서는 여전히 client 모듈을 선언하고 있지만,
// 코드 블록을 세미콜론으로 대체함으로써, 
//우리는 러스트에게 client 모듈의 스코프 내에 
//정의된 코드를 다른 위치에서 찾으라고 말하는 것입니다. 
//달리 말하면, mod client;라는 라인의 뜻은 이렇습니다:
//mod client {
//  contents of client.rs
//}
pub mod network;
// namespace에 있으므로, 이를 network::space()로 호출해야한다


pub fn add(left: usize, right: usize) -> usize {
    left + right
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
