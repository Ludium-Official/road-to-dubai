# 002. Go Control Structure
> This article deals with the control structure of the Go language. Explain how to control the flow of a program through a control structure such as a conditional statement (if, else), a repetitive statement (for), and a switch statement. It provides grammar and use examples of each control structure, and aims to improve skills through practical tasks using it.

## 1. Conditional statement: if, else if, else
In Go, conditional statements are defined using the if, if, and else keywords. Conditional statements allow code blocks to be executed or skipped according to specific conditions.

### 1. Default if statement
If the condition is true, the code block is executed. The following shows how to check that x is greater than 5:
```go
package main

import (
	"fmt"
)

func main() {
	x := 10
	if x > 5 {
		fmt.Println("x is greater than 5")
	}
}
```

### 2. if-else clause
If the condition is true, the first code block is executed, and if it is false, the second code block is executed. The following shows how to check that x is greater than 5, and if it is not too large, to execute the else block: 
```go
package main

import (
	"fmt"
)

func main() {
	x := 3
	if x > 5 {
		fmt.Println("x is greater than 5")
	} else {
		fmt.Println("x is not greater than 5")
	}
}
```

### 3. if-else if-else clause
It checks several conditions sequentially. It executes the code block of the first true condition, and if all conditions are false, it executes the last else block. The following shows how you can add an else if statement to make the condition branch more detailed:
```go
package main

import (
	"fmt"
)

func main() {
	x := 8
	if x < 3 {
		fmt.Println("x is less than 3")
	} else if x < 7 {
		fmt.Println("x is less than 7 but greater than or equal to 3")
	} else {
		fmt.Println("x is greater than or equal to 7")
	}
}
```

### 4. Declaration of variables in conditional statements
It is possible to declare variables within an if statement. This variable is valid only within the if statement block. The following shows how to initialize x to 10 within a condition statement, and check for x > 5 conditions:
```go
package main

import (
	"fmt"
)

func main() {
	if x := 10; x > 5 {
        // x 존재 
		fmt.Println("x is greater than 5")
	}
    // x 존재하지 않음
}
```

## 2. Repeated cluase: for, break, continue
Go uses the for statement as the only iteration. You can use it in many ways.

### 1. Basic for clause
It is the basic form including initialization, conditions, and post-processing. The following is an example code that initializes i to 0, repeats while i is less than 5, and outputs the value of i by increasing i after each iteration:
```go
package main

import (
	"fmt"
)

func main() {
	for i := 0; i < 5; i++ {
		fmt.Println(i)
	}
}
```

### 2. For clause with conditional expression only
It is possible to have a form with only a conditional expression without initialization and post-processing. The following is an example code that performs the above functions but only has a conditional expression:
```go
package main

import (
	"fmt"
)

func main() {
	i := 0
	for i < 5 {
		fmt.Println(i)
		i++
	}
}
```

### 3. Infinite Loop
If a condition is not specified in the repetition sentence, a function that is repeatedly executed infinitely can be implemented. The concept of an infinite loop often appears in blockchain. This is because if these functions are executed in a blockchain network by malicious users, it will have a great impact on Liveness. This can interfere with other transactions in the network and degrade the performance of the entire system. To prevent this problem, blockchain systems such as programmable EVM (Ethermal Virtual Machine) introduced the concept of gas. If there was a machine that could determine whether it was an infinite loop or not, it can be lightly considered that the concept of Ethereum gas would not have existed.

#### Turing Stop Problem
This stems from the famous Turing stationary problem. The following is a simple example of the Turing stationary problem in the Go language. This example shows an attempt to determine whether a function is stationary. In practice, there is no code to tell exactly which functions behave indefinitely, so let's just take it lightly:
```go
// This function is a hypothetical function that determines whether a given program stops for a particular input.
// In practice, a function like this is always implemented to return false to indicate that it cannot exactly work for all cases.
func halts(program func(), input int) bool {
// These functions do not exist.
return false
}

func problematicFunction() {
	for {
		// Infinite loop
		fmt.Println("This function never halts.")
	}
}

func main() {
	// problematicFunction(); // 무한 실행
	input := 42 // Example input
	if halts(problematicFunction, input) {
		fmt.Println("The program halts.")
	} else {
		fmt.Println("The program does not halt.")
	}
}
```
- Why can't we discriminate the stopping problem? For a detailed explanation of this, see [SNUON_The world's 2.4 Turing machine opened by computer science: One Turing machine is natural number one_Lee Kwang-geun](https://www.youtube.com/watch?si=5aWY7S7tfIaplFF7&t=453&v=RINdVaoXV5c&feature=youtu.be) .
> Check the example code: [02_halting_problem](../code/02_halting_problem/)

### 4. For Door with range
It is used to tour arrays, slices, maps, channels, etc. The following is an example code that repeats the index i and the value num for each element of the `nums` slice:
```go
package main

import (
	"fmt"
)

func main() {
	nums := []int{2, 3, 4}
	for i, num := range nums {
		fmt.Printf("Index: %d, Value: %d\n", i, num)
	}

	m := map[string]string{"a": "apple", "b": "banana"}
	for k, v := range m {
		fmt.Printf("Key: %s, Value: %s\n", k, v)
	}
}
```

### 5. Break and continue
Break ends the iteration, and continue skips to the next iteration. Here is an example code that utilizes these two features:
```go
package main

import (
	"fmt"
)

func main() {
	for i := 0; i < 10; i++ {
		if i == 5 {
			break
		}
		if i%2 == 0 {
			continue
		}
		fmt.Println(i)
	}
}
```

## 3. Switch cluase
In Go, the switch statement is a structure in which one or more conditions are evaluated in order and a code block of matching conditions is executed. If it does not correspond to each case clause, the default clause is executed. If the switch statement is well utilized, it can increase code readability and handle various conditions neatly.

### 1. Default switch statement
The default switch statement compares the value of one variable with several case clauses and executes the corresponding code block when it matches. The example code is as follows:
```go
package main

import (
	"fmt"
)

func main() {
	x := 2
	switch x {
	case 1:
		fmt.Println("One")
	case 2:
		fmt.Println("Two")
	case 3:
		fmt.Println("Three")
	default:
		fmt.Println("Other")
	}
}
```
- `switch x`: Evaluate the value of x.
- `Case 1`, `case 2`, and `case 3`: Define the code block to be executed when the values of x match 1, 2, and 3, respectively.
- `default`: Defines the code block to execute when it does not fall under any case clause.

### 2. Switch statement that inspects multiple values
In a switch statement that examines multiple values, each case clause can contain multiple values. This is useful when running the same code block on multiple values. The following is an example of a program that distinguishes odd numbers and even numbers of 1 or more and 6 or less through a switch statement:
```go
package main

import (
	"fmt"
)

func main() {
	x := 4
	switch x {
	case 1, 3, 5:
		fmt.Println("Odd")
	case 2, 4, 6:
		fmt.Println("Even")
	default:
		fmt.Println("Other")
	}
}
```
- `Case 1, 3, 5`: Output "Odd" when the value of x is one of 1, 3, and 5.
- `case 2, 4, 6`: Output "even" when the value of x is one of 2, 4, and 6.
- `default`: Outputs "Other" when the value of x does not correspond to the listed value.

### 3. Switch statements using conditional expressions
In a switch statement that uses a conditional expression, conditions can be evaluated using a conditional expression in each case clause. This method is useful when dealing with more complex conditions as well as specific values.
```go
package main

import (
	"fmt"
)

func main() {
	x := 10
	switch {
	case x < 5:
		fmt.Println("x is less than 5")
	case x < 10:
		fmt.Println("x is less than 10 but greater than or equal to 5")
	default:
		fmt.Println("x is 10 or more")
	}
}
```
- `switch`: You don't have to specify anything after the switch keyword. Each case section evaluates the condition directly.
- `case x < 5`: If x is less than 5, output "x is less than 5".
- `case x < 10`: If x is less than 10 and more than 5, "x is less than 10 but equal to 5" is output.
- `default`: If x is 10 or more, "xis 10 or more" is output.

### 4. fallthrough
The fallthrough keyword is a special keyword used in Go's switch statement, and after the current case clause is executed, the next case clause is forcibly executed. That is, if the fallthrough keyword is used, the code in the next case clause is executed without checking the condition.
```go
package main

import (
	"fmt"
)

func main() {
	x := 1
	switch x {
	case 1:
		fmt.Println("One")
		fallthrough
	case 2:
		fmt.Println("Two")
		fallthrough
	case 3:
		fmt.Println("Three")
	default:
		fmt.Println("Other")
	}
	// One 
	// Two
	// Three
}
```
- If `case 1`: x is 1, output "one" and execute the following case clause (i.e., case 2) by the fallthrough keyword.
- `case 2`: After outputting "Two" without inspecting the condition, execute the following case clause (i.e., case 3) again by the fallthrough keyword.
- `Case 3`: Output "Three" without checking the condition.
- `default`: executed in the absence of the next case clause by fallthrough. It does not run in the code above.

Fallthrough can only force execution to the following case clause. That is, it can only be used at the end of the current case clause, and multiple fallthrough is not allowed. When using this, the conditions in the next case clause are not inspected and must be used carefully to prevent logical errors.


## Resources 
1. Go Docs, "The Go Programming Language Specification: Language version go1.22", Feb 6. 2024, https://go.dev/ref/spec
