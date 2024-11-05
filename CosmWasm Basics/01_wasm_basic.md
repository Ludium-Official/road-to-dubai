# WebAssembly Basic
> Before learning about Cosmwasm, the article was created to develop an understanding of WebAssembly (Wasm), which is the core of the technology

## 0. WebAssembly(Wasm)
WebAsembly (Wasm) is an open standard that defines the format of binary commands that may generate implantable binary executable files from various source languages. These binaries may run in a variety of environments and are supported by all major browsers originating from the web. Using Wasm, programs that are more portable and secure than ever before can be executed anywhere, such as a server or an edge.


## 1. Charaterisatics of WebAssmebly(Wasm)
### 1. Resource Efficiency and Speed
Wasm applications are designed to run with minimal memory space and CPU requirements. It provides speeds similar to native code and has no cold start time, unlike VM booting or container startup.

### 2. Outstanding Security
Wasm runtime basically runs in a sandbox environment and allows secure access to memory. The feature-based model allows Wasm applications to access only explicitly permitted items. This strengthens supply chain security.
- Most languages assign addresses to functions at runtime. If you look at memory in the form of a byte array, it is not secure because the function code is not properly distinguished.
- Wasm encapsulates program memory in a secure area, so it does not allow code that can affect or compromise the security of the host running the program.

### 3. Portability
Wasm supports most CPU architectures (x86, ARM, RISC-V) at multiple major run-time and can also run on Linux, Windows, macOS, and Non-POSIX operating systems.

### 4. Transpilability
Various programming languages can be compiled into Wasm, and a modern and continuously improved toolchain is used. Compilers can create Wasm programs after compiling into LLVM Intermediate Expression (IR) using the low-level virtual machine (LLVM) backend.


## 2. WebAssembly Interface(WASI)
WASI and Wasm runtime play an important role in the Wasm ecosystem.
- WASI allows WebAssembley programs to interact with the operating system, extending the scope of WebAssembley's utilization outside of the web browser.
- Wasm runtime supports these WebAscemly programs to run smoothly in a variety of environments.

At the application level, you do not have access to perform tasks such as opening or creating files. This is because system resources such as files, memory, and network connections are very important for stability and security. If one program accidentally destroys the resources of another program, that program can be stopped. Moreover, if a program (or user) deliberately touches the resources of another program, there is a risk of stealing important data.

### Protection ring security
Protection ring security is used as a way to control which programs and users can access which resources. The OS places a security barrier around system resources by the kernel. The kernel performs the tasks of creating, opening, or making network connections. User programs run outside the kernel called `user mode`. A program must ask the kernel to perform tasks such as opening a file.

When a program requests a kernel to work, it uses system call. This allows the kernel to determine which users are requesting and to determine whether they are authorized to access the requested file. Most devices use the system resource access method through this system call. The OS makes system call available. However, if each OS has a system call, wouldn't a different version of code be needed for each OS? Fortunately, that's not the case. This is because there is a system interface.

### System Interface
Most programming languages provide standard libraries. When coding, programmers do not need to know about the OS or system, they only need to handle the interface (framework, library). When compiling, the toolchain selects the interface implementation to be used depending on the target system. This implementation uses the functions of the OS API, so it depends on the system. These examples are as follows:
- That's why you choose OS and system architecture (x86, ARM) when installing tools such as Docker Desktop and JVM.
- If the 'printf' function is compiled for a Windows machine, it uses the Windows API, and if it is compiled for Mac or Linux, it uses POSIX instead.

The system interface refers to a common interface that may be used even in different environments.

### Absence of interface in WASM
This structure creates problems for Wasm. When using Wasm, you cannot use the system interface of a single OS within the Wasm implementation of the standard library, because you do not know what type of OS it is targeting when compiling. Previously, "WebAssetbly is a machine language for conceptual machines, not real machines." In the same vein, WebAssetbly needs a system interface for conceptual OSs, not real OSs.

Most early Wasm codes were run with Emscripten, so when I tried to run Wasm outside of the browser, I ran the code compiled with Emscripten in the same way. There were a number of issues with this:
- Legacy overhead: Emscripten was designed to compile C/C++ code via wasm after ams.js. Relying on emulation of emscripten also required unnecessary legacy overhead for the latest wasm app outside the browser.
- Emulation of Emulation: Emscripten uses the JS Glue code to emulate POSIX on the web. Relying on this structure outside of the browser means that Wasm runtime essentially emulates the emulation. This has high potential for inefficiency, complex code paths, and performance degradation.
- Non-standard interface: JS Glue Code is not designed as a public system interface for Wasm. Therefore, it is more difficult to achieve interoperability and standardization as runtime outside the browser risks involving non-standard behavior when implementing versions based on Emscripten's Glue Code.
- Security issues: Emscripten does not provide a sandbox. It is likely to be exposed to risks in relation to direct system interaction.

Because of this, we had to create our own implementation of all the features in JS Glue code in these Wasm runtime structures.

#### The Rise of WASI
In conclusion, Emscripten played an important role in the early days of Wasm, but it was considered that excessive dependence on the emulation layer in a non-browser environment is likely to result in inefficiency, security issues, and barriers to standardization and innovation. The standard cannot consistently be the emulation of emulation. As the Wasm ecosystem grew, it was determined that it was more important to build a robust and future-oriented foundation than the legacy emulation layer. So it came out with WASI, the WebAsembly system interface.

### 1. WASI Features
#### 1. Portability Portability Portability
POSIX is a technology that allows the compilation of the same source code on various operating systems. This allows developers to execute code once created in multiple locations without the need to write code for multiple systems.

WebAssembly goes one step further. Code written in WebAssembly is designed to be 'compiled once and executed on various devices'. For example, if the basic module of Node.js is written in WebAssembly, users can easily install apps without a complicated installation process. Developers do not need to write codes separately for multiple platforms.

#### 2. Security Security Security
##### 1. Existing access methods
Traditionally, computer operating systems (OS) verify permissions when users access files or use networks. This is a method created for security between users.
- For example, when multiple employees of a company use the same computer, each employee's file is not easily opened by another employee.

##### 2. Risk of third party code
These days, however, most systems have a single user running multiple programs. The problem here arises when the program we use itself is unreliable.
- For example, if a program we use contains malicious code, it can sneak open our files or steal our information over the network.

Because of this risk, it is very dangerous to use unreliable third-party programs.

##### 3. WASI's approach to security
The host environment (browser or Wasm runtime) may select the WASI function that may be used by the Wasm program. This enhances security by preventing the program from having unnecessary system privileges. And WASI limits direct system access by executing the program in a sandbox.
- Because the program runs in a sandbox, the program cannot access files or use the network at will.
- The host environment can limit what a program can do. This prevents programs from doing risky tasks with full operating system privileges.


#### 3. Modular Design
WASI includes a 'wasi-core' module that provides basic system interface functions. It provides essential functions such as file access, network connection, time check, and random number generation. WASI provides this standardized interface so that the Wasm program may operate in a consistent manner in various host environments.

## 3. Wasm Runtime - Wasmer
Wasm runtime is an environment in which web assembly code can be loaded and executed, providing the infrastructure required to execute WASM binaries. The runtime is an interpreter or virtual machine for executing WASM instructions. There are various Wasm runtime developed specifically for environments other than browsers. These runtime increasingly began to support WASI interfaces, allowing Wasm applications to be executed outside of the browser through standardized system access. Currently, Wasm runtime exists as Wasm runtime, such as Wasmer, WasmTime, and WasmEdge.

WASI and Wasm runtime allow Wasm modules to run safely and quickly on existing operating systems such as Linux, Windows, and macOS. This process is largely similar to browser execution. WASI provides a consistent interface that abstracts system-specific differences, ensuring that Wasm binaries can be universally executed on servers.
1. Compile to Wasm: Like browser scenarios, source code is compiled into Wasm directly through tools such as Emscripten or through the Wasm backend of LLVM.
2. Runtime Utilization: Deploy Wasm binaries to servers with Wasm runtime, such as Wasmer.
3. Run: Runtime loads Wasm modules and interacts seamlessly with server resources, including file systems, network protocols, etc., with the help of WASI.

Wasmer is one of the WebAssembley runtime, which allows WebAssembley binaries to be executed in a variety of environments. Wasmer can be used in many forms, including CLI, native applications, and server applications.

### The process by which Wasm runs on the OS
WASI and Wasm runtime allow Wasm modules to run safely and quickly on existing operating systems such as Linux, Windows, and macOS. This process is largely similar to browser execution. WASI provides a consistent interface that abstracts system-specific differences, ensuring that Wasm binaries can be universally executed on servers.
1. Compile to Wasm: Like browser scenarios, source code is compiled into Wasm directly through tools such as Emscripten or through the Wasm backend of LLVM.
2. Runtime Utilization: Deploy Wasm binaries to servers with Wasm runtime, such as Wasmer.
3. Run: Runtime loads Wasm modules and interacts seamlessly with server resources, including file systems, network protocols, etc., with the help of WASI.

## Resources
- https://hacks.mozilla.org/category/code-cartoons/a-cartoon-intro-to-webassembly/
- https://rsms.me/wasm-intro
- https://github.com/bytecodealliance/wasmtime/blob/main/docs/WASI-intro.md
- https://hacks.mozilla.org/2019/03/standardizing-wasi-a-webassembly-system-interface/
- https://github.com/wasmerio/wasmer