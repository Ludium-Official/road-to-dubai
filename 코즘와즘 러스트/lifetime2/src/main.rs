#[derive(Debug)]
struct Book<'a> {
    title: &'a str,
    author: &'a str,
}


fn main() {
    let title = String::from("The Rust Programming Language");
    let author = String::from("Steve Klabnik and Carol Nichols");

    let book;
    {
        let short_title = &title[..4];
        book = Book {
            title: short_title,
            author: &author,
        };
    } // short_title의 라이프타임이 여기서 끝남

    println!("{} by {}", book.title, book.author); // 유효하지 않은 참조 아님! 
                                                    // 데이터 자체는 title에 있으므로 유효함


    // 그래서 재귀적으로 되어있어서
    // 명시적으로 라이프타임을 달지 않으면 그런 검사를 못하는데,
    // 명시적으로 달아준다면 로직이 생겨서 검사할 수 있다 ! 

    /* 이제 문제가 되는 예시를 알아보자!  */

    let book;

    {

        let title = String::from("Hello");
        let title2 = String::from("World");

        
        book = Book {
            title: &title,
            author: &title2,
        };
        
    }
    //println!("{:?}", book);
}


/*
    결론 : 라이프타임은 함수의 결과값의 라이프타임을 알 수 없을 떄 
    그 때 명시를 하는 것. 명시적 라이프타임을 바꾸거나 그러지 않는다
    그러니까 함수의 결과값이 참조값일 때, 엘리시온 규칙을 따르지 않을 때 명시

    '일반적으로' 컴파일러는 함수의 결과 값이 참조값일 때 다음 규칙을 통해 라이프 타임 추론
    1. 각각의 파라미터는 독립적인 라이프타임을 가진다
    2. 파라미터가 1개일 때 함수는 그 파라미터의 라이프타임을 라이프타임으로 가진다
    3. 파라미터가 여러개고, 그 중 하나가 &self나 &mut self일 때 함수는
        그 self의 라이프타임을 라이프타임으로 가진다 

    이 중 없는 내용이면 명시를 해줘야 컴파일러가 추론을 함 

 */ 