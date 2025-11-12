# Tiny Language Parser

A parser and interpreter for a tiny programming language implemented in Rust using Pest.

## Description

This project implements a complete parser and interpreter for a simple language that supports variable assignments and basic arithmetic operations. The parser generates AST which can then be executed by the interpreter.

## Technical Description

### Parsing Process

The parsing process follows these stages:

1. **Lexical Analysis**: The input string is tokenized using Pest grammar rules
2. **Syntax Analysis**: Tokens are parsed into an Abstract Syntax Tree (AST)
3. **Semantic Analysis**: The AST is executed by the interpreter

### Grammar Rules

The language supports four main grammar rules:

1. **Program**: Root rule containing zero or more statements
2. **Statement**: Basic execution units (assignments or expressions) 
3. **Assignment**: Variable assignments (`identifier = expression`)
4. **Expression**: Arithmetic expressions with operator precedence

### Grammar Diagram

program = { statement* }
statement = { (assignment | expression) ";" }
assignment = { identifier "=" expression }
expression = { term (add_op term)* }
term = { factor (mul_op factor)* }
factor = { number | identifier | "(" expression ")" }
add_op = { "+" | "-" }
mul_op = { "*" | "/" }
number = { ASCII_DIGIT+ }
identifier = { (ASCII_ALPHA_LOWER | "_")+ }

## Features

- **Parser**: Converts source code to AST using Pest
- **Interpreter**: Executes AST with variable storage
- **Error Handling**: Error types for parsing and evaluation
- **CLI**: CLI for file parsing
- **Testing**: Complete test coverage for all grammar rules