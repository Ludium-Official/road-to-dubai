# 004. Go Data Structure
> 이 아티클에서는 Go 언어의 주요 데이터 구조에 대해 다룬다. 배열(Array)과 슬라이스(Slice), 맵(Map), 큐(Queue), 스택(Stack), 트리(Tree) 등을 통해 데이터를 효율적으로 관리하고 사용할 수 있는 방법을 설명한다. 특히, 트리 구조를 이해하는 것은 Cosmos-SDK의 상태 저장에 사용되는 IAVL 트리를 이해하는 데 도움이 된다. 이를 활용한 실습 과제를 통해 자료구조의 이해도를 더욱 높이는 것을 목표로 한다. 

## 목차 
1. (Built-in) 배열(Array)과 슬라이스(Slice)
   1. Array
   2. Slice
   3. Array와 Slice의 차이점
2. 큐(Queue)와 스택(Stack)
3. (Built-in) 맵(Map)
4. 트리(Tree)
   1. 이진 검색 트리(BST, Binary Search Tree)
   2. AVL Tree
5. Cosmos-SDK의 IAVL 트리 

## 1. 배열(Array)과 슬라이스(Slice)
Array와 Slices는 가장 기본적인 자료구조로, Go에서 직접적으로 지원해준다. 

### 1. Array
배열(Array)은 고정된 크기의 동일한 타입 요소의 집합이다. 배열의 크기는 선언 시 정해지며 변경할 수 없다.

### 2. Slice
슬라이스(Slice)는 동적 배열로, 배열과 달리 크기를 유연하게 조절할 수 있다. Slice는 배열의 부분집합을 참조할 수 있으며, 배열보다 훨씬 더 자주 사용된다.

### 3. Array와 Slice의 차이점
- Array는 크기를 고정으로 할당이고, Slice는 크기를 동적으로 할당한다. Slice는 Array와 달리 선언에 미리 크기를 지정하지 않아도 된다. 
- Slice는 배열의 부분집합을 참조할 수 있다.
- Slice는 더 많은 기능을 제공하며, Array보다 더 자주 사용된다. 

## 2. 큐(Queue)와 스택(Stack)
큐(Queue) 스택(Stack)은 각각 FIFO(First-In-First-Out)와 LIFO(Last-In-First-Out) 원칙을 따르는 데이터 구이다. 

## 3. 맵(Map)
맵(Map)은 Key-Value 쌍을 저장하는 데이터 구조이다. Map의 주요 특성은 다음과 같다:
- Key는 유니크해야 한다. 
- Key와 Value의 타입은 같아야 한다. 
- 크기는 동적으로 할당된다. 
- Key를 통해 Map에 저장된 데이터의 빠른 접근(O(1))이 가능하다.

## 4. 트리(Tree)
트리(Tree)는 계층적인 데이터 구조로, 노드로 구성되며 각 노드는 자식 노드를 가질 수 있다. Array, Slice와 같이 선형적인 구조가 아니라 부모-자식 개념을 가지는 비선형적인 구조를 가지는 자료구조이다. 가장 흔한 트리 형태는 이진 트리(Binary Tree)와 그 변형인 이진 검색 트리(Binary Search Tree)이다. Go에서는 다양한 트리 구조를 구현할 수 있다.

### 1. 이진 검색 트리(BST, Binary Search Tree)
이진 검색 트리는 각 노드가 최대 두 개의 자식 노드를 가지며, 왼쪽 자식 노드는 부모 노드보다 작고, 오른쪽 자식 노드는 부모 노드보다 크다. 
- 계층적 구조: 데이터의 계층적 표현을 가능하게 한다.
- 효율적인 검색: 이진 검색 트리는 평균적으로 O(log n)의 검색 시간 복잡도를 제공한다.

이진 검색 트리의 구현된 코드는 다음과 같다:
```go
package main

import (
	"fmt"
)

type Node struct {
	value int
	left  *Node
	right *Node
}

// 새로운 노드를 추가하는 함수
func (n *Node) Insert(value int) {
	if value < n.value {
		if n.left == nil {
			n.left = &Node{value: value}
		} else {
			n.left.Insert(value)
		}
	} else {
		if n.right == nil {
			n.right = &Node{value: value}
		} else {
			n.right.Insert(value)
		}
	}
}

// 트리에서 값을 찾는 함수
func (n *Node) Search(value int) bool {
	if n == nil {
		return false
	}
	if value < n.value {
		return n.left.Search(value)
	} else if value > n.value {
		return n.right.Search(value)
	}
	return true
}

func main() {
	root := &Node{value: 10}
	root.Insert(5)
	root.Insert(15)
	root.Insert(3)
	root.Insert(7)

	fmt.Println(root.Search(7))  // true
    fmt.Println(root.Search(3))  // true
	fmt.Println(root.Search(12)) // false
}
```
> 예제 코드 확인하기: [04_bst](../code/04_bst/)


#### 이진 검색 트리의 문제점 
이진 검색 트리(Binary Search Tree, BST)는 간단하고 직관적인 트리 구조로 데이터의 삽입, 삭제, 검색을 평균적으로 O(log n) 시간 복잡도로 수행할 수 있다. 하지만, 특정한 상황에서는 성능이 크게 저하될 수 있는 문제점이 존재한다. 해당 트리는 기본적으로 검색, 삽입, 삭제는 모두 트리의 높이에 의존되는 구조이기 때문이다. 

자체적으로 균형을 잡을 수 없기 때문에 데이터 순서에 따라 편향 트리(skewed tree)가 될 수도 있다. 편향 트리에서는 트리의 높이가 n에 가까워지며, 이 경우 검색, 삽입, 삭제의 시간 복잡도가 O(n)으로 증가한다. 이는 리스트 구조와 동일하게 되어, BST의 장점을 잃게 된다. 그래서 고안된 것이 Balanced BST이다.  

가장 간단한 예시로는, 만약 각 노드에 무작위로 우선순위를 두면 다음과 같이 밸런스있는 트리를 만들 수 있다:
<div style="text-align: center;">
   <img src="../assets/04_skewed_tree.png" width="600"/>
</div>

### 2. AVL 트리(AVL Tree)
AVL 트리는 위에서 구현해 본 균형 이진 검색 트리(Balanced BST)의 일종으로, 각 노드의 왼쪽과 오른쪽 서브트리의 높이 차이가 1 이하가 되도록 균형을 유지한다. 이는 삽입과 삭제 시 트리가 자동으로 균형을 유지하도록 회전 연산을 수행하여 검색, 삽입, 삭제 연산이 항상 O(log n)의 시간 복잡도를 가지도록 한다.
- 기본 이진 탐색 트리에서 편향 트리와 같은 불균형 구조의 단점을 극복하기 위해 고안되었다.
- 균형 유지: AVL 트리와 같은 균형 트리는 삽입과 삭제 연산 후에도 트리의 균형을 유지하여 성능을 보장한다.


## 5. Cosmos-SDK의 IAVL 트리 
블록체인은 상태를 저장하는 데이터베이스이고 합의를 통해 상태를 변경하는 SMR(상태 머신 복제) 분산 시스템이다. IAVL 트리는 이러한 블록체인 상태를 효율적으로 관리하고 검증하는 데 중요한 역할을 한다. 이를 위해 Cosmos SDK에서는 AVL 트리의 변형인 IAVL 트리를 사용한다.

IAVL 트리는 이진 검색 트리, AVL 트리, 그리고 Merkle 트리의 특성을 결합한 데이터 구조이다. 이는 주로 블록체인에서 상태를 저장하고 검증하는 데 사용된다. IAVL 트리는 다음과 같은 주요 특성을 가진다:
- 이진 검색 트리: 각 노드는 최대 두 개의 자식 노드를 가지며, 왼쪽 자식 노드는 부모 노드보다 작고, 오른쪽 자식 노드는 부모 노드보다 크다.
- AVL 트리: 각 노드의 왼쪽과 오른쪽 서브트리의 높이 차이가 1 이하가 되도록 균형을 유지한다. 이는 삽입과 삭제 시 트리가 자동으로 균형을 유지하여 검색, 삽입, 삭제 연산이 항상 O(log n)의 시간 복잡도를 가지도록 한다.
- Merkle 트리: 각 노드가 해시 값을 가지며, 이를 통해 트리의 모든 하위 노드의 데이터 무결성을 검증할 수 있다. Merkle 트리는 블록체인의 상태를 암호학적으로 안전하게 관리하는 데 필수적이다.

이러한 IAVL 트리는 다음과 같은 장점을 제공한다:
- 효율적인 검색, 삽입, 삭제: 트리의 균형을 유지하여 이러한 연산이 항상 O(log n)의 시간 복잡도를 가지도록 한다.
- 암호학적 검증: Merkle 트리의 특성을 이용하여 데이터의 무결성을 검증할 수 있다. 이는 블록체인의 상태를 안전하게 관리하는 데 필수적이다.
- 버전 관리: IAVL 트리는 상태의 버전을 관리하여, 특정 시점의 상태로 되돌아갈 수 있는 기능을 제공한다. 이는 블록체인의 상태 롤백과 검증에 유용하다.


# Resources 
1. Go Docs, "The Go Programming Language Specification: Language version go1.22", Feb 6. 2024, https://go.dev/ref/spec
3. Cosmos IAVL Spec, https://github.com/cosmos/iavl/blob/master/docs/overview.md