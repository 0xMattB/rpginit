/***************************************************************************************************
* RPG Initiative Tracker
* Version 0.3.0 (08/04/23)
* entities.rs
* 0xMattB
***************************************************************************************************/
pub mod entity;

use entity::Entity;

pub struct Entities {
	list: Vec<Entity>,
}

impl Entities {
	pub const fn new() -> Self {
		Self {
			list: Vec::new(),
		}
	}

	pub fn len(&self) -> usize {
		self.list.len()
	}
	
	pub fn add(&mut self, name: String, permanent: bool) -> bool {
		if let Some(_) = Self::get_id_from_name(self, &name) {
			return false;
		} else {
			self.list.push(Entity::new(name, permanent));
			return true;
		}
	}
	
	pub fn remove(&mut self, name: &str) -> bool {
		if let Some(id) = Self::get_id_from_name(self, &name) {
			self.list.remove(id);
			return true;
		} else {
			return false;
		}
	}
	
	pub fn score_1_set(&mut self, name: &str, score: i32) -> bool {
		if let Some(id) = Self::get_id_from_name(self, &name) {
			self.list[id].score_1_set(score);
			return true;
		} else {
			return false;
		}
	}

	pub fn score_2_set(&mut self, name: &str, score: i32) -> bool {
		if let Some(id) = Self::get_id_from_name(self, &name) {
			self.list[id].score_2_set(score);
			return true;
		} else {
			return false;
		}
	}
	
	pub fn sort(&mut self) {
		self.list.sort_by_key(|e| e.score_2());
		self.list.sort_by_key(|e| e.score_1());
		self.list.reverse();
	}
	
	pub fn clear(&mut self) {
		let mut i = 0;
		
		while i < self.list.len() {
			if self.list[i].permanent() == true {
				self.list[i].clear();
				i += 1;
			} else {
				self.list.remove(i);
			}
		}
	}
	
	pub fn reset(&mut self) {
		self.list.clear();
	}
	
	pub fn get_as_string(&self, id: usize, show_init: bool) -> Option<String> {
		if self.list.len() > 0 && id < self.list.len() {
			return Some(self.list[id].get_as_string(Self::get_longest_name_len(self), show_init));
		}
		
		None
	}

	pub fn get_name_as_string(&self, name: &str, show_init: bool) -> Option<String> {
		if let Some(id) = Self::get_id_from_name(self, name) {
			return Some(format!["\n  {}", self.list[id].get_as_string(name.len(), show_init)]);
		}
		
		None
	}

	fn get_id_from_name(&self, name: &str) -> Option<usize> {
		let mut index = 0;
		
		for e in &self.list {
			if e.name() == name {
				return Some(index);
			}
			
			index += 1;
		}
		
		None
	}
	
	fn get_longest_name_len(&self) -> usize {
		let mut len = 0;
		
		for e in &self.list {
			if e.name().len() > len {
				len = e.name().len();
			}
		}
		
		len
	}
}