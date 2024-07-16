# Cosmos-SDK dApp 개발환경

미션들을 수행하기 위한 개발 환경을 설정한다.

## Terminal

Warp, iTerm2, Terminal 등

예제에서 입력은 $, 출력은 > 로 한다. #는 주석을 나타낸다.

```bash
$ echo A
> A
# 주석
```

## IDE

IDE는 자유롭게 사용하면 되지만, 미션 예제는 VSCode를 기반으로 한다.

VSCode https://code.visualstudio.com/

## nodejs 및 관련 패키지 설치

dApp 개발 미션은 nodejs, nextjs, shadcn/ui를 프론트앤드 개발에 사용하고, 패키지 매니저는 yarn을 사용하기로 한다.

```bash
#nodejs 설치
https://nodejs.org/en

$ npm i -g yarn
```

## 지갑 설치

Cosmos에는 [Cosmostation](https://chromewebstore.google.com/detail/cosmostation-wallet/fpkhgmpbidmiogeglndfbkegfdlnajnf), [Keplr](https://chromewebstore.google.com/detail/keplr/dmkamcknogkgcdfhhbddcghachkejeap) 등 지갑이 있다.

미션에서는 모든 지갑을 잘 활용할 예정이지만 EVM transaction(미션#9-EVM)까지 지원하는 Cosmostation 지갑을 활용해 예제를 진행할 예정이다.

## 테스트 앱체인 정보

미션에서는 Cosmos, Neutron, Osmosis 테스트넷을 사용한다. 각 체인 정보와 Faucet 정보를 통해 미리 준비를 한다.

- Cosmoshub Testnet
  - Explorer
    - https://www.mintscan.io/cosmoshub-testnet
  - Faucet
    - https://testnet.ping.pub/cosmos/faucet
- Neutron Testnet
  - Explorer
    - https://www.mintscan.io/neutron-testnet
    - https://neutron.celat.one/pion-1
  - Faucet
    - https://docs.neutron.org/neutron/faq/#where-is-the-testnet-faucet
- ## Osmosis Testnet
  - Explorer
    - https://www.mintscan.io/osmosis-testnet
  - Faucet
    - https://testnet.ping.pub/osmosis/faucet
