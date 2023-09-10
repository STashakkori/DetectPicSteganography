// PicSec
// By: m0nZSt3r and $t@$h, QVLx Labs

use std::process::Command;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::fs::File;
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 3 {
    println!("Not enough arguments given.");
    return;
  }
  if args.len() > 3 {
    println!("Too many arguments given.");
    return;
  }
  if Path::new("diff.txt").exists() { return; }

  let mut arguments: Vec<&str> = Vec::new();
	let mut arguments_for_src: Vec<&str> = Vec::new();
	let mut arguments_for_sus: Vec<&str> = Vec::new();

  let source_string = args[1].trim();
  let suspect_string = args[2].trim();	
	
  // if src path exists, then we can at least run rascii once, and print it to std out
	let path_check = Path::new(&source_string);
	let path_check2 = Path::new(&suspect_string);
	if path_check.exists() && path_check2.exists() {
		// add default arguments for printing in terminal
		arguments.push(&source_string);
		arguments.push("-p");
		arguments.push("2");
		let output_sh = match Command::new("./rascii").args(arguments.clone()).output() {
			Ok(out) => out,
			Err(e) => { 
			println!("heres an error: {}", e);return;}
		};    
			
		let stdout_str = String::from_utf8_lossy(&output_sh.stdout);
		print!("{}", stdout_str);
		print_in_color("Analyzing...","grey");
		
		// done with first command call, prep two argument vectors 
		// for the hi-res calls for source and suspect.
		arguments_for_src.push(&source_string);
		arguments_for_src.push("-p");
		arguments_for_src.push("1.5");
		arguments_for_src.push("-w");
		arguments_for_src.push("600");
		arguments_for_sus.push(&suspect_string);
		arguments_for_sus.push("-p");
		arguments_for_sus.push("1.5");
		arguments_for_sus.push("-w");
		arguments_for_sus.push("600");	
		
		// ascii_src is the raw result of running rascii, ascii_src_str is converted to utf-8  
		let ascii_src = match Command::new("./rascii").args(arguments_for_src).output() {
			Ok(out) => out,
			Err(e) => {println!("heres an error: {}", e);return;}
		};
		let ascii_src_str = String::from_utf8_lossy(&ascii_src.stdout);
	
		// ascii_sus is the raw result of running rascii, ascii_sus_str is converted to utf-8 
		let ascii_sus = match Command::new("./rascii").args(arguments_for_sus).output() {
			Ok(out) => out,
			Err(e) => {println!("heres an error: {}", e);return;}
		};	
		let ascii_sus_str = String::from_utf8_lossy(&ascii_sus.stdout);

		// do the diff of the two files and store it in diff_result, convert it to utf-8
		let diff_result = match Command::new("./diff").arg(source_string).arg(suspect_string).output() {
			Ok(out) => out,
			Err(e) => {println!("heres an error: {}", e);return;}
		};
		let diff_check = String::from_utf8_lossy(&diff_result.stdout);

		// create out_file path and BufWriter.
		let out_filename = String::from("diff.txt");
	  let out_path = Path::new(&out_filename);
		let output_file = match File::create(out_path) {
			Ok(x) => x,
			Err(e) => { println!("Error: {}", e);return;}
		};
		let mut out_handle = BufWriter::new(output_file);
	
		// for loop for iterating through strings so we can either write the same characters,
		// or write a block character if they arent the same.
		for i in 0..ascii_src_str.len() {
		
			if ascii_src_str.as_bytes()[i] == ascii_sus_str.as_bytes()[i] {
				write!(out_handle, "{}", ascii_sus_str.as_bytes()[i] as char).expect("error couldnt write.");	
			}
			else {
				write!(out_handle, "█").expect("error couldnt write.");
			}
		}

		// diff results
		if diff_check.contains("differ") {
			print_in_color("Found evidence of tampering. Possible malicious insertion.", "orange");
		}
		else {
			print_in_color("Two files are equivalent. No tampering was detected.", "green");
		}
		print_in_color("Comparison successful, details written to: diff.txt", "grey");
	}
	else {
		println!("Uh oh, your file paths were invalid.");
	}
}

pub fn print_in_color(msg: &str, color: &str) {
  match color {
    "orange" => print!("\x1b[38;5;208m{}\n", msg),
    "green" => print!("\x1b[38;5;118m{}\n", msg),
    "grey" => print!("\x1b[38;5;111m{}\n", msg),
    _ => print!("{}", msg)
  }
}
