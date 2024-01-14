#![allow(dead_code)]
#![allow(unused_labels, unreachable_code)]

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

use deeply::nested::function as other_function;

fn main() {
    other_function();
    println!("Entering block");
    {
        use deeply::nested::function;
        function();
        println!("Leaving block");
    }

    function();
}
