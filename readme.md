# rust

vscode 调试文件

launch.json

```
{
    // Use IntelliSense to learn about possible attributes.
    // Hover to view descriptions of existing attributes.
    // For more information, visit: https://go.microsoft.com/fwlink/?linkid=830387
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "play_ground",
            "program": "${workspaceRoot}/target/debug/play_ground",
            "args": [],
            "cwd": "${workspaceRoot}/target/debug/"
        }
    ]
}
```

rust编译命令 

    rustc main.rs

# Common Programming Concepts
> This chapter convers concepts that appear in almst every programming and how they work in Rust.

## Keywords

The Rust language has a set of keywords that are reserved for use by the language only,much as in other languages.

## Identifiers

A name in Rust is called an "identifier",and can be made up of any nonempty ASCII string,with some restrictions.

## Variables and Mutablility

- By default variables are immutable
- You can make variables by adding mut in front of the variable name

## Data Types

     let guess: u32 = "42".parse().expect("Not a number!");
     
- Scalar Types
    - Integer Types
    - Floating-Point Types
    - The Boolean Type
    - The Character Type
- Compound Types
    - The Tuple Type
    - The Array Type
 
## How Functions Work 
> Functions are pervasive in Rust code.

Rust code uses snake case as the converntional style for function and variable names.

- Function Bodies Contain Statements and Expressions

Function bodies are made up of a series of statements optionally ending in an expression.

## Control Flow


## Ownership

for Rust to make memory safety guarantees without needing a garbage collector

- what is ownership
>All programs have to manage the way they use a computer's memory while running. 

Memory is managed through a system of ownership with a set of rules that the compiler checks at compile time. 

None of the ownership features slow down your program while it's running.

## ownership rules

- Each value in Rust has a variable that's called its owner
- There can only be one owner at a time
- When the owner goes out of scope,the value will be dropped

## Defining an Enum

    enum IpAddrKind {
        V4,
        V6,
    }

# Snake Game