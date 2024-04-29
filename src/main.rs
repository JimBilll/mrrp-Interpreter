mod lexer;

fn main() {
    let test_code = String::from("mew mew mrrp chrrp nyaa *push off* *swipe* *swipe into littertray* *chomp* meow meow? *pat* *pounce* *scratch* mraow yowl *paw* purr hiss *stop pawing* *zoomies* *eepy*");
    lexer::lex_program(test_code);
}
