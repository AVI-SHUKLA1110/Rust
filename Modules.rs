
mod house { 
    pub const HOUSE_NUMBER : u32 = 55;

    pub mod bedroom1 {
        pub fn is_light_on() -> bool {
            false
        }
        pub fn is_neighbour_light_on() -> bool {
            super::bedroom2::is_light_on()
        }
    }
    
    pub mod bedroom2 {
        pub fn is_light_on() -> bool {
            true
        }
    }
}

use house::{bedroom1, bedroom2};

fn main() {
    println!("Hello thetechnofeak");
    println!("{}", bedroom1::is_light_on());
    println!("{}", bedroom1::is_neighbour_light_on());
    println!("{}", house::HOUSE_NUMBER);
}
