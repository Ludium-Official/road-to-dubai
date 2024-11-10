# 01m11. Closure 기능 사용해보기

## 목차
0. Closure 기능 사용해보기
1. 기본 설정하기
2. 코드 작성하기
   1. outer 함수 정의하기
   2. main 함수 작성하기
3. Closure 프로그램 실행화면 제출 예시

## 0. Closure 기능 사용해보기
실습을 통해 closure 기능을 직접 사용해보도록 하자. 

## 1. 기본 설정하기
기본 설정은 다음과 같다:
```sh
# closure 디렉토리 생성
$ mkdir closure && cd closure

# closure go module 생성 
$ go mod init closure
```

## 2. 코드 작성하기
### 1. outer 함수 정의하기
outer 함수는 count 변수를 가지고 있으며, 내부에 정의된 익명 함수가 이 count 변수에 접근할 수 있다. increment 변수에 outer 함수를 할당하면 increment는 count 변수를 "기억"하고 호출될 때마다 증가된 값을 반환한다.
```go
func outer() func() int {
    count := 0
    return func() int {
        count++
        return count
    }
}
```

### 2. main 함수 작성하기
main 함수에서 outer 함수를 호출하여 increment 변수에 할당한다. 그 다음 increment 함수를 호출할 때마다 count 변수가 증가된 값을 반환하는지 확인해보도록 하자. 
```go
func main() {
    increment := outer()

    fmt.Println(increment()) // 1
    fmt.Println(increment()) // 2
    fmt.Println(increment()) // 3
}
```
> 전체 실습 코드 확인하기: [01_closure](../code/01_closure/)

## 3. Closure 프로그램 실행화면 제출 예시
프로그램을 실행하여 출력된 결과는 다음과 같다:
<div style="text-align: center;">
   <img src="../assets/01_basic_closure_result_example.png" alt="01_basic_closure_result_example" width="600"/>
</div>
