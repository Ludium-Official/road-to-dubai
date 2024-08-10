# 코즘와즘 베이직 모듈 
해당 모듈은 Cosmwasm에 대해 다루는 교육 자료이다. 
- 0~5: wasm 기초와 cosmwasm 기술에 대한 기초적인 내용을 다룬다. 
- n0~n5: 아티클 형식으로 nameservice 컨트랙트를 구현하기 위한 기본적인 설정과 함수를 직접 구현해본다.
- nm0~nm3: 직접 구현한 nameservice 컨트랙트를 직접 배포하고 초기화하며, register와 transfer 함수를 실행하며 결과를 쿼리해본다.

## 모듈 구성
| # | Topic | Type | Description | Link |
|---|-------|------|-------------|------|
| 1 | wasm 기초 | Article | Cosmwasm 기술의 근간이 되는 WASM 기술과 이와 연관된 WASI, Wasmer에 대해 간략하게 알아본다 | [Wasm Basic](./01_wasm_basic.md) |
| 2 | cosmwasm 기초 | Article | Cosmwasm의 배경과 특징 그리고 Cosmos SDK와 어떻게 상호작용하는지 알아본다  | [Cosmwasm Basic](./02_cosmwasm_basic.md) |
| 3 | cosmwasm - state | Article | Cosmwasm 컨트랙트가 데이터를 저장하고 조회하는 State에 대해 알아본다 | [Cosmwasm State](./03_state.md) |
| 4 | cosmwasm - Entrypoint | Article | Cosmwasm 컨트랙트에서 사용하는 `entry_point` 매크로 함수에 대해서 알아본다 | [Cosmwasm Entrypoint](./04_entrypoint.md) |
| 5 | cosmwasm - Message & Event | Article | CosmWasm 스마트 컨트랙트와 상호작용하는 메시지 종류와 Event에 대해서 알아본다 | [Cosmwasm Message & Event](./05_message_and_event.md) |
| 6 | cosmwasm - Query | Article | CosmWasm 스마트 컨트랙트에서 쿼리를 사용하는 방법에 대해 알아본다 | [Cosmwasm Query](./06_query.md) |
| 7 | CosmWasm 기본 설정하기 | Article | nameserivce 빌드를 위해 CosmWasm 기본적인 환경을 설정한다 | [CosmWasm 기본 설정하기](./07_build_nameservice.md) |
| 8 | EntryPoint 함수 생성하기 | Article | EntryPoint 함수를 추가해보며 직접 빌드해본다  | [EntryPoint 함수 생성하기](./08_create_entrypoint.md) |
| 9 | instantiate 함수 구현하기 | Article | nameserivce의 instantiate 함수를 구현한다 | [Instantiate 구현하기](./09_impl_instantiate.md) |
| 10 | register 함수 구현하기 | Article | 사용자가 입력한 이름을 등록하는 register 함수를 구현한다 | [Register 구현하기](./10_impl_register.md) |
| 11 | register 함수 개선하기 - 수수료 검증 | Article | register 함수에 사용자로부터 충분한 수수료가 지불되었는지 검증하는 로직을 추가해본다 | [Register 개선하기 - InsufficientFundsSend](./11_improvement_insufficeint_coin.md) | 
| 12 | register 함수 개선하기 - name 입력 데이터 검증 | Article | register 함수에 사용자가 입력한 이름이 요청된 name의 길이와 잘못된 문자열을 사전 필터링하는 로직을 추가해본다 | [Register 개선하기 - NamValidation](./12_improvement_name_validation.md)
| 13 | transfer 함수 구현하기 | Article | 등록된 이름을 다른 사용자에게 전송하는 transfer 함수를 구현한다 | [Transfer 구현하기](./13_impl_transfer.md) |
| 14 | schema 생성하기 | Article | nameservice의 schema를 생성한다 | [Schema 생성하기](./14_create_schema.md) |
| 15 | nameservce 컨트랙트 배포하기 | Mission | `n0`~`n5` 아티클을 통해 구현한 nameservice를 Neutron 테스트넷 네트워크에 배포해본다 | [Nameservice 배포하기](./15_deploy_contract.md) |
| 16 | nameservice 컨트랙트 초기화하기 | Mission | 배포한 컨트랙트를 직접 초기화해본다 | [Nameservice 초기화하기](./16_instantiate_contract.md) |
| 17 | nameservice 컨트랙트 register 함수 실행하기 | Mission | 배포한 컨트랙트에 register 함수를 실행하여 이에 대한 결과를 직접 쿼리해본다 | [Register 함수 실행하기](./33a_register_tx_and_query.md) |
| 18 | nameservice 컨트랙트 transfer 함수 실행하기 | Mission | 배포한 컨트랙트에 transfer 함수를 실행하여 이에 대한 결과를 직접 쿼리해본다 | [Transfer 함수 실행하기](./33b_transfer_tx_and_query.md) |


## 제안 및 추가 
- 코즘와즘 베이직 교육 모듈은 오픈 소스 컨트리뷰션을 통해 지속적으로 자료를 보완, 발전시킨다.
- 현존하는 모듈에 제안을 원하는 빌더는 Issue를 통해 제안 내용을 작성하거나 리포를 포킹해서 개선된 내용을 Pull Request로 바로 요청할 수도 있다.
- 제안, 요청된 내용은 루디움에서 검토 이후 적절성을 판단하여 자료를 업데이트 한다.
