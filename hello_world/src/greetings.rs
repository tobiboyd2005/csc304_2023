///greeting function simply returns a String
///
pub mod spanish;
pub mod french;
pub mod japanese;
pub fn default_greeting() -> String {
    let message = String::from("Hi!");
    message
    }