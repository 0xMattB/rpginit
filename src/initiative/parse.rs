/***************************************************************************************************
* RPG Initiative Tracker
* Version 0.3.0 (08/04/23)
* parse.rs
* 0xMattB
***************************************************************************************************/
pub fn parse(input: &str) -> Vec<String> {
	let collect: Vec<&str> = input.split_whitespace().collect();
	let mut result: Vec<String> = Vec::new();
	
	for c in collect {
		result.push(c.to_string());
	}
	
	result
}