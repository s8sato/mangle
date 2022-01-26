mod one {
    #[no_mangle]
    pub extern "C" fn run() {
        println!("One");
    }
}

mod two {
    #[no_mangle]
    pub extern "C" fn run() {
        println!("Two");
    }
}

fn main() {
    one::run();
    two::run();
}
