/***************************************************************************************************
* RPG Initiative Tracker
* Version 0.3.0 (08/04/23)
* command.rs
* 0xMattB
***************************************************************************************************/
use super::entities::Entities;
use super::options::Options;
use super::info;
use super::messages;

struct Command<'a> {
	text: &'a str,
	code: CommandCode,
	argc: usize,
}

#[derive(Copy, Clone)]
enum CommandCode {
	Invalid,
	WrongArgs,
	Null,
	Add0,
	Add1,
	Set,
	Set2,
	Start,
	Next,
	Remove,
	Clear,
	Reset,
	Print,
	PrintName,
	Sort,
	Show,
	Hide,
	Help,
	Example,
	Version,
}

const COMMAND_COUNT: usize = 20;

const COMMAND_LIST: [Command; COMMAND_COUNT] = [
	Command{ text: ""          , code: CommandCode::Invalid  , argc: 0 },
	Command{ text: ""          , code: CommandCode::WrongArgs, argc: 0 },
	Command{ text: ""          , code: CommandCode::Null     , argc: 0 },
	Command{ text: "!add0"     , code: CommandCode::Add0     , argc: 1 },
	Command{ text: "!add1"     , code: CommandCode::Add1     , argc: 1 },
	Command{ text: "!set"      , code: CommandCode::Set      , argc: 2 },
	Command{ text: "!set2"     , code: CommandCode::Set2     , argc: 2 },
	Command{ text: "!start"    , code: CommandCode::Start    , argc: 0 },
	Command{ text: "!next"     , code: CommandCode::Next     , argc: 0 },
	Command{ text: "!remove"   , code: CommandCode::Remove   , argc: 1 },
	Command{ text: "!clear"    , code: CommandCode::Clear    , argc: 0 },
	Command{ text: "!reset"    , code: CommandCode::Reset    , argc: 0 },
	Command{ text: "!print"    , code: CommandCode::Print    , argc: 0 },
	Command{ text: "!printname", code: CommandCode::PrintName, argc: 1 },
	Command{ text: "!sort"     , code: CommandCode::Sort     , argc: 0 },
	Command{ text: "!show"     , code: CommandCode::Show     , argc: 0 },
	Command{ text: "!hide"     , code: CommandCode::Hide     , argc: 0 },
	Command{ text: "!help"     , code: CommandCode::Help     , argc: 0 },
	Command{ text: "!example"  , code: CommandCode::Example  , argc: 0 },
	Command{ text: "!version"  , code: CommandCode::Version  , argc: 0 },
];

static mut CURRENT_SELECTION: usize = 0;
static mut CURRENT_ROUND: usize = 1;

pub fn execute(entities: &mut Entities, options: &mut Options, c: &mut Vec<String>, user: &mut String, response: &mut String) {
	let user = fix_name(user);
	
	if c.len() > 0 {
		let s = match get_command_code(c, user) {
			CommandCode::Invalid   => { command_invalid()                           },
			CommandCode::WrongArgs => { command_wrongargs()                         },
			CommandCode::Null      => { command_null()                              },
			CommandCode::Add0      => { command_add0(entities, &c[1])               },
			CommandCode::Add1      => { command_add1(entities, &c[1])               },
			CommandCode::Set       => { command_set(entities, &c[1], &c[2])         },
			CommandCode::Set2      => { command_set2(entities, &c[1], &c[2])        },
			CommandCode::Start     => { command_start(entities, options)            },
			CommandCode::Next      => { command_next(entities, options)             },
			CommandCode::Remove    => { command_remove(entities, &c[1])             },
			CommandCode::Clear     => { command_clear(entities)                     },
			CommandCode::Reset     => { command_reset(entities)                     },
			CommandCode::Print     => { command_print(entities, options)            },
			CommandCode::PrintName => { command_printname(entities, &c[1], options) },
			CommandCode::Sort      => { command_sort(entities)                      },
			CommandCode::Show      => { command_show(options)                       },
			CommandCode::Hide      => { command_hide(options)                       },
			CommandCode::Help      => { command_help()                              },
			CommandCode::Example   => { command_example()                           },
			CommandCode::Version   => { command_version()                           },
		};
		
		response.push_str(&s);
	}
}

fn fix_name(user: &mut String) -> String {
	let user = user.replace(" ", "-");
	let user = user.to_lowercase();
	
	user
}

fn get_command_code(c: &mut Vec<String>, user: String) -> CommandCode {
	for i in 3..COMMAND_COUNT {
		if c[0] == COMMAND_LIST[i].text {
			if COMMAND_LIST[i].argc == c.len() - 1 {
				return COMMAND_LIST[i].code;
			} else {
				c.insert(1, user);
				
				if COMMAND_LIST[i].argc == c.len() - 1 {
					return COMMAND_LIST[i].code;
				} else {
					return CommandCode::WrongArgs;
				}
			}
		}
	}
	
	CommandCode::Invalid
}

fn command_invalid() -> String {
	"! invalid command\n".to_string()
}

fn command_wrongargs() -> String {
	"! wrong number of arguments\n".to_string()
}

fn command_null() -> String {
	"".to_string()
}

fn command_add0(entities: &mut Entities, name: &str) -> String {
	if entities.add(name.to_string(), true) {
		"* entity succesfully added to list\n".to_string()
	} else {
		"! name already in use\n".to_string()
	}	
}

fn command_add1(entities: &mut Entities, name: &str) -> String {
	if entities.add(name.to_string(), false) {
		"* entity succesfully added to list\n".to_string()
	} else {
		"! name already in use\n".to_string()
	}	
}

fn command_set(entities: &mut Entities, name: &str, value: &str) -> String {
	let score: i32;
	
	if let Ok(n) = value.parse() {
		score = n;
	} else {
		return "! invalid number for 'score'\n".to_string();
	}
	
	if entities.score_1_set(name, score) {
		return "* score successfully updated\n".to_string();
	} else {
		return "! name not found\n".to_string();
	}
}

fn command_set2(entities: &mut Entities, name: &str, value: &str) -> String {
	let score: i32;
	
	if let Ok(n) = value.parse() {
		score = n;
	} else {
		return "! invalid number for 'score'\n".to_string();
	}
	
	if entities.score_2_set(name, score) {
		return "* score successfully updated\n".to_string();
	} else {
		return "! name not found\n".to_string();
	}
}

fn command_start(entities: &mut Entities, options: &Options) -> String {
	if entities.len() == 0 {
		return "! list empty\n".to_string();
	}
	
	entities.sort();
	
	unsafe {
		CURRENT_SELECTION = 0;
		CURRENT_ROUND = 1;
	}
	
	generate_turn_string(entities, options.show_initiative())
}

fn command_next(entities: &Entities, options: &Options) -> String {
	if entities.len() == 0 {
		return "! list empty\n".to_string();
	}
	
	unsafe {
		CURRENT_SELECTION += 1;
		
		if CURRENT_SELECTION >= entities.len() {
			CURRENT_SELECTION = 0;
			CURRENT_ROUND += 1;
		}
	}
	
	generate_turn_string(entities, options.show_initiative())
}

fn command_remove(entities: &mut Entities, name: &str) -> String {
	if entities.len() == 0 {
		"! list is empty\n".to_string()
	} else if entities.remove(name) {
		"* name removed from list\n".to_string()
	} else {
		"! name not found\n".to_string()
	}
}

fn command_clear(entities: &mut Entities) -> String {
	entities.clear();
	"* list cleared\n".to_string()
}

fn command_reset(entities: &mut Entities) -> String {
	entities.reset();
	"* list reset\n".to_string()
}

fn command_print(entities: &Entities, options: &Options) -> String {
	if entities.len() == 0 {
		return "! list is empty\n".to_string();
	} else {
		let mut result = String::from("\n");
		
		for id in 0..entities.len() {
			if let Some(s) = entities.get_as_string(id, options.show_initiative()) {
				result.push_str("  ");
				result.push_str(&s);
			}		
		}
		
		return result;
	}
}

fn command_printname(entities: &Entities, name: &str, options: &Options) -> String {
	if entities.len() == 0 {
		return "! list is empty\n".to_string();
	} else {
		if let Some(s) = entities.get_name_as_string(name, options.show_initiative()) {
			return s;
		} else {
			return "! name not found\n".to_string();
		}
	}
}

fn command_sort(entities: &mut Entities) -> String {
	if entities.len() == 0 {
		"! list is empty\n".to_string()
	} else {
		entities.sort();
		"* list sorted\n".to_string()
	}
}

fn command_show(options: &mut Options) -> String {
	options.show(true);
	"* scores will now be visible\n".to_string()
}

fn command_hide(options: &mut Options) -> String {
	options.show(false);
	"* scores will now be hidden\n".to_string()
}

fn command_help() -> String {
	messages::help()
}

fn command_example() -> String {
	messages::example()
}

fn command_version() -> String {
	format!["* {}\n", info::PROGRAM_VERSION]
}

fn generate_turn_string(entities: &Entities, show_init: bool) -> String {
	let mut response = String::new();

	unsafe {
		response.push_str("\n");
		response.push_str(&format!["Round {}\n", CURRENT_ROUND]);
		
		for i in 0..entities.len() {
			if i == CURRENT_SELECTION {
				response.push_str("* ");
			} else {
				response.push_str("  ");
			}
			
			response.push_str(&entities.get_as_string(i, show_init).unwrap());
		}
	}
	
	response
}