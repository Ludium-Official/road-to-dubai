# Go Introduction
> The Golang baisc module was created for the purpose of understanding and customizing the Cosmos-SDK written in Go. Furthermore, it aims to understand the technology of IBC, the core of the CometBFT consensus engine and interchain.

## 0. Background of Go Language Birth
First released on Google in November 2009, the Go language was developed by three Google engineers, Robert Griezmer, Rob Pike, and Ken Thompson, who realized the complexity of C++. In particular, Ken Thompson was awarded the Turing Award, contributing to the creation of the Unix and C languages, which influenced the Go language. The Go language values simplicity and efficiency, and it is optimized for large-scale systems with its high performance and concurrent capabilities. The Go language is often called Golang, too.

### 1. Development of the Go language
The Go language has continued to improve stability and performance through these constant updates:
- In 2012 [1.0 version](https://go.dev/doc/go1), Go 1.0 laid the foundation for early Go users to continue using it, focusing on stability and compatibility.
- In 2015, [version 1.5](https://go.dev/doc/go1.5) achieved [bootstrap](https://ko.wikipedia.org/wiki/%EB%B6%80%ED%8A%B8%EC%8A%A4%ED%8A%B8%EB%9E%A9_(%EC%BB%B4%ED%8C%8C%EC%9D%BC%EB%9F%AC))  from C language to Go for compiler performance and language development at the same time.
- In 2016 [version 1.7](https://go.dev/doc/go1.7), performance optimization and tooling improvements are the main features. Many modifications have been made to the compiler, runtime, and several packages.
- In 2017 [version 1.8](https://go.dev/doc/go1.8), shortening compilation time and adding new features are the main changes. Faster compilation speed and improved language capabilities have increased the productivity of developers.
- In 2018 [version 1.11](https://go.dev/doc/go1.11), module systems were introduced to greatly improve dependency management. This new module system simplified the package management and build process, and minimized the dependency conflict problem.
- In 2022 [version 1.18](https://go.dev/doc/go1.18), generic functions have been added to improve code flexibility and reusability. Through the introduction of generic types, code bases can be managed more concisely and efficiently.
- 2023 [version 1.21](https://go.dev/doc/go1.21), performance optimization and new function addition were performed. In particular, profile-based optimization (PGO) function was introduced to maximize execution performance.
- It included 2024 [version 1.22](https://go.dev/doc/go1.22), memory optimization, addition of new standard libraries, and static dispatch of multiple interface method calls. In addition, the memory usage of Go runtime was optimized, resulting in a 1-3% improvement in CPU performance.

Go did not receive much attention at first, but the number of users increased due to the fact that it was a language created by developers from Google and the advantages of the language itself. The fact that container technology was greatly commercialized and Docker and Kubernetes were also written as Go also played a part. In the blockchain, the main client of Ethereum, Geth, was also written as Go, but the topic of the article, Cosmos-SDK, was also written as Go. In other words, Go can already be seen as a language that has been recognized in the market and verified for stability.

## 1. Features of the Go language
The Go language has the following advantages:
- Simplicity: Go avoids complicated features and provides simple grammar and structure to make code easier to understand and maintain.
- Clarity: A clear and consistent code style is recommended, and concise grammar is used to improve the readability of the code.
- Safety: A strong type system and a concise error handling mechanism can create stable code.
- Efficiency: Compilers and runtime systems work efficiently to maximize performance.
- Simultaneous: Support simple and efficient simultaneous programming through goroutines and channels.

### 1. Simple is the Best
One of Go's strengths is that the syntax is simple and clear. Well-made abstractions help developers write clear, maintainable codes. Go language specifications are designed intuitively, making it easy to learn and use. By using only 25 keywords and concise operators and punctuation marks, Go maintains a balance between simplicity and expressiveness. This design philosophy improves developer productivity and helps write neat, readable, and efficient codes.

#### Keywords
|             |          |           |            |        |
|-------------|----------|-----------|------------|--------|
| break       | default  | func      | interface  | select |
| case        | defer    | go        | map        | struct |
| chan        | else     | goto      | package    | switch |
| const       | fallthrough | if     | range      | type   |
| continue    | for      | import    | return     | var    |


#### 2. More Fast, Goroutine
Simultaneous is implemented through an asynchronous mechanism called Goroutine (hereinafter referred to as Goroutine). This was influenced by Erlang's concurrency model. Erlang is a multi-process mechanism, but Goroutine follows a multi-thread mechanism, and it is a lightweight thread structure that is managed with its own schedule rather than an OS. Therefore, regardless of the number of CPU cores, even if hundreds or thousands of Goroutines are created, they are efficiently managed and operated through the scheduler.

Each Gorroutine operates in parallel and sends and receives values through the message channel. Using this, event processing and simultaneous programming can be implemented simply. However, it is the responsibility of the developer that may occur in simultaneous programming. If this is not handled well, there is a risk of abnormal termination during the program's execution. This is dealt with directly through the code in [05_concurrent] (./05_concurrent.md).


### Disadvantages
Up to this point, the Go language can be seen as the really optimal language. Anyone can easily use it with simple grammar, and high performance can be achieved with a compiler language and multi-thread simultaneous programming with a type system. Memory management is also handled automatically through the GC.

1. Cross-compilation: When you do the compiler, there are no intermediate languages, so you need to compile them according to their respective architectures (x86, arm, ..). If cross-compilation is required, let's study [goreleaser](https://goreleaser.com/).
2. Direct memory management: Different developers have different tendencies. Some aim to automatically manage memory through GC, while others don't. That seems to be the case, especially in the field of system program development. System programming prefers languages that can directly manage memory, such as C, C++, and Rust. In fact, [It is sometimes said that Go is not suitable for system programming because of GC](https://www.quora.com/Is-Go-a-systems-programming-language)
3. Type system: Instead of aiming for simple grammar, the type system did not follow the way previously studied. A strong type system can catch many bugs and problems at the time of compilation. However, Go's type system causes many problems such as null pointer or unsafe type casting.


## 2. Installing Go Language
Before learning basic grammar, those who are developing Go for the first time need to do the default settings. The Go language installation depends on the OS environment. Try downloading it through [Go official download link](https://go.dev/doc/install). 

### Linux Ubuntu
```sh
sudo apt install golang -y or
sudo apt-get install golang -y or
```
If the go version is incorrect, or installations fails, 
```sh
wget https://golang.org/dl/go1.20.linux-amd64.tar.gz
sudo tar -C /usr/local -xzf go1.20.linux-amd64.tar.gz
```
The above is the command to install go v1.20 on /usr/local.
### MacOS
```sh
brew install go
```
### Windows
https://go.dev/doc/install 
Go to the link and download.

If the download is complete, open commandline prompt and type in the following command to see if Go is installed properly:
```sh
$ go version
```

If go command not found eroor occus asfter the installation
## Setting Environmental Variables
#### PATH:
Add the directory where the Go executable file is located to the system PATH so that Go commands can be executed anywhere on the terminal.

Set value: $PATH:$HOME/go/bin


#### GOPATH:
Specify the Go workspace (where you store modules, packages, binaries, etc.).

Set value: $HOME/go (typically GOPATH should be set to $HOME/go).


#### GOROOT:
Specifies the Go installation path, which is typically the installation directory of Go.

Set value: $HOME (typically a directory with Go installed, such as /usr/local/go).

##### basic command
```sh
which go   # Check where go is installed 
export PATH=$PATH:$HOME/go/bin   # Execute go command
export GOPATH=$HOME/go   # Designate go workspace
export GOROOT=$HOME   # Put loactaion found from which go
```


### Linux Ubuntu
```sh
nano ~/.bashrc  # or nano ~/.profile   # Open environment variable through nano text editor
source ~/.bashrc  # or source ~/.profile
```


### MacOS
```
nano ~/.bash_profile  # or nano ~/.zshrc
source ~/.bash_profile  # or source ~/.zshrc
```


### Windows
##### 1. Opening the Environmental Variable Settings window:
Search the Start menu for "Edit Environment Variables" and open the "System Properties" window.
On the Advanced tab, click the Environmental Variables button.
##### 2. GOPATH Settings:
Click New in "User Variables" and set GOPATH, for example, C:\Users\YourUsername\go.
##### 3. Modify PATH Variables:
Under System Variables, select Path and click Edit.
Add C:\Go\bin and %GOPATH%\bin.
##### 4. Apply:
Click "OK" in all windows to apply the changes.

A development tool that supports Go is Golang from JetBrains, but this is a fee, and you can easily develop the Go language by installing [Go plug-in](https://code.visualstudio.com/docs/languages/go) from vscode).

> Tip) Simple grammar use can be easily implemented on the web through [The Go Playground](https://go.dev/play/p/1u5bSZlh80h).

## 3. (Practice) Hello, printing the world
To create a program in the Go language, you must first declare a package. Each package consists of one Go source file (`.go`) in a single directory. If the package name is `main`, Go creates an executable file.

After the package declaration, the package fmt is imported. The package name is the last element of the package file path.
- For example, if you import the "lib/math" package, use it as "math".
- fmt implements input and output (IO) and is part of Go's standard library.

The execution begins with a function called main(), which simply calls the Go I/O function Printf() from the fmt package.

Now compile this program. Create a hello.go file in the folder you want.
```sh
# hello createi=e dirctory
$ mkdir hello && cd hello

# Create hello go module 
$ go mod init hello

# Write "Hello, world" return program
$ echo 'package main
import "fmt"
func main() {
    fmt.Printf("Hello, World!")
}' > hello.go
```

Now build the program. Go only outputs errors, so no output or other command prompts should be displayed unless something goes wrong. No news is good news. 
```sh
# Build hello program
$ go build
```

When the build is completed, an executable file such as the module name is created. When you run it, 'Hello, World!' will be output normally.
```sh
$ ./hello
Hello, World!
```
> Check the mission code: [00_hello](../code/00_hello/) 


## Resources 
1. Go Docs, "The Go Programming Language Specification: Language version go1.22", Feb 6. 2024, https://go.dev/ref/spec
