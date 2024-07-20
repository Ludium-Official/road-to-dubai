pub trait Draw {
    fn draw(&self);
}

pub struct Screen<T: Draw> {
    pub components: Vec<T>,
    //동일 유형의 콜렉션만 사용한다면 제네릭과 특성 범위를 사용하는 것이 바람직한데,
    // 왜냐하면 그 정의들은 구체 타입을 사용하기 위해 
    //컴파일 타임에 단형성화 (monomorphize) 되기 때문입니다.
}

impl<T> Screen<T>
    where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
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
