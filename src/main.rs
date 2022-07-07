fn read_move(state: &mut [i32; 9], player1: &mut bool) {
    let char = if *player1 { 'x' } else { 'o' };

    loop {
        println!();
        println!("place {char} on [0..9]: ");
        
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Reading expected");

        match input.trim_end().parse::<usize>() {
            Ok(idx) => {
                if idx > 9 {
                    println!("Invalid field!1");
                    continue;
                }

                if state[idx] != 0 {
                    println!("Invalid field!2");
                    continue;
                }
                state[idx] = if *player1 { 1 } else { 2 };
                break;
            }
            Err(_) => {
                println!("Invalid field!3");
                continue;
            },
        }
    }
    *player1 = !*player1;
}

fn check_win(states: &[i32; 9], player1: bool) -> bool {
    let search = if player1 { 1 } else { 2 };

    for row in 0..3 {
        let mut occurence = 0;
        for col in 0..3 {
            if states[col * 3 + row] == search {
                occurence+=1
            }
        }

        if occurence == 3 {
            return true;
        }

        let mut occurence = 0;
        for col in 0..3 {
            if states[row * 3 + col] == search {
                occurence += 1;
            }
        }
        if occurence == 3 {
            return true;
        }
    }
    if states[0] == search && states[4] == search && states[8] == search {
        return true;
    }
    if states[2] == search && states[4] == search && states[6] == search {
        return true;
    }
    false
}

fn main() {
    let mut states = [0, 0, 0, 
                            0, 0, 0, 
                            0, 0, 0];

    let mut player1 = true;
    loop {

        // clear loop
        for _ in 0..100 {
            println!();
        }

        for (idx, state) in states.into_iter().enumerate() {
            if idx % 3 == 0 {
                println!();
            } 
            
            if state == 1 {
                print!("x")
            } else if state == 2 {
                print!("o")
            } else {
                print!("-")
            }     
        }
        if check_win(&states, !player1) {
            break;
        }
        read_move(&mut states, &mut player1);
    }
}
