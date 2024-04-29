pub fn lex_program(src_code: String) {
    let mut words: Vec<&str> = src_code.split(" ").collect();

    words.retain(|&x| x != "");

    let tokens: Vec<String> = lex_line(words, Vec::new(), 1);
    println!("{:?}", tokens)
}

fn lex_line(mut words: Vec<&str>, mut tokens: Vec<String>, line_num: i32) -> Vec<String> {
    if words.len() == 0 {
        return Vec::new();
    }
    
    let first: &str = words.remove(0);

    match first {
        // Unary data values - This could be changed to use lex_int to get direct value
        "mew" => {tokens.push("UNI".to_string()); tokens.append(&mut lex_line(words, Vec::new(), line_num))},
        
        // Variables
        "mrrp" => {tokens.push("VAR1".to_string()); tokens.append(&mut lex_line(words, Vec::new(), line_num))},
        "chrrp" => {tokens.push("VAR2".to_string()); tokens.append(&mut lex_line(words, Vec::new(), line_num))},
        "nyaa" => {tokens.push("VAR3".to_string()); tokens.append(&mut lex_line(words, Vec::new(), line_num))},
        
        // Push to stack operation
        "*push" => {
            let first: &str = words.remove(0);
            match first {
                "off*" => {tokens.push("PUSH".to_string()); tokens.append(&mut lex_line(words, Vec::new(), line_num))},
                "off" => {println!("Syntax Error (Line {}) - Missing *. Think about your actions.", line_num)},
                _ => println!("Syntax Error (Line {}) - What are you pushing?", line_num)
            }
        },
        "push" => println!("Syntax Error (Line {}) - Missing *. Think about your actions.", line_num),

        // Pull from stack operation
        "*swipe*" => {tokens.push("POP".to_string()); tokens.append(&mut lex_line(words, Vec::new(), line_num))},
        "swipe" => println!("Syntax Error (Line {}) - Missing *. Think about your actions.", line_num),
        "swipe*" => println!("Syntax Error (Line {}) - Missing *. Think about your actions.", line_num),

        // Discard top stack value operation
        "*swipe" => {
            let first: &str = words.remove(0);
            match first {
                "into" => {
                    let first: &str = words.remove(0);
                    match first {
                        "littertray*" => {tokens.push("DISC".to_string()); tokens.append(&mut lex_line(words, Vec::new(), line_num))},
                        "littertray" => println!("Syntax Error (Line {}) - Missing *. Think about your actions.", line_num),
                        _ => println!("Syntax Error (Line {}) - Swipe into {}? Is that some sort of euphemism?", line_num, first)
                    }
                },
                _ => println!("Syntax Error (Line {}) - {}? What are you trying to swipe?", line_num, first)
            }
        },

        // Negation operation
        "*chomp*" => {tokens.push("NEG".to_string()); tokens.append(&mut lex_line(words, Vec::new(), line_num))},
        "chomp" => println!("Syntax Error (Line {}) - Missing *. Think about your actions.", line_num),
        "*chomp" => println!("Syntax Error (Line {}) - Missing *. Think about your actions.", line_num),
        "chomp*" => println!("Syntax Error (Line {}) - Missing *. Think about your actions.", line_num),

        // Print operation
        "meow" => {tokens.push("PRINT".to_string()); tokens.append(&mut lex_line(words, Vec::new(), line_num))},

        // Read operation
        "meow?" => {tokens.push("READ".to_string()); tokens.append(&mut lex_line(words, Vec::new(), line_num))},

        // Add operation
        "*pat*" => {tokens.push("ADD".to_string()); tokens.append(&mut lex_line(words, Vec::new(), line_num))},
        "pat" => println!("Syntax Error (Line {}) - Missing *. Think about your actions.", line_num),
        "*pat" => println!("Syntax Error (Line {}) - Missing *. Think about your actions.", line_num),
        "pat*" => println!("Syntax Error (Line {}) - Missing *. Think about your actions.", line_num),

        // Multiplication operation
        "*pounce*" => {tokens.push("MULT".to_string()); tokens.append(&mut lex_line(words, Vec::new(), line_num))},
        "pounce" => println!("Syntax Error (Line {}) - Missing *. Think about your actions.", line_num),
        "*pounce" => println!("Syntax Error (Line {}) - Missing *. Think about your actions.", line_num),
        "pounce*" => println!("Syntax Error (Line {}) - Missing *. Think about your actions.", line_num),

        // Mod operation
        "*scratch*" => {tokens.push("NEG".to_string()); tokens.append(&mut lex_line(words, Vec::new(), line_num))},
        "scratch" => println!("Syntax Error (Line {}) - Missing *. Think about your actions.", line_num),
        "*scratch" => println!("Syntax Error (Line {}) - Missing *. Think about your actions.", line_num),
        "scratch*" => println!("Syntax Error (Line {}) - Missing *. Think about your actions.", line_num),

        // Equal operator
        "mraow" => {tokens.push("EQ".to_string()); tokens.append(&mut lex_line(words, Vec::new(), line_num))},

        // Greater than operator
        "yowl" => {tokens.push("GTT".to_string()); tokens.append(&mut lex_line(words, Vec::new(), line_num))},

        // If
        "*paw*" => {tokens.push("IF".to_string()); tokens.append(&mut lex_line(words, Vec::new(), line_num))},
        "paw" => println!("Syntax Error (Line {}) - Missing *. Think about your actions.", line_num),
        "*paw" => println!("Syntax Error (Line {}) - Missing *. Think about your actions.", line_num),
        "paw*" => println!("Syntax Error (Line {}) - Missing *. Think about your actions.", line_num),

        // Then
        "purr" => {tokens.push("THEN".to_string()); tokens.append(&mut lex_line(words, Vec::new(), line_num))},

        // Else
        "hiss" => {tokens.push("ELSE".to_string()); tokens.append(&mut lex_line(words, Vec::new(), line_num))},

        // End if
        "*stop" => {
            let first: &str = words.remove(0);
            match first {
                "pawing*" => {tokens.push("EIF".to_string()); tokens.append(&mut lex_line(words, Vec::new(), line_num))},
                "pawing" => {println!("Syntax Error (Line {}) - Missing *. Think about your actions.", line_num)},
                _ => println!("Syntax Error (Line {}) - Stop {}? Why don't you stop being an idiot?", line_num, first)
            }
        },
        "stop" => println!("Syntax Error (Line {}) - Missing *. Think about your actions.", line_num),

        // Loop
        "*zoomies*" => {tokens.push("LOOP".to_string()); tokens.append(&mut lex_line(words, Vec::new(), line_num))},
        "zoomies" => println!("Syntax Error (Line {}) - Missing *. Think about your actions.", line_num),
        "*zoomies" => println!("Syntax Error (Line {}) - Missing *. Think about your actions.", line_num),
        "zoomies*" => println!("Syntax Error (Line {}) - Missing *. Think about your actions.", line_num),

        // End loop
        "*eepy*" => {tokens.push("ELOOP".to_string()); tokens.append(&mut lex_line(words, Vec::new(), line_num))},
        "eepy" => println!("Syntax Error (Line {}) - Missing *. Think about your actions.", line_num),
        "*eepy" => println!("Syntax Error (Line {}) - Missing *. Think about your actions.", line_num),
        "eepy*" => println!("Syntax Error (Line {}) - Missing *. Think about your actions.", line_num),

        // Completely unknown word
        _ => println!("Syntax Error (Line {}) - What is {}? Are you some kind of low intelligence quadrupedal mammal?", line_num, first)
    }

    return tokens;
}



// Old but potentially useful function
fn lex_int(mut words: Vec<&str>, mut count: i32) -> i32 {
    count += 1;
    if words.len() == 0 {
        return count
    }
    let first: &str = words.remove(0);
    match first {
        "mew" => count = lex_int(words, count),
        &_ => return count
    }
    return count;
}