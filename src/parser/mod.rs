use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "tiny_lang.pest"]
pub struct TinyLangParser;

/// Abstract Syntax Tree nodes representing the parsed program structure
#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    /// Represents a numeric literal (e.g., `42`)
    Number(i64),
    /// Represents a variable identifier (e.g., `x`)
    Identifier(String),
    /// Represents a variable assignment (e.g., `x = 5`)
    Assignment {
        /// The variable name being assigned to
        name: String,
        /// The value being assigned
        value: Box<ASTNode>,
    },
    /// Represents an addition operation (e.g., `a + b`)
    Add(Box<ASTNode>, Box<ASTNode>),
    /// Represents a subtraction operation (e.g., `a - b`)
    Sub(Box<ASTNode>, Box<ASTNode>),
    /// Represents a multiplication operation (e.g., `a * b`)
    Mul(Box<ASTNode>, Box<ASTNode>),
    /// Represents a division operation (e.g., `a / b`)
    Div(Box<ASTNode>, Box<ASTNode>),
}

/// Parser error types
#[derive(Debug, Error)]
pub enum ParseError {
    /// Error returned by the Pest parser
    #[error("Pest parse error: {0}")]
    PestError(#[from] Box<pest::error::Error<Rule>>),
    /// Unexpected grammar rule encountered during parsing
    #[error("Unexpected rule: {0:?}")]
    UnexpectedRule(Rule),
    /// Invalid number format
    #[error("Invalid number: {0}")]
    InvalidNumber(String),
    /// Unexpected end of input
    #[error("Expected {expected:?}, but found end of input")]
    UnexpectedEnd { expected: Rule },
}

/// Interpreter error types
#[derive(Debug, Error)]
pub enum EvalError {
    #[error("Undefined variable '{0}'")]
    UndefinedVariable(String),
    #[error("Division by zero")]
    DivisionByZero,
    #[error("Runtime error: {0}")]
    RuntimeError(String),
}

/// Interpreter that executes the AST and maintains variable state
#[derive(Debug, Default)]
pub struct Interpreter {
    /// HashMap storing variable names and their current values
    pub variables: HashMap<String, i64>,
}

impl Interpreter {
    /// Creates a new interpreter with empty variable state
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    /// Evaluates a sequence of AST nodes
    ///
    /// # Arguments
    ///
    /// * `nodes` - Slice of AST nodes to evaluate
    ///
    /// # Errors
    ///
    /// Returns `EvalError` if evaluation fails (e.g., undefined variable, division by zero)
    pub fn eval(&mut self, nodes: &[ASTNode]) -> Result<(), EvalError> {
        for node in nodes {
            self.eval_node(node)?;
        }
        Ok(())
    }

    /// Evaluates a single AST node and returns its value
    fn eval_node(&mut self, node: &ASTNode) -> Result<i64, EvalError> {
        match node {
            ASTNode::Number(n) => Ok(*n),
            ASTNode::Identifier(name) => self
                .variables
                .get(name)
                .copied()
                .ok_or_else(|| EvalError::UndefinedVariable(name.clone())),
            ASTNode::Assignment { name, value } => {
                let val = self.eval_node(value)?;
                self.variables.insert(name.clone(), val);
                Ok(val)
            }
            ASTNode::Add(l, r) => {
                let left_val = self.eval_node(l)?;
                let right_val = self.eval_node(r)?;
                Ok(left_val + right_val)
            }
            ASTNode::Sub(l, r) => {
                let left_val = self.eval_node(l)?;
                let right_val = self.eval_node(r)?;
                Ok(left_val - right_val)
            }
            ASTNode::Mul(l, r) => {
                let left_val = self.eval_node(l)?;
                let right_val = self.eval_node(r)?;
                Ok(left_val * right_val)
            }
            ASTNode::Div(l, r) => {
                let left_val = self.eval_node(l)?;
                let right_val = self.eval_node(r)?;
                if right_val == 0 {
                    return Err(EvalError::DivisionByZero);
                }
                Ok(left_val / right_val)
            }
        }
    }
}

/// Parses a complete program into a sequence of AST nodes
///
/// # Grammar Rule: program
///
/// A program consists of zero or more statements. This is the root-level
/// parsing function that processes the entire input according to the
/// program grammar rule.
///
/// # Arguments
///
/// * `input` - The source code to parse
///
/// # Returns
///
/// A vector of AST nodes representing the statements in the program
///
/// # Errors
///
/// Returns `ParseError` if the input doesn't conform to the grammar
pub fn parse_program(input: &str) -> Result<Vec<ASTNode>, ParseError> {
    let pairs = TinyLangParser::parse(Rule::program, input)
        .map_err(|e| ParseError::PestError(Box::new(e)))?;
    let mut nodes = Vec::new();

    for pair in pairs {
        if pair.as_rule() == Rule::program {
            for inner_pair in pair.into_inner() {
                if inner_pair.as_rule() == Rule::statement {
                    nodes.push(parse_statement(inner_pair)?);
                }
            }
        }
    }

    Ok(nodes)
}

/// Parses a single statement
///
/// # Grammar Rule: statement  
///
/// A statement is either an assignment or an expression followed by a semicolon.
/// This rule defines the basic units of execution in the language.
///
/// # Arguments
///
/// * `pair` - The Pest parse tree pair for the statement
///
/// # Returns
///
/// An AST node representing the statement
fn parse_statement(pair: pest::iterators::Pair<Rule>) -> Result<ASTNode, ParseError> {
    let mut inner = pair.into_inner();
    let stmt = inner.next().ok_or(ParseError::UnexpectedEnd {
        expected: Rule::statement,
    })?;

    match stmt.as_rule() {
        Rule::assignment => parse_assignment(stmt),
        Rule::expression => parse_expression(stmt),
        rule => Err(ParseError::UnexpectedRule(rule)),
    }
}

/// Parses a variable assignment
///
/// # Grammar Rule: assignment
///
/// An assignment consists of an identifier followed by an equals sign and
/// an expression. It creates or updates a variable in the interpreter's
/// environment.
///
/// Format: `identifier = expression`
///
/// # Arguments
///
/// * `pair` - The Pest parse tree pair for the assignment
///
/// # Returns
///
/// An AST node representing the assignment
fn parse_assignment(pair: pest::iterators::Pair<Rule>) -> Result<ASTNode, ParseError> {
    let mut inner = pair.into_inner();

    let name_pair = inner.next().ok_or(ParseError::UnexpectedEnd {
        expected: Rule::identifier,
    })?;
    let name = name_pair.as_str().to_string();

    let expr_pair = inner.next().ok_or(ParseError::UnexpectedEnd {
        expected: Rule::expression,
    })?;
    let value = parse_expression(expr_pair)?;

    Ok(ASTNode::Assignment {
        name,
        value: Box::new(value),
    })
}

/// Parses an expression with addition and subtraction operations
///
/// # Grammar Rule: expression
///
/// An expression consists of terms separated by addition or subtraction
/// operators. This rule handles operator precedence where addition and
/// subtraction have lower precedence than multiplication and division.
///
/// Format: `term (add_op term)*`
///
/// # Arguments
///
/// * `pair` - The Pest parse tree pair for the expression
///
/// # Returns
///
/// An AST node representing the expression
fn parse_expression(pair: pest::iterators::Pair<Rule>) -> Result<ASTNode, ParseError> {
    let mut pairs: Vec<_> = pair.into_inner().collect();

    if pairs.is_empty() {
        return Err(ParseError::UnexpectedEnd {
            expected: Rule::term,
        });
    }

    let mut current_node = parse_term(pairs.remove(0))?;

    // Process pairs in chunks of 2: (operator, term)
    let mut i = 0;
    while i < pairs.len() {
        if i + 1 >= pairs.len() {
            return Err(ParseError::UnexpectedEnd {
                expected: Rule::term,
            });
        }

        let op_pair = &pairs[i];
        let term_pair = &pairs[i + 1];

        current_node = match op_pair.as_rule() {
            Rule::add_op => match op_pair.as_str() {
                "+" => ASTNode::Add(
                    Box::new(current_node),
                    Box::new(parse_term(term_pair.clone())?),
                ),
                "-" => ASTNode::Sub(
                    Box::new(current_node),
                    Box::new(parse_term(term_pair.clone())?),
                ),
                _ => return Err(ParseError::UnexpectedRule(op_pair.as_rule())),
            },
            _ => return Err(ParseError::UnexpectedRule(op_pair.as_rule())),
        };

        i += 2;
    }

    Ok(current_node)
}

/// Parses a term with multiplication and division operations  
///
/// # Grammar Rule: term
///
/// A term consists of factors separated by multiplication or division
/// operators. This rule handles the higher precedence of multiplication
/// and division over addition and subtraction.
///
/// Format: `factor (mul_op factor)*`
///
/// # Arguments
///
/// * `pair` - The Pest parse tree pair for the term
///
/// # Returns
///
/// An AST node representing the term
fn parse_term(pair: pest::iterators::Pair<Rule>) -> Result<ASTNode, ParseError> {
    let mut pairs: Vec<_> = pair.into_inner().collect();

    if pairs.is_empty() {
        return Err(ParseError::UnexpectedEnd {
            expected: Rule::factor,
        });
    }

    let mut current_node = parse_factor(pairs.remove(0))?;

    // Process pairs in chunks of 2: (operator, factor)
    let mut i = 0;
    while i < pairs.len() {
        if i + 1 >= pairs.len() {
            return Err(ParseError::UnexpectedEnd {
                expected: Rule::factor,
            });
        }

        let op_pair = &pairs[i];
        let factor_pair = &pairs[i + 1];

        current_node = match op_pair.as_rule() {
            Rule::mul_op => match op_pair.as_str() {
                "*" => ASTNode::Mul(
                    Box::new(current_node),
                    Box::new(parse_factor(factor_pair.clone())?),
                ),
                "/" => ASTNode::Div(
                    Box::new(current_node),
                    Box::new(parse_factor(factor_pair.clone())?),
                ),
                _ => return Err(ParseError::UnexpectedRule(op_pair.as_rule())),
            },
            _ => return Err(ParseError::UnexpectedRule(op_pair.as_rule())),
        };

        i += 2;
    }

    Ok(current_node)
}

/// Parses a factor (number, identifier, or parenthesized expression)
///
/// # Grammar Rule: factor
///
/// A factor is the most basic unit in an expression. It can be:
/// - A numeric literal
/// - A variable identifier  
/// - A parenthesized expression (for explicit precedence control)
///
/// Format: `number | identifier | "(" expression ")"`
///
/// # Arguments
///
/// * `pair` - The Pest parse tree pair for the factor
///
/// # Returns
///
/// An AST node representing the factor
fn parse_factor(pair: pest::iterators::Pair<Rule>) -> Result<ASTNode, ParseError> {
    let inner = pair.into_inner().next().ok_or(ParseError::UnexpectedEnd {
        expected: Rule::number,
    })?;

    match inner.as_rule() {
        Rule::number => {
            let num_str = inner.as_str();
            num_str
                .parse()
                .map(ASTNode::Number)
                .map_err(|_| ParseError::InvalidNumber(num_str.to_string()))
        }
        Rule::identifier => Ok(ASTNode::Identifier(inner.as_str().to_string())),
        Rule::expression => parse_expression(inner),
        rule => Err(ParseError::UnexpectedRule(rule)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        let result = parse_program("42;").unwrap();
        assert_eq!(result, vec![ASTNode::Number(42)]);
    }

    #[test]
    fn test_parse_identifier() {
        let result = parse_program("x;").unwrap();
        assert_eq!(result, vec![ASTNode::Identifier("x".to_string())]);
    }

    #[test]
    fn test_parse_assignment() {
        let result = parse_program("x = 5;").unwrap();
        assert_eq!(
            result,
            vec![ASTNode::Assignment {
                name: "x".to_string(),
                value: Box::new(ASTNode::Number(5))
            }]
        );
    }

    #[test]
    fn test_parse_addition() {
        let result = parse_program("1 + 2;").unwrap();
        assert_eq!(
            result,
            vec![ASTNode::Add(
                Box::new(ASTNode::Number(1)),
                Box::new(ASTNode::Number(2))
            )]
        );
    }
}
