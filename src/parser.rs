//Main parsing code
//Breaks input down into a tree of nodes which are passed to runtime

//Nom imports
use nom::{
  IResult,
  branch::alt,
  multi::{many1, many0, separated_list},
  bytes::complete::{tag, is_not},
  character::complete::{alphanumeric1, digit1},
  sequence::delimited,
};

//Node types. See grammar.ebnf for details
#[derive(Debug, Clone)]
pub enum Node {
  Program { children: Vec<Node> },
  Statement { children: Vec<Node> },
  FunctionReturn { children: Vec<Node> },
  FunctionDefine { children: Vec<Node> },
  IfStatement { children: Vec<Node> },
  FunctionArguments { children: Vec<Node> },
  FunctionStatements { children: Vec<Node> },
  Expression { children: Vec<Node> },
  MathExpression {name: String, children: Vec<Node> },
  FunctionCall { name: String, children: Vec<Node> },
  VariableDefine { children: Vec<Node> },
  Number { value: i32 },
  Bool { value: bool },
  Identifier { value: String },
  String { value: String },
  Conditional {name: String, children: Vec<Node>},
  //TODO move print to standard library
  //TODO make hello world example with functions
  //TODO alter existing variables
  //TODO conditionals
  //TODO fib function example
  //TODO looping (non recursive)
}

//Define production rules

//variables
pub fn identifier(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(tag(" "))(input)?; //whitespace
  let (input, result) = alphanumeric1(input)?;
  Ok((input, Node::Identifier{value: result.to_string()}))
}

//number, boolean, and string define primative types
pub fn number(input: &str) -> IResult<&str, Node> {
  let (input, result) = digit1(input)?;
  let number = result.parse::<i32>().unwrap();
  Ok((input, Node::Number{value: number}))
}

pub fn boolean(input: &str) -> IResult<&str, Node> {
  let (input, result) = alt((tag("true"), tag("false")))(input)?;
  Ok((input, Node::Bool{value: result == "true"}))
}

pub fn string(input: &str) -> IResult<&str, Node> {
  let (input, result) = delimited(tag("\""), is_not("\""), tag("\""))(input)?;
  let string = result.parse::<String>().unwrap();
  Ok((input, Node::String {value: string}))
}

//In the form
//number (+, -, *, /, ^) number
pub fn math_expression(input: &str) -> IResult<&str, Node> {
  l1(input)
}

// Math expressions with parens
pub fn parenthetical_expression(input: &str) -> IResult<&str, Node> {
  let (input, _) = tag("(")(input)?;
  let (input, insides) = l1(input)?;
  let (input, _) = tag(")")(input)?;
  Ok((input, insides))
}

// L1 - L4 handle order of operations for math expressions
pub fn l4(input: &str) -> IResult<&str, Node> {
  alt((function_call, number, identifier, parenthetical_expression))(input)
}

pub fn l3_infix(input: &str) -> IResult<&str, Node> { //Exponentiation
  let (input, _) = many0(tag(" "))(input)?;
  let (input, op) = tag("^")(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, args) = l4(input)?;
  Ok((input, Node::MathExpression{name: op.to_string(), children: vec![args]}))
}

pub fn l3(input: &str) -> IResult<&str, Node> {
  let (input, mut head) = l4(input)?;
  let (input, tail) = many0(l3_infix)(input)?;
  for n in tail {
    match n {
      Node::MathExpression{name, mut children} => {
        let mut new_children = vec![head.clone()];
        new_children.append(&mut children);
        head = Node::MathExpression{name, children: new_children};
      }
      _ => ()
    };
  }
  Ok((input, head))
}

pub fn l2_infix(input: &str) -> IResult<&str, Node> { //multiplication and division
  let (input, _) = many0(tag(" "))(input)?;
  let (input, op) = alt((tag("*"),tag("/")))(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, args) = l3(input)?;
  Ok((input, Node::MathExpression{name: op.to_string(), children: vec![args]}))
}

pub fn l2(input: &str) -> IResult<&str, Node> {
  let (input, mut head) = l3(input)?;
  let (input, tail) = many0(l2_infix)(input)?;
  for n in tail {
    match n {
      Node::MathExpression{name, mut children} => {
        let mut new_children = vec![head.clone()];
        new_children.append(&mut children);
        head = Node::MathExpression{name, children: new_children};
      }
      _ => ()
    };
  }
  Ok((input, head))
}

pub fn l1_infix(input: &str) -> IResult<&str, Node> { //addition and subtraction
  let (input, _) = many0(tag(" "))(input)?;
  let (input, op) = alt((tag("+"),tag("-")))(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, args) = l2(input)?;
  Ok((input, Node::MathExpression{name: op.to_string(), children: vec![args]}))
}

pub fn l1(input: &str) -> IResult<&str, Node> {
  let (input, mut head) = l2(input)?;
  let (input, tail) = many0(l1_infix)(input)?;
  for n in tail {
    match n {
      Node::MathExpression{name, mut children} => {
        let mut new_children = vec![head.clone()];
        new_children.append(&mut children);
        head = Node::MathExpression{name, children: new_children};
      }
      _ => () 
    };
  }
  Ok((input, head))
}

//In the form
//<name>(<args>);
pub fn function_call(input: &str) -> IResult<&str, Node> {
  let (input, name) = alphanumeric1(input)?;
  let (input, _) = tag("(")(input)?;
  let (input, arguments) = separated_list(tag(","), expression)(input)?;
  let (input, _) = tag(")")(input)?;
  Ok((input, Node::FunctionCall{name: name.to_string(), children: vec![Node::FunctionArguments {children: arguments}]}))
}

//Expression is anything that can be evaluated to a result (function calls, math, identifiers) and primatives (boolean, number, string)
pub fn expression(input: &str) -> IResult<&str, Node> {
  let (input, expression) = alt((boolean, math_expression, number, function_call, string, identifier))(input)?; //order most specific to least specific to avoid errors
  Ok((input, Node::Expression{ children: vec![expression]}))
}

pub fn conditional(input: &str) -> IResult<&str, Node> {
  let (input, lhs) = expression(input)?;
  let (input, _) = many0(tag(" "))(input)?; //whitespace
  let (input, op) = alt((tag("=="), tag("!=")))(input)?;
  let (input, _) = many0(tag(" "))(input)?; //whitespace
  let (input, rhs) = expression(input)?;
  Ok((input, Node::Conditional{name: op.to_string(), children: vec![lhs, rhs]}))
}

pub fn if_statement(input: &str) -> IResult<&str, Node> {
  let (input, _) = tag("if")(input)?;
  let (input, _) = many0(tag(" "))(input)?; //whitespace
  let (input, _) = tag("(")(input)?;
  let (input, condition) = conditional(input)?;
  let (input, _) = tag(")")(input)?;
  let (input, _) = many0(alt((tag(" "), tag("\t"), tag("\n"))))(input)?; //whitespace
  let (input, _) = tag("{")(input)?;
  let (input, _) = many0(alt((tag(" "), tag("\t"), tag("\n"))))(input)?; //whitespace
  let (input, mut statements) = many1(statement)(input)?;
  let (input, _) = many0(alt((tag(" "), tag("\t"), tag("\n"))))(input)?; //whitespace
  let (input, _) = tag("}")(input)?;

  let mut ifstatement = vec![condition];
  ifstatement.append(&mut statements);
  Ok((input, Node::IfStatement { children: ifstatement}))
}

//In the form
//let x = expression;
pub fn variable_define(input: &str) -> IResult<&str, Node> {
  let (input, _) = tag("let ")(input)?;
  let (input, variable) = identifier(input)?;
  let (input, _) = many0(tag(" "))(input)?; //whitespace
  let (input, _) = tag("=")(input)?;
  let (input, _) = many0(tag(" "))(input)?; //whitespace
  let (input, expression) = expression(input)?;
  let (input, _) = tag(";")(input)?;
  Ok((input, Node::VariableDefine{ children: vec![variable, expression]}))
}

//In the form
//return <evaluable>
pub fn function_return(input: &str) -> IResult<&str, Node> {
  let (input, _) = tag("return ")(input)?;
  let (input, return_val) = alt((function_call, expression, identifier))(input)?;
  let (input, _) = tag(";")(input)?;
  Ok((input, Node::FunctionReturn {children: vec![return_val]}))
}

pub fn statement(input: &str) -> IResult<&str, Node> {
  let (input, statement) = alt((variable_define, function_call, function_return, if_statement))(input)?;
  let (input, _) = many0(alt((tag(" "), tag("\t"), tag("\n"))))(input)?; //whitespace
  Ok((input, Node::Statement{ children: vec![statement]}))
}

//In the form
//fn <name>(<args>){<statements>}
pub fn function_definition(input: &str) -> IResult<&str, Node> { //TODO: find more elegant way to handle whitespace
  let (input, _) = many0(alt((tag(" "), tag("\t"), tag("\n"))))(input)?; //whitespace
  let (input, _) = tag("fn ")(input)?;
  let (input, _) = many0(alt((tag(" "), tag("\t"), tag("\n"))))(input)?; //whitespace
  let (input, name) = identifier(input)?;
  let (input, _) = tag("(")(input)?;
  let (input, arguments) = separated_list(tag(","), expression)(input)?;
  let (input, _) = tag(")")(input)?;
  let (input, _) = many0(alt((tag(" "), tag("\t"), tag("\n"))))(input)?; //whitespace
  let (input, _) = tag("{")(input)?;
  let (input, _) = many0(alt((tag(" "), tag("\t"), tag("\n"))))(input)?; //whitespace
  let (input, mut statements) = many1(statement)(input)?;
  let (input, _) = many0(alt((tag(" "), tag("\t"), tag("\n"))))(input)?; //whitesapce
  let (input, _) = tag("}")(input)?;
  let (input, _) = many0(alt((tag(" "), tag("\t"), tag("\n"))))(input)?; //whitespace

  let mut function = vec![name];
  function.append(&mut vec![Node::FunctionArguments { children: arguments }]);
  function.append(&mut statements);
  Ok((input, Node::FunctionDefine {children: function}))
}

/*pub fn comment(_input: &str) -> IResult<&str, Node> {
  unimplemented!();
}
 //TODO: implement comments
 */

/*
pub fn print(input: &str) -> IResult<&str, Node> {
  let (input, _) = tag("print(")(input)?;
  let (_, child) = expression(input)?;
  let (input, _) = tag(");")(input)?;
  Ok((input, Node::Print {child: vec![child]}))
}
 */

//Full program
//Either functions or statement/expression (statement/expression allows asa to be interpreted)
pub fn program(input: &str) -> IResult<&str, Node> {
  let (input, result) = many1(alt((function_definition, statement, expression)))(input)?;
  Ok((input, Node::Program{ children: result}))
}