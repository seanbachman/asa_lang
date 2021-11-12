//Reads file, parses for tree, and executes

extern crate nom;
extern crate asa_lang;

use asa_lang::{program, run};
use std::{env, fs};
use regex::Regex;

fn main() {
  let re = Regex::new(r".+?\.asa").unwrap();
  let args: Vec<String> = env::args().collect();
  if args.len() == 2 {
    if re.is_match(&*args[1]) { //&* converts to &str from String
      let contents = fs::read_to_string(&*args[1]).expect("Something went wrong reading the file");
      let (_, parsetree) = program(&*contents).unwrap();
      //println!("Parse Tree:  {:?}", parsetree); //optionally print parse tree
      run(&parsetree);
    }
    else {
      println!("Error: files must end in .asa");
    }
  }
  else {
    println!("Error: Invalid input, try \"cargo run <filename>.asa\"");
  }
}