# 01-06. Function - Implement Swap Function

## 0. Function - Implementing Swap Functions
Let's try to implement a simple function that changes the values of x and y. You can find out from this practice.
- The function can return multiple values, making it easy to exchange values without additional data structures or temporary variables.
- Because the function argument is passed as a `value`, a copy of the argument is made when the function is called. Therefore, even if a variable is modified within a function, the existing variable is not affected.

## 1. Setting Preferences
The default settings are as follows:
```sh
# Create function_swap directory
$ mkdir function_swap && cd function_swap

# Create function_swap go module 
$ go mod init function_swap
```

## 2. Creating Swap Functions
Implement `swap` and `namedReturnSwap` functions that satisfy each condition:
1. The `swap` function takes two strings (x, y) as parameters and returns the two values in exchange.
2. Like the `swap` function, the `namedReturnSwap` function receives two strings (x, y) as parameters and exchanges the two values using the named return values (r1, r2).

You can call the implemented swap function in the main function as follows:
```go
func main() {
    x, y := "a", "b"
    cx, cy := swap(x, y)
    fmt.Println("x:", x, "y:", y) // x: a y: b
	fmt.Println("cx:", cx, "cy:", cy) // cx: b cy: a

	cx2, cy2 := namedReturnSwap(x,y)
	fmt.Println("cx2:", cx2, "cy2:", cy2) // cx2: b cy2: a
}
```
> Check the implemented practice code: [01_function_swap](../code/01_function_swap/)

## 3. Example of submitting a Swap program execution screen
The results printed by running the program are as follows:
<div style="text-align: center;">
   <img src="../assets/01_basic_function_swap_result_example.png" alt="01_basic_function_swap_result_example" width="600"/>
</div>

The Swap function created in this way performs the function of simply exchanging and returning two string values. This allows you to understand that a function can return multiple values in Go and that a copy of the argument is made when the function is called.