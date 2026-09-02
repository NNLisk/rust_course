use std::io;

fn main() {
    
    let mut current = 0i32;

    println!("By how much do you want to increment the number?");
    loop {
        
        if current >= i16::MAX.into() {
            println!("Enough incrementations.");
            break;
        }
        
        println!("Current: {}. Increment by: ", current);
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("InputError");
        

        
        let intinput: i16 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Parsing failed. Was the number too long for a 16-bit variable?");
                continue;
            }
        };
        // println!("{}", intinput);

        if intinput == 0 {
            println!("The given value is 0. Ending the program.");
            return;
        }

        if intinput < 0 {
            println!("The given value is lower than 0.");
            continue;
        }

        current += intinput as i32;
    }
}
