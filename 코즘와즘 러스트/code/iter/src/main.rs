fn main() {
    let v1 = vec![1,2,3];
    let v2 = v1.iter().map(|x| x+1);
    //map은 반복자를 다시 '생성'하는 어댑터이다! 
    let v3: Vec<u32> = v2.collect();
    //collect는 타입을 알아야한다 Vector<T>이므로! 

    assert_eq!(vec![2,3,4],v3);
}
