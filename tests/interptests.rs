use tiny_lang_parser::{parse_program, Interpreter};

#[test]
fn test_simple_assignment() {
    let ast = parse_program("x = 10;").unwrap();
    let mut interp = Interpreter::new();
    interp.eval(&ast).unwrap();
    assert_eq!(interp.variables.get("x"), Some(&10));
}

#[test]
fn test_expression_evaluation() {
    let ast = parse_program("a = 5;").unwrap();
    let mut interp = Interpreter::new();
    interp.eval(&ast).unwrap();
    assert_eq!(interp.variables.get("a"), Some(&5));
}

#[test]
fn test_nested_expression() {
    let ast = parse_program("x = (2 + 3) * 4;").unwrap();
    let mut interp = Interpreter::new();
    interp.eval(&ast).unwrap();
    assert_eq!(interp.variables.get("x"), Some(&20));
}

#[test]
fn test_undefined_variable_error() {
    let ast = parse_program("y + 1;").unwrap();
    let mut interp = Interpreter::new();
    let result = interp.eval(&ast);
    assert!(result.is_err());
}
