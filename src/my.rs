mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called `my::function()`");
}
fn private_function() {
    println!("called `my::private_function()`");
}

pub fn indirect_call() {
    println!("called `my::indirect_call()`, that\n> ");
    private_function();
}
