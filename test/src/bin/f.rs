//= {
//=   "output": {
//=     "1": [
//=       "",
//=       true
//=     ],
//=     "2": [
//=       "",
//=       true
//=     ]
//=   },
//=   "children": [],
//=   "exit": {
//=     "Left": 1
//=   }
//= }

extern crate deploy;
use std::{process,thread,time};
use deploy::*;
fn main() {
	init(Resources{mem:20*1024*1024,..Resources::default()});
	thread::sleep(time::Duration::new(0,100_000_000));
	process::exit(1);
}
