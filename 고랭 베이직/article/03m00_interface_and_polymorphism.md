# 03m00. Interface를 통해 다형성 구현하기

## 목차 
0. Interface를 통해 다형성 구현하기
1. 기본 설정하기
2. 프로그램 구현 요구사항
3. Interface를 통해 다형성을 구현한 프로그램 실행화면 예시

## 0. Interface를 통해 다형성 구현하기
이번 실습에서는 `Speaker` 인터페이스를 활용하여 다양한 타입의 객체가 동일한 메서드를 구현하도록 하고, 이를 통해 다형성을 구현해보자. 

## 1. 기본 설정하기
기본 설정은 다음과 같다:
```sh
# interface_polymorphism 디렉토리 생성
$ mkdir interface_polymorphism && cd interface_polymorphism

# interface_polymorphism go module 생성 
$ go mod init interface_polymorphism
```

## 2. 프로그램 구현 요구사항
다음은 실습을 위한 boilerpalte 코드이다.
```go
package main

type Speaker interface {
	Speak() string
}

func main() {
	
}
```

프로그램 구현 요구 사항은 다음과 같다:
1. Person 타입을 정의하고, 이 타입이 Speaker 인터페이스를 구현하도록 한다.
2. Dog 타입을 정의하고, 이 타입이 Speaker 인터페이스를 구현하도록 한다.
3. Greet 함수를 정의한다. 이 함수는 Speaker 인터페이스를 인수로 받아서 Speak 메서드를 호출하고 결과를 출력한다.
4. main 함수에서 Person과 Dog 타입의 인스턴스를 생성하고, Greet 함수에 전달하여 각각의 인사를 출력한다.
> 구현된 실습 코드 확인하기: [03_interface_polymorphism](../code/03_interface_polymorphism/)


## 3. Interface를 통해 다형성을 구현한 프로그램 실행화면 예시
프로그램을 실행하여 출력된 예시 결과는 다음과 같다:
<div style="text-align: center;">
   <img src="../assets/03_interface_result_example.png" alt="03_interface_result_example" width="600"/>
</div>

