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

// 트리의 계층 구조를 출력하는 함수
func printTree(root *AVLNode) {
	if root == nil {
		return
	}

	queue := []*AVLNode{root}
	for len(queue) > 0 {
		levelSize := len(queue)
		for i := 0; i < levelSize; i++ {
			node := queue[0]
			queue = queue[1:]
			fmt.Printf("%d ", node.value)

			if node.left != nil {
				queue = append(queue, node.left)
			}
			if node.right != nil {
				queue = append(queue, node.right)
			}
		}
		fmt.Println()
	}
}

func main() {
	root := NewAVLNode(10)
	root = root.Insert(20)
    root = root.Insert(25)
	root = root.Insert(30)
	root = root.Insert(40)
	root = root.Insert(50)

	fmt.Println("Root:", root.value) // Root: 30
    printTree(root)
    //      30 
    //   20    40 
    // 10  25    50 

    root = root.Insert(5)
	root = root.Insert(7)
	root = root.Insert(9)
	root = root.Insert(11)
	root = root.Insert(15)
	fmt.Println("Root:", root.value) // Root: 20
    printTree(root)
    //         20 
    //    10        30 
    //   7  11    25  40 
    // 5  9   15        50 

}
