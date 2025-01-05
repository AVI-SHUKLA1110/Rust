fn main() {
    let mut x = 1; //variables are by default immutable in rust .
    // let mut y = 60 * 5 ;
    // println!("The value of y is: {y}");
    // //x=6;
    // y = 7;
    // println!("The value of y is: {y}");
    // let x  = x + 1 ;
    // {
    //     let x = x + 2 ;
    //     println!("The value of x in the inner scope is : __{x}");
    // }
    // println!("The value of x in the outer scope is : __{x}");
    
    // COMPOUND TYPES 
    // let tup: (&str, i32) = ("Hello abhishek!", 100_000);
    // let (channel, subscribers) = tup;  // destructuring method
    // let sub_count = tup.1;            // index access method

    // println!("Using destructuring: channel = {}, subscribers = {}", channel, subscribers);
    // println!("Using index access: message = {}, sub_count = {}", tup.0, sub_count);

    // let error_codes: [i32; 3] = [200, 404, 500];
    // let not_found: i32 = error_codes[1];
    // // This line would panic at runtime since index 3 is out of bounds
    // // let x: i32 = error_codes[3]; 
    // let bytes: [i32; 8] = [0; 8];
    // let tup: (&str, i32) = ("Abhishek", 123);
    // let (name, fav_number) = tup;
    // let num = tup.0;
    // let num2 = tup.1;

    // println!("Hello my name is {num}");
    // println! ("And my favourite number is {num2}");

    // CONTROL FLOW 
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is :{}", element);
    }
    for number in 1..4 {
        println!("{}!", number);
    }
}
