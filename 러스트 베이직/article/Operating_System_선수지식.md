# 비동기/병렬 프로그래밍을 위한 운영체제 기초

## 학습 목표
- 운영체제의 기본 개념과 Rust의 비동기/병렬 프로그래밍과의 관계를 이해한다.
- 프로세스와 스레드의 차이점을 파악하고 Rust에서의 활용 방법을 학습한다.
- 컨텍스트 스위칭과 스케줄링의 개념을 이해하고 Rust의 비동기 실행과의 연관성을 파악한다.
- 동기화 기법과 Rust의 동시성 프리미티브를 연관지어 이해한다.
- 메모리 관리와 가상 메모리의 개념을 학습하고 Rust의 메모리 안전성과의 관계를 이해한다.
- I/O 모델과 Rust의 비동기 I/O 구현 방식을 비교하여 학습한다.
- 실제 Rust 코드에서 운영체제 개념을 활용하는 방법을 습득한다.

## 운영체제와 Rust의 비동기/병렬 프로그래밍

<img width="802" alt="image" src="https://github.com/user-attachments/assets/3214f8f5-a746-4b38-88fb-9ec000985a37">

운영체제는 주로 컴퓨터 하드웨어를 관리하고, 응용 프로그램에 다양한 기능을 제공하는 시스템 소프트웨어이다. 운영체제에 대한 이해는, Rust의 비동기 및 병렬성을 잘 활용하고 Rust의 장점을 잘 이해하기 위해 필요하다. 위와 같은 하드웨어 자원을 관리하는 역할을 한다.

## CPU의 2가지 모드

<img width="775" alt="image" src="https://github.com/user-attachments/assets/f1d10765-05a2-497d-b967-2e0483a9165b">

CPU는 User Mode와 Kernel Mode라는 상태를 가지고 있다. 모드는 protected register의 status bit에 의해서 셋업된다. 

### User Mode (사용자 모드)
User Mode는 일반적인 응용 프로그램이 실행되는 모드라고 이해하면 된다. 해당 모드에서는 제한된 권한 때문에 하드웨어에 직접 접근할 수 없고, 시스템의 중요한 부분을 수정할 수 없다.

예시) 웹 브라우저, 워드 프로세서 등의 일반 응용 프로그램 실행, 사용자 레벨의 라이브러리 함수 호출

### Kernel Mode (커널 모드)
Kernel Mode는 운영체제 코드(커널)이 실행되는 모드라고 이해하면 된다.(Supervisor Mode, System Mode라고도 부른다)

Kernel Mode에서는 모든 하드웨어와 CPU 명령어에 접근 가능하다.(커널 프로그램 실행을  통해서!) 

예시) 메모리 관리,프로세스 스케줄링, 파일 시스템 관리

사용자 프로그램이 I/O 같은 특정 작업이 필요하다면, 모드 전환이 필요하다 (User Mode -> Kernel Mode)
이 모드 변경은 다음과 같은 상황에서 발생한다.
(1) Hardware Interrupt - 예를 들면, Timeout 인터럽트 

(2) Software interrupt (exception) - 에러 발생

(3) System call - 사용자가 원할 때 

정리 해보면, 내가 하나의 프로세스라고 가정할 때, (1)이랑 (2)는 `나와 관계 없는 프로세스에 대한 처리`이고 (3)은 나(사용자)가 원한 커널 권한 요청이다. 하지만 2,3을 너무 엄격하게 분리할 수는 없는 노릇이다. 

다음은 실제 예시이다. 
<img width="693" alt="image" src="https://github.com/user-attachments/assets/12deab84-0bf0-4992-8ed0-cad649e87f8a">

## OS가 하는 역할 

(1) User Service
다양한 유저 친화적인 서비스를 제공한다.
- 프로그램 실행
  프로그램을 메모리에 적제하고 실행한다.
- I/O Operations
- 파일 시스템
  파일 읽기, 쓰기 
- Communications
  네트워크를 통한 물리적으로 다른 시스템간, 혹은 한 컴퓨터 내의 프로세스 간 통신
- Error Detection
  에러 처리
(2) Resource Allocation
  Multiple User(예를 들면, 서버 시스템)나 Multiple task(여러 프로세스)에 대한 리소스를 할당한다. 
(3) Accounting
  유저나 리소스에 대해서 usage 등을 측정할 수 있다.
(4) Protection
  시스템에 대한 제어/접근에 대한 안전성 보장

## System Call 
Running Program(User Program)과 OS는 syscall 인터페이스를 통해 상호작용한다. 

<img width="690" alt="image" src="https://github.com/user-attachments/assets/09d82c62-9823-4369-865c-489e0ceda79f">

이런 인터페이스(추상화)를 통해서 프로그래밍 사용자단 프로그램 개발자는 OS의 기능을 자세히 모른 채로 개발 하기도 쉬워지고(사실 실제로는 syscall도 커널의 기능을 알아야하므로 프로그래밍 언어 라이브러리에서는 syscall를 래핑하여 사용자 라이브러리를 만들어 제공한다), 시스템 보안성, 이식성도 좋아진다.

다음과 같은 POSIX API가 그렇다. POSIX API는 syscall을 추상화한다. 
![image](https://github.com/user-attachments/assets/405b3ce9-2430-4b63-9a70-1ef53214d410)


## Process

프로세스는 OS가 프로그램을 관리하는 단위로 이해하면 편할 것이다. 프로세스의 정의는 다음과 같다.

<img width="406" alt="image" src="https://github.com/user-attachments/assets/3b02f5df-d7aa-4626-8842-d1104e0691ae">
즉, 실행중인 프로그램의 인스턴스라고 할 수 있다. 

프로세스는 다음 요소로 구성되어있다.
1. images
   -Code: 기계어
   -Data: 변수
   -Stack: States for function calls
   -Heap: dynamic memory

   ![image](https://github.com/user-attachments/assets/1875bc5b-b822-420a-977d-14c71bfa74c1)

  
2. Process context
   -Program Context: data registers, pc, sp ... 
   -Kernel Context: pid, gid, open files, paging tables ... 

그래서 이런 정보들 다 모아가지고, essential kernel data structure인 PCB를 설계한다. 
PCB의 element는 앞서 말한 Images, Process Context를 이용해서 설계한다. 즉, 프로세스를 관리하기 위한 추상 데이터 구조를 설계한 것이다. 다음은 실제 리눅스 커널에서 쓰이는 PCB이다. 

![image](https://github.com/user-attachments/assets/30cde532-ec60-4fdb-a69a-40f92a1ffc77)

어렵게 생각할 필요 없고, 모든 소프트웨어는 데이터 구조를 설계하고 그 데이터 구조를 잘 바꾸는 일 밖에 할 것이 없다. 운영체제도 똑같은 소프트웨어이므로, PCB라는 데이터 구조를 설계한 뒤, 그것을 통해 추상화된 프로세스들을 관리한다. 결국 Stored Program Concept에 의헤 Kernel도 Main memory에 적재되어 똑같이 움직인다. 

PCB말고도 Kernel이 운용하는 자료구조는 다음과 같이 더 있다.
- Memory data structures
  프로세스에 대한 메모리 할당, 프로세스에 대한 디스크의 할당, 가상 메모리에 대한 info .. 
- I/O data structures
  I/O device의 가능성, I/O operation의 상태, I/O transfer에 대한 source 혹은 destination이 되는 main memory address
- File data structures
  Current states of file, 파일의 디스크 상 주소 

프로세스의 생명주기 관리는 다음과 같은 다이어그램 처럼 이루어진다.
![image](https://github.com/user-attachments/assets/a88308df-af19-4b10-9221-dbe936b1c383)

이 외에도 OS는 프로세스에 대해서
- 멀티 프로그래밍 Degree
- CPU 할당
- swap Out/in
등에 대한 스케줄링을 실행한다.

### Process switching vs Mode switching
process swithcing은 mode switching에 비해 현저히 적게 일어나지만, process switching은 process context를 저장/불러오기 하는 등의 오버헤드가 발생한다. 
<img width="421" alt="image" src="https://github.com/user-attachments/assets/b8ce8d77-be06-4ffd-a4df-dd6dc6ed472d">


### Process Managing
Process는 creation, exit, 자원 공유 등의 여러가지 생로병사를 가지고 있다.
내 개인적인 생각으로는, 관리(뭐든!)를 하기 위한 가장 좋은 자료구조는 tree 구조라고 생각한다. 왜냐하면, 의존 관계가 명확하고, 
그로 인한 각각 인스턴스의 관할 하의 전체적이면서도 세부적인 관리가 가능하기 때문이다. 그래서 역사책에 중앙집권 체제의 그래프를 보면 보통 트리 구조이다.(ㅋㅋ)
![image](https://github.com/user-attachments/assets/64b2cdfc-ed58-4dc3-8f2d-6a1b4f0854ff)

잡소리 그만 하고, 프로세스는 다음과 같은 형태로 생성되고 관리된다. 즉, 부모 프로세스 및 자식 프로세스의 관계가 존재한다. 
![image](https://github.com/user-attachments/assets/320bf2e9-7e72-4de5-8f33-bf3c5a87c72b)



## Excution of OS

아까도 말했듯이, Stored Program Concept에 의하면 OS도 프로그램 중 하나이며, OS도 프로세스로 올라갈 것이다. 즉, `OS is just subject for Scheduling`.

그렇다면, OS는 누가 controll하는가?
-> OS 디자인에 따라 다르다. 

Non-Process Kernel의 경우 process로서 실행되지 않는다. 
<img width="533" alt="image" src="https://github.com/user-attachments/assets/48381f9d-1a8a-44f6-b2b4-c17a108ff615">


User Process안에 Kernel이 들어있는 경우, Switching을 Process 밖에 있는 PSF에게 스위칭을 위임한다.
<img width="550" alt="image" src="https://github.com/user-attachments/assets/43c3e9e7-fdff-4d2d-8e3e-d4b8746005bb">

이는 완전 독립된 OS Process를 가정한다. 이 경우, 또한 Process 밖에 있는 PSF에게 위임한다. 이 모델은 멀티 프로세서 환경에 적합하다고 할 수 있다. 
<img width="587" alt="image" src="https://github.com/user-attachments/assets/ca31dca8-0a28-4bb1-9def-97b0c884c1d8">

## Multithreading
- process model의 트리 위계 구조를 통해 Web server의 여러 클라이언트의 요청을 동시에 처리하기엔 부하가 너무 크다.
- Traditional process는 cpu를 동시에 하나밖에 쓸 수 없어서 멀티 프로세서 아키텍처의 장점을 충분히 활용할 수가 없었다. 물론 멀티 프로세싱을 할 수는 있었지만, single process에 multiple processor을 할당할 수가 없었다는 소리. 

따라서 Multiple Threads Model이 제기되었다.
- Thread는 image와 context를 다 가지고 있는 Process와 달리, sp, pc와 local variable, return register을 트래킹 하기 위한 Stack 정도로만 구성되어있다. 간단히 말하면 Thread = Stack + Thread Context
- Thread들끼리는 코드와 대부분의 data를 공유한다. Context도 공유한다.

Modern Process 모델은 다음과 같이 구성되어있다.
<img width="802" alt="image" src="https://github.com/user-attachments/assets/d863aef8-1efb-4259-9cc0-1c8a59b641bd">

논리적 구조로 보면, Process는 트리 형태, Thread는 병렬 관계로 관리한다. 

<img width="661" alt="image" src="https://github.com/user-attachments/assets/dd7123d3-e9f3-4de4-93a7-3919edb63bed">

Threads와 Processes의 공통점:
- 고유의 logical flow가 있다.
- 각각 다 스케줄링 된다.

차이점:
- Thread들 끼리는 코드와 데이터를 공유한다.
- Thread들은 트리 위계 구조가 없고, 경량화 되어있으므로 process보다 쉽게 생성된다.
- IPC와 다르게, Thread들 끼리는 주소 공간과 메모리를 공유하므로, kernel 개입 없이 커뮤니케이션이 가능하다.

이를 통해, 단일 프로세스를 구성하는 Thread들에 CPU가 하나씩 할당되어 단일 프로세스에 대한 병렬 처리가 가능해졌다. 

### User-Level Thread vs Kernel-Level Thread

#### User-Level Thread



