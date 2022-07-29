# Asa Lang

**Sean Bachman**

## Introduction

Asa lang (named after Lehigh University's [Asa Packer](https://en.wikipedia.org/wiki/Asa_Packer)) is a project created to explore the facets of writing your own language. Originally inspired by an assignment from Lehigh's [CSE 262](https://engineering.lehigh.edu/cse/academics/course-index/cse-262-programming-languages-3), the project contains a tree walk interpreter written in rust and leveraging the [nom](https://docs.rs/nom/7.1.0/nom/) crate. The project started off as a formal EBNF grammar and has slowly evolved into an actual programming language.

## Usage

To use this project, install rust and clone the repository to your local machine. Write an asa program and then type "cargo run filename.asa" to see the results!

## Asa Syntax

Warning: Asa was designed as a project to facilitate learning, and not for practical use. However, if you want to write your own asa program, the syntax is not too hard to pick up. If you are familiar with EBNF, check out the "grammar.ebnf" file. There are also some example .asa files in the repository that may be good to look at too. Here are some of the basics.

### Variable definition
```aidl
let <identifier> = <expression>;
```

### Function definition
```aidl
fn foo() {
    return 1;
}
```

### Full Program
```
fn foo(a,b,c) {
  let x = a + 1; 
  // This is a comment
  let y = bar(c - b);
  return x * y; // Multiply the results
}

fn bar(a) {
  return a * 3;
}

fn main() {
  return foo(1,2,3);  
}
```

So far the language supports writing functions, calling functions, math, and return values. Strings and Booleans are also data types in the language, but the runtime doesn't do anything with them. See `tests/parser.rs` for more examples of valid and invalid asa programs.

### Quirks of Asa
Asa contains a few "odd" features that may introduce some difficulties when writing programs. These may be fixed in the future but I will just list them here for now
- Function arguments and parameters cannot contain spaces. Write like this:
```
fn main() {
  foo(a,b,c)
}
fn foo(a,b,c) {
  return 0;
}
```
Not like this:
```
fn main() {
  foo(a, b, c)
}
fn foo(a, b, c) {
  return 0;
}
```
- Lines that only contain a function call do not end in semicolons. Write like this:
```
fn main() {
  foo(a,b,c)
  print(foo(a,b,c))
}
fn foo(a,b,c) {
  return 0;
}
```
Not like this:
```
fn main() {
  foo(a, b, c);
  print(foo(a,b,c));
}
fn foo(a, b, c) {
  return 0;
}
```

## Tree walk interpreters
The output of my parser is a tree of Nodes. You can find a list of Node types in `src/parser.rs` The tree should contains single root "Program" node. The rest of the tree is determined by what is entered in the .asa file. If you want to learn more, I recommend checking out this awesome book, [crafting interpreters](https://craftinginterpreters.com/).

## Parser Combinators

This project uses the [nom](https://crates.io/crates/nom) parser combinator library in Rust. Here is a blurb from their page:

Parser combinators are an approach to parsers that is very different from software like lex and yacc. Instead of writing the grammar in a separate syntax and generating the corresponding code, you use very small functions with a very specific purpose, like "take 5 bytes", or "recognize the word 'HTTP'", and assemble then in meaningful patterns like "recognize 'HTTP', then a space, then a version". The resulting code is small, and looks like the grammar you would have written with other parser approaches.

This gives us a few advantages:

- the parser is small and easy to write
- the parser's components are easy to reuse (if they're general enough, please add them to nom!)
- the parser's components are easy to test separately (with unit tests as in `/tests/parser.rs`)
- the parser combination code looks close to the grammar you would have written
- you can build partial parsers, specific to the data you need at the moment, and ignore the rest
- Here is an example of one such parser, to recognize text between parentheses:

```rust
use nom::{
  IResult,
  sequence::delimited,
  // see the "streaming/complete" paragraph lower for an explanation of these submodules
  character::complete::char,
  bytes::complete::is_not
};

fn parens(input: &str) -> IResult<&str, &str> {
  delimited(char('('), is_not(")"), char(')'))(input)
}
```

It defines a function named `parens` which will recognize a sequence of the character `(`, the longest byte array not containing `)`, then the character `)`, and will return the byte array in the middle.

Here is another parser, written without using nom's combinators this time:

```rust
#[macro_use]
extern crate nom;

use nom::{IResult, Err, Needed};

fn take4(i: &[u8]) -> IResult<&[u8], &[u8]>{
  if i.len() < 4 {
    Err(Err::Incomplete(Needed::Size(4)))
  } else {
    Ok((&i[4..], &i[0..4]))
  }
}
```

This function takes a byte array as input, and tries to consume 4 bytes. Writing all the parsers manually, like this, is dangerous, despite Rust's safety features. There are still a lot of mistakes one can make. That's why nom provides a list of function and macros to help in developing parsers.

With nom, you would write it like this:

```rust
use nom::{IResult, bytes::streaming::take};
fn take4(input: &str) -> IResult<&str, &str> {
  take(4u8)(input)
}
```

A parser in nom is a function which, for an input type I, an output type O and an optional error type E, will have the following signature:

```rust
fn parser(input: I) -> IResult<I, O, E>;
```

Or like this, if you don't want to specify a custom error type (it will be u32 by default):

```rust
fn parser(input: I) -> IResult<I, O>;
```

IResult is an alias for the Result type:

```rust
use nom::{Needed, error::ErrorKind};

type IResult<I, O, E = (I,ErrorKind)> = Result<(I, O), Err<E>>;

enum Err<E> {
  Incomplete(Needed),
  Error(E),
  Failure(E),
}
```

It can have the following values:

- a correct result Ok((I,O)) with the first element being the remaining of the input (not parsed yet), and the second the output value;
- an error Err(Err::Error(c)) with c an error that can be built from the input position and a parser specific error
- an error Err(Err::Incomplete(Needed)) indicating that more input is necessary. Needed can indicate how much data is needed
- an error Err(Err::Failure(c)). It works like the Error case, except it indicates an unrecoverable error: we cannot backtrack and test another parser

Please refer to the ["choose a combinator"](https://github.com/Geal/nom/blob/master/doc/choosing_a_combinator.md) guide for an exhaustive list of parsers. See also the rest of the documentation [here](https://docs.rs/nom/5.0.1/nom/).

