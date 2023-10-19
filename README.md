# bytecode-compiler

This project is built to explore how lexers, parsers, compilers and virtual machines work.

This project doesn't have any dependency and is built from scratch.

This project has unit tests for many of its modules.

# Virtual Machine Overview
Virtual machine implementation is like below.
```rust
pub struct VirtualMachine {
    stack: Vec<Value>,
    register: [Value; REGISTER_SIZE],
    bytecode: Vec<u8>,
    program_counter: usize,
}
```

# Language Overview
A sample program is below.
```js
PUSH 10 
PUSH 40
ADD
RET
```

# Opcodes
Opcode: **PUSH**

Pushes a value to the stack of the virtual machine.
```js
PUSH <value> // type of value is `i64` 
```

<br>

Opcode: **POP**

Removes the last value from the stack.
```js
POP 
```

<br>

Opcode: **STORE**

Stores the last value from the stack at specified index in the register of the virtual machine.
```js
STORE <index> // type of index is `u8` 
```

<br>

Opcode: **LOAD**

Pushes the value at the specified index in the register to the stack.
```js
LOAD <index> // type of index is `u8` 
```

<br>

Opcode: **ADD**

Removes the last 2 values from the stack. And pushes the sum of them back.
```js
ADD
```

<br>

Opcode: **SUB**

Removes the last 2 values from the stack. And pushes the subtraction of them back.
```js
SUB
```

<br>

Opcode: **MUL**

Removes the last 2 values from the stack. And pushes the multiplication of them back.
```js
MUL
```

<br>

Opcode: **DIV**

Removes the last 2 values from the stack. And pushes the division of them back.
```js
DIV
```

<br>

Opcode: **MOD**

Removes the last 2 values from the stack. And pushes the modulo of them back.
```js
MOD
```

<br>

Opcode: **RET**

Stops the execution of the program. And returns the values in the stack.
```js
RET
```





# Development

### Setup A Development Environment
You need [Rust Language](https://www.rust-lang.org/) installed.

Or you can use [Dev Containers](https://containers.dev/) to easily setup a development environment.

### Clone The Repo
Run the command below to clone the repository.
```sh
git clone https://github.com/BerzanXYZ/bytecode-compiler.git
```

### Set Your Working Directory
Run the command below to set your working directory to `bytecode-compiler/`.
```sh
cd bytecode-compiler/
```

### Build The Program
Run the command below to build the program.
```sh
cargo build --release
```

### Run The Examples
Run the command below to run the examples.
```sh
./target/release/bytecode-compiler examples/adding.code # or examples/complex.code
```