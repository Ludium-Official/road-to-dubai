# 04m02. Slice로 Queue 구현하기

## 목차 
0. Slice로 Queue 구현하기
1. 기본 설정하기
2. 프로그램 구현 요구사항
3. Queue 실행화면 제출 예시

## 0. Slice로 Queue 구현하기
Go에 built-in 되어있는 Slice를 사용하여 Queue를 구현해보도록 하자. 

## 1. 기본 설정하기
기본 설정은 다음과 같다:
```sh
# queue 디렉토리 생성
$ mkdir queue && cd queue

# queue go module 생성 
$ go mod init queue
```

## 2. 프로그램 구현 요구사항
프로그램 구현 요구사항은 다음과 같다:
1. queue에 int 타입의 데이터를 추가하는 `Enqueue` 메서드를 구현한다.
2. queue에 가장 먼저 저장된 int 타입의 데이터를 제거하는 `Dequeue` 메서드를 구현한다. queue가 비었을 경우에는 -1을 반환한다. 

<div style="text-align: center;">
   <img src="../assets/04_queue.png" alt="04_queue" width="450"/>
</div>

> 구현된 실습 코드 확인하기: [04_queue](../code/04_queue/)


## 3. Queue 실행화면 제출 예시
프로그램을 실행하여 출력된 결과는 다음과 같다:
<div style="text-align: center;">
   <img src="../assets/04_data_structure_queue_result_example.png" alt="04_data_structure_queue_result_example" width="600"/>
</div>
