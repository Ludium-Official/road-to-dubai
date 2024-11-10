
# 02m00. 조건문 실습 - 숫자 판별 프로그램 구현하기 

## 목차 
0. 조건문을 통해 숫자 판별 프로그램 구현하기 
1. 기본 설정하기
2. 프로그램 구현 요구사항
3. 조건문을 통해 숫자 판별 프로그램 실행화면 제출 예시

## 0. 조건문을 통해 숫자 판별 프로그램 구현하기 
조건문을 사용하여 숫자를 판별하는 프로그램을 구현해보도록 하자.

## 1. 기본 설정하기
기본 설정은 다음과 같다:
```sh
# if_condition 디렉토리 생성
$ mkdir if_condition && cd if_condition

# if_condition go module 생성 
$ go mod init if_condition
```

## 2. 프로그램 구현 요구사항
조건문을 사용하여 사용자가 입력한 숫자를 기준으로 해당 숫자의 범위를 판별하는 프로그램 코드를 작성해야 한다. 

프로그램 구현 요구 사항은 다음과 같다: 
1. 입력한 숫자가 음수이면 "The number is negative"를 출력한다. 
2. 입력한 숫자가 0이면 "The number is zero"를 출력한다.
3. 입력한 숫자가 0보다 크고 10 이하이면 "The number is between 1 and 10"를 출력한다.
4. 입력한 숫자가 10보다 크면 "The number is greater than 10"를 출력한다.
> 구현된 실습 코드 확인하기: [02_if_condition](../code/02_if_condition/)


## 3. 조건문을 통해 숫자 판별 프로그램 실행화면 제출 예시
프로그램을 실행하여 출력된 결과는 다음과 같다:
<div style="text-align: center;">
   <img src="../assets/02_control_structure_if_condition_result_example.png" alt="02_control_structure_if_condition_result_example" width="600"/>
</div>
