use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "tiny_lang.pest"]
pub struct TinyLangParser;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Number(i64),
    Identifier(String),
    Assignment { name: String, value: Box<ASTNode> },
    Add(Box<ASTNode>, Box<ASTNode>),
    Sub(Box<ASTNode>, Box<ASTNode>),
    Mul(Box<ASTNode>, Box<ASTNode>),
    Div(Box<ASTNode>, Box<ASTNode>),
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Pest parse error: {0}")]
    PestError(#[from] pest::error::Error<Rule>),
    #[error("Unexpected rule: {0:?}")]
    UnexpectedRule(Rule),
    #[error("Invalid number: {0}")]
    InvalidNumber(String),
    #[error("Expected {expected:?}, but found end of input")]
    UnexpectedEnd { expected: Rule },
}

pub fn parse_program(input: &str) -> Result<Vec<ASTNode>, ParseError> {
    let pairs = TinyLangParser::parse(Rule::program, input)?;
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

fn parse_expression(pair: pest::iterators::Pair<Rule>) -> Result<ASTNode, ParseError> {
    let mut pairs: Vec<_> = pair.into_inner().collect();
    
    if pairs.is_empty() {
        return Err(ParseError::UnexpectedEnd {
            expected: Rule::term,
        });
    }
    
    let mut current_node = parse_term(pairs.remove(0))?;
    
    //Process pairs in chunks of 2: (operator, term)
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
                "+" => ASTNode::Add(Box::new(current_node), Box::new(parse_term(term_pair.clone())?)),
                "-" => ASTNode::Sub(Box::new(current_node), Box::new(parse_term(term_pair.clone())?)),
                _ => return Err(ParseError::UnexpectedRule(op_pair.as_rule())),
            },
            _ => return Err(ParseError::UnexpectedRule(op_pair.as_rule())),
        };
        
        i += 2;
    }
    
    Ok(current_node)
}

fn parse_term(pair: pest::iterators::Pair<Rule>) -> Result<ASTNode, ParseError> {
    let mut pairs: Vec<_> = pair.into_inner().collect();
    
    if pairs.is_empty() {
        return Err(ParseError::UnexpectedEnd {
            expected: Rule::factor,
        });
    }
    
    let mut current_node = parse_factor(pairs.remove(0))?;
    
    //Process pairs in chunks of 2: (operator, factor)
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
                "*" => ASTNode::Mul(Box::new(current_node), Box::new(parse_factor(factor_pair.clone())?)),
                "/" => ASTNode::Div(Box::new(current_node), Box::new(parse_factor(factor_pair.clone())?)),
                _ => return Err(ParseError::UnexpectedRule(op_pair.as_rule())),
            },
            _ => return Err(ParseError::UnexpectedRule(op_pair.as_rule())),
        };
        
        i += 2;
    }
    
    Ok(current_node)
}

fn parse_factor(pair: pest::iterators::Pair<Rule>) -> Result<ASTNode, ParseError> {
    let inner = pair.into_inner().next().ok_or(ParseError::UnexpectedEnd {
        expected: Rule::number,
    })?;
    
    match inner.as_rule() {
        Rule::number => {
            let num_str = inner.as_str();
            num_str.parse()
                .map(ASTNode::Number)
                .map_err(|_| ParseError::InvalidNumber(num_str.to_string()))
        }
        Rule::identifier => Ok(ASTNode::Identifier(inner.as_str().to_string())),
        Rule::expression => parse_expression(inner),
        rule => Err(ParseError::UnexpectedRule(rule)),
    }
}

pub struct Interpreter {
    pub variables: HashMap<String, i64>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn eval(&mut self, nodes: &[ASTNode]) -> Result<(), String> {
        for node in nodes {
            self.eval_node(node)?;
        }
        Ok(())
    }

    fn eval_node(&mut self, node: &ASTNode) -> Result<i64, String> {
        match node {
            ASTNode::Number(n) => Ok(*n),
            ASTNode::Identifier(name) => {
                self.variables.get(name)
                    .copied()
                    .ok_or_else(|| format!("Undefined variable '{}'", name))
            }
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
                    return Err("Division by zero".to_string());
                }
                Ok(left_val / right_val)
            }
        }
    }
}