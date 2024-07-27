# 04m05. AVL Tree 사용해보기

## 목차 
0. AVL Tree 사용해보기
1. 기본 설정하기
2. AVL Tree 함수 이해하기 
3. print 함수 구현하기
4. AVL Tree를 print 함수를 통해 출력하는 실행화면 제출 예시

## 0. AVL Tree 사용해보기
AVL 트리나 RB 트리와 같은 Balanced BST는 다양한 경우의 수를 따져서 구현해야 하기 때문에 비교적 코드가 길다. 그래서 해당 실습에서는 이미 구현된 AVL 트리의 일부 주요 함수를 이해하고 간단하게 트리 구조를 출력하는 함수를 구현해보도록 한다.

## 1. 기본 설정하기
기본 설정은 다음과 같다:
```sh
# map 디렉토리 생성
$ mkdir avl_tree && cd avl_tree

# map go module 생성 
$ go mod init avl_tree
```

## 2. AVL Tree 함수 이해하기 
현재 코드에는 AVL 트리에 값을 삽입하는 `Insert` 함수만 구현되어 있다. 해당 함수에는 호출하는 다양한 함수들을 존재한다: 
- `height`: AVL 트리의 노드 높이를 반환한다. 
- `getBalance`: 왼쪽 트리와 오른쪽 트리의 높이의 차를 계산하여 이를 반환한다. 한 쪽의 높이가 쏠리게 되면 skewed tree가 되기 때문에 밸런스를 잡는 데 중요한 지표로 사용된다. 
- `rightRotate`: AVL 트리의 노드를 오른쪽으로 회전하는 함수이다. 이는 전체적인 노드의 높이가 왼쪽으로 치우친 경우에 동작한다.
- `leftRotate`: AVL 트리의 노드를 왼쪽으로 회전한다. 이는 전체적인 노드의 높이가 오른쪽으로 치우진 경우에 동작한다.

`Insert` 함수에서 호출되는 이들 함수들은 트리의 균형을 유지하는 데 필수적이다. AVL 트리의 균형 상태를 유지하기 위해 노드를 삽입할 때 트리의 밸런스를 계산하고, 필요할 경우 회전 연산을 수행한다. 다음 코드는 Insert 함수 내에서 사용되는 회전 조건을 보여준다:
```go
// Left Left Case
if balance > 1 && value < n.left.value {
	return rightRotate(n)
}

// Right Right Case
if balance < -1 && value > n.right.value {
	return leftRotate(n)
}

// Left Right Case
if balance > 1 && value > n.left.value {
	n.left = leftRotate(n.left)
	return rightRotate(n)
}

// Right Left Case
if balance < -1 && value < n.right.value {
	n.right = rightRotate(n.right)
	return leftRotate(n)
}
```
이 조건들은 삽입된 값에 따라 트리의 균형을 맞추기 위해 필요하다. AVL 트리는 각 노드의 왼쪽 서브트리와 오른쪽 서브트리의 높이 차이가 1 이하가 되도록 유지해야 하므로, 이러한 회전 연산을 통해 균형을 유지한다.


주어진 코드는 다음과 같다: 
```go
package main

import (
	"fmt"
)

type AVLNode struct {
	value int
	left  *AVLNode
	right *AVLNode
	height int
}

func NewAVLNode(value int) *AVLNode {
	return &AVLNode{value: value, height: 1}
}

// AVL 트리의 높이를 계산하는 함수
func height(n *AVLNode) int {
	if n == nil {
		return 0
	}
	return n.height
}

// AVL 트리의 균형 인수를 계산하는 함수
func getBalance(n *AVLNode) int {
	if n == nil {
		return 0
	}
	return height(n.left) - height(n.right)
}

// 오른쪽 회전
func rightRotate(y *AVLNode) *AVLNode {
	x := y.left
	T2 := x.right

	// 회전 수행
	x.right = y
	y.left = T2

	// 높이 업데이트
	y.height = max(height(y.left), height(y.right)) + 1
	x.height = max(height(x.left), height(x.right)) + 1

	return x
}

// 왼쪽 회전
func leftRotate(x *AVLNode) *AVLNode {
	y := x.right
	T2 := y.left

	// 회전 수행
	y.left = x
	x.right = T2

	// 높이 업데이트
	x.height = max(height(x.left), height(x.right)) + 1
	y.height = max(height(y.left), height(y.right)) + 1

	return y
}

// 최대값을 계산하는 함수
func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

// AVL 트리에 값을 삽입하는 함수
func (n *AVLNode) Insert(value int) *AVLNode {
	if n == nil {
		return NewAVLNode(value)
	}

	if value < n.value {
		n.left = n.left.Insert(value)
	} else if value > n.value {
		n.right = n.right.Insert(value)
	} else {
		return n
	}

	n.height = 1 + max(height(n.left), height(n.right))

	balance := getBalance(n)

	// Left Left Case
	if balance > 1 && value < n.left.value {
		return rightRotate(n)
	}

	// Right Right Case
	if balance < -1 && value > n.right.value {
		return leftRotate(n)
	}

	// Left Right Case
	if balance > 1 && value > n.left.value {
		n.left = leftRotate(n.left)
		return rightRotate(n)
	}

	// Right Left Case
	if balance < -1 && value < n.right.value {
		n.right = rightRotate(n.right)
		return leftRotate(n)
	}

	return n
}

// todo: implementation
func printTree(root *AVLNode) {
	// fill out 
}

func main() {
	root := NewAVLNode(10)
	root = root.Insert(20)
    root = root.Insert(25)
	root = root.Insert(30)
	root = root.Insert(40)
	root = root.Insert(50)

	fmt.Println("Root:", root.value) // Root: 30
    // todo: printTree(root)

    root = root.Insert(5)
	root = root.Insert(7)
	root = root.Insert(9)
	root = root.Insert(11)
	root = root.Insert(15)
	fmt.Println("Root:", root.value) // Root: 20
    // todo: printTree(root)
   
}
```

## 3. print 함수 구현하기
이미 구현된 AVL Tree를 직접 출력해보면서 이해해보고 이러한 트리 구조를 쉽게 디버깅할 수 있도록 `printTree` 함수를 직접 작성해보도록 하자. 
> print 함수 구현된 실습 코드 확인하기: [04_avl_tree](../code/04_avl_tree/)


## 4. AVL Tree를 print하는 실행화면 제출 예시
프로그램을 실행하여 출력된 예시 결과는 다음과 같다:
<div style="text-align: center;">
   <img src="../assets/04_data_structure_avl_tree_result_example.png" alt="04_data_structure_avl_tree_result_example" width="600"/>
</div>
