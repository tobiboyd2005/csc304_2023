///associate greetings module with this crate
mod greetings;
extern crate hello_world_lib;
///Optionally oad each member of greetings
/*use greetings::default_greeting;
use greetings::spanish;
use greetings::french;*/
///Alternatively, use * to load them all
//use greetings::*;
///Alternatively, load them in one line
use greetings::{default_greeting, spanish, french, japanese};
fn main() {
println!("Hello, world!");
println!("{}", default_greeting());
println!("{}", spanish::default_greeting());
println!("{}", french::default_greeting());
println!("{}", japanese::default_greeting());
println!("{}", hello_world_lib::greeting_from_lib());
}
