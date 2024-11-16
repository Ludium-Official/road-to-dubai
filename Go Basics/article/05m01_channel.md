# 05-01. Creating a transmit-only and receive-only channel

## 0. Create a transmit-only and receive-only channel
Channels can be declared transmission-only or reception-only. Transmission-only channels can only send values, and reception-only channels can only receive values. Let's create transmission-only channels and reception-only channels through practice. Let's create transmission-only channels and reception-only channels through practice.

## 1. Setting Preferences
The default settings are as follows:
```sh
# Create send_recv_channel directory
$ mkdir send_recv_channel && cd send_recv_channel

# Create send_recv_channel go module
$ go mod init send_recv_channel
```

## 2. Creating code
The full code is as follows:
```go
package main

import (
	"fmt"
)

func send(ch chan<- int, val int) {
	ch <- val
}

func receive(ch <-chan int) int {
	return <-ch
}

func main() {
	ch := make(chan int)

	go send(ch, 42)
	val := receive(ch)
	fmt.Println(val) // 42
}
```
> Check the practice code: [05_send_recv_channel](../code/05_send_recv_channel/)

## 3. Example of submission of a transmit-only and receive-only channel execution screen
The results printed by running the program are as follows:
<div style="text-align: center;">
   <img src="../assets/05_concurrency_send_recv_channel_result_example.png" alt="05_concurrency_send_recv_channel_result_example" width="600"/>
</div>
