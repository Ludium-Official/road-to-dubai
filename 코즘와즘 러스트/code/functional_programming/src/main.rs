fn main() {

    let v = vec![1,2,3,4];
    let v = add(v,32);

    let numbers = vec![1, 2, 3, 4, 5];

    // map을 사용하여 각 요소를 제곱
    let squares: Vec<i32> = numbers.into_iter().map(|x| x * x).collect();

    println!("{:?}", squares); // [1, 4, 9, 16, 25]


    let numbers = vec![1, 2, 3, 4, 5];

    // filter를 사용하여 짝수만 선택
    let evens: Vec<i32> = numbers.into_iter().filter(|&x| x % 2 == 0).collect();

    println!("{:?}", evens); // [2, 4]

    let numbers = vec![1, 2, 3, 4, 5];

    // fold를 사용하여 벡터의 모든 요소를 합산
    let sum = numbers.into_iter().fold(0, |acc, x| acc + x);

    println!("Sum: {}", sum); // Sum: 15


    let numbers = vec![1, 2, 3, 4, 5];
    let vec = numbers.into_iter().fold(vec![],|mut acc:Vec<i32>,x|
    {
        let ac = &mut acc;
        ac.push(x);
        acc
    });

    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    let vec = numbers.into_iter().fold(vec![], |mut acc: Vec<i32>, x| -> Vec<i32> {
        let ac = &mut acc;
        if x%2 == 0 {ac.push(x);}
        acc
    });



}

#[cfg(test)]
mod tests {
    #[test]
    fn test_fold_function() {
        let numbers = vec![1, 2, 3, 4, 5];
        let vec = numbers.into_iter().fold(vec![], |mut acc: Vec<i32>, x| {
            let ac = &mut acc;
            ac.push(x);
            acc
        });

        assert_eq!(vec, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_fold_function_with_even_numbers() {
        let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
        let vec = numbers.into_iter().fold(vec![], |mut acc: Vec<i32>, x| -> Vec<i32> {
            let ac = &mut acc;
            if x % 2 == 0 {
                ac.push(x);
            }
            acc
        });

        assert_eq!(vec, vec![2, 4]);
    }
}

fn add(v: Vec<i32>, target:i32 ) -> Vec<i32>{
    let mut v1 = v.clone(); 
    // 소유권 시스템으로 더 빠른 ... 
    v1.push(target);
    v1 

    // 순수함수, + CoW 
}

fn withVecCopy<F,T>(v: Vec<T>, modify: F ) -> Vec<T> 
where F: FnOnce(Vec<T>)->Vec<T>, T: Clone
{
    let mut copy = v.clone();
    modify(copy)
}

fn pushVec<T>(v: Vec<T>, param:T) -> Vec<T>
where T: Clone
{
    withVecCopy(v, |mut copy| -> Vec<T> {
        copy.push(param);
        copy //closure을 통한 환경 캡쳐 
    })
}

fn droplast<T>(v: Vec<T>) -> Vec<T>
where T: Clone
{
    withVecCopy(v, |mut copy| -> Vec<T> {
        copy.pop();
        copy//closure을 통한 환경 캡쳐
    })
}

fn dropfirst<T>(v: Vec<T>) -> Vec<T>
where T: Clone
{
    withVecCopy(v, |mut copy| -> Vec<T> {
        copy.remove(0);
        copy//closure을 통한 환경 캡쳐
    })
}

fn OkorFail<F1,F2,T, E> (target: F1, handlingfunc:F2) -> T
where F1: Fn()->Result<T,E>, F2: FnOnce(E)->T
{
    target().unwrap_or_else(|e|{
        handlingfunc(e)
    })
}

fn wrapLogging<F>(target: F) -> impl FnOnce()
where  F: FnOnce() //Fn으로 한다면 사라져가는 target을 스코프 넘어서까지 가지고 있을 수 없으므로 오류가 발생한다. 
{
    || {
        println!("Logging Start");
        target();
        println!("Logging End");
    }
}

