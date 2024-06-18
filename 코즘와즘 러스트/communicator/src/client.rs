//이미 src/lib.rs 안에다 client 모듈을 mod를 이용하여 선언을 했기 때문에,  
//파일 안에는 mod 선언이 필요없다는 점을 기억하세요. 
//이 파일은 단지 client 모듈의 내용물만 제공할 뿐입니다.
// 만일 mod client를 여기에 또 집어넣는다면, 
//이는 client 모듈 내에 서브모듈 client를 만들게 됩니다!
pub fn connect() {
}