use rand::Rng;

fn main() {


    let mut count = 0;

    loop {
        let x: i32 = receive_random();

        match x {
            1..=3 => println!("Low..."),
            4..=6 => println!("Middle!"),
            7..=9 => println!("High!"),
            10 => {
                println!("JACKPOT!!!");
                break;
            },
            _ => unreachable!(),
        };

        count += 1;
    }

    let luck = measure_luck(count);

    println!("You were {}", luck);
}


fn receive_random() -> i32 {
    rand::random_range(1..=10)
    
}

fn measure_luck(x: i32) -> &'static str {
    
    if x > 3 {
        "UNLUCKY"
    } else {
        "LUCKY"
    }
}