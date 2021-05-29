use std::io;

pub fn some() {
    println!("Digite o primeiro número: ");

    let mut first_input = String::new();
    io::stdin()
        .read_line(&mut first_input)
        .expect("falha ao ler o número digitado");

    let first_number = first_input.trim().parse::<f64>();
    match first_number {
        Ok(x) => {
            println!("Digite o segundo número: ");

            let mut second_input = String::new();
            io::stdin()
                .read_line(&mut second_input)
                .expect("falha ao ler o número digitado");

            let second_number = second_input.trim().parse::<f64>();
            match second_number {
                Ok(y) => {
                    println!("Resultado: {}", x + y);
                },
                Err(..) => println!("Este não é um número valido"),
            }
        },
        Err(..) => println!("Este não é um número valido"),
    };
}

pub fn sub() {
    println!("Digite o primeiro número: ");

    let mut first_input = String::new();
    io::stdin().read_line(&mut first_input).expect("Erro ao ler o número digitado");

    let first_number = first_input.trim().parse::<f64>();
    match first_number {
        Ok(x) => {
            println!("Digite o segundo número: ");

            let mut second_input = String::new();
            io::stdin().read_line(&mut second_input).expect("Erro ao ler o número");

            let second_number = second_input.trim().parse::<f64>();
            match second_number {
                Ok(y) => {
                    println!("Resultado: {}", x - y);
                },
                Err (..) => println!("Este não é um número valido"),
            }
        },
        Err (..) => println!("Este não é um número valido"),
    }
}

pub fn mult() {
    println!("Digite o primeiro número: ");

    let mut first_input = String::new();
    io::stdin()
        .read_line(&mut first_input)
        .expect("falha ao ler o número digitado");

    let first_number = first_input.trim().parse::<f64>();
    match first_number {
        Ok(x) => {
            println!("Digite o segundo número: ");

            let mut second_input = String::new();
            io::stdin()
                .read_line(&mut second_input)
                .expect("falha ao ler o número digitado");

            let second_number = second_input.trim().parse::<f64>();
            match second_number {
                Ok(y) => {
                    println!("Resultado: {}", x * y);
                },
                Err(..) => println!("Este não é um número valido"),
            }
        },
        Err(..) => println!("Este não é um número valido"),
    };
}

pub fn div() {
    println!("Digite o primeiro número: ");

    let mut first_input = String::new();
    io::stdin()
        .read_line(&mut first_input)
        .expect("falha ao ler o número digitado");

    let first_number = first_input.trim().parse::<f64>();
    match first_number {
        Ok(x) => {
            println!("Digite o segundo número: ");

            let mut second_input = String::new();
            io::stdin()
                .read_line(&mut second_input)
                .expect("falha ao ler o número digitado");

            let second_number = second_input.trim().parse::<f64>();
            match second_number {
                Ok(y) => {
                    println!("Resultado: {}", x / y);
                },
                Err(..) => println!("Este não é um número valido"),
            }
        },
        Err(..) => println!("Este não é um número valido"),
    };
}