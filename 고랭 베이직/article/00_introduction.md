# 000. Go Introduction
> golang baisc 모듈은 Go로 작성되어있는 Cosmos-SDK를 이해하고 직접 커스텀할 수 있게하기 위한 목적으로 작성되었다. 더 나아가 CometBFT 합의 엔진과 인터체인의 핵심인 IBC에 대한 기술 이해도 목표로 한다.

## 목차
0. Go 언어 탄생 배경
   1. Go 언어의 발전 
1. Go 언어의 특징
   1. Simple is the Best
   2. More Fast, Goroutine 
2. Go 언어 설치하기
3. (실습) Hello, World 출력하기

## 0. Go 언어 탄생 배경
2009년 11월, 구글에서 처음 발표된 Go 언어는 C++의 복잡함에 실증을 느낀 세 명의 구글 엔지니어인 로버트 그리즈머, 롭 파이크, 그리고 켄 톰프슨에 의해 개발되었다. 특히, 켄 톰프슨은 튜링상을 수상한 인물로 Unix와 C 언어 제작에 기여했으며, 이는 Go 언어에 영향을 미쳤다. Go 언어는 간단함과 효율성을 중시하며, 높은 성능과 동시성을 지니고있어 대규모 시스템에 최적화되어 있다. Go 언어는 종종 Golang으로도 불린다.

### 1. Go 언어의 발전 
Go 언어는 이러한 꾸준한 업데이트를 통해 안정성과 성능을 지속적으로 개선해왔다:
- 2012년 [1.0 버전](https://go.dev/doc/go1), Go 1.0은 안정성과 호환성을 중시하여 초기 Go 사용자들이 지속적으로 사용할 수 있는 기반을 마련했다.
- 2015년 [1.5 버전](https://go.dev/doc/go1.5), Go를 실행하는 컴파일러를 C언어에서 Go로 [부트스트랩](https://ko.wikipedia.org/wiki/%EB%B6%80%ED%8A%B8%EC%8A%A4%ED%8A%B8%EB%9E%A9_(%EC%BB%B4%ED%8C%8C%EC%9D%BC%EB%9F%AC))을 이뤘다. 이를 통해 컴파일러 성능과 언어의 발전을 동시에 도모했다. 
- 2016년 [1.7 버전](https://go.dev/doc/go1.7), 성능 최적화와 툴링 개선이 주요 특징이다. 컴파일러와 런타임, 그리고 여러 패키지들에 대한 많은 수정이 이루어졌다.
- 2017년 [1.8 버전](https://go.dev/doc/go1.8), 컴파일 시간 단축과 새로운 기능 추가가 주요 변경사항이다. 더욱 빠른 컴파일 속도와 향상된 언어 기능을 통해 개발자들의 생산성을 높였다. 
- 2018년 [1.11버전](https://go.dev/doc/go1.11), 모듈 시스템이 도입되어 의존성 관리가 크게 개선되었다. 이 새로운 모듈 시스템은 패키지 관리와 빌드 프로세스를 단순화하고, 의존성 충돌 문제를 최소화했다.
- 2022년 [1.18 버전](https://go.dev/doc/go1.18), 제네릭 기능이 추가되어 코드의 유연성과 재사용성이 향상되었다. 제네릭 타입 도입을 통해 코드베이스를 더욱 간결하고 효율적으로 관리할 수 있게 되었다.
- 2023년 [1.21 버전](https://go.dev/doc/go1.21), 성능 최적화와 새로운 기능 추가가 이루어졌다. 특히, 프로파일 기반 최적화(PGO) 기능이 도입되어, 실행 성능을 극대화할 수 있었다. 
- 2024년 [1.22 버전](https://go.dev/doc/go1.22), 메모리 최적화와 새로운 표준 라이브러리 추가, 여러 인터페이스 메서드 호출의 정적 디스패치 등이 포함되었다. 또한, Go 런타임의 메모리 사용량이 최적화되어 CPU 성능이 1-3% 향상되었다.


Go는 처음에는 크게 관심을 받지 못했지만, 구글 출신 개발자들이 만든 언어라는 점과 언어 자체가 가진 장점들로 인해서 사용자가 점점 늘어났다. 컨테이너 기술이 크게 상용화되고 Docker, Kubernetes도 Go로 작성되었다는 점도 한 몫을 했다. 블록체인에서는 이더리움 메인 클라이언트인 Geth 역시 Go로 작성되었도 해당 아티클 주제인 Cosmos-SDK 또한 Go로 작성되었다. 즉 이미 Go는 시장에서 인정받고 안정성이 검증된 언어라고 볼 수 있다.

## 1. Go 언어의 특징
Go 언어의 장점은 다음과 같다: 
- 단순함: Go는 복잡한 기능을 지양하고 단순한 문법과 구조를 제공하여 코드를 쉽게 이해하고 유지보수할 수 있도록 한다.
- 명료함: 명확하고 일관된 코드 스타일을 권장하며, 코드의 가독성을 높이기 위해 간결한 문법을 사용한다.
- 안전성: 강력한 타입 시스템과 간결한 에러 처리 메커니즘을 통해 안정적인 코드를 작성할 수 있다.
- 효율성: 컴파일러와 런타임 시스템이 효율적으로 동작하여 성능을 극대화한다.
- 동시성: 고루틴(goroutine)과 채널(channel)을 통해 간단하고 효율적인 동시성 프로그래밍을 지원한다.

### 1. Simple is the Best
Go의 강점 중 하나는 구문이 단순하고 명확하다는 점이다. 잘 이뤄진 추상화는 개발자가 명확하고 유지 관리 가능한 코드를 작성하는 데 도움이 된다. Go 언어 사양은 직관적으로 설계되어 있어 배우고 사용하기가 쉽다. 25개의 키워드와 간결한 연산자 및 문장 부호만 사용함으로써 Go는 단순함과 표현력 사이의 균형을 유지한다. 이러한 디자인 철학은 개발자의 생산성을 향상시키고 깔끔하고 읽기 쉽고 효율적인 코드를 작성하는 데 도움이 된다.

#### Keywords
|             |          |           |            |        |
|-------------|----------|-----------|------------|--------|
| break       | default  | func      | interface  | select |
| case        | defer    | go        | map        | struct |
| chan        | else     | goto      | package    | switch |
| const       | fallthrough | if     | range      | type   |
| continue    | for      | import    | return     | var    |


#### 2. More Fast, Goroutine 
Goroutine(이하 고루틴)이라는 비동기 메커니즘을 통해 동시성을 구현한다. 이는 Erlang의 동시성 모델에서 영향을 받게 되었다. Erlang은 멀티 프로세스 메커니즘이지만 고루틴은 멀티 스레드 메커니즘을 따르며, OS가 아닌 자체적인 스케줄로러 관리되는 경량 스레드 구조이다. 그래서 CPU 코어 수와는 무관하게 수백, 수천 고루틴을 작성해도 스케줄러를 통해 효율적으로 관리되고 동작한다. 

각각의 고루틴은 병렬로 동작하며 메시지 채널을 통해 값을 주고 받는다. 이를 사용하면 이벤트 처리, 동시성 프로그래밍을 간단하게 구현할 수 있다. 단, 동시성 프로그래밍에서 발생할 수 있는 문제는 개발자의 책임이다. 이를 잘 다루지 못할 경우 프로그램이 실행 중 비정상 종료가 될 위험이 있다. 이는 [05_concurrency](./05_concurrency.md)에서 코드를 통해 직접 다뤄본다. 


### 단점
이렇게 까지만 보면 Go 언어는 정말 최적의 언어라고 볼 수 있다. 간단한 문법으로 누구나 쉽게 사용할 수 있으며 타입 시스템을 지는 컴파일러 언어와 멀티 쓰레드 동시성 프로그래밍으로 높은 성능을 낼 수도 있다. GC를 통해 메모리 관리도 자동으로 처리된다. 

1. 크로스 컴파일: 컴파일러를 할 때 중간언어가 존재하지 않으므로 각각의 아키텍처(x86, arm, ..)에 맞게 컴파일을 해야 한다. 만약 크로스 컴파일이 필요하다면 [goreleaser](https://goreleaser.com/)를 공부해보자.
2. 메모리 직접 관리: 개발자들마다 성향은 제각각이다. GC를 통해 자동으로 메모리 관리를 하는 것을 지향하는 사람도 있고 아닌 사람도 있다. 특히나 시스템 프로그램 개발 분야가 그런 것 같다. 시스템 프로그래밍은 C, C++, Rust와 같이 메모리를 직접 관리할 수 있는 언어를 선호한다. 실제로 [Go는 GC 때문에 시스템 프로그래밍에는 적합하지 않다는 이야기도 가끔씩 들려온다.](https://www.quora.com/Is-Go-a-systems-programming-language) 
3. 타입 시스템: 간단한 문법을 지향하는 대신 타입 시스템이 기존까지 연구된 방식을 따르지 않았다. 강력한 타입시스템은 컴파일 시점에 많은 버그와 문제점을 잡을 수 있다. 하지만, Go의 타입 시스템은 Null pointer나 안전하지 않은 타입 캐스팅 유발 등 많은 문제를 야기한다.


## 2. Go 언어 설치하기
기초 문법을 알아보기 전에 Go를 처음으로 개발하는 사람들은 기본 설정을 해야한다. Go언어 설치는 OS 환경에 따라 방법에 제각각이다. [Go 공식 다운로드 링크](https://go.dev/doc/install)를 통해 다운로드 해보자. 

### Linux Ubuntu
```sh
sudo apt install golang -y or
sudo apt-get install golang -y or
```
go 버전이 맞지 않거나, 설치가 제대로 안된다면
```sh
wget https://golang.org/dl/go1.20.linux-amd64.tar.gz
sudo tar -C /usr/local -xzf go1.20.linux-amd64.tar.gz
```
go 버전 1.20 버전을 다운로드 해서 /usr/local 위치에 압축을 풀어 설치하는 명령어이다.
### MacOS
```sh
brew install go
```
### Windows
https://go.dev/doc/install 
들어가서 다운로드 후 설치하면 된다.

설치가 끝났으면, 명령 프롬프트를 열고 다음 명령을 입력하여 Go가 제대로 설치되었는지 확인해본다:
```sh
$ go version
```

만약 설치 후에도 go command not found 에러가 난다면
## 환경변수 설정
#### PATH:
Go 실행 파일이 위치한 디렉토리를 시스템 PATH에 추가하여 터미널에서 Go 명령어(go)를 어디서나 실행할 수 있도록 한다. 

설정된 값: $PATH:$HOME/go/bin


#### GOPATH:
Go 작업 공간(모듈, 패키지, 바이너리 등을 저장하는 위치)을 지정한다.

설정된 값: $HOME/go (일반적으로 GOPATH는 $HOME/go로 설정되어야 한다).


#### GOROOT:
Go 설치 경로를 지정합니다. 일반적으로 Go의 설치 디렉토리이다.
 
설정된 값: $HOME (일반적으로는 /usr/local/go 등 Go가 설치된 디렉토리여야 한다).

##### 기본 커맨드
```sh
which go   # go 설치 위치 확인
export PATH=$PATH:$HOME/go/bin   # go 커맨드 실행
export GOPATH=$HOME/go   # go 작업공간 지정
export GOROOT=$HOME   # which go로 확인한 위치 넣기
```


### Linux Ubuntu
```sh
nano ~/.bashrc  # or nano ~/.profile   # nano 텍스트 편집기로 환경변수 파일 열기
source ~/.bashrc  # 또는 source ~/.profile
```


### MacOS
```
nano ~/.bash_profile  # 또는 nano ~/.zshrc
source ~/.bash_profile  # 또는 source ~/.zshrc
```


### Windows
##### 1. 환경 변수 설정 창 열기:
시작 메뉴에서 "환경 변수 편집"을 검색하고 "시스템 속성" 창을 연다.
"고급" 탭에서 "환경 변수" 버튼을 클릭한다.
##### 2. GOPATH 설정:
"사용자 변수"에서 "새로 만들기"를 클릭하고 GOPATH를 설정한다. 예를 들어, C:\Users\YourUsername\go.
##### 3. PATH 변수 수정:
"시스템 변수"에서 Path를 선택하고 "편집"을 클릭한다.
C:\Go\bin 및 %GOPATH%\bin을 추가한다.
##### 4. 적용:
모든 창에서 "확인"을 클릭하여 변경 사항을 적용한다.



Go를 지원하는 개발 도구는 JetBrains의 Golang가 있지만 이는 유료이고, vscode의 [Go 플러그인](https://code.visualstudio.com/docs/languages/go)을 설치하면 쉽게 Go 언어 개발을 할 수 있다. 

> Tip) 간단한 문법 사용은 [The Go Playground](https://go.dev/play/p/1u5bSZlh80h)을 통해 웹에서 쉽게 실행해볼 수 있다. 

## 3. (실습) Hello, World 출력하기
Go 언어로 프로그램을 제작하려면 패키지 선언부터 해야 한다. 각 패키지는 단일 디렉토리에 있는 하나의 Go 소스 파일(`.go`)로 구성된다. 패키지 이름이 `main`인 경우, Go는 실행 파일을 생성한다.

패키지 선언 후, 패키지 fmt를 가져온다. 패키지 이름은 패키지 파일 경로의 마지막 요소이다. 
- 예를 들어, "lib/math" 패키지를 가져오면 "math"로 사용한다.
- fmt는 입력과 출력(IO)을 구현하며 Go의 표준 라이브러리의 일부이다. 

실행은 main()이라는 함수에서 시작된다. 이 함수는 fmt 패키지에서 Go I/O 함수 Printf()를 호출하기만 하면 된다.

이제 이 프로그램을 컴파일한다. 원하는 폴더에 hello.go 파일을 만든다.
```sh
# hello 디렉토리 생성
$ mkdir hello && cd hello

# hello go module 생성 
$ go mod init hello

# "Hello, world" 출력 프로그램 작성
$ echo 'package main
import "fmt"
func main() {
    fmt.Printf("Hello, World!")
}' > hello.go
```

이제 프로그램을 빌드한다. Go는 오류만 출력하므로 문제가 발생하지 않는 한 출력이나 다른 명령 프롬프트가 표시되지 않아야 한다. 무소식이 희소식이다. 
```sh
# hello 프로그램 빌드
$ go build
```

빌드가 완료되면 module이름과 같은 실행 파일이 생성된다. 이를 실행해보면 'Hello, World!'가 정상적으로 출력될 것이다.
```sh
$ ./hello
Hello, World!
```
> 실습 코드 확인하기: [00_hello](../code/00_hello/) 


## Resources 
1. Go Docs, "The Go Programming Language Specification: Language version go1.22", Feb 6. 2024, https://go.dev/ref/spec
