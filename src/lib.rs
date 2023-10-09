#![feature(c_unwind)]
#[macro_use] extern crate gmod;

use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use gmod::lua::{State};

// files

unsafe extern "C-unwind" fn create_file(lua: State) -> i32 {
	let name = lua.check_string(1).to_string();
	let content = lua.check_string(2).to_string();

	let mut file = File::create(format!("garrysmod/{}", name)).expect("Couldn't create file");
	file.write_all(content.as_bytes()).expect("Couldn't write data in file");

	0
}

unsafe extern "C-unwind" fn remove_file(lua: State) -> i32 {
	let name = lua.check_string(1).to_string();
	fs::remove_file(format!("garrysmod/{}", name)).expect("Couldn't remove file");
	
	0
}

// dirs

unsafe extern "C-unwind" fn create_dir(lua: State) -> i32 {
	let name = lua.check_string(1).to_string();
	fs::create_dir(format!("garrysmod/{}", name)).expect("Couldn't create dir");

	0
}

unsafe extern "C-unwind" fn remove_dir(lua: State) -> i32 {
	let name = lua.check_string(1).to_string();
	fs::remove_dir(format!("garrysmod/{}", name)).expect("Couldn't remove dir");

	0
}

// other

unsafe extern "C-unwind" fn exists(lua: State) -> i32 {
	let name = lua.check_string(1).to_string();

	lua.push_boolean(Path::new(&format!("garrysmod/{}", name)).exists());

	1
}

#[gmod13_open]
unsafe fn gmod13_open(lua: State) -> i32 {
  lua.get_global(lua_string!("filesystem"));

	if lua.is_nil(-1) {
		lua.pop();
		lua.new_table();
	}

	lua.push_string(env!("CARGO_PKG_VERSION"));
	lua.set_field(-2, lua_string!("version"));

	// files

	lua.push_function(create_file);
	lua.set_field(-2, lua_string!("Create"));

	lua.push_function(remove_file);
	lua.set_field(-2, lua_string!("Remove"));

	// dirs

	lua.push_function(create_dir);
	lua.set_field(-2, lua_string!("DirCreate"));

	lua.push_function(remove_dir);
	lua.set_field(-2, lua_string!("DirRemove"));

	lua.push_function(exists);
	lua.set_field(-2, lua_string!("Exists"));

	lua.set_global(lua_string!("filesystem"));

  0
}

#[gmod13_close]
unsafe fn gmod13_close(_lua: State) -> i32 {
  0
}