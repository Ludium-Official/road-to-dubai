# 02m02. switch문 실습 - CLI(Command Line Interface) 프로그램 구현하기

## 목차 
0. switch문을 활용하여 CLI(Command Line Interface) 프로그램 구현하기
1. 기본 설정하기
2. 프로그램 구현 요구사항
3. CLI 프로그램 실행화면 제출 예시

## 0. switch문을 활용하여 CLI(Command Line Interface) 프로그램 구현하기
이번 실습에서는 [반복문 실습에서 작성한 CLI 프로그램](./02m01_iteration_io_handler.md)을 switch문을 활용하여 작성해보도록 하자. switch 문을 사용하면 더 가독성있는 코드를 작성할 수 있다. 

## 1. 기본 설정하기
기본 설정은 다음과 같다:
```sh
# switch_simple_cli 디렉토리 생성
$ mkdir switch_simple_cli && cd switch_simple_cli

# switch_simple_cli go module 생성 
$ go mod init switch_simple_cli
```

## 2. 코드 작성하기
프로그램 요구 사항은 다음과 같다:
1. 사용자가 입력한 명령어에 따라 다양한 동작을 수행한다.
2. 명령어는 다음과 같다:
   1. "exit": 프로그램을 종료한다.
   2. "hello": "Hello, world!"를 출력한다.
   3. "even": 0부터 10까지의 짝수를 출력한다.
   4. "odd": 1부터 10까지의 홀수를 출력한다.
   5. "help": 사용 가능한 명령어 목록을 출력한다.
   6. 그 외의 입력에 대해서는 "Unknown command"를 출력한다.
> 구현된 실습 코드 확인하기: [02_switch_simple_cli](../code/02_switch_simple_cli/)

## 3. CLI 프로그램 실행화면 제출 예시
프로그램을 실행하여 출력된 결과는 다음과 같다:
<div style="text-align: center;">
   <img src="../assets/02_control_structure_switch_simple_cli_result_example.png" alt="02_control_structure_switch_simple_cli_result_example" width="600"/>
</div>

