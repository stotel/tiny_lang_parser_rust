//! # Tiny Language Parser
//!
//! A parser and interpreter for a tiny programming language that supports
//! variable assignments and arithmetic operations.
//!
//! ## Example
//!
//! ```rust
//! use tiny_lang_parser::{parse_program, Interpreter};
//!
//! let code = "x = 10; y = x + 5;";
//! let ast = parse_program(code).unwrap();
//! let mut interpreter = Interpreter::new();
//! interpreter.eval(&ast).unwrap();
//! ```

mod parser;

pub use parser::{parse_program, ASTNode, EvalError, Interpreter, ParseError};

/// Main parsing function that takes source code and returns AST
///
/// Returns `ParseError` if the input cannot be parsed according to the grammar
///
/// # Examples
///
/// ```
/// use tiny_lang_parser::parse_program;
///
/// let ast = parse_program("x = 5 + 3;").unwrap();
/// assert!(!ast.is_empty());
/// ```
pub fn parse(input: &str) -> Result<Vec<ASTNode>, ParseError> {
    parse_program(input)
}
