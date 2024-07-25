package main

import (
	"fmt"
)

func swap(x, y string) (string, string) {
	return y, x
}

func swap2(x, y string) (r1 string, r2 string) {
    r1, r2 = y, x
    return
}

func main() {
    x, y := "a", "b"
    cx, cy := swap(x, y)
    fmt.Println("x:", x, "y:", y) // x: a y: b
	fmt.Println("cx:", cx, "cy:", cy) // cx: b cy: a

	cx2, cy2 := swap2(x,y)
	fmt.Println("cx2:", cx2, "cy2:", cy2) // cx2: b cy2: a
}