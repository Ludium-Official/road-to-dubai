# 005. Go Concurrency
> In this article, the basis for the simultaneous programming of the Go language is explained based on the code. A method of controlling the simultaneity using a goroutine, a channel, a select statement, a sync package, etc. This module is optional and, if necessary, may improve the simultaneous programming ability through additional learning.

## 0. Simultaneity
First of all, let's simply point out the concept of simultaneity/parallelity that cannot be left out if it is simultaneous.
- Concurrency: Dealing with multiple tasks at once. It has multiple logical control flows.
- Parallelism: Running multiple tasks at once. One or more logical control flows.

Simultaneousity logically has several logical control flows because it looks at multiple task processing methods. On the other hand, parallelism is a way to actually process multiple tasks physically. It is more interested in execution solutions, not how to deal with multiple tasks. There is also a method of parallel processing with only one logical control flow with SIMD in a single core used in image processing or deep learning, and CPU instruction level parallel processing solutions include pipe lining, non-sequential execution, and speculative execution. Using a 64-bit computer rather than 32 bits in this technique increases data that can be processed at once with a single command, leading to performance improvement

Reorganizing this concept with a multi-threading technique using commonly used threads and locks, a simultaneous program with a control flow (ex. mutex, scheduler, critical section, etc.) that can handle multiple tasks at once is executed on hardware (multicore) operating in parallel. The reason why it is difficult to deal with such a program is that it is non-deterministic.

Therefore, when developing the Cosmos SDK application, simultaneous programming is avoided because it is necessary to write decisive and stable code. However, if you go further, if you can understand and use this simultaneous programming, you will be able to write more efficient code.

## 0. Go routine
Gorroutine is a multi-thread technique implementation using lightweight threads. Gorroutine uses less memory than OS threads and is created and executed faster. Gorroutine is managed by the Go runtime scheduler, and Go runtime is executed by distributing Gorroutine to several threads. Even if thousands of Gorroutines are generated with this design, there is an advantage that the performance is not significantly affected. This played a role in the popularity of high languages.


## 1. Channel
Channels enable safe communication between goreutins and are used as a major means of controlling concurrency. In addition, channels automatically synchronize goreutins when sending and receiving data based on blocking communication. Channels are declared by specifying a type and are created using a make function.
- Synchronization: Synchronization occurs between gorroutines when values are exchanged through the channel.
- Directional: Channels can be declared bidirectional or transmit-only, receive-only.

The following is an example code that uses channels to send and receive data between go routines:
```go
package main

import (
	"fmt"
)

func main() {
	// Create channel
	ch := make(chan int)

	// Send value to the channel through Go routine
	go func() {
		ch <- 42
	}()

	// Recieve the value through the channel
	val := <-ch
	fmt.Println(val) // 42
}
```
> Check the example code: [05_channel](../code/05_channel/)

### Simultaneous Control with Channel
Buffered channels allow for asynchronous data transmission and reception, allowing high-routine to continue to work without being blocked. This enables efficient work distribution and load distribution among high-routines. We show how to use buffered channels:
```go
package main

import (
	"fmt"
)

func main() {
	// Create a channel with buffer volume of 2 
	ch := make(chan int, 2)

	// Send the value to the channel
	ch <- 1
	ch <- 2

	// Redeive the value from the channel
	fmt.Println(<-ch) // 1
	fmt.Println(<-ch) // 2
}
```
> Check the example code: [05_buffer_channel](../code/05_buffer_channel/)

