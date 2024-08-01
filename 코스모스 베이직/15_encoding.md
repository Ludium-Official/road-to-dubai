# 15. Encoding 
> cosmos-sdk, cometbft에서 사용하는 gRPC, REST, CometBFT에 대해 알아보기 전에 이를 이루는 기술의 근간에 대해 먼저 학습하는 것을 목적으로 작성되었다. 

## 목차 
0. 정보량이란? 
1. 정보량을 줄이는 방법: 인코딩(Encoding)
2. Cosmos SDK의 인코딩(Encoding) 라이브러리 톺아보기 
	1. `Any` 타입과 인터페이스 Encoding (ADR-019)
    2. Cosmos SDK `Any` Type
    3. 트랜잭션 Encoding (ADR-020)
3. Codec

## 0. 정보량이란?
블록체인 네트워크에서는 수 많은 메시지가 오고가기 때문에 이러한 인코딩 개념은 매우 중요하다. 특히, [텐더민트](./99c1_tendermint_with_bft.md)의 경우에는 더더욱 그렇다. Cosmos SDK Encoding 라이브러리에 대해 알아보기 전에 인코딩을 왜 해야하며, 이를 하며 무엇이 좋은지에 대해 정보 이론에서 정의하는 정보량에 대한 개념을 통해 알아보자. 

이전 아티클에서 알아본 [IPC 기법](./14_rpc_basic.md#0-ipcinter-process-communication)의 근간에는 섀넌의 정보이론이 있다. 기존 전문가들은 통신의 문제를 물리적으로 풀려고만 하였고 잡음 문제 극복 등 문제의 본질을 파악하지 못해 초보적인 수준에 머물러있었다. 1948년 섀넌은 [통신이란 무엇인지 정의]((https://people.math.harvard.edu/~ctm/home/text/others/shannon/entropy/entropy.pdf))하여 통신의 문제를 혁신적으로 바라보게 하였다. 섀넌의 1948년 논문(정보이론)과 튜링의 1936년 논문(튜링기계)은 같은 플롯으로 구성되어 있다:
- 우선 애매했던 대상(섀넌은 '정보량', 튜링은 '기계적인 계산')을 과감하게 정의한다
- 그리고 그 정의가 받아들일 만하다고 설득한다.
- 그런 후 그 정의로부터 논리적으로 엄밀한 사실들(셰넌은 메시지 전달의 한계, 튜링은 기계적인 계산의 한계)을 유도한다. 세상을 바꾼 두 논문은 같은 패턴이다. 

### 정보량
통신의 주인공이 '정보량'이라는 주장은 '메시지의 정보량이 뭐냐'는 정의에서부터 시작한다. 섀넌이 정보량을 정의하는 관점은 이렇다. 잦은 것은 정보량이 적고 드문 것은 정보량이 많다. 자주 쓰는 건 예측하기 쉽기 때문이다. 자주 보이고 드물게 보이는 차이. 이 차이가 없으면 예측이 어렵다. 
- 정보 많음: 차이 없이 모든 글자가 골고루 사용되는 세계에서 온 메시지는 정보가 많다.
- 정보 적음: 반면에 그 차이가 있는 세계에서 온 메시지는 정보가 적다. 자주 보이는 글자는 흔히 나타날 것이므로 보지 않고도 맞추기 쉽기 때문이다. 

정보의 양이란 무질서의 정도, 정보량 정의는 열역학에서 이야기하는 엔트로피의 정의와도 같다. 메시지의 정보량은 글자들의 예측불허의 정도와 일치한다. 메시지에 나타나는 글자들이, 흔하거나 드문 게 따로 있다면 정보가 적다. 흔한 것들이 대다수일 테고 이는 예측이 쉬우므로 무질서가 적은 대상이다. 정리하면 다음과 같다: 
- 정보량이 많은 경우: 흔하거나 드문 차이 없이 골고루라면 예측이 어렵기 떄문에 정보량이 많다고 볼 수 있다. 즉, 무질서가 큰 것이다. 
- 정보량이 적은 경우: 드문 단어(= 불필요한 단어)가 맥락에 포함되면 해당 메시지의 정보량은 줄어든다고 볼 수 있다. 이는 '불필요하다'는 말 속에 이미 '분별이 있다'는 뜻이고, '분별이 있다'는 건 그만큼 무질서가 적은 것이다. 
 
섀넌은 위와 같이 메시지의 정보량을 정의하고 그 정의가 적절하다고 설득한 후, 온전한 소통을 가능하게 하는 정리 두 개(잡음이 있는 채널, 잡음이 없는 채널)를 도출해내었다. 정리하면 다음과 같다:
1. 잡음이 없는 채널: 전달하고자 하는 정보량이 H고, 채널 용량이 C라고 하자. 메시지 전달은 최대 초당 C/H로 항상 가능하다.
2. 잡음이 있는 채널: 정보량이 초당 H라고 하고 채널 용량은 초당 C라고 하자. 
	- H <= C 이면 온전히(잡음에 의한 생채기가 충분히 적어지도록) 전달할 수 있다. 
	- H > C 이면 잡음에 의한 생채기를 (H-C) 미만으로 줄일 수 없다.

이러한 섀넌의 정보 이론은 현대 디지털 통신의 기초가 되었다. 이를 통해 설계된 모든 정보 통신 시스템 구조는 다음과 같은 다이어그램의 구조를 가지고 있다.

<div style="text-align: center;">
	<img src="./assets/15_diagram_of_a_general_communication_system.png" alt="Diagram of a General Communication System" width="650" height="400">
</div>


## 1. 정보량을 줄이는 방법: 인코딩(Encoding)
정보 이론에서 정의한 정보량을 줄이는 방법은 메시지에 있는 특정 패턴을 반복하거나 잡음으로 손상된 메시지를 복구시키는 방법을 추가하면 된다. 이런 부가적인(정보량 줄이기) 방법들을 메시지에 추가하다 보면 단위 시간당 전달할 수 있는 정보량은 언젠가는 H<=C 가 되어서 그런 메시지는 온전히 전달할 수 있게 된다.
- 정보량 줄이기 = 엔트로피 낮추기(무질서 낮추기) = 흔하거나 드문게 따로 있는 (예측 쉬운) 형태로 만들기

정보 이론에 따르면 어떠한 잡음에서도 온전히 통신할 수 있고 방법은 하드웨어가 아니라 소프트웨어(메시지 자체)에 있다고 말한 것이다. 그렇게 찾은 방법이 인코딩(Encoding)이다. 이러한 인코딩(Encoding)은 같은 맥락에서 사용되는 마샬링(Marshaling) 또는 직렬화(Serialization)라고 부르기도 하는데, 앞으로 사용할 용어를 정리하기 위해 이에 대해서 짚고 넘어가도록 하자.

### 인코딩(Encoding)
> 'Encode'는 'En-'(안으로)와 'Code'(코드, 부호)의 결합으로, '특정한 형식으로 변환하다'라는 의미이다. 

인코딩(Encoding)은 데이터나 정보를 다른 형식으로 변환하는 과정을 말한다. 주로 데이터의 저장, 전송 및 처리 효율성을 높이기 위해 사용된다. 
- 예: 텍스트 데이터를 바이너리 형태로 변환하기 

### 마샬링(Marshaling)
> 'Marshal'은 고대 프랑스어 'maréchal'에서 유래되었으며, 이는 고대 프랑크어 '*marhskalk(="말 관리인, 하인")'에서 유래되었다. 이 단어는 '말의 하인'이라는 의미로 시작되어, 점차 '조직하다', '배열하다'라는 의미로 발전했다.

[마샬링(Marshaling)](https://en.wikipedia.org/wiki/Marshalling_(computer_science))은 데이터를 전송하거나 저장하기 위해 지정된 형식으로 정리하고 준비하는 과정을 의미한다. 주로 복잡한 데이터 구조를 표준화된 형식으로 변환하여 전송 가능하게 한다.
1. 마샬링: 직렬화된 객체를 바이트 단위로 변환하여 전송 준비를 한다.
2. 바이트 스트림 전송: 직렬화 되어 분해된 데이터를 순서에 따라 전송한다
3. 언마샬링: 전송 받은 데이터를 원래대로 역직렬화하여 복구한다.

### 직렬화(Serialization)
> 'Serialize'는 'Serial'(일련의, 연속적인)에서 파생된 것으로, '일련의 형태로 만들다'라는 의미이다. 

마샬링과 직렬화은 다소 유사한 의미로 사용된다. [직렬화(Serialization)](https://en.wikipedia.org/wiki/Serialization)는 객체나 데이터 구조를 연속된 바이트 스트림으로 변환하는 과정을 의미한다. 이를 통해 데이터를 저장하거나 네트워크를 통해 전송할 수 있다.
1. 직렬화: 객체의 상태를 바이트 스트림으로 변환하여 저장하거나 전송할 수 있게 합니다.
2. 바이트 스트림 전송: 네트워크를 통해 바이트 스트림을 전송합니다.
3. 역직렬화: 바이트 스트림을 다시 원래의 객체로 복구합니다.

</br>

이를 정리하면 다음과 같다:
- 인코딩 (Encoding): 데이터를 특정한 형식으로 변환하는 모든 과정을 포함하는 가장 큰 범주이다. 
- 마샬링 (Marshalling): 데이터를 전송 가능하도록 정리하고 준비하는 과정으로, 직렬화를 포함한다.
- 직렬화 (Serialization): 데이터를 바이트 스트림으로 변환하여 저장하거나 전송할 수 있게 과정을 나타낸다.

인코딩은 마샬링, 직렬화의 개념을 내포하는 큰 개념으로 봐도 된다. 마샬링이 직렬화의 과정을 포함하고 있지만서도 포함 개념이 아닌 유사한 개념으로 보는 게 더 좋을 듯 싶다. Cosmos SDK에서는 주로 인코딩 과정에 있어서 마샬링 단어를 주로 사용하니 참고하자.


### 인코딩(Encoding) 종류 
이러한 인코딩은 현재 컴퓨터 과학에서 다양한 방식으로 이루어진다. 간략하게 알아보면 다음과 같다: 
- 문자열 인코딩: ASCII, UTF-8, Base64 등
- 이미지 인코딩: JPEG, PNG, GIF 등
- 오디오 인코딩: MP3, WAV, AAC 등
- 비디오 인코딩: MPEG, AVI, WMV 등
- 압축 인코딩: ZIP, RAR, GZIP 등

이러한 인코딩은 데이터를 효율적으로 네트워크를 통해 서로 간의 통신을 할 때 정보량을 줄여 많은 정보를 주고 받을 수 있게 사용된다. 이전 아티클에서 프로그래밍 언어로 작성된 애플리케이션 간의 통신을 위해 [JSON과 ProtoBuf와 같은 IDL](./14_rpc_basic.md#2-idlinterface-definitionlanguage)을 사용하는 것을 알아보았는데, 이도 마찬가지로 네트워크를 통해 통신을 하게 되면 인코딩 및 디코딩 과정을 위러서 통신이 이루어지고 있다. 그 중에서 [Protobuf는 메시지 정보를 줄이는 인코딩 및 디코딩 방식에 특화](./14_rpc_basic.md#protobuf-인코딩-및-디코딩)되어 설계되었기 때문에 블록체인 네트워크와 같이 수 많은 메시지가 오고가는 환경에서는 매우 유용하게 사용된다. Protobuf에 다음과 같이 1,2,3과 같은 `field_tag`가 존재하는 것도 이러한 이유에서이다. 
```protobuf
message Person { 
	required string user_name = 1; 
	optional int64 favourite_number = 2; 
	repeated string interests = 3; 
}
```

## 2. Cosmos SDK의 인코딩(Encoding) 라이브러리 톺아보기 
우리가 알아볼 Cosmos SDK는 ProtoBuf를 기반으로 사용하여 통신을 하고 있다. Cosmos SDK의 인코딩 라이브러리 변동 히스토리는 다음과 같다:
1. `Protobuf`의 단점 인터페이스 인코딩 기능을 탑재한 [`Amino`](https://github.com/tendermint/go-amino)
2. Go 언어 공식 `Protobuf` 구현 라이브러리 커스텀한 [`Gogoproto`](https://github.com/cosmos/gogoproto)
3. Go 언어 공식 [`Protobuf` 구현 라이브러리](https://github.com/golang/protobuf)

초기 Cosmos SDK가 [`Proto3`](https://protobuf.dev/programming-guides/proto3/)를 사용하지 않은 이유는 `Proto3`는 Go의 인터페이스와 같은 개념을 직접 지원하지 않기 떄문이다. 대신 `oneof`라는 유사한 기능을 제공했지만 `oneof`는 모든 가능한 타입을 사전에 정의해야 한다. 이는 개발자는 `Proto3`으로 생성된 코드와 애플리케이션의 논리적 객체 간에 변환 로직을 직접 작성해야 해서 중복 코드와 추가적인 유지보수 비용을 초래하게 된다. 

그래서 인터페이스 지원을 위한 확장 기능을 갖춘 `Proto3` 스펙에 부합하는 [`Amino`](https://github.com/tendermint/go-amino)를 사용하였다. 애플리케이션에서 사용하는 논리적인 객체와 블록체인에 영구적으로 보관된 지속성 있는 객체의 패리티를 지원하였고, 인터페이스 지원을 통해 유연성을 증가하고 인코딩/디코딩 프로세스를 단순화시켰다. 그러나 `Amino`는 다양한 언어로 작성된 클라이언트들이 존재하는 크로스 플랫폼 환경에서 필요한 유연성이 다소 부족한 부분이 있었다.
- 하위 호환성 및 유연한 확장성(업그레이드 가능성)을 거의 제공하지 않았다. 
- 프로파일링과 벤치마킹을 통해 `Amino`는 Cosmos SDK 매우 큰 성능 병목 현상을 일으키는 것으로 밝혀졌다. 이는 트랜잭션 인코딩/ 디코딩을 하는 부분에서 드러났다. 

`Amino`의 대안으로 `Protobuf`를 선택하게 되었다. Cosmos SDK에서는 Golang 공식 `Protobuf` 라이브러리인 [`google.golang.org/protobuf`](https://pkg.go.dev/google.golang.org/protobuf)에 비해 속도와 DX가 개선된 동시에 `Protobuf` 스펙에 충족하는 [`Gogoproto`](https://github.com/gogo/protobuf) 라이브러리를 사용하였다. 또한 기존 인터페이스 인코딩의 문제점은 `Any` 타입을 통해 해결하였다. (이는 아래 ADR-019, ADR-020에서 자세히 알아보자.)

그런데 `Gogoproto`는 현재 [여러가지 사정으로 인해 deprecated가 된 상황](https://youtu.be/HTIltI0NuNg?si=0W_5V1Yq53m3Tw_a)이다. 이를 사용중이던 Cosmos 팀은 이를 fork하여 [`cosmos/gogoproto`](https://github.com/cosmos/gogoproto)로 유지 보수하며 사용하는 것으로 보이나, 장기적으로 Golang 공식 지원 `google.golang.org/protobuf` 구현으로 전환하고 있는 것으로 보인다. 

### 2-1. `Any` 타입과 인터페이스 인코딩 (ADR-019)
> 이 ADR은 Cosmos SDK 상태 머신의 `상태 인코딩`에 중점을 둔다.

[ADR 019: Protocol Buffer State Encoding](https://docs.cosmos.network/main/build/architecture/adr-019-protobuf-state-encoding)에서 `Protobuf`의 인터페이스 인코딩하는 데에 Protobuf의 [`Any`](https://protobuf.dev/programming-guides/proto3/#any)타입을 사용하기로 결정했다. `Any`는 임의의 인코딩된 메시지를 바이트 단위로 포함하며, 해당 메시지 타입에 대한 식별자 역할을 하고 해당 타입으로 해석되는 URL(TypeURL)을 포함한다. 이 전략을 사용하면 `.proto` 정의 없이도 Protobuf 메시지 안에 임의의 Go 타입을 담을 수 있게 된다. 

그래서 현재 인터페이스 인코딩은 다음과 같이 이뤄진다: 
1. 인터페이스를 `Any`로 패킹한다.
2. 패킹된 `Any`를 마샬링한다. 

Cosmos SDK는 이 두 단계를 한 번에 처리할 수 있는 [`MarshalInterface` 메서드](https://github.com/cosmos/cosmos-sdk/blob/main/codec/proto_codec.go#L223-L240)를 제공한다:
```go
func (pc *ProtoCodec) MarshalInterface(i gogoproto.Message) ([]byte, error) {
	if err := assertNotNil(i); err != nil {
		return nil, err
	}
	any, err := types.NewAnyWithValue(i)
	if err != nil {
		return nil, err
	}
	err = pc.interfaceRegistry.EnsureRegistered(i)
	if err != nil {
		return nil, err
	}

	return pc.Marshal(any)
}
```

[`x/auth` 모듈의 keeper](https://github.com/cosmos/cosmos-sdk/tree/v0.47.0/x/auth/keeper)에서는 이를 다음과 같이 사용하고 있다:
```go
func (ak AccountKeeper) MarshalAccount(accountI types.AccountI) ([]byte, error) { 
	return ak.cdc.MarshalInterface(accountI)
}

func (ak AccountKeeper) SetAccount(ctx sdk.Context, acc types.AccountI) {
    // ... 

    bz, err := ak.MarshalAccount(acc)
    if err != nil {
        panic(err)
    }

    // ... 
}
```

### 2-2. Cosmos SDK `Any` Type
일반적으로 `Any` 타입을 사용하려면 `google/protobuf/any.proto`를 가져와야 하는데, [Cosmos SDK는 자체적으로 Any 타입을 구현](https://github.com/cosmos/cosmos-sdk/tree/main/codec/types)하여 사용하고 있다. 

Cosmos SDK에서는 여러  곳에서 `Any` 인코딩을 사용한다:
- 다양한 타입의 공개 키를 인코딩하기 위한 `cryptotypes.PubKey` 인터페이스
- 트랜잭션에서 다양한 `Msg`를 인코딩하기 위한 `sdk.Msg` 인터페이스
- `x/auth` 쿼리 응답에서 다양한 유형의 계정을 인코딩하기 위한 `AccountI` 인터페이스(위의 예와 유사)
- `x/evidence` 모듈에서 다양한 유형의 증거를 인코딩하기 위한 `EvidenceI` 인터페이스
- 다양한 유형의 `x/authz` 인증을 인코딩하기 위한 `AuthorizationI` 인터페이스
- 유효성 검사기에 대한 정보를 포함하는 `Validator` 구조체

다음 예시는 `x/staking`의 `Validator` 구조체 내부에서 [공개 키를 Any로 인코딩하는 코드](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/x/staking/types/validator.go#L41-L64)이다: 
```go
func NewValidator(operator sdk.ValAddress, pubKey cryptotypes.PubKey, description Description) (Validator, error) {  
	pkAny, err := codectypes.NewAnyWithValue(pubKey)  
	if err != nil {  
		return Validator{}, err  
	}  
  
	return Validator{  
		ConsensusPubkey: pkAny,  

		// ...
	}, nil  
}  
```

#### `Any`'s TypeURL
`Any` 안에 Protobuf 메시지를 패킹할 때 메시지의 타입은 `/(slash)` 문자가 접두사로 붙은 메시지의 정규화된 이름인 TypeURL에 의해 고유하게 정의된다. 

`gogoproto`와 같은 일부 Any 구현에서는 일반적으로 `type.googleapis.com`과 같이 확인 가능한 접두사가 있다. 하지만 Cosmos SDK에서는 이러한 접두사를 포함하지 않고 더 짧은 형식의 URL을 사용하기로 결정했다. 
- Cosmos SDK의 [자체 Any 구현 코드](https://github.com/cosmos/cosmos-sdk/tree/main/codec/types)

또한 Cosmos SDK는 위에서 언급한대로, gogoproto에서 공식 google.golang.org/protobuf(Protobuf API v2로 알려짐)로 전환하고 있다. 기본 Any 구현에도 "type.googleapis.com" 접두사가 포함되어 있다. SDK와의 호환성을 유지하려면 "google.golang.org/protobuf/types/known/anypb" 의 다음 메서드는 사용하지 않아야 한다:
- `anypb.New`
- `anypb.MarshalFrom`
- `anypb.Any#MarshalFrom`

대신 Cosmos SDK는 접두사를 삽입하지 않고 공식 `anypb.Any`를 생성하는 [헬퍼 함수](https://github.com/cosmos/cosmos-proto/tree/main/anyutil)를 제공한다:
- `anyutil.New`
- `anyutil.MarshalFrom`

### 2-3. 트랜잭션 Encoding (ADR-020)
Protobuf의 또 다른 중요한 용도는 트랜잭션의 인코딩과 디코딩이다. [ADR 019](https://docs.cosmos.network/main/build/architecture/adr-019-protobuf-state-encoding)의 주 목적은 `Any`를 사용하여 인터페이스 인코딩을 통해 많은 체인과 안전하게 호환될 수 있도록 하는 것이다. [ADR 020: Protocol Buffer Transaction Encoding](https://docs.cosmos.network/main/build/architecture/adr-020-protobuf-transaction-encoding)에서는 이러한 호환성을 깨지 않으면서 유연한 크로스체인 트랜잭션 형식을 제공하고자 하는 것을 주 목적으로 삼는다. 

트랜잭션은 애플리케이션이나 Cosmos SDK에 의해 정의되지만, 기본 합의 엔진으로 전달되어 다른 피어에게 전달된다. 이전 [트랜잭션 라이프사이클 브로드캐스팅](./10_transaction_and_mempool.md#1-트랜잭션-생성-및-브로드캐스팅하기) 과정에서 트랜잭션을 `[]byte` 형태의 인코딩을 한 예시를 잠깐 다뤄보았듯이, 기본 합의 엔진은 애플리케이션에 구애받지 않으므로 `[]byte` 형태의 트랜잭션만 허용하고 있다. 
- [app -> cometbft] `TxEncoder` 객체는 인코딩을 수행한다. (`sdk.Tx` -> `[]byte` 변환)
- [cometbft -> app] `TxDecoder` 객체는 디코딩을 수행한다. (`[]byte` ->  `sdk.Tx` 변환)
```go
// TxDecoder unmarshals transaction bytes  
type TxDecoder func(txBytes []byte) (Tx, error)  
  
// TxEncoder marshals transaction to bytes  
type TxEncoder func(tx Tx) ([]byte, error)
```

이 두 표준 구현 예시로는 [`x/auth모듈`의 tx](https://docs.cosmos.network/v0.47/build/modules/auth/tx)에서 찾을 수 있다:
- [`x/auth` tx encoder](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/x/auth/tx/encoder.go)
- [`x/auth` tx decoder](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/x/auth/tx/decoder.go)


## 3. Codec
Cosmos SDK의 `codec` 패키지에 대표적으로 [`Amino Codec`](https://github.com/cosmos/cosmos-sdk/blob/main/codec/amino_codec.go)과 [`Proto Codec`](https://github.com/cosmos/cosmos-sdk/blob/main/codec/proto_codec.go)이 있다. 

기존 Cosmos SDK 모든 모듈은 `Amino Codec`을 사용하여 타입과 인터페이스를 인코딩하였다. 이 `Codec`에는 일반적으로 해당 모듈의 도메인에만 등록된 타입과 인터페이스가 있다. 각 모듈은 사용자가 `Codec`을 제공하고 모든 타입을 등록할 수 있는 `RegisterLegacyAminoCodec` 함수를 통해 앱은 필요한 각 모듈에 대해 이 메서드를 호출한다. 위에서 언급한대로, 현재는 지속적으로 `Protobuf` 인코딩으로 전환하고 있으며 모듈에 대한 `Protobuf` 기반 타입 정의가 없는 경우, `Amino`를 통해 바이트를 구체적인 타입 또는 인터페이스로 인코딩 및 디코딩한다:
```go
bz := keeper.cdc.MustMarshal(typeOrInterface)  
keeper.cdc.MustUnmarshal(bz, &typeOrInterface)
```


# Resources
- https://docs.cosmos.network/v0.47/learn/advanced/encoding
- 이광근, 컴퓨터 과학이 여는 세계, 인사이트(2017)

