use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;


fn main() {

	let f = File::open("puzzlebaseline.txt").unwrap();
	let reader = BufReader::new(f);
	
	for line_ in reader.lines() {
		let line = line_.unwrap();
		println!("{} ({} bytes long)", line, line.len());
		
		// create regex pattern
		
		let re = Regex::new("\\d").unwrap();
		
		match re.find(line) {
		 
		 Some(_) => println!("{}", line),
		 None => (),
		 
		}
		
		
		// regex find first number from beginning of line
		
		
		
		// regex find first number from end of line
		
		// loop from the front to the first number
/* 
		for char_ in line.???() {
			
		}
		
 */
		
/* 
		// loop from back to front
		for char_ in line.??() {
		
		}
		
 */
		
		// convert to number
		// put the number in an array
		// Add up the number
	}

}
