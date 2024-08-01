# 고랭 베이직 모듈 

## 모듈 구성
| # | Topic | Type | Description | Link |
|---|-------|------|-------------|------|
| 0 | golang 소개 및 시작하기 | Article | 고 언어 탄생 배경, 특징에 대해 설명하고 설치하는 방법에 대해서 알아본다 | [Introduction](./article/00_introduction.md) |
| 1 | golang 기초 | Article | 고 언어의 기본적인 데이터 타입(Numerics, String, Booleans)과 변수 선언 및 초기화 방법을 설명한다. | [Basic](./article/01_basic.md) |
| 1-mission-0 | Integer | Mission | 미션을 통해 Integer 타입 이해해본다 | [Integer](./article/01m00_numeric_integer.md) |
| 1-mission-1 | Float & Complex 사용해보기 | Mission | 미션을 통해 Float & Complex 타입 이해해본다 | [Float & Complex](./article/01m01_numeric_float_and_complex.md.md) |
| 1-mission-2 | String 사용해보기 | Mission | 미션을 통해 String 타입 이해해본다 | [String](./article/01m02_string.md) |
| 1-mission-3 | String Formatting 사용해보기 | Mission | 미션을 통해 String Formmating 사용에 익숙해지기 | [String Format](./article/01m03_string_formatting.md) |
| 1-mission-4 | Boolean 사용해보기 | Mission | 미션을 통해 Boolean 타입 이해해본다 | [Boolean](./article/01m04_boolean.md) |
| 1-mission-5 | Function 사용해보기 | Mission | 미션을 통해 Function 사용에 익숙해진다 | [Function](./article/01m05_function.md) |
| 1-mission-6 | Function - Swap 함수 구현하기 | Mission | 미션을 통해 Function 호출 시 인수의 복사본이 만들어진다는 점 이해해본다 | [Swap Function](./article/01m06_function_swap.md) |
| 1-mission-7 | Struct 사용해보기 | Mission | 미션을 통해 Struct 사용에 익숙해진다 | [Struct](./article/01m07_struct.md) |
| 1-mission-8 | Method 사용해보기 | Mission | 미션을 통해 Method 사용에 익숙해진다 | [Method](./article/01m08_method.md.md) |
| 1-mission-9 | Pointer 사용해보기 | Mission | 미션을 통해 Pointer 이해해본다| [Pointer](./article/01m09_pointer.md) |
| 1-mission-10 | Pointer를 활용하여 Swap 함수 구현하기 | Mission | 미션을 통해 Pointer 메모리 주소 참조에 대해 이해해본다 | [Swap Pointer Function](./article/01m10_function_swap_using_pointer.md) |
| 1-mission-11 | Closure 기능 사용해보기 | Mission | 미션을 통해 Closure 기능 이해해본다 | [Closure](./article/01m11_closure.md) |
| 2 | 제어 구조(조건문, 반복문, switch문) | Article | 고 언어의 조건문(if, else), 반복문(for), 그리고 switch문과 같은 제어 구조를 통해 프로그램의 흐름을 제어하는 방법을 설명한다 | [Control Structure](./article/02_control_structure.md) |
| 2-mission-0 | 숫자 판별 프로그램 구현하기 | Mission | 조건문을 사용하여 숫자를 판별하는 프로그램을 직접 구현해본다 | [Number Decision Program](./article/02m00_condition_number_decision.md) |
| 2-mission-1 | Simpe CLI(Command Line Interface) 프로그램1 | Mission | 조건문과 반복문을 모두 활용하여 간단한 CLI 프로그램을 직접 구현해본다 | [Simple CLI1](./article/02m01_iteration_simple_cli.md) |
| 2-mission-2 | switch문을 활용하여 간단한 CLI 프로그램 구현하기 | Mission | switch문을 활용하여 CLI(Command Line Interface) 프로그램을 직접 구현해본다 | [Simple CLI2](./article/02m02_switch_simple_cli.md) |
| 3 | 인티페이스와 다형성 | Article  | 인터페이스의 개념과 이를 통한 다형성 개념에 대해서 알아본다  | [Interface](./article/03_interface.md)
| 3-mission-0 | Interface를 통해 다형성 구현하기 | 인터페이스를 활용하여 다양한 타입의 객체가 동일한 메서드를 구현해보며 다형성을 이해해본다 | Numerics | [Array](./article/03m00_interface_and_polymorphism.md) |
| 4 | Data Structure | Article & Mission | Go 언어의 주요 데이터 구조에 대해 다룬다. 배열(Array)과 슬라이스(Slice), 맵(Map), 큐(Queue), 스택(Stack), 트리(Tree) 등을 통해 데이터를 효율적으로 관리하고 사용할 수 있는 방법을 설명한다. 더 나아가 Cosmos-SDK의 상태 저장에 사용되는 IAVL 트리를 이해해본다. | [Data Structure](./article/04_data_structure.md) |
| 4-mission-0 | Array 사용해보기 | Mission | 미션을 통해 Array을 직접 선언하고 초기화해보면서 이해해본다 | [Array](./article/04m00_array.md) |
| 4-mission-1 | Slice 사용해보기 | Mission | 미션을 통해 Slice을 직접 선언하고 초기화해보면서 이해해본다 | [Slice](./article/04m01_slice.md) |
| 4-mission-2 | Slice로 Queue 구현하기 | Mission | 미션을 통해 Slice로 Queue을 직접 구현해보면서 이해해본다 | [Queue](./article/04m02_queue.md) |
| 4-mission-3 | Slice로 Stack 구현하기 | Mission | 미션을 통해 Slice로 Stack을 직접 구현해보면서 이해해본다 | [Stack](./article/04m03_stack.md) |
| 4-mission-4 | Map 사용해보기 | Mission | 미션을 통해 Map을 직접 사용해보면서 이해해본다 | [Map](./article/04m04_map.md) |
| 4-mission-5 | AVL Tree 사용해보기 | Mission | 미션을 통해 이미 구현된 AVL 트리의 일부 주요 함수를 이해하고 간단하게 트리 구조를 출력하는 함수를 구현해본다 | [AVL Tree](./article/04m05_avl_tree.md) |
| 4-mission-6 | Cosmos SDK IAVL 사용해보기 | Mission | 미션을 통해 Cosmos SDK IAVL의 기능을 직접 사용해본다 | [Cosmos SDK IAVL](./article/04m06_cosmos_sdk_iavl.md) |
| 5 | Concurreny(Opt) | Article | Go 언어의 동시성 프로그래밍에 대한 기초에 대해 이해해본다 | [Concurrency](./article/05_concurrency.md) |
| 5-mission-0 | 고루틴(goroutine) 사용해보기 | Mission | 미션을 통해 고루틴을 직접 사용해보며 이해해본다 | [Goroutine](./article/05m00_goroutine.md) |
| 5-mission-1 | 송신 전용 및 수신 전용 channel 만들어보기 | Mission | 미션을 통해 송수신 채널을 구현해보며 이해해본다 | [Send/Recv Channel](./article/05m01_channel.md) |
| 5-mission-2 | channel을 이용한 동시 작업 패턴 사용해보기 | Mission | 미션을 통해 채널 작업 패턴에 대해서 이해해본다 | [Worker Pattern Channel](./article/05m02_chaneel_worker.md) |
| 5-mission-3 | select문 사용해보기 | Mission | 미션을 통해 select문 사용법에 대해서 알아본다 | [Select문](./article/05m03_select.md) |
| 5-mission-4 | sync 패키지 사용해보기 - WaitGroup | Mission | 미션을 통해 sync 패키지의 WaitGroup 기능에 대해 알아본다 | [Sync - WaitGroup](./article/05m04_sync_waitgroup.md) |
| 5-mission-5 | sync 패키지 사용해보기 - Mutex | Mission | 미션을 통해 sync 패키지의 Mutex 기능에 대해 알아본다 | [Sync - Mutex](./article/05m05_sync_mutex.md) | 
| 5-mission-6 | 클로저(closure)를 활용하여 고루틴 사용해보기 | Mission | 미션을 통해 고루틴이 상태를 유지하거나 공유 상태를 안전하게 변경할 수 있는 방법에 대해 알아본다 | [Closure Goroutine](./article/05m06_closure_goroutine.md) |
  

## 제안 및 추가 
- 고량 베이직 교육 모듈은 오픈 소스 컨트리뷰션을 통해 지속적으로 자료를 보완, 발전시킨다.
- 현존하는 모듈에 제안을 원하는 빌더는 Issue를 통해 제안 내용을 작성하거나 리포를 포킹해서 개선된 내용을 Pull Request로 바로 요청할 수도 있다.
- 제안, 요청된 내용은 루디움에서 검토 이후 적절성을 판단하여 자료를 업데이트 한다.
