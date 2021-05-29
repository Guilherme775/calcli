use std::env::args;
pub mod calcs;

fn main() {
    let args = args().collect::<Vec<String>>();
    let arg = &args[1];

    let commands = 
        "some: somar dois números \n\
        sub: subtrair dois números \n\
        mult: multiplicar dois números \n\
        div: dividir dois números";

    match arg.as_ref() {
        "commands" => println!("{}", commands),
        "some" => calcs::some(),
        "sub" => calcs::sub(),
        "mult" => calcs::mult(),
        "div" => calcs::div(),
        _ => println!("por favor digite uma opção valida :)"),
    };
}
