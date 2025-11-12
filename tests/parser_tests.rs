use anyhow::Result;
use tiny_lang_parser::{parse_program, ASTNode, EvalError, Interpreter};

///Test grammar rule: program
#[test]
fn test_program_rule() -> Result<()> {
    //Test empty program
    let result = parse_program("")?;
    assert!(result.is_empty());

    //Test program with multiple statements
    let result = parse_program("x = 1; y = 2;")?;
    assert_eq!(result.len(), 2);

    Ok(())
}

///Test grammar rule: assignment  
#[test]
fn test_assignment_rule() -> Result<()> {
    let result = parse_program("answer = 42;")?;

    if let ASTNode::Assignment { name, value } = &result[0] {
        assert_eq!(name, "answer");
        assert!(matches!(**value, ASTNode::Number(42)));
    } else {
        panic!("Expected assignment node");
    }

    Ok(())
}

///Test grammar rule: expression with operator precedence
#[test]
fn test_expression_rule() -> Result<()> {
    //Test that multiplication has higher precedence than addition
    let result = parse_program("2 + 3 * 4;")?;

    //Should parse as 2 + (3 * 4), not (2 + 3) * 4
    if let ASTNode::Add(left, right) = &result[0] {
        assert!(matches!(**left, ASTNode::Number(2)));
        if let ASTNode::Mul(l, r) = &**right {
            assert!(matches!(**l, ASTNode::Number(3)));
            assert!(matches!(**r, ASTNode::Number(4)));
        } else {
            panic!("Expected multiplication in right operand");
        }
    } else {
        panic!("Expected addition node");
    }

    Ok(())
}

///Test grammar rule: factor with parentheses
#[test]
fn test_factor_rule() -> Result<()> {
    //Test parentheses override precedence
    let result = parse_program("(2 + 3) * 4;")?;

    //Should parse as (2 + 3) * 4
    if let ASTNode::Mul(left, right) = &result[0] {
        assert!(matches!(**right, ASTNode::Number(4)));
        if let ASTNode::Add(l, r) = &**left {
            assert!(matches!(**l, ASTNode::Number(2)));
            assert!(matches!(**r, ASTNode::Number(3)));
        } else {
            panic!("Expected addition in left operand");
        }
    } else {
        panic!("Expected multiplication node");
    }

    Ok(())
}

///Test interpreter execution
#[test]
fn test_interpreter() -> Result<()> {
    let code = r#"
        x = 10;
        y = 5;
        z = x + y * 2;
    "#;

    let ast = parse_program(code)?;
    let mut interpreter = Interpreter::new();
    interpreter.eval(&ast)?;

    assert_eq!(interpreter.variables.get("x"), Some(&10));
    assert_eq!(interpreter.variables.get("y"), Some(&5));
    assert_eq!(interpreter.variables.get("z"), Some(&20));

    Ok(())
}

///Test error handling for undefined variable
#[test]
fn test_undefined_variable() -> Result<()> {
    let code = "result = undefined + 1;";
    let ast = parse_program(code)?;
    let mut interpreter = Interpreter::new();
    let result = interpreter.eval(&ast);

    assert!(result.is_err());

    if let Err(EvalError::UndefinedVariable(var_name)) = result {
        assert_eq!(var_name, "undefined");
    } else {
        panic!("Expected UndefinedVariable error");
    }

    Ok(())
}

///Test division by zero error
#[test]
fn test_division_by_zero() -> Result<()> {
    let code = "result = 5 / 0;";
    let ast = parse_program(code)?;
    let mut interpreter = Interpreter::new();
    let result = interpreter.eval(&ast);

    assert!(result.is_err());

    if let Err(EvalError::DivisionByZero) = result {
        //Expected error
    } else {
        panic!("Expected DivisionByZero error");
    }

    Ok(())
}

///Test complex expression evaluation
#[test]
fn test_complex_expression() -> Result<()> {
    let code = r#"
        a = 10;
        b = 2;
        c = (a + b) * 3 - 4 / 2;
    "#;

    let ast = parse_program(code)?;
    let mut interpreter = Interpreter::new();
    interpreter.eval(&ast)?;

    assert_eq!(interpreter.variables.get("a"), Some(&10));
    assert_eq!(interpreter.variables.get("b"), Some(&2));
    assert_eq!(interpreter.variables.get("c"), Some(&34));

    Ok(())
}
