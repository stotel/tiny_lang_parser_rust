use tiny_lang_parser::{parse_program, Interpreter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Test 1");
    let code1 = "x = 10;";
    let ast1 = parse_program(code1).map_err(|e| format!("Parse error: {:?}", e))?;
    println!("AST: {:#?}", ast1);
    let mut interp = Interpreter::new();
    interp.eval(&ast1)?;
    println!("Variables: {:?}", interp.variables);
    println!();

    println!("Test 2");
    let code2 = "y = 5 + 3;";
    let ast2 = parse_program(code2).map_err(|e| format!("Parse error: {:?}", e))?;
    println!("AST: {:#?}", ast2);
    interp.eval(&ast2)?;
    println!("Variables: {:?}", interp.variables);
    println!();

    println!("Test 3");
    let original_code = r#"
        x = 10;
        y = 5;
        z = x + y * 2;
    "#;
    let ast_original = parse_program(original_code).map_err(|e| format!("Parse error: {:?}", e))?;
    println!("AST: {:#?}", ast_original);
    let mut interp2 = Interpreter::new();
    interp2.eval(&ast_original)?;
    println!("Variables: {:?}", interp2.variables);

    Ok(())
}