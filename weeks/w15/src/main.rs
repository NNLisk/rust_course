use rand::Rng;
use std::io;

struct Character {
    hp: f32,
    potions: i32,
}


fn main() {

    let mut player = Character {
        hp: 100.0,
        potions: 3,
    };

    let mut boss = Character {
        hp: 150.0,
        potions: 0,
    };

    let mut p_dmg: f32 = 0.0;
    let mut b_dmg: f32 = 0.0;
    let mut def_mult: f32 = 1.0;
    
    
    loop {
        if player.hp <= 0.0 {
            println!("You have been defeated!");
            break;
        }
        
        println!("| Your HP - {} | Boss HP - {} |", player.hp, boss.hp);
        println!("| 1) Attack | 2) Defend | 3) Heal |");
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("InputError");

        let in_value: i8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Parsing failed. Was the number too long for a 16-bit variable?");
                continue;
            }
        };
        match in_value {
            1 => {
                p_dmg = receive_player_attack_dmg();
                boss.hp -= p_dmg;
                println!("Your attack deals {p_dmg} damage.");
            },
            2 => {
                def_mult = receive_defense_multiplier();
                println!("Defence activated!");
            },
            3 => {
                receive_heal(&mut player);
            },
            _ => {
                println!("Invalid action");
                continue;
            }
        };

        if boss.hp <= 0.0 {
            println!("You have defeated the boss!");
            break;
        }

        b_dmg = receive_boss_attack_dmg();
        player.hp -= b_dmg * def_mult;
        println!("You take {} damage.", b_dmg * def_mult);

        b_dmg = 0.0;
        p_dmg = 0.0;
        def_mult = 1.0;
    }
}

fn receive_player_attack_dmg() -> f32 {
    rand::random_range(12.5..=20.0)
}

fn receive_boss_attack_dmg() -> f32 {
    rand::random_range(5.0..=25.0)
}

fn receive_defense_multiplier() -> f32 {
    1.0 / rand::random_range(2.0..=4.0)
}

fn receive_heal(p: &mut Character) {
    if p.potions > 0 {
        p.hp += 25.0;
        p.potions -= 1;
        println!("You consume a potion.")
    }
}
