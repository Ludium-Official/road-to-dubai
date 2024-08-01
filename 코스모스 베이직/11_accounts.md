# 11. Accounts
> cosmos-sdk v0.47부터 Tendermint에서 [CometBFT로 마이그레이션](https://github.com/cosmos/cosmos-sdk/issues/14870) 했기 때문에, 아티클은 cosmos-sdk v0.47, cometbft v0.38 기반으로 작성되었다. 

## 목차 
0. 공개 키 암호화 
1. Account
2. Signature
3. Address
4. Keyring

## 0. 공개 키 암호화 
공개 키 암호화 알고리즘은 public key와 private key를 통해 데이터를 암호화하고 복호화한다. 이는 비대칭 키 암호화라고도 알려져 있다. public key는 누구에게나 공유해도 되지만 private key는 누구에게도 알려지면 안된다. 암호 통신 방식으로는 디지털 서명과 암호/복호 통신 방식이 존재한다.

### 디지털 서명
- 서명: Sender는 자신의 private key로 메시지에 서명한다. 이 서명은 Sender의 public key를 가진 사람 누구나 검증할 수 있다.
- 검증: Receiver는 Sender의 public key를 사용하여 서명을 검증한다. 이를 통해 메시지가 Sender로부터 왔음을 확인할 수 있다.

### 암호/ 복호 통신
- 암호화: Sender는 Receiver의 public key로 메시지를 암호화한다. public key로 암호화된 메시지는 Receiver의 private key로만 복호화할 수 있다.
- 복호화: Receiver는 자신의 private key로 암호화된 메시지를 복호화한다. 이를 통해 메시지를 안전하게 전달받을 수 있다.


### 블록체인 사용 사례
블록체인에서는 공개키 암호화가 다양한 방식으로 사용된다:
- 트랜잭션 서명 및 검증: 자신의 private key로 트랜잭션에 디지털 서명(sign)을 하고, 네트워크 다른 노드들은 사용자의 public key를 사용하여 서명을 검증(verify)한다. 
- address 생성: public key를 통해 파생된 address를 생성하여 공개한다. 다른 사용자는 해당 유저의 address를 참고하여 트랜잰션을 전송할 수 있다. 

### 공개 키 암호화 종류
공개 키 암호화 알고리즘은 여러가지 종류가 있다. 그 중에서 SSL/TLS 인증서 암호화에서도 사용되는 RSA와 ECC에 대해서 알아보자. 

#### 1. RSA (Rivest-Shamir-Adleman)
RSA는 1977년 Ron Rivest, Adi Shamir 및 Leonard Adleman에 의해 발명된 소인수분해의 어려움을 기반으로 하는 공개키 암호화 알고리즘이다. 엄청난 큰 두 소수를 곱하여 공개 키를 만드는 방법으르로 소수 값을 모르면 메시지를 해독할 수 없다는 것이 주요 포인트이다. 전통적인 공개키 암호화 알고리즘으로 검증되어 널리 사용되고 있다. 

RSA를 채택하는 이유는 간단한 수학적 원리를 기반으로 하며 ECC에 비해 더 빠르게 실행될 수 있기 때문이다. ECC가 무엇인지는 잠시 후에 알아보도록 하자. 다만 단순한만큼 높은 보안 수준을 위해서 2048비트 이상의 Key size가 필요하다는 단점을 가지고 있다. 컴퓨터 계산 능력이 나날이 발전함에 따라 해당 취약점을 마냥 무시할 수는 없다. 

#### 2. ECC (Elliptic Curve Cryptography)
타원 곡선 암호화(ECC)의 역사는 닐 코블리츠(Neal Koblitz)와 빅터 S. 밀러(Victor S. Miller)라는 수학자 두 명이 암호화에서 1985년에 타원 곡선의 사용을 제안했다. 이는 유한 필드에 대한 타원 곡선의 대수적 구조를 사용하는 비대칭 암호화 알고리즘이다. 이 암호화 방법은 그래프에서 타원 곡선을 생성하는 방정식에 의해 주어진 수학적 문제에 대한 알려진 해결책이 없기 때문에 해독하기가 더 어렵다고 알려져 있다. 

따라서 ECC는 RSA에 비해 비교적 작은 Key Size로도 더 높은 보안을 제공할 수 있다. 예를 들어, 256비트의 ECC 키는 15360비트의 RSA 키와 유사한 보안 수준을 제공한다고 한다. 많은 전문가들은 2030년이 되면 RSA가 더 이상 사용되지 않을 것이라고 믿고 있다. 반면에 ECC는 성숙 단계에 있으며 많은 사용자가 사용하기 시작했다. 디지털 서명을 통해 트랜잭션을 사용하는 블록체인 분야 또한 ECC를 주로 채택하여 사용하고 있고, Cosmos SDK 또한 ECC를 사용하고 있다.

## 1. Account
Account는 공개 키 암호화 알고리즘에서 사용하는 public key와 private key를 담은 객체를 말한다. 앞서 말했다시피 public key는 공개해도 안전한 사용자의 고유 식별자(identifier)를 나타낸다. private key는 사용자가 직접 서명했음을 다른 사람에게 증명할 때 사용하므로 기밀로 관리해야 한다. 
- public key는 애플리케이션에서 사용자를 식별하는 데 사용되는 다양한 주소를 생성하기 위해 파생될 수 있다.
- address는 메시지 발신자를 식별하기 위해 메시지와도 연결된다. 
- private key는 디지털 서명(Signature)을 생성하는 데 사용되어 private key와 연결된 주소가 특정 메시지를 승인했음을 증명한다.

HD(erarchical Deterministic) 키 생성을 위해 Cosmos SDK는 BIP32라는 표준을 사용한다. BIP32를 통해 사용자는 초기  seed에서 파생된 계정 집합인 HD 지갑(BIP44에 명시된 대로)을 만들 수 있다. 
- 시드는 보통 12단어 또는 24단어 니모닉으로 생성된다. 
- 하나의 시드는 단방향 암호화 함수를 사용하여 원하는 수의 private key를 파생할 수 있다. 그런 다음 private key에서 public key를 파생할 수 있다. 
- 니모닉이 보존되어 있으면 언제든지 private key를 다시 생성할 수 있기 때문에 당연히 니모닉은 가장 민감한 정보이다.

### HD(Hierarchical Deterministic) Wallets
[BIP32](https://en.bitcoin.it/wiki/BIP_0032)는 HD(Hierarchical Deterministic) Wallets에 대한 주제를 다룬 Bitcoin 제안 문서이다. 
- 블록체인은 일반적으로 사용자 계정의 원장을 유지하며 사용자 인증을 위해 공개 키 암호화를 사용한다. 트랜잭션을 실행하려면 자신의 public key와 private key에 대한 정보가 필요하다. 
- 지갑으로 알려진 클라이언트 앱은 새로운 키 쌍 (public key, private key)을 생성하고 저장하는 방법과 트랜잭션 생성, 메시지 서명, 애플리케이션과의 상호 작용, 블록체인과의 통신과 같은 기본 서비스를 제공한다. 

<p align="center">
  <img width="460" height="300" src="https://github.com/bitcoin/bips/blob/master/bip-0032/derivation.png?raw=true">
</p>

BIP39 이후, 이 초기 seed는 대부분 표준화된 사전에서 가져온 니모닉이라고 하는 12개 또는 24개의 단어로 생성된다. 니모닉은 결정론적이기 때문에 모든 키 쌍을 니모닉으로 재구성할 수 있다. 그래서 니모닉만 안전하게 보관하면 된다. 하나의 니모닉에서 생성할 수 있는 키 쌍의 수에는 실질적인 상한선이 없다. [BIP44](https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki) derivation 경로에서 가져온 입력은 하나의 니모닉을 사용하여 모든 블록체인의 키 쌍을 생성하는 데 사용되기 때문에 "Hierarchical Deterministic(HD)"라는 이름이 지어졌다. 

### Public Key
Cosmos SDK의 공개 키는 [`PubKey`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/crypto/types/types.go#L8-L17) 인터페이스에 의해 정의된다. 공개 키는 저장소에 저장되므로 이는 proto.Message 인터페이스를 확장한다.
```go
// PubKey defines a public key and extends proto.Message.
type PubKey interface {
	proto.Message

	Address() Address
	Bytes() []byte
	VerifySignature(msg []byte, sig []byte) bool
	Equals(PubKey) bool
	Type() string
}
```

사용자 상호작용을 위해 `PubKey`는 Protobufs JSON(ProtoMarshalJSON 함수)을 사용하여 형식이 지정된다. 다음은 해당 방식을 사용하는 [`NewKeyOutput`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/crypto/keyring/output.go#L23-L39) 함수이다: 
```go
// NewKeyOutput creates a default KeyOutput instance without Mnemonic, Threshold and PubKeys
func NewKeyOutput(name string, keyType keyring.KeyType, a sdk.Address, pk cryptotypes.PubKey) (KeyOutput, error) {
	apk, err := codectypes.NewAnyWithValue(pk)
	if err != nil {
		return KeyOutput{}, err
	}
	bz, err := codec.ProtoMarshalJSON(apk, nil)
	if err != nil {
		return KeyOutput{}, err
	}
	return KeyOutput{
		Name:    name,
		Type:    keyType.String(),
		Address: a.String(),
		PubKey:  string(bz),
	}, nil
}
```

### Private Key 
Cosmos SDK의 개인 키는 [`PrivKey`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/crypto/types/types.go#L19-L39)는 인터페이스에 의해 정의된다. 이는 proto.Message 인터페이스와 LedgerPrivKey를 확장한다:
```go
// LedgerPrivKey defines a private key that is not a proto message.
type LedgerPrivKey interface {
	Bytes() []byte
	Sign(msg []byte) ([]byte, error)
	PubKey() PubKey
	Equals(LedgerPrivKey) bool
	Type() string
}

// PrivKey defines a private key and extends proto.Message.
type PrivKey interface {
	proto.Message
	LedgerPrivKey
}
```

## 2. Signature
Cosmos SDK는 디지털 서명(Signature)을 생성하기 위해 다음과 같은 디지털 키 체계를 지원한다. 이는 `PrivKey` 인터페이스에 정의된 `Sign(msg []byte) ([]byte, error)` 함수에 의해 각 알고리즘마다 구현된다:
- `secp256k1`:  비트코인에서도 사용하는 타원 곡선 디지털 서명 알고리즘(ECDSA) 중 하나이다. Cosmos SDK의 [`crypto/keys/secp256k1`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/crypto/keys/secp256k1/secp256k1.go) 패키지에 구현되어 있다.
- `secp256r1`: 다른 타원 곡선 디지털 서명 알고리즘(ECDSA)으로, 일반적으로 P-256 또는 prime256v1로도 알려져 있다. Cosmos SDK의 [`crypto/keys/secp256r1`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/crypto/keys/secp256r1/pubkey.go) 패키지에 구현되어 있다.
- `tm-ed25519`: 디지털 서명 알고리즘으로, 타원 곡선 ED25519를 사용한다. Cosmos SDK [`crypto/keys/ed25519`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/crypto/keys/ed25519/ed25519.go) 패키지에 구현되어 있다.




## 3. Address
`Address`와 `PubKey`는 모두 애플리케이션에서 액터를 식별하는 공개 정보이다. `Account`은 인증 정보를 저장하는 데 사용된다. 기본 계정 구현은 `BaseAccount` 객체에 의해 제공된다.

각 계정은 공개 키에서 파생된 바이트 시퀀스인 `Address`를 사용하여 식별된다. Cosmos SDK에서는 계정이 사용되는 컨텍스트를 지정하는 3가지 유형의 주소를 정의한다:
- [`AccAddress`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/types/address.go#L132)는 사용자(메시지 발신자)를 식별한다.
  - Bech32를 사용하여 문자열로 표현된다. 이는 일반적인 송금, 계정 간 자산 전송, 스마트 계약과의 상호작용 등에 사용된다.
- [`ValAddress`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/types/address.go#L316)는 유효성 검사를 하는 검증자 노드를 식별한다.
  - Bech32를 사용하여 문자열로 표현된다. 이는 검증자 등록, 스테이킹(Staking), 검증자 투표 등에 사용된다.
- [`ConsAddress`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/types/address.go#L466)는 합의에 참여하는 노드를 식별한다.
  - Bech32를 사용하여 문자열로 표현된다. 이는 합의 알고리즘에서 블록 서명 및 검증자 합의 절차에 사용에 사용된다.

이러한 유형은 [`Address`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/types/address.go#L108-L117) 인터페이스를 구현한다:
```go
type Address interface {
	Equals(Address) bool
	Empty() bool
	Marshal() ([]byte, error)
	MarshalJSON() ([]byte, error)
	Bytes() []byte
	String() string
	Format(s fmt.State, verb rune)
}
```
- 참고로, `Marshal()` 및 `Bytes()` 메서드는 모두 동일한 원시 `[]byte` 형식의 주소를 반환한다. `Marshal()`은 Protobuf 호환성을 위해 필요하다.

### 공개 키로 AccAccount 생성하기 
Address 구성 알고리즘은 [ADR-28](https://github.com/cosmos/cosmos-sdk/blob/main/docs/architecture/adr-028-public-key-addresses.md)에 정의되어 있다. 다음은 `pub` 공개 키에서 account address를 얻는 표준 방법이다:
```go
sdk.AccAddress(pub.Address().Bytes())
```

### Account 가져오기 
사용자 상호작용을 위한 Address는 [Bech32](http://wiki1.kr/index.php/Bech32)를 사용하여 형식이 지정되고 [String 메서드](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/types/address.go#L281-L295)로 구현된다. Bech32 메서드는 블록체인과 상호작용할 때 사용할 수 있는 유일한 지원 형식이다. Bech32 사람이 읽을 수 있는 부분(Bech32 접두사)은 주소 유형을 나타내는 데 사용된다.

## 4. Keyring
Cosmos SDK에서 키는 [`Keyring`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/crypto/keyring/keyring.go#L53-L101)이라는 객체를 사용하여 저장하고 관리한다. 이는 여러 account를 저장하고 관리할 수 있다. 

유저가 다음과 같은 [`addKey`](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/client/keys/add.go) 명령어를 통해 key를 생성하게 되면 이는 `Keyring` 객체로 관리되게 된다: 
```sh
simd keys add <name>
```

# Resources
- https://docs.cosmos.network/main/
- https://cheapsslsecurity.com/p/ecc-vs-rsa-comparing-ssl-tls-algorithms/
- https://ida.interchain.io/academy/2-cosmos-concepts/2-accounts.html
- https://github.com/cosmos/cosmos-sdk/blob/main/docs/learn/beginner/03-accounts.md