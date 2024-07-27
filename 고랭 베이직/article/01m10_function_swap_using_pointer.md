# 01m10. Pointer를 활용하여 Swap 함수 구현하기

## 목차
0. Pointer를 활용하여 Swap 함수 구현하기
1. 기본 설정하기
2. Swap Pointer 함수 작성하기
3. Pointer를 활용한 Swap 함수 프로그램 실행화면 제출 예시


## 0. Pointer를 활용하여 Swap 함수 구현하기
[`01m06_function_swap`](./01m06_function_swap.md) 실습에서 다룬 swap 예제는 단순히 값을 교환하는 함수였다. 여기서는 포인터를 사용하여 메모리 주소에서 값을 직접 수정하여 a와 b의 값을 바꾸는 함수를 만들어보자. 연산자 `&`는 a와 b의 주소를 함수에 전달하는 데 사용된다. 

## 1. 기본 설정하기
기본 설정은 다음과 같다:
```sh
# pointer_swap 디렉토리 생성
$ mkdir pointer_swap && cd pointer_swap

# pointer_swap go module 생성 
$ go mod init pointer_swap
```

## 2. Pointer를 활용한 Swap 함수 작성하기
포인터를 사용하여 두 정수의 값을 교환하는 `swap`함수를 작성한다: 
- `swap` 함수는 두 개의 int 타입의 포인터(x, y)을 매개변수로 받아, 두 값을 교환하고 반환하지 않는다.

pointer를 활용하여 구현한 swap 함수를 main 함수에서 다음과 같이 호출할 수 있다:
```go
func main() {
    a, b := 5, 10
    fmt.Println("Before swap: a =", a, "b =", b) // Before swap: a = 5 b = 10
    swap(&a, &b)
    fmt.Println("After swap: a =", a, "b =", b) // After swap: a = 10 b = 5
}
```
> 구현된 실습 코드 확인하기: [01_pointer_swap](../code/01_pointer_swap/)

## 3. Pointer를 활용한 Swap 함수 프로그램 실행화면 제출 예시
프로그램을 실행하여 출력된 결과는 다음과 같다:
<div style="text-align: center;">
   <img src="../assets/01_basic_pointer_swap_result_example.png" alt="01_basic_pointer_swap_result_example" width="600"/>
</div>

이 실습을 통해 포인터를 사용하여 함수에서 값을 직접 수정하는 방법을 배웠다. 포인터를 사용하면 함수가 변수의 복사본이 아닌 실제 메모리 주소를 참조하므로, 함수 내에서 변수의 값을 직접 변경할 수 있다. 