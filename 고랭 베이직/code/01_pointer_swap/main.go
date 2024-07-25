package main

import "fmt"

func swap(x, y *int) {
    *x, *y = *y, *x
}

func main() {
    a, b := 5, 10
    fmt.Println("Before swap: a =", a, "b =", b) // Before swap: a = 5 b = 10
    swap(&a, &b)
    fmt.Println("After swap: a =", a, "b =", b) // After swap: a = 10 b = 5
}
