# ETC Homeworks

1. BIP39가 무엇인지 알아보기

> Derive a new private key and encrypt to disk.
> Optionally specify a BIP39 mnemonic, a BIP39 passphrase to further secure the mnemonic,
> and a bip32 HD path to derive a specific account. The key will be stored under the given name
> and encrypted with the given password. The only input that is required is the encryption password.

**과제 : 만약 우리 체인이 여러명의 밸리데이터로 구성되어있다면 어떻게 될까? 블록이 생성되려면 몇 명이상이 합의에 동의해야할까?**

<!-- 과제 관련 내용

> As previously explained, a Cosmos SDK blockchain relies on identified validators to produce blocks. Initially there is no validator to generate blocks. You are in a catch-22 situation: your initialized and unstarted chain needs a genesis account and a validator for bootstrapping purposes.

> You must make your key, also known as an account, have an initial balance in the genesis file. For that, you need to know the staking denomination:
> In this scenario, for your network to even run you must meet the 2/3rds threshold of the weighted validators. -->
