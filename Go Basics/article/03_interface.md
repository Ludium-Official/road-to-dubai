# 003. Go Interface
> In this article, we explain the concept of interface and the concept of polymorphism through it. Learn how to use the interface to process various types of objects in the same way, and through this, practice how to increase the flexibility and scalability of code.

## 0. What is an interface?
An interface (hereinafter referred to as an interface) defines a set of method signatures, and which type implements that interface depends on whether that type implements all the methods of the interface. Interface features are as follows:
- Implicit implementation: type automatically meets the interface by implementing the method.
- Polymorphism: The interface allows the function to accept and return other types as long as it implements the method it needs.


Thanks to this flexibility and simplicity, Go's interface system is powerful, easy to use, and polymorphism and separation can create clean, easy-to-maintain code.

## 1. Interface and polymorphism
Polymorphism is an important concept in object-oriented programming and functional programming, and refers to a function that allows several different data types to be handled in the same way. Polymorphism makes code more flexible and reusable. These functions may be implemented using interfaces.
> Let's learn more about this by writing the code ourselves through the practice.

## 2. Empty Interface
An empty interface is a container that can hold any type. This allows you to assign any type to a variable, so it is generally useful when you have to deal with generics, arbitrary data storage, and various types. The empty interface is very flexible, but since the type of the value cannot be known where you use it, you should check the original type through type assistance or type switch when using the value.

The following example shows how to use an empty interface to store different types of values, and to identify the type of values using a type switch:
```go
package main

import (
	"fmt"
)

// Defined type struct
type Person struct {
	Name string
	Age  int
}

type Animal struct {
	Species string
	Age     int
}

// function that compares the entered value with the struct
func compareStructs(i interface{}) {
	switch v := i.(type) {
	case Person:
		fmt.Printf("Value is a Person: %+v\n", v)
	case Animal:
		fmt.Printf("Value is an Animal: %+v\n", v)
	default:
		fmt.Printf("Unknown type: %T\n", v)
	}
}

func main() {
	// Empty interfact slide that can contain diverse type value
	var i interface{}

	// Compare with Person struct
	i = Person{Name: "Alice", Age: 30}
	compareStructs(i) // Value is a Person: {Name: Alice Age: 30}

	// Compare with Animal struct
	i = Animal{Species: "Dog", Age: 5}
	compareStructs(i) // Value is an Animal: {Species: Dog Age: 5}

	// Compare with other types 
	i = "Hello"
	compareStructs(i) // Unknown type: string
}
```


## Resources 
1. Go Docs, "The Go Programming Language Specification: Language version go1.22", Feb 6. 2024, https://go.dev/ref/spec
