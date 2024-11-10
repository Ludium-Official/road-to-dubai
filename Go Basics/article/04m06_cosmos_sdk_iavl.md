# 04m06. Cosmos-SDK의 IAVL 다뤄보기

## 목차 
0. Cosmos-SDK의 IAVL 다뤄보기
1. 기본 설정하기
2. 코드 작성하기
3. IAVL 실행화면 제출 예시

### 3. (실습) Cosmos-SDK의 IAVL 다뤄보기
IAVL 트리는 다음과 같은 주요 연산을 제공한다:
- 삽입(Insertion): 새로운 키-값 쌍을 트리에 추가한다.
- 삭제(Deletion): 트리에서 특정 키를 삭제한다.
- 검색(Search): 특정 키의 값을 조회한다. 
- 검증(Verification): Merkle 트리의 해시 값을 통해 데이터의 무결성을 검증한다.

실습을 통해 다음과 같은 연산을 직접 사용해보도록 하자. 

## 1. 기본 설정하기
기본 설정은 다음과 같다:
```sh
# iavl 디렉토리 생성
$ mkdir iavl && cd iavl

# iavl go module 생성 
$ go mod init iavl
```

## 2. 코드 작성하기
다음과 같이 코드를 cosmos iavl 바이너리를 가져와서 직접 사용해보도록 하자.
```go
package main

import (
	"fmt"

	"cosmossdk.io/log"
	"github.com/cosmos/iavl"
	idb "github.com/cosmos/iavl/db"
)

func main() {
	// 메모리 데이터베이스 생성
	idb := idb.NewMemDB()

	// IAVL 트리 생성
	tree := iavl.NewMutableTree(idb, 100, false, log.NewNopLogger())
	
	// 키-값 쌍 삽입
	var value1 = []byte("value1")
	_, err := tree.Set([]byte("key1"), value1)
	if err != nil {
		fmt.Println(err)
	}
	_, err = tree.Set([]byte("key2"), []byte("value2"))
	if err != nil {
		fmt.Println(err)
	}
	
	// 트리 커밋 (루트 해시 생성)
	hash, version, err := tree.SaveVersion()
	if err != nil {
		fmt.Println(err)
	}
	fmt.Printf("Tree Version: %d, Root Hash: %X\n", version, hash) // Tree Version: 1, Root Hash: 3E6BA7783BA8C545FFE0755365F4C77C941602CA079EB55C9AF585F1339E53BB

	// 값 조회
	val, err := tree.Get([]byte("key1"))
	if err != nil {
		fmt.Println(err)
	}
	fmt.Printf("key1: %s\n", val) // key1: value1

	val, err = tree.Get([]byte("key2"))
	if err != nil {
		fmt.Println(err)
	}
	fmt.Printf("key2: %s\n", val) // key2: value2
	
	// 키-값 쌍 삭제
	_, success, err := tree.Remove([]byte("key1"))
	if success {
		fmt.Println("key1 삭제 성공")
	}
	if err != nil {
		fmt.Println(err)
	}

	// 삭제 후 값 조회
	val, err = tree.Get([]byte("key1"))
	if err != nil {
		fmt.Println(err)
	}
	fmt.Printf("key1: %s\n", val) // key1: 

	// 트리 커밋 후 루트 해시 출력
	hash, version, err = tree.SaveVersion()
	if err != nil {
		fmt.Println(err)
	}
	fmt.Printf("Tree Version: %d, Root Hash: %X\n", version, hash) // Tree Version: 2, Root Hash: 446B91A73E8164CD193AFDE5B378DCB8234697E20E89BA9D5651EC1C7352534D
}
```
> 실습 코드 확인하기: [04_iavl](../code/04_iavl/)


## 3. IAVL 실행화면 제출 예시
프로그램을 실행하여 출력된 결과는 다음과 같다:
<div style="text-align: center;">
   <img src="../assets/04_data_structure_iavl_result_example.png" alt="04_data_structure_iavl_result_example" width="600"/>
</div>

