fn largest<T: PartialOrd + Copy >(li: &[T]) -> T {
    let mut largest = li[0];
    for &item in li.iter(){
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T,U> {
    x: T,
    y: U,
} // T를 가지고 있는데 , 둘 다 같은 타입이어야한다 
struct Point2<T,U> {
    x: T,
    y: U,
} //  T,U는 다른 타입 가용 ! 

impl<T,U> Point2<T,U> {
    fn x(&self) -> &T{
        &self.x 
    }

    fn y(&self) -> &U{
        &self.y
    }
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x, // 스택에서 관리될 수도, 힙에서 관리될 수도 있으므로 ... 
                //
            y: other.y,
        }
    }
}


fn main() {
    let numbers = vec![34, 50, 25, 100, 65];

    let result = largest(&numbers);
    println!("The largest number is {}", result);

    let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&numbers);
    println!("The largest number is {}", result);

    let interger1 = Point{x:1, y:2};
    let interger2 = Point2{x:1.0 , y:4};


    let s = vec![String::from("hell"),String::from("lo"),String::from("world")];

    let k = &s;

    let m = &k[0];

}
