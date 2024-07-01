# 코스모스의 어제와 오늘

### 1\. 코스모스 체인의 역사

![telegram-cloud-photo-size-1-5168403098585116704-c.jpg](https://i.ibb.co/RH0C5YL/telegram-cloud-photo-size-1-5168403098585116704-c.jpg)
([코스모스 코파운더 Ethan Buchman과 Jae Kwon](https://twitter.com/gogoDiegoCrypto/status/1804226130712449286))

* 코스모스는 2014년 지분 증명 방식으로 블록체인의 보안성을 제공하고자 했던 [Jae Kwon의 텐더민트(Tendermint) 아이디어에서 시작](https://blog.cosmos.network/cosmos-history-inception-to-prelaunch-b05bcb6a4b2b)했다. 텐더민트는 [비잔틴 장군 장애 허용(Byzantine Fault Tolerant)](https://ko.wikipedia.org/wiki/%EB%B9%84%EC%9E%94%ED%8B%B0%EC%9B%80_%EC%9E%A5%EC%95%A0_%ED%97%88%EC%9A%A9) 기반 컨센서스 알고리즘에 예치금과 잘못된 증거 생산에 대한 몰수(Slashing) 기능을 추가함으로서 지분 증명 기반 네트워크의 가능성을 제시했다.
* 이후 2015년 시스템 공학 석사 학위를 공부 중이던[ Ethan Buchman이 코파운더로 합류](https://iq.wiki/ko/wiki/ethan-buchman)하면서 텐더민트 기반 지분 증명 네트워크에 허브와 존(Hub and Zone) 방식으로 다양한 블록체인을 연결하는 [코스모스 백서를 출시](https://github.com/cosmos/cosmos/blob/master/WHITEPAPER.md)한다. 그리고 1680만 달러를 모금하면서 본격적인 프로젝트에 개발에 착수한다. 이후 2018년 CosmosSDK를 중심으로 체인들을 연결하는 인터 블록체인 커뮤니케이션(Inter-Blockchain Communications, IBC)가 출범하면서 재단(InterChain Foundation, ICF)을 중심으로 생태계가 본격적으로 태동한다.
* [코스모스 허브(Cosmos Hub 혹은 Gaia)는](https://hub.cosmos.network/main) CosmosSDK에 기반한 IBC 생태계 최초의 체인으로서 자체 토큰인 $ATOM을 기반으로 구동한다. $ATOM은 트랜젝션 수수료 지급을 위한 [유틸리티 토큰](https://github.com/Ludium-Official/road-to-dubai/blob/main/%EC%BD%94%EC%8A%A4%EB%AA%A8%EC%8A%A4%20%EB%B2%A0%EC%9D%B4%EC%A7%81/12_gas_fees.md)이자 IBC의 업데이트 결정권을 제공하는 [거버넌스 토큰](https://www.mintscan.io/cosmos/proposals)이다. 허브 출현 이후 오스모시스(Osmosis), 카바(Kava), 아카시(Akash)를 비롯해 수 많은 체인들이 파생되어 발전했다. 현재 CosmosSDK는 EVM 이외에 가장 많이 활용되는 웹3 기술 스택이다. 코스모스 생태계 존은 89개에 달하며 이더리움은 물론 솔라나, 리플, 카르다노, 니어를 비롯한 레이어 1체인에서도 IBC 기반 네트워크 소통이 가능하다.

### 2\. 코스모스 인센티브 일원화 문제의 발생

![image.png](https://i.ibb.co/197KjDD/image.png)
(이미지 출처 - [Cosmos Hub, 3 years of interchain expansion](https://twitter.com/cosmos/status/1759519043176784291/photo/1))

* 코스모스 기술 스택의 괄목할 성장에 반해 코스모스 허브와 기조 화폐인 $ATOM이 가진 입지는 크게 향상되지 못했다는 평가를 받았다. CosmosSDK의 특성상 체인에게 주권(Soveriegnty)이 보장되기 때문에 가스비와 같은 경제적 혜택의 공유가 이뤄지지 않는다. 다시 말해 CosmosSDK 기반의 네트워크가 얼마나 성장하는지와 코스모스 허브, 즉 $ATOM의 성장에는 상관 관계가 없었던 것이다. 반면 이더리움의 경우 레이어2 로드맵을 통해 이더리움 메인넷은 지분 증명을 통한 경제적 보안을 제공하고 예하 레이어 2는 궁극적으로 이더리움에 일부 수수료를 지급함으로서 확장성과 경제적 인센티브 상관성을 동시에 담보할 수 있게 되었다.
* 이러한 코스모스의 문제는 2022년 하락장과 테라의 몰락 이후 더욱 가시화된 위협으로 작용했다. CosmosSDK 기반 최대 네트워크 중 하나였던 테라가 순식간에 증발하고 이더리움이 지분 증명으로 전환하는 가운데 멀티체인으로의 로드맵이 성큼 다가오자 코스모스가 가진 최대의 장점이 퇴색되면서 코스모스가 ["존재적 위기(existential threat)"에 직면하고 있다](https://www.coindesk.com/tech/2023/07/26/once-a-pioneer-cosmos-blockchain-project-faces-existential-crisis/)는 평가를 받았다. 이 과정에서 가장 큰 문제로 지적된 것은 파편화된 생태계와 지휘부의 부재로 인한 코스모스의 방향성 상실이었다.

# $ATOM 되살리기 운동

### 1\. Ethan Buchman과 ATOM 2\.0

![image.png](https://i.ibb.co/7gxkGGd/image.png)(이미지 출처 - [Ethan Buchman & Sam Hart, The Cosmos Hub is a Port City](https://blog.cosmos.network/the-cosmos-hub-is-a-port-city-5b7f2d28debf))

* 코스모스의 존재적 위기가 가시화되는 가운데 코스모스의 코파운더 중 한 명인 Ethan Buchman은 코스모스 생태계를 되살리기 위해서는 허브 중심의 생태계 결성이 시급함을 역설하며 개혁 단행을 주장한다. 그는 [2022년 글을 통해](https://ebuchman.github.io/posts/phases-of-cosmos/) 코스모스가 성공적인 시작(Initiation) 단계를 지나 실질적인 도입(Integration)의 단계에 접어들었음을 천명한다. CosmosSDK는 가장 성공적인 인터체인 기술 스택으로서 무수히 많은 체인을 파생시켰지만 하나의 결속력 있는 생태계를 구축하지 못했다. 따라서 현실 세계에 도입이 될 수 있는 수준으로의 생태계 발전을 이루기 위해서는 코스모스 허브와 $ATOM 중심의 생태계 단일화가 필수적이라는 것이다.
* Ethan이 제안한 개혁안은 다음을 포함한다
    1. 조직 개편: 현재 사라진 Tendermint, Ignite와 현재 투명성 재고가 필요한 ICF의 신뢰성 회복 및 생태계 중심 조직 개편
    2. 인플레이션 감소: 연 15%가 넘는 $ATOM 인플레이션의 점진적 감소
    3. 경제적 보안 공유: 허브 체인 벨리데이터 공유를 통한 예하 체인(컨슈머 체인, Consumer Chain)의 보안성 보장
* 개혁안은 [2022년 ATOM 2.0 제안](https://forum.cosmos.network/t/proposal-82-rejected-atom-2-0-a-new-vision-for-cosmos-hub/7328)을 통해 정식 안건으로 상정되지만 [찬성 47%, 반대 37%, 기권 13%로 부결](https://www.mintscan.io/cosmos/proposals/82)된다. 그러나 $ATOM과 코스모스 생태계의 위기에 대한 공감대는 이미 형성되어 있었기 때문에 Ethan은 Informal Systems를 새롭게 조직하여 허브에 필요한 기술적 지원을 제공함과 동시에 [개혁 실행을 위한 준비](https://medium.com/the-interchain-foundation/introducing-the-informal-hypha-cosmos-hub-roadmap-860c41594fe8)를 다져갔다. 결국 인터체인 보안성(Interchain Security)에 기반한 [코스모스 2024 로드맵을 발표](https://docs.google.com/document/d/1GZ3ebosxwOwrqExekG1N74NDJ5rGtigq2MXOjbfReaw/edit#heading=h.mqu46rxxg27c)하며 새로운 청사진을 그려나가고 있다.

### 2\. 아톰 경제 권역\(ATOM Economic Zone\, AEZ\)의 출현

![image.png](https://i.ibb.co/0MtcjzG/image.png)
(이미지 출처 - [Cosmos Hub, AEZ 공식 라이브 발표](https://twitter.com/cosmoshub/status/1709542369404551331))

* 코스모스 2024 로드맵은 다음의 핵심 내용을 포함한다
    1. 보안성과 호환성(Security and Composability): 코스모스 허브와 벨리데이터를 공유하는 인터체인 보안 기반의 컨슈머 체인 확장을 통해 아톰 경제 권역(AEZ) 도입 체인을 늘린다.
    2. 유동성(Liquidity): 프로토콜 소유 유동성(Protocol Owned Liquidity, POL) 확장과 리퀴드 스테이킹 토큰(Liquid Staking Token, LST) 출시를 통해 AEZ 체인의 유동성 및 담보 자산 기능을 강화한다.
    3. IBC 라우팅(IBC Routing): IBC의 패킷 라우팅 방식의 업데이트를 통해 인터체인 간의 릴레이와 클라이언트 업데이트 비용을 감소시킴으로서 허브로의 패킷 전송 방식 용이성을 높인다. 먼저 AEZ 컨슈머 체인에 라우팅을 도입하여 편의성을 입증하고 이후 CosmosSDK를 사용하는 모든 인터체인으로의 확장을 추구한다.
* 허브의 새로운 방향성 발표 이후 [거버넌스 제안이 매우 활성화](https://medium.com/simplystaking/the-cosmos-hub-an-introduction-to-the-internet-of-blockchains-c856949d4733)되고 생태계의 변화를 견인하기 위한 조직들이 두각을 나타내고 있다. 이 중 가장 대표적인 조직이 [아톰 엑셀러레이터 다오(ATOM Accelerator DAO, AADAO)](https://github.com/gaiaus/aadao)이다. AADAO는 AEZ 저변 확대를 위한 프로젝트에 자금을 수혈하는 생태계 육성 조직이다. 컨슈머 체인 온보딩 뿐 아니라 AEZ에 필요한 교육부터 인프라 개발, 프로젝트 발굴까지 전방위적 영역에서 생태계 프로젝트를 지원하는 역할을 수행하고 있다.
* 예를 들어 AADAO는 코스모스 생태계의 안정화와 확장을 위해 [8가지 아이디어](https://forum.cosmos.network/t/recap-of-the-8-ideas-from-the-tokenomics-rfp/13824)에 대한 리서치를 진행하고 있다. 여기에는 $ATOM의 인플레이션율 조정, 커뮤니티 풀 운영 방안, 개발자 수익 인센티브 제안(DRIP), 쿼드라틱 투표(Quadratic Voting) 등의 방안이 포함되어 있다.

# AEZ의 현재와 코스모스의 미래

### 1\. AEZ 컨슈머 체인 현황

![image.png](https://i.ibb.co/kB1VCjR/image.png)
(이미지 출처 - [Cosmos Hub, Should AEZ Chains have their own tokens?](https://forum.cosmos.network/t/should-aez-chains-have-their-own-tokens/11652))

* 현재 등록된 AEZ 컨슈머 체인은 다음을 포함한다.
    1. [Neutron](https://twitter.com/Neutron_org/status/1805703229537403321): CosmWasm 기반의 통합 어플리케이션(Integrated Application) 체인으로 스마트 컨트랙트 배포만으로도 앱체인과 같은 성능을 제공하는 어플리케이션 인프라 체인이다.
    2. [Stride](https://twitter.com/stride_zone): IBC 기반 토큰들의 LST를 제공하는 체인이다.
    3. [Noble](https://twitter.com/noble_xyz): $USDC를 발행한 Circle에서 개발한 체인으로 스테이블 토큰 기반 통화 발행, 관리를 담당하는 체인이다.
* 2024년 6월 현재 AEZ의 파트너십은 공격적인 속도로 증가하고 있다. 승인 대기 중인 네트워크는 EVMOS와 같은 이더리움 호환 체인을 비롯하여 COMDEX, ELYS NETWORK와 같은 디파이 앱체인을 포함한다.

### 2\. 코스모스의 미래는 어떤 모습일까?

![image.png](https://i.ibb.co/4WFk7Ks/image.png)
(이미지 출처 -[ System Flows from The Cosmos Hub is a Port City)](https://blog.cosmos.network/the-cosmos-hub-is-a-port-city-5b7f2d28debf)

* Ethan은 개혁의 시작을 알린 2021년 발표한 코스모스 허브는 항구 도시이다(The Cosmos Hub is a Port City)에서 다음과 같이 말한다.
    * `이것(코스모스 허브)은 연결로 풍족해지는 도시이다. 이것과 연결된 도시를 지원하는 도시인 것이다. 거주자에게 경제적, 정치적 시스템을 강요하는 대신 허브는 최선의 서비스를 제공함으로 공평하게 경쟁한다.`
* 코스모스 생태계는 지난 수 년간 빠른 확장에 대비하여 내부적 분열과 파편화된 조직 체계로 인해 존재 자체의 위기를 겪었다. 이제 새로운 조직 결성과 활성화된 거버넌스, 그리고 경제 전략적 방향성 아래 출항을 시작하는 프로젝트들의 새로운 귀추가 주목된다.