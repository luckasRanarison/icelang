# Icelang

## Description

Icelang is a minimal, dynamically-typed scripting language inspired by Lua and Rust. It is my first attempt at writing a tree-walking interpreter and also my first Rust project. As you might expect, it was not intended for serious use. It is not fast or efficient, but it is decent enough for basic computation.

## Overview

See [examples](./examples/) to see some of the features in action.

```sql
-- icelang syntaxes 🥶
-- Designed to be minimal and concise, inspired by lua and rust

-- inline comment

-- some conventions:
-- filename ends with .ic
-- semicolon ';' is optional
-- both ' and " can be used to create strings
-- sanke case for identifiers

<--
    This
    is
    a
    multi
    line
    comment
-->  

-- module system
set default_import = import("module_name"); -- default import
set component = import("module_name").prop; -- import a specific object prop
set component_with_path = import("../module_name"); -- unix like path

-- data types
set string = "Hello World";
set integer = 123;
set float = 1.2;
set bool = true;
set range = 0 to 5; -- non inclusive 
set array = [1, 2, 3, 4];
set undefined = null;
set object = {
    prop: "value",
    another: 1,
    method: lambda() {
        print(self.prop); 
    }
}

-- operators
-- arithmetic
-- + - * / % += -= *= /= %=
-- logic
-- ! == != > < >= <= and or

-- function (hoisted)
function hello(name) {
    print("Hello " + name);
}

-- lambda function
set lambda_function = lambda(name) {
    print("Hello " + name);
}

-- loops
for i in 0 to 5 {
    print("Hello World");
}

-- collection loop
-- number, string, array and object
for key, value in object {
    print(key + ": " + value);
}

set i = 1;

while(i <= 5) {
    print("Hello World");
    i += 1;
}

set i = 1;

loop {
    i += 1;

    if (i >= 5) {
        break;
    }
}

set cond_1 = true;
set cond_2 = false;

-- conditionals 
if (cond_1 and cond_2) {
    print("no");
} else if (cond_1) {
    print("yes");
} else {
    print("yes");
}

-- conditionals are expressions
set n = if (true) { 1 } else { 0 }

set a = 1;

match(a) {
    0, 4, 1: a = a + 2,
    2: {
        print("unreachable");
    },
    _: {
        print("Default");
    },
}

-- some builtin functions
-- I/O
print("Hello World");
set input = readline();

-- Utility
type_of(a);
parse_number("1");
length("hello");

-- Math
sqrt(8);
pow(2, 5);
floor(2.5);
round(2.5);
ceil(2.5);

-- module export
set my_var = "some text";

-- only one export per file is allowed
export(my_var);
```

## Usage

Download icelang from [release](https://github.com/luckasRanarison/icelang/releases/) or [build](##Build) it from source.

```bash
icelang # no arguments for REPL mode
icelang script.ic # to run a file
```

## Build

**NB: You must have the rust tool chain installed.**

Clone the repository and then enter the following commands.

```bash
cargo install --path . # install dependencies
cargo build --release
```

## Todo 

Some add-ons not directly related to the project itself:

- [ ] WASM Playground

- [ ] Vscode semantic highlight extension

- [ ] Bytecode interpreter

- [ ] Language server and Vscode client

