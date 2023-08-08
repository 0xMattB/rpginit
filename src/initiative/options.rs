/***************************************************************************************************
* RPG Initiative Tracker
* Version 0.3.0 (08/04/23)
* options.rs
* 0xMattB
***************************************************************************************************/
pub struct Options {
	show_initiative: bool,
}

impl Options {
	pub const fn new() -> Options {
		Options {
			show_initiative: true,
		}
	}
	
	pub fn show_initiative(&self) -> bool {
		self.show_initiative
	}
	
	pub fn show(&mut self, show: bool) {
		self.show_initiative = show;
	}
}