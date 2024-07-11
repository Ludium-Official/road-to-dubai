fn main() {
    let v: Vec<i32> = Vec::new();
    
    let v = vec![1,2,3];
    // 컴파일러가 알아서 T 타입을 추론한다 


    let mut v = Vec::new();
    v.push(1);
    v.push(2);

    // 우리가 집어넣는 숫자는 모두 i32 타입이며, 러스트는 데이터로부터 이 타입을 추론하므로, 우리는 Vec<i32> 명시를 붙일 필요가 없습니다.

    {
        let v = vec![1,2,3,4];

    } // <- v가 스코프 밖으로 벗어났고, 여기서 해제됩니다


    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    // 인덱스 밖이면 치명적 에러 
    let third: Option<&i32> = v.get(2);
    // Some() , None 류의 처리를 수행한다 


    let mut v = vec![1,2,3,4,5];

    let first = &v[0];
    // 빌림 검사기 -> 벡터는 만일 resize 연산이 필요하면
    // 새로운 공간에 저장해여하는데, 그러면 유효하지 않은 참조가 될 수 있다.

    // v.push(6); -> 이게 그래서 동작을 안 함 

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);  
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }


    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}