fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    //모든 키는 같은 타입이어야 하고, 모든 값도 같은 타입이여야 합니다.

    let teams = vec!["Blue","Yellow"];
    let initial_scores = vec![10,50];

    let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();


    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name,field_value);
    // println!("{}",field_name);

    let team_name = "Blue";

    let score = scores.get(&team_name);
    // get은 Option을 반환한다. 

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.insert(&"Blue", &30);


    println!("{:?}", scores);

    let  a = String::from("Blue");
    let binding = &&a[..]; // lifetime 연장. 표현식 값은 표현식이 끝날 때 소멸 
    // String의 전체 슬라이스로 &&str로 참조 가능 
    scores.entry(binding).or_insert(&70); 
    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        // or_insert는 &mut v를 반환한다.
        *count += 1;
        // 역참조 해서 v에 카운터를 증가시켜야한다. 

    }


    //기본적으로, HashMap은 서비스 거부 공격(Denial of Service(DoS) attack)에
    // 저항 기능을 제공할 수 있는 암호학적으로 보안되는 해쉬 함수를 사용합니다. 이
}
