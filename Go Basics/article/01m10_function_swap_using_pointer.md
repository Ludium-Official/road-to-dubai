# 01-10. Using Pointer to Implement Swap Functions

## 0. Using Pointer to Implement Swap Functions
[`01m06_function_swap`](./01m06_function_swap.md) The swap example in the practice was simply a function of exchanging values. Here, let's create a function that changes the values of a and b by directly modifying the values at the memory address using a pointer. The operator '&' is used to transfer the addresses of a and b to the function.

## 1. Setting Preferences
The default settings are as follows:
```sh
# Create pointer_swap directory
$ mkdir pointer_swap && cd pointer_swap

# Create pointer_swap go module 
$ go mod init pointer_swap
```

## 2. Create a Swap function using Pointer
Write a `swap` function that exchanges the values of two integers using a pointer:
- The `swap` function takes two int-type pointers (x, y) as parameters, exchanges and does not return the two values.

The swap function implemented using pointer can be called in the main function as follows:
```go
func main() {
    a, b := 5, 10
    fmt.Println("Before swap: a =", a, "b =", b) // Before swap: a = 5 b = 10
    swap(&a, &b)
    fmt.Println("After swap: a =", a, "b =", b) // After swap: a = 10 b = 5
}
```
> Check the implemented practice code: [01_pointer_swap](../code/01_pointer_swap/)

## 3. Example of submission of the Swap function program execution screen using Pointer
The results printed by running the program are as follows:
<div style="text-align: center;">
   <img src="../assets/01_basic_pointer_swap_result_example.png" alt="01_basic_pointer_swap_result_example" width="600"/>
</div>

Through this exercise, we learned how to directly modify values from functions using pointers. Using pointers, you can directly change the value of a variable within the function, since the function refers to the actual memory address rather than a copy of the variable.