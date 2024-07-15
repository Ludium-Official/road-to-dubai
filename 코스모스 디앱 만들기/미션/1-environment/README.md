# dApp 개발환경 만들기

미션들을 수행하기 위한 개발 환경을 설정한다.

## Terminal

Warp, iTerm2, Terminal 등

예제에서 입력은 $, 출력은 > 로 한다. #는 주석을 나타낸다.

```
$ echo A
> A
# 주석
```

## nodejs 및 관련 패키지 설치

```
#nodejs 설치
https://nodejs.org/en

$ npm i -g yarn

#cosmos-kit 설치
$ npm i -g create-cosmos-app
```

## IDE

VSCode https://code.visualstudio.com/

## 지갑 설치

Cosmos에는 [Cosmostation](https://chromewebstore.google.com/detail/cosmostation-wallet/fpkhgmpbidmiogeglndfbkegfdlnajnf), [Keplr](https://chromewebstore.google.com/detail/keplr/dmkamcknogkgcdfhhbddcghachkejeap) 등 지갑이 있다.

미션에서는 모든 지갑을 잘 활용할 예정이지만 EVM transaction(미션#11)까지 지원하는 Cosmostation 지갑을 활용해 예제를 진행할 예정이다.

지갑을 설치하고 Neutron Testnet을 추가한다.


## 미션을 위한 앱체인 정보

- Neutron
  - https://www.mintscan.io/neutron-testnet
- Cosmos
  - https://www.mintscan.io/cosmoshub-testnet
