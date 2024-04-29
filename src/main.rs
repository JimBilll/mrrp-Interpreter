mod lexer;

fn main() {
    let test_code = String::from("mew mew mrrp chrrp nyaa *push off* *swipe* 
            *swipe into littertray* *chomp* meow meow? *pat* *pounce* *scratch* mraow yowl *paw* purr hiss *stop pawing* *zoomies* *eepy*");
    let lexed_code: Vec<Vec<String>> = lexer::lex_program(test_code);

    println!("Lexed Program");
    let mut line_num: i32 = 0;
    for line in lexed_code {
        line_num += 1;
        println!("Line {}: {:?}", line_num, line);
    }
}
