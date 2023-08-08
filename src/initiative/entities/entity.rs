/***************************************************************************************************
* RPG Initiative Tracker
* Version 0.3.0 (08/04/23)
* entity.rs
* 0xMattB
***************************************************************************************************/
pub struct Entity {
	name: String,
	score_1: i32,
	score_2: i32,
	permanent: bool,
}

pub static ENTITY_NAME_LEN_MAX: usize = 26;

impl Entity {
	pub fn new(mut name: String, permanent: bool) -> Self {
		name.truncate(ENTITY_NAME_LEN_MAX);
		
		Self {
			name,
			score_1: 0,
			score_2: 0,
			permanent,
		}
	}
	
	pub fn name(&self) -> &String {
		&self.name
	}
	
	pub fn score_1(&self) -> i32 {
		self.score_1
	}
	
	pub fn score_2(&self) -> i32 {
		self.score_2
	}
	
	pub fn permanent(&self) -> bool {
		self.permanent
	}
	
	pub fn score_1_set(&mut self, score: i32) {
		self.score_1 = score;
	}
	
	pub fn score_2_set(&mut self, score: i32) {
		self.score_2 = score;
	}
	
	pub fn clear(&mut self) {
		self.score_1 = 0;
		self.score_2 = 0;
	}
	
	pub fn get_as_string(&self, name_len: usize, show_initiative: bool) -> String {
		if show_initiative {
			format!["{0:wname$} ({1:2}, {2:2})\n", self.name, self.score_1, self.score_2, wname=name_len]
		} else {
			format!["{}\n", self.name]
		}
	}
}