
# 02m01. Repeat statement & condition statement practice - Implement command line interface (CLI) program

## 0. Implement CLI (command line interface) programs using repeat statements and conditional statements
Let's make a simple CLI program using both conditional and repetitive statements. [Previous conditional statement practice](./02m00_condition_nnumber_decision.md) had no repetitive statements, so it was cumbersome to run the program again and again in order to enter other input values. Now, repetitive statements can relieve such inconvenience.

## 1. Setting Preferences
The default settings are as follows:
```sh
# Create simple_cli direcotry
$ mkdir simple_cli && cd simple_cli

# Create simple_cli go module
$ go mod init simple_cli
```

## 2. Program Implementation Requirements
The program receives input from the user and performs different operations depending on the input. It continues to be executed in an infinite loop until the user enters "exit".

Program implementation requirements are as follows:
1. Various operations are performed according to the command input by the user.
2. The commands are as follows:
1. "exit": Shut down the program.
2. "Hello": outputs "Hello, world!"
3. "even": outputs an even number from 0 to 10.
4. "odd": Outputs odd numbers from 1 to 10.
6. For other inputs, "Unknown command" is output.
> Check the implemented practice code: [02_simple_cli](../code/02_simple_cli/)


## 3. Example of submitting an I/O handler program execution screen using repeated statements and conditional statements
The results printed by running the program are as follows:
<div style="text-align: center;">
   <img src="../assets/02_control_structure_simple_cli_result_example.png" alt="02_control_structure_simple_cli_result_example" width="600"/>
</div>
