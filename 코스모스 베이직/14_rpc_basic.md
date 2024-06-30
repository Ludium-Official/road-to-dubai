# 14. RPC basic 
> cosmos-sdk, cometbft에서 사용하는 gRPC, REST, CometBFT에 대해 알아보기 전에 이를 이루는 기술의 근간에 대해 먼저 학습하는 것을 목적으로 작성되었다. 

## 목차 
0. IPC(Inter-Process Communication)
1. IPC in Client-Server Systems
	1. Socket
	2. HTTP API
	3. RPC
2. IDL(Interface Definition Language)
	1. JSON
	2. Protobuf(Protocol Buffer)
3. gRPC
	1. gRPC message
	2. gRPC Communication
	3. gRPC Gateway

## 0. IPC(Inter-Process Communication)
블록체인 노드의 표면적인 부분을 다 걷어내면 OS 위에서 동작하는 하나의 프로세스에 불과하다. 고로 블록체인은 여러 프로세스 간의 커뮤니케이션을 통해 합의를 이뤄 상태 전환을 하고 지속적으로 상태 동기화를 이루는 복제된 상태 머신(state machine replication)라는 개념으로 표현할 수 있다. 이때, '프로세스 간의 커뮤니케이션'은 IPC(Inter-Process Communication)이라 불리우는 서로 다른 프로세스 간에 데이터를 주고받거나 동기화를 하기 위한 메커니즘을 뜻한다. 다른 노드와 쉽게 통신하고 상호 운용할 수 있도록 설계된 Cosmos SDK를 다루는 데에 있어서 이러한 기초적인 부분부터 다뤄보고자 한다. IPC는 다양한 상황에서 효율적으로 프로세스 간 상호 작용을 가능하게 하며, 리소스 제약이나 과도한 연산이 필요한 상황에서 특히 유용하다. 여러 프로세스가 협력하여 단일된 목표를 달성하는 분산 시스템의 핵심이기도 하다. 대표적인 IPC 방식으로는 공유 메모리와 메시지 패싱이 있다.

### 공유 메모리
프로세스 간에 공유가 되도록 설정해놓은 메모리이며, 모든 프로세스가 접근이 가능하다. 이는 효율적인 데이터 액세스가 가능하지만, 여러 프로세스가 동시에 공유 메모리에 액세스할 수 있기 때문에 메모리 안전성을 헤치는 동기화가 문제가 발생할 수 있다는 단점이 있다. 

### 메시지 패싱 
이는 공유하는 데이터 저장 공간을 두지 않고 서로에게 필요한 데이터를 직접 통신하여 전달하는 방법을 말한다. 이는 동시성으로 인해 발생하는 동기화에 대해 고민할 필요가 없다. 그러나 잦은 통신이나 많은 양의 데이터를 주고 받을 경우 리소스가 부족하여 성능 저하로 이어질 수 있다. 

## 1. IPC in Client-Server Systems
IPC 기술은 Client-Server 통신에도 사용할 수 있다. 이는 메시지 패싱 방식으로 가장 저수준으로는 Socket이 있고 그 위에 추상화된 기술로 RPC와 HTTP 통신 기법이 있다.

### 1-1. Socket 
소켓은 외부 통신을 위한 엔드포인트이다. 한 쌍의 프로세스가 네트워크에서 통신할 때 각 프로세스는 한 쌍의 소켓을 사용한다. 소켓은 port 번호와 ip 주소로 식별되며, 일반적으로 Client-Server 아키텍처를 사용한다. 
- Server: 지정된 port를 통해 들어오는 클라이언트 요청을 대기한다.
- Client: Server에게 연결을 요청하면 해당 프로세스의 OS가 임의의 port를 할당하여 연결을 시도한다. 

소켓을 이용한 통신은 일반적이고 효율적이지만 IPC의 저수준 형태에 속한다. OSI 모델의 전송 계층(L4)의 TCP 또는 UDP를 이용하기 위한 수단으로 보면 된다. 소켓이 통신 스레드 간에 교환되는 데이터 형식이 비구조화된 바이트스트림으로 이루어지기 때문에 통신이 증가함에 따라 처리가 더욱 복잡해진다.


### 1-2. HTTP API
HTTP API는 HTTP(Hypertext Transfer Protocol)를 사용하여 클라이언트와 서버 간의 요청-응답 방식으로 통신을 쉽게 사용할 수 있다. 전 세계적으로 표준화된 이 프로토콜은 다양한 플랫폼과 언어에서 쉽게 사용할 수 있으며 웹 기술과 잘 통합된다. HTTP API는 OSI 모델의 애플리케이션 계층(L7)에서 작동하며 전송 계층(L4)의 소켓이 제공하는 기본 전송 메커니즘을 활용한다. 이러한 추상화는 소켓 프로그래밍에 비해 데이터 송수신 작업을 쉽게 처리할 수 있다. 

#### HTTP Method 
HTTP API는 자원을 URL로 표현하고, HTTP 메서드(GET, POST, PUT, DELETE 등)를 사용하여 자원에 대한 작업을 수행한다. 다음은 간단한 HTTP GET 요청 예시이다.

```http
GET /users/123 HTTP/1.1
Host: example.com
Accept: application/json
```
서버는 다음과 같이 JSON 형태의 응답을 보낼 수 있다.

```http
HTTP/1.1 200 OK
Content-Type: application/json

{
    "id": 123,
    "name": "John Doe",
    "email": "john.doe@example.com"
}
```

#### REST(Representational State Transfer)
REST는 Roy T. Fielding이 논문 ['Architectural Styles and the Design of Network-based Software Architectures'](https://ics.uci.edu/~fielding/pubs/dissertation/top.htm)에서 소개한 분산 시스템의 아키텍처 스타일이다. 많은 API가 RESTful이라고 주장하지만, REST 아키텍처 스타일을 엄격하게 따르지 않는 경우가 많다. 실은 못한다고 하는 게 맞다. HATEOS와 같은 조건까지 충족시키기에는 API 설계에 많은 비용이 투입되기 때문이다. 그럼에도 모두가 지키고자 하는 것은 올바른 HTTP 메서드 사용과 자원 명시적으로 설계하고자 하는 것이다. REST API 설계에 대한 자세한 내용은 다음을 참조해보자:
- [Day1, 2-2. 그런 REST API로 괜찮은가](https://www.youtube.com/watch?v=RP_f5dMoHFc&ab_channel=NAVERD2)
- [MicroSoft REST API 가이드라인](https://github.com/Microsoft/api-guidelines/blob/master/Guidelines.md) 


### 1-3. RPC(Remote Procedure Call)
RPC(Remote Procedure Call, 원격 프로시저 호출)를 사용하면 원격 제어 코딩 없이도 다른 주소 공간에서 함수나 프로시저를 실행할 수 있다. 즉, RPC를 사용하면 프로그래머는 함수가 로컬인지 원격인지에 관계없이 동일한 코드를 사용할 수 있다. HTTP API와 마찬가지로 RPC는 애플리케이션 계층(L7)에서 작동하며 전송 계층(L4)에서 기본 소켓 메커니즘을 사용해 통신 세부 사항을 처리한다. 이러한 추상화는 전송 계층 프로토콜의 복잡성을 줄여 개발자가 애플리케이션 수준 코드를 사용하여 통신을 위한 함수 호출을 할 수 있게 해준다.
- Client-Server 간의 커뮤니케이션에 필요한 상세정보는 최대한 감춘다.
- Client와 Server는 각각 일반 메소드를 호출하는 것처럼 원격지의 프로시저를 호출할 수 있다.


## 2. IDL(Interface Definition Language)
HTTP API와 RPC는 C++, Java, Go, Rust와 같은 언어를 사용해 프로그램을 빌드하는 애플리케이션 계층에서 동작한다. 이러한 프로그램이 특정 프로그래밍 언어의 제약을 받지 않도록 하기 위해 IDL(Interface Definition Language, 인터페이스 정의 언어)을 정의하여 저수준 소켓 메시지 형식이 아닌 자체 데이터 형식을 사용하여 메시지를 교환할 수 있도록 한다.

### 2-1. JSON
JSON은 HTTP API와 RPC 통신 모두에 사용되는 일반적인 IDL 형식이다. 인간 친화적인 언어이기 때문에 유지 관리와 접근성이 매우 뛰어나다. 이러한 특징으로 Ethereum에서 [JSON-RPC](https://ethereum.org/ko/developers/docs/apis/json-rpc/)를 사용하고 있다:
- 간결함과 가독성: JSON은 텍스트 기반 형식으로 쉽게 읽고 쓸 수 있어 디버깅과 로그 기록에 유리하다.
- 웹 친화성: JSON은 웹 기술과의 호환성이 높아 Dapp과의 통합에 용이하다.
- 범용성: 다양한 프로그래밍 언어에서 JSON을 쉽게 파싱하고 생성할 수 있어, Ethereum 클라이언트와의 상호 운용성이 높다.

그러나 JSON은 데이터 내용에 비해 크기가 효율적이지 못하다. 이는 위에서 언급한 IPC 메시지 패싱 방식의 단점으로, 대용량 데이터를 송수신하게 되면 리소스 부족으로 인한 지연이나 대기가 발생할 수 있다. 

### 2-2. Protobuf(Protocol Buffer)
2016년 구글이 만든 gRPC 프레임워크에서 사용하는 [Protobuf](https://ko.wikipedia.org/wiki/%ED%94%84%EB%A1%9C%ED%86%A0%EC%BD%9C_%EB%B2%84%ED%8D%BC)가 오픈소스로 공개되었다. 이는 데이터의 직렬화 및 역직렬화를 효율적으로 처리하여 대용량 데이터를 다루는 데에 적합하다.

#### Protobuf 인코딩 및 디코딩
다음과 같이 JSON 형태로 표현된 데이터가 있다고 하자. 
```json
{ 
	"userName": "Martin", 
	"favouriteNumber": 1337, 
	"interests": ["daydreaming", "hacking"] 
}
```

Protobuf를 사용하려면 간단하고 사람이 읽을 수 있는 스키마 언어를 사용하여 `message`라는 데이터 구조를 정의한다. JSON의 데이터에서 key와 같은 역할을 하는 데이터 속성 값을 field_tag로 대체하여 데이터를 줄이는 게 핵심이다. 'userName', 'favouriteNumber', 'interests'와 같은 key-value 구조에서 key 역할을 하는 데이터의 속성 값과 type을 조합하여 1바이트 메타정보로 표현할 수 있다. 
- [Protobuf 인코딩 더 자세히 보기](https://medium.com/naver-cloud-platform/nbp-%EA%B8%B0%EC%88%A0-%EA%B2%BD%ED%97%98-%EC%8B%9C%EB%8C%80%EC%9D%98-%ED%9D%90%EB%A6%84-grpc-%EA%B9%8A%EA%B2%8C-%ED%8C%8C%EA%B3%A0%EB%93%A4%EA%B8%B0-2-b01d390a7190)

```protobuf
message Person { 
	required string user_name = 1; 
	optional int64 favourite_number = 2; 
	repeated string interests = 3; 
}
```

이러한 특징으로 인터체인 기능을 가진 Cosmos-SDK도 이를 사용하고 있다:
- 성능: 압축률이 좋아서 다른 IDL(XML, JSON..)에 비해 더 적은 용량으로 데이터를 다룰 수 있다. 
- 유연성: [다양한 언어를 지원](https://protobuf.dev/overview/#cross-lang)하여 여러 언어로 작성된 인터체인 간의 상호 운용성을 높인다.

그러나 데이터가 인코딩되어 네트워크 상에 패킷 송신 오류나 지연이 발생할 때 디코딩 되기 전에는 바이너리 데이터로 표현되기 때문에 디버깅이 어렵다는 단점이 있다.

## 3. gRPC
gRPC는 Google에서 개발한 고성능 RPC 프레임워크로, 직렬화를 위해 Protobuf를 활용한다. 여러 환경과 언어를 지원하여 클라이언트와 서버 간의 원활한 통신을 가능하게 한다.
- Server: 인터페이스를 구현하고 gRPC 서버를 실행하여 클라이언트 호출을 처리한다.
- Client: 클라이언트에는 서버와 동일한 방법을 제공하는 Stub(클라이언트)이 있다.

![](https://i.imgur.com/pvr2BJe.png)

### 3-1. gRPC Messages and Services
gRPC에서 클라이언트와 서버 간의 통신은 Protobuf를 사용하여 정의된다. gRPC service는 입력/출력 message 타입과 함께 원격으로 호출할 수 있는 메서드를 지정한다.

#### Example 
다음은 간단한 gRPC 서비스 정의의 예이다:
```protobuf
syntax = "proto3";

package example;

service Greeter {
  // A simple RPC.
  rpc SayHello (HelloRequest) returns (HelloReply) {}
}

// The request message containing the user's name.
message HelloRequest {
  string name = 1;
}

// The response message containing the greeting.
message HelloReply {
  string message = 1;
}
```
- `Greeter`는 RPC `SayHello`를 정의하는 service이다.
- `HelloRequest`는 사용자 name이 포함된 message입니다.
- `HelloReply`는 greeting message가 포함된 message이다.

클라이언트 애플리케이션은 `Greeter` 서비스에서 `SayHello`를 호출하여 `HelloRequest` 메시지를 전달하고 그 대가로 `HelloReply` 메시지를 받을 수 있다.

### 3-2. gRPC Communication
- Server: Greeter 서비스를 구현하고 클라이언트 호출을 처리하기 위해 gRPC 서버를 실행한다.
- Client: Stub을 사용하여 서버에서 로컬 메서드인 것처럼 SayHello 메서드를 호출한다.

### 3-3. gRPC Gateway
[gRPC-Gateway](https://github.com/grpc-ecosystem/grpc-gateway)는 리버스 프록시 서버를 생성하는 Protobuf 컴파일러 protc용 플러그인으로, RESTful HTTP API 호출을 gRPC로 변환한다. Protobuf 서비스 정의를 읽고 `google.api.http` 어노테이션을 기반으로 서버를 생성하여 gRPC와 RESTful API를 동시에 제공할 수 있도록 한다. 

![](https://i.imgur.com/YTMLSbJ.png)

# Resources
- https://grpc.io/docs/what-is-grpc/introduction/
- https://ida.interchain.io/tutorials/1-tech-terms/#protobuf
- https://ida.interchain.io/academy/2-cosmos-concepts/6-protobuf.html
- Silberschatz, Avraham, PETER BAER GALVIN, GREG GAGNE, Operating System Concepts 10/E, John Wiley & Sons Inc(2019), 123-155

