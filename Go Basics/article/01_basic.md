# Go Basic
> This article explains Golang's basic data types (Numerics, String, Booleanans) and how to declare and initialize variables. Then, you learn about function definitions and calls, structures and methods, the use of pointers, and closers necessary for practical programming. For each concept, write your own code through a hands-on task to help you understand.

## 0. Numerics
Integrer, floating-point, complex, and rune constants are collectively called numeric constants.

### 1. Integer
go offers several Interger types.

#### general types
- 'int': The size varies depending on the OS (32 bits or 64 bits). It can represent both positive and negative numbers.
- 'uint': a signless integer type that can only represent positive integers that are not negative.


#### specific types
The specific type can explicitly specify the bit length.
- `int8`: -128 ~ 127
- `int16`: -32768 ~ 32767
- `int32`(= `rune`): -2147483648 ~ 2147483647
- `int64`: -9223372036854775808 ~ 9223372036854775807

The following are explicit types that represent only positive integers that are not negative.
- `uint8`(= `byte`): 0 ~ 255
- `uint16`: 0 ~ 65535
- `uint32`: 0 ~ 4294967295
- `uint64`: 0 ~ 18446744073709551615

#### special types
- 'byte': an alias for uint8, which is commonly used to represent raw binary data.
- 'rune': an alias for int32 used to represent Unicode code points.
- 'uintptr': an integer that "holds the bit pattern of all pointers". It is sometimes used to communicate directly with the OS via a system call, but rarely used. ([Note](https://stackoverflow.com/questions/59042646/whats-the-difference-between-uint-and-uintptr-in-golang))


### 2. Float & Complex
go provides several types for representing floating points and complex numbers. These types are used for various program operations, including equation calculation and spatial data processing.

#### float types
Go offers two floating point types:
- `float32`: represents a 32-bit floating-point number.
- `float64`: represents a 64-bit floating-point number.

Because the precision is finite, floating-point types give only a rough representation of the real numbers.

#### complex types
- `complex64`: consists of two float32 values (the real part and the imaginary part).
- `complex128`: consists of two float64 values (the real part and the imaginary part).

How to use complex can be handled using the built-in function provided by go:
- complex(r, i): create a complex number with the real part r and the imaginary part i.
- real(c): Returns the real part of the complex c.
- imag(c): returns the imaginary part of the complex c.


Float 32 and float64 are the floating point types. It is only an approximation to the real number because the precision is finite. Complex64 and complex128 represent complex numbers. They are useful in geospatial coordinates and scientific applications. Complex numbers always have floating points, such as the 'real number' part and the ' imaginary number'.
- If the real and imaginary parts are float32, the complex number is complex 64.
- Similarly, if the real and imaginary parts are float64, the complex number becomes complex128.


## 1. String
In Go, `string` is basically a read-only byte sequence encoded in UTF8 and is immutable. In other words, you can read and manipulate the string in a variety of ways, but the bytes constituting the string cannot be modified directly because the string is invariant:
- UTF-8 Encoding: The string is encoded in UTF-8 so it can represent all valid Unicode characters.
- Immutability: Once created, the contents of the string cannot be changed. Every action to modify the string creates a new string.
- Length and Indexing: the len function returns the number of bytes of the string, not the number of characters. Indexing the string returns bytes at that location, not the letter.


## 2. String Formatting
The fmt package provides a function such as 'fmt.Printf' that allows you to format a string in a variety of ways. 

| verb  |    description                 | 
|-------|--------------------------------|
| %v    | value (default format)         |
| %T    | The value of the type                |
| %x    | Hexadecimal encoding           |
| %d    | Integer (base 10)              |
| %f    | Floating-point number          |
| %e    | Scientific notation (lowercase)|
| %E    | Scientific notation (uppercase)|
| %p    | Pointer Address                |
| %s    | String                         |
| %c    | Letter shown through Unicode point  |


## 3. Boolean
In Go, the 'bool' type literally represents a boolean value. It can be expressed as 'true' or 'false'.

The main characteristics of the bol type in Go are as follows:
- Default: The default value for the bol type variable is false.
- Comparison operator: It is expressed as the result of comparison operators such as ==, !=, <, >, <=, > along with the bool type.
- Logical operator: It is expressed as a logical operation such as &&(AND), ||(OR), and !(NOT) along with the bol type.


## 4. Declaring and Initializing Variables
In go, you can define and initialize variables in many ways. Here are various ways to define variables:
```go
var s string = "initial"

// Alternatively, the shorthand notation can be used with the := operator
s := "initial"
```

### 1. Declaring multiple variables without initialization
You can also use the var keyword to declare multiple variables without initializing them:
```go
var (
    a, b int
    s string
    c complex64
)

// This is the same as declaring each variable individually.
// If not initialized, the variable has a value of 0, depending on the type.
var a, b int
var s string
var c complex64
```

### 2. Define Constant
You must use the const keyword to define a constant:
```go
const hello = "Hello, World!"
```

## 5. Function
Function is a key part of Go.
- Functions allow us to encapsulate the code into reusable blocks.
- A function can receive more than 0 parameters and more than 0 return values.

To define and use functions:
```go
func functionName(parameterName parameterType) returnType {
    // function body
    return value
}
```


## 5. Struct
Structures are complex data types that group variables under one name. These variables are called fields. The structure is similar to the class of object-oriented programming, but does not support inheritance.

To define and use structures:
```go
type StructName struct {
    field1 fieldType1
    field2 fieldType2
    // more fields...
}
```
 
### 2. Comparison of Structures between Class and Go in Object-Oriented Programming
Class is an essential function of existing object-oriented programming languages. However, this is because inheritance problems such as diamond problems are troublesome. So Go created a structure that does not have the function of inheritance. The characteristics of the structure from the class comparison point of view are as follows:
- No Inheritance: Unlike the class, Go's structure does not support inheritance. Instead, Go uses composition to reuse the code.
- Encapsulation: Go structures provide encapsulation, but do not have features such as private or protected access indicators in the class. Instead, Go indicates that it is a public field using a rule that capitalizes the field name.
- No method within the structure definition: the method is not defined within the structure itself, but is associated with the structure type externally.


#### 상속과 컴포지션 관련 참고 글 
- [Why is there no type inheritance?](https://go.dev/doc/faq#Is_Go_an_object-oriented_language:~:text=always%20resolved%20statically.-,Why%20is%20there%20no%20type%20inheritance%3F,-Object%2Doriented%20programming) 
- [Why no "class" keyword in Go?](https://groups.google.com/g/golang-nuts/c/aJ6JiiIusqg/m/TJM09vOkv9YJ)
- [Why there are no classes in the GoLang programming language?](https://www.quora.com/Why-there-are-no-classes-in-the-GoLang-programming-language)

## 6. Method
The method is similar to Function but associated with a specific type (typically a structure). The method can define the type of behavior, access the fields of the structure, and define the behavior.

Therefore, it is often used with structures:
```go
type TypeName struct {
    // fields
}

func (receiver TypeName) methodName(parameters) returnType {
    // method body
}
```

### 2. Comparing the Method of Class and Go in Object-Oriented Programming
- Receiver:Go's methods are associated with types using receiver arguments, similar to how methods are associated with objects in a class.
- No `this` keyword: the Go method explicitly uses the receiver name instead of the implicit `this` keyword found in many object-oriented languages.
- The Pointer Receiver: Go method can modify the value of the recipient using a pointer receiver, similar to modifying the state of an object in a class.

## 7. Pointer
Pointer (hereinafter referred to as a pointer) is a variable that stores memory addresses of other variables. Pointers allow you to directly refer to and manipulate actual memory locations. The main characteristics are as follows:
- Address operator (`&`): Gets the address of the variable.
- Reverse reference operator (`*`): Gets the value of the address the pointer points to.
- Change the value: If you change the value through a pointer, the actual value stored at that address changes.

A simple example sample code is as follows: 
```go
var ptr *int
ptr = &variable
```
- `*int`: A pointer type, which means a pointer to an int-type variable.
- `&variable`: Returns the memory address of the variable.


## 8. Closure
Closure is an important concept in programming. This is because it is useful for increasing the reuse of code, encapsulating variables, and implementing delayed execution patterns. A quick look at history is as follows:
- The concept of closure originated from Lambda calculus, which became the beginning of functional programming.
- The LISP language, born in 1950, is a practical programming language implementation of the idea of lambda algebra, with functions treated as top-level objects and containing the concept of closure.
- In the early 2000s, the new language revival of Javascript, Python, and Ruby equipped with the functions of Closer and other reasons also raised Closer's presence.
- The Java language, born in 1991, also added a closure feature in the 2014 Java 8 version. [Like it or not, closures are coming to Java](https://www.infoworld.com/article/2078659/like-it-or-not--closures-are-coming-to-java.html)

A closure is a function defined within a function and has the ability to access variables of an external function. Closer allows internal functions to "remember" and refer to variables within the scope of external functions. Go supports closure from early versions. Go's closure has been usefully treated to implement several advanced functions such as simultaneous programming.

## Links
There are relatively many documents on the Go language compared to other languages. Compared to C, C++, Java, etc., it is the latest language and there are many people who like the advantages of Go, so there are many well-written documents in Korean.
- [Effective go(in Korean)](https://gosudaweb.gitbooks.io/effective-go-in-korean/content/)
- [Tucker의 Go Lang Programming](https://www.youtube.com/playlist?list=PLy-g2fnSzUTBHwuXkWQ834QHDZwLx6v6j)
- [golang korea github](https://github.com/golangkorea)

## Resources
1. Go Docs, "The Go Programming Language Specification: Language version go1.22", Feb 6. 2024, https://go.dev/ref/spec


