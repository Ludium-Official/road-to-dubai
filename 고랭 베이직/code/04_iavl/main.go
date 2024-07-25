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