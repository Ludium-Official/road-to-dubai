# 32. 컨트랙트 배포하기 
## 목차
0. Neutron
1. Neutron 테스트넷 faucet 받기 
2. namespace 컨트랙트 빌드하기
   1. 기본으로 빌드하기
   2. wasm 사이즈 압축해서 빌드하기 
3. celatone으로 테스트넷에 배포하기

## 0. Neutron
[Neutron](https://docs.neutron.org/)은 Cosmwasm을 사용해 스마트 컨트랙트를 Cosmos 계열 블록체인에 도입한 블록체인 네트워크이다. Neutron은 IBC 프로토콜을 사용하는 네트워크에서 작동한다. Neutron 보안(블록 검증)은 ICS(Interchain Security)을 통해 Cosmos Hub 네트워크에서 제공한다. 

Mintscan Explorer를 통해 메인넷과 테스트넷에 대한 블록 현황을 살펴볼 수 있다: 
- Mainnet(`neutron-1`): https://mintscan.io/neutron; 
- Testnet(`pion-1`): https://mintscan.io/neutron-testnet. 

[Celatone](https://neutron.celat.one/neutron-1)은 컨트랙트 업로드, 쿼리, 실행하는 UI를 제공하는 스마트 컨트랙트 explorer이다. 추후 이를 통해 컨트랙트를 Neutron 테스트넷에 배포해 볼 예정이다.

## 1. Neutron 테스트넷 faucet 받기 
다음 명령어로 [telegram faucet](https://t.me/+SyhWrlnwfCw2NGM6)을 사용할 수 있다:
```
/request <NEUTRON-ADDRESS>
```

## 2. namespace 컨트랙트 빌드하기
### 1. 기본으로 빌드하기
```sh
$ cargo wasm
```

사이즈 크기를 확인해보자:
```sh
$ ls -lh ./target/wasm32-unknown-unknown/release/

# ... 1.6M  7 12 23:53 namespace.wasm
```

### 2. wasm 사이즈 압축해서 빌드하기 
현재 1.6MB임을 알 수 있다. 현재 업로드할 수 있는 최대 사이즈의 크기는 800KB이기 때문에 사이즈를 더 압축해줘야 한다. 
1. RUSTFLAGS 를 통해 빌드 파일 용량 줄이기
2. Cosmwasm의 [rust optimizer](https://github.com/CosmWasm/optimizer)로 파일 용량 줄이기

우선 가장 간단한 1번 방식을 사용해서 줄여보자:
```sh
RUSTFLAGS='-C link-args=-s' cargo wasm
```

빌드가 완료된 후 사이즈 크기를 확인해보자:
```sh
$ ls -lh ./target/wasm32-unknown-unknown/release/

# ... 195K  7 13 00:06 namespace.wasm
```

그러면 용량이 195KB로 줄어서 배포 가능한 크기가 되었음을 확인할 수 있다. 이대로 배포해도 되지만 cosmwasm이 제공해주는 2번 방법을 사용해보면 크기를 더 작게 압축할 수 있다. 명령어는 다음과 같다:
```sh
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/optimizer:0.16.0
```
> M1?

빌드가 완료된 후 사이즈 크기를 확인해보자:
```sh
$ ls -lh ./artifacts

# ... 165K  7 13 00:12 namespace.wasm
```
그러면 더 작은 크기인 165KB가 된 것을 확인할 수 있다. 


## 3. celatone으로 테스트넷에 배포하기
GUI를 제공하고 있어서 쉽게 테스트넷에 배포할 수 있다. 
- https://neutron.celat.one/pion-1/deploy

### 1. wasm 파일 업로드하기
![](./assets/32_contract_upload_1.png)
![](./assets/32_contract_upload_2.png)

### upload 완료 
그러면 다음과 같이 code id와 트랜잭션 해시 값을 결과로 받을 수 있다:
- code id: 5509
- hash: 5497E176EBE4107F6BCD65A96071325D8E3DFD75201F17BA89FC421D76D9BC6E
![](./assets/32_contract_upload_complete.png)



## Resources
- https://docs.neutron.org/