# 01. Understanding Why modular blockcahins had come

Hello World !

이번 챕터에 있어서 가장 중요한 점은 코스모스 생태계를 이해하기 위해서는 공감해줘야 해! 내가 질문을 하나 던져볼게! **코스모스 SDK와 코스모스라는 App specific blockchain theme는 대체 왜 나왔을까?**

이 메인 질문은 머릿속에 품고 계속해서 설명을 이어나가보자

아! 여기선 굳이 컨센서스 레벨에 대해서는 건들지는 않을 거야. (오히려 공감하고 이해하는데 방해가 된다고 생각하거든)

## Review

다시 이 코스모스를 이해하려면 일단!! 큰 챕터에서 볼 때 과거의 블록체인들이 어떻게 발전해왔길래 코스모스가 출시될 수 있었을지를 보면 될거야. 왜냐? 새로운 기술은 기존의 기술의 문제점이 있다고 생각해서 아이디어를 얹혀서 개선하려고 하기 때문이지. 그렇다면, 우리가 기존 비트코인과 이더리움의 특징을 본다면 코스모스 SDK의 필요성에 공감하고 입문하는데 더 도움이 되지 않을까 싶어

아래 내용은 'Interchain Developer Academy'에서 0주차에 나오는 그림을 인용했어.

그림으로 보는게 편한데 내가 가져다가 쓰기 뭐해서.. 궁금하면 [여기](https://ida.interchain.io/ida-course/lps/week-0/#)로 가서 한번 보고 시작했으면 좋겠어.

자, 그럼 아래와 같이 비트코인, 이더리움, 텐더민트(현 CometBFT)&코스모스와 같은 블록체인 프레임들이 나왔다고 보자.
(그 사이 무수히 많았던 체인들이 있겠지만, 다른 체인들은 이해를 위해 생략)

- In 2008 years, Bitcoin Whitepaper was released

- In 2013 years, Ethereum Whitepare was released

- In 2016 years, Cosmoso Whitepaper was released

## Purpose of App specific chain

위의 내용으로 보면 우린 크게 비트코인 / 이더리움 / 코스모스로 나누어 본거지.

그리고 비트코인 -> 이더리움 / 이더리움 -> 코스모스를 생각해보자

우선 비트코인과 이더리움을 비교하기 전에 우린 비트코인을 payment용 블록체인 시스템이라고 간주할거야.

(물론 여기서 TPS 때문에 실질적인 사용성이 떨어지느니 같은 얘기는 뒤로하자)

다시 말해, 비트코인 앨리스가 밥에게 일정량의 코인(ex: 1BTC)를 보내기 위해 그리고 보낸다는 행위(하나의 트랜잭션)를 글로벌한 장부를 통해서 증명하기 위해 쓰이는 거라고 표현해볼 수 있어

지금 생각해보면 굉장히 단순히 돈을 보내고, 돈을 받고만 하는 단순한 페이먼트를 위한 용도의 블록체인 시스템이 비트코인이라고 볼 수 있지

단, 어떤 주고 받는 행위(트랜잭션 단위)를 보장해주고 인정하게 합의를 해주는 시스템(컨센서스 시스템) 위에 말이지.

0. 비트코인 (consensus system + payment)

1. 비트코인 -> 이더리움 (consensus system + computing)

   근데, 비탈릭이란 사람은 이런 생각을 한거야. 그 어떠한 글로벌한 합의 시스템이 존재한다면! 거기에 programable한 computing을 올릴 순 없을까? 란 생각이지

   이게 바로 현재의 Smart Contract(Program)가 생겨나고, Solidity란 언어가 생겨난 바탕이라고 볼 수 있어.

   내가 앞서 위에서 언급했던 이더리움이 비트코인과 다른 프레임을 제안했다고 볼 수 있는 거지.

2. 이더리움 -> 코스모스 (horizontal consensus system + computing)

   자, 여기서부터 각자 머릿속으로 집중을 하고 공감을 해보자. 내가 앞서 말했던 이더리움은 글로벌 컴퓨팅 시스템했잖아? 근데 그러면 지금 우리의 삶을 본다면 각자 노트북이나 컴퓨터가 하나씩 있는 시대에 글로벌 컴퓨터를 여러명이 사용할 수 있을까? 차라리 특정용도에 맞게 수평적인 확장을 하는 건 어떨까? 싶어지는거야.

   이런 프레임을 제안한게 재권이라고 볼 수 있는거지. 예시를 들어 설명을 해본다면. 이더리움을 어떠한 임의의 프로그램이든 올릴 수 있는 글로벌 컴퓨터를 하나 만들고 유저들이 올리고 싶은 거 올리면 돼! 라고 말하는 거라면.

   코스모스는 은행용 시스템에는 은행에 필요한 송금, 대출, 적금등의 딱 필요한 프로그램만 가지고 컴퓨팅 시스템을 구축하고, 또 다른 도서관 시스템이라고 하면 도서관에 필요한 대출, 연체등의 프로그램만 올라가있는 시스템을 만들어서 여러 목적에 맞게 컴퓨팅 시스템을 분할해서 구축하고 이를 연결지으면 어떠냐? 라고 본거지.

사실 이걸 그림을 그려서 표현하면 굉장히 좋은데.. 내가 여기 밑에다가 얼릉 하나 그려서 올려 놓을게.

그림.1 이더리움 같인 블록체인 위에 vm을 두고 여러 프로그램을 올림.

그림.2 블록체인 위에 애초에 딱 그 프로그램만 올림.

여기서 질문! 그럼 어떤 프레임이 넌 더 좋다고 생각해?

.
.
.
.
.
.
.
.
.
.

사실 정답은 없어. 음.. 내가 경험적으로 느끼기에는 EVM 같은 글로벌 컴퓨팅이 가능한 VM을 올리는 체인들은 좀 더 자유도가 높고 새로운 프로그램을 배포하는데 부담이 덜 되는 것 같아. 따로 뭐 합의가 필요하지는 않기 때문이지, 컨트랙트 자체적으로만 신경을 쓰면 되니까!

하지만, 반대로 여러명이 똑같은 프로그램을 굳이 불필요하게 반복해서 올리게 될 수도 있어. 생각해보면 굳이 EVM에는 잘 짜둔 은행 프로그램 하나만 올리고 여러명이 그걸 다같이 사용하면 되는데. EVM은 퍼블릭하게 열려있어서 똑같은 프로그램을 여러명이 다 올리고 각자 자기 프로그램을 사용하게 되는 경우지.

그리고, App specific chain이라고 표현한 코스모스는 모듈로서 어떤 프로그램을 관리하기 때문에 하나의 시스템 위에 불필요한 반복적인 프로그램이 올라가지 않고 딱 하나만 올리고 모두가 그걸 사용하게 되어있어. 좀 더 효율적이긴 하지. 다만 새로운 프로그램을 올려야한다면 그냥 배포하면 장땡이 아니라 그 체인에 관계된 여러 이해관계자들의 합의가 필요하게 되.

---

앞서 얘기했던 것들은 IDA에 표현된 자료들을 바탕으로 다시 요약해서 보자!

- From Bitcoint to Ethereum

> Bitcoin's monolithic codebase and limited scripting language made dApp development a tedious and complex process for developers.
> → Ethereum is a public blockchain with smart contract functionality that enables applications based on self-executing, self-enforcing, and self-verifying account holding objects.

- From Ethereum to Cosmos

dapp이 제너럴 체인에 올라가는 한계점

> Not all dApps have the same throughput requirements, but they all have to make do with the *average* resulting throughput of the platform itself if they are implemented on a general-purpose blockchain. This impacts the **scalability** of dApps.

## How does the interchain solve the scalability issue?

Scalability is a core challenge of blockchain technology. The interchain allows applications to scale to millions of users. This degree of scalability is possible as the interchain addresses two types of scalability:

Horizontal scalability: scaling by adding similar machines to the network. When "scaling out", the netwㄴork can accept more nodes to participate in the state replication, consensus observation, and any activity that queries the state.
Vertical scalability: scaling by improving the network's components to increase its computational power. When "scaling up", the network can accept more transactions and any
activity that modifies the state.

(ida week1 인용)

https://github.com/Jeongseup/jeongseupchain/blob/main/app/app.go

https://ida.interchain.io/ida-course/lps/week-0/#

---

그리고 마지막 plus +a

cosmwasm
ethermint
polarisEVM
etc..
