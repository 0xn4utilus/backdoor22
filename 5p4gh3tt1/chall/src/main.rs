use once_cell::sync::Lazy;
use std::io;
use std::process::exit;
use std::sync::Mutex;


static RECIPE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

fn update_recipe(value: String) {
    *RECIPE.lock().unwrap() = value;
}

fn get_recipe() -> String {
    RECIPE.lock().unwrap().to_string()
}

fn main() {
    println!("Enter your great spaghetti recipe: ");
    new_recipe();
    
    if boiling_spaghetti() && chopping_veggies() && garnishing(){
        println!("It is indeed a great recipe! ");
    }else{
        println!("You call this your great recipe? Disappointing.");
        
    }
}

fn new_recipe() {
    let mut input_recipe = String::new();
    io::stdin()
        .read_line(&mut input_recipe)
        .expect("Are you sure this is a spaghetti recipe?");
    input_recipe.pop();
    update_recipe(input_recipe);

    if get_recipe().len() != 40 {
        println!("Are you sure this is a spaghetti recipe?");
        exit(0);
    }
    
}

fn boiling_spaghetti() -> bool {
    let spaghetti = String::from(&get_recipe()[0..10]);
    let mut processed = vec![0; 0];

    for i in 0..10 {
        let ch = spaghetti.chars().nth(i).unwrap();
        if i % 2 == 0 {
            if (ch as u32) % 2 == 0 {
                processed.push(add_oil(add_water(ch as u32)));
            } else {
                processed.push(add_water(add_salt(ch as u32)));
            }
        } else {
            if (ch as u32) % 2 == 0 {
                processed.push(add_pepper(add_oil(ch as u32)));
            } else {
                processed.push(add_salt(add_oil(ch as u32)));
            }
        }
    }

    processed == vec![183, 41, 431, 114, 519, 250, 359, 209, 193, 19]
}

fn chopping_veggies() ->bool {
    let spaghetti = String::from(&get_recipe()[10..30]);
    let mut processed = vec![0; 0];
    let mut quantity1: u32 = 0; 
    let mut quantity2: u32 = 0; 
    let mut quantity3: u32 = 0; 
    for i in String::from(&get_recipe()[0..10]).chars() {
        if (i as u32) % 3 == 0 {
            quantity1 += i as u32;
        } else if (i as u32) % 3 == 1 {
            quantity2 += i as u32;
        } else {
            quantity3 += i as u32;
        }
    }
    
    
    for i in 0..20 {
        if i % 3 == 0 {
            processed.push(chop_veggies(
                spaghetti.chars().nth(i).unwrap() as u32,
                quantity1,
            ));
        } else if i % 3 == 1 {
            processed.push(chop_veggies(
                spaghetti.chars().nth(i).unwrap() as u32,
                quantity2,
            ));
        } else {
            processed.push(chop_veggies(
                spaghetti.chars().nth(i).unwrap() as u32,
                quantity3,
            ));
        }
    }
    

    processed== vec![167424, 39205, 23198, 20405, 34216, 48364, 42350, 97109, 20277, 20405, 101441, 48364, 187008, 37568, 10868, 44660, 41952, 49434, 36575, 125989]

}


fn garnishing()->bool{
    let spaghetti = String::from(&get_recipe()[30..40]);
    let mut garnished_spaghetti = String::from("");
    for i in spaghetti.chars(){
        if i as u32 == 48 {
            garnished_spaghetti.push('/');
        }
        if i as u32 == 50 {
            garnished_spaghetti.push('*');
        }
        if i as u32 == 51 {
            garnished_spaghetti.push('&');
        }
        if i as u32 == 53 {
            garnished_spaghetti.push(')');
        }
        if i as u32 == 65 {
            garnished_spaghetti.push('.');
        }
        if i as u32 == 76 {
            garnished_spaghetti.push('+');
        }
        if i as u32 == 72 {
            garnished_spaghetti.push('@');
        }
        if i as u32 == 66 {
            garnished_spaghetti.push('#');
        }
        if i as u32 == 125 {
            garnished_spaghetti.push('%');
        }
        if i as u32 == 52 {
            garnished_spaghetti.push('!');
        }
    }
    
    garnished_spaghetti = garnished_spaghetti.chars().rev().collect::<String>();
    garnished_spaghetti== "%@!&*/+#.)"
    
}
    

fn add_salt(ingredient: u32) -> u32 {
    ingredient * 4 + 6
}

fn add_pepper(ingredient: u32) -> u32 {
    ((ingredient ^ 56) + 6) ^ 5
}

fn add_water(ingredient: u32) -> u32 {
    (ingredient + 88) ^ 77
}

fn add_oil(ingredient: u32) -> u32 {
    ((((ingredient + 3) ^ 79) + 49) ^ 68) + 9
}

fn chop_veggies(veggie: u32, quant: u32) -> u32 {
    if veggie % 3 == 0 {
        (veggie * quant) ^ quant
    } else if veggie % 3 == 1 {
        (veggie ^ quant) * quant
    } else {
        (veggie * quant) ^ veggie
    }
}

