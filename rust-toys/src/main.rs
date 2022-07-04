#[allow(unused_variables)]
#[allow(dead_code)]

mod fib;

fn main() {
    //fibonacci();
    //fizzbuzz(3,5,100)
    println!("\n{:?}", calc_imc(1.83, 115.0));
}

#[derive(Debug)]
enum IMC{
    MagrezaGrave,
    MagrezaModerada,
    MagrezaLeve,
    Saudavel,
    Sobrepeso,
    Obesidade,
    ObesidadeSevera,
    ObesidadeMorbida
}

fn calc_imc(altura: f64, peso: f64)-> IMC {
    let imc = peso / (altura*altura);
    match imc {
        x if (x<16.0) => IMC::MagrezaGrave,
        x if (x<17.0) => IMC::MagrezaModerada,
        x if (x<18.5) => IMC::MagrezaLeve,
        x if (x<25.0) => IMC::Saudavel,
        x if (x<30.0) => IMC::Sobrepeso,
        x if (x<35.0) => IMC::Obesidade,
        x if (x<40.0) => IMC::ObesidadeSevera,
        _ => IMC::ObesidadeMorbida
    }
}


fn fizzbuzz(fizz_num: usize, buzz_num: usize, limit: usize){
    for i in 1..=limit{
        let fizz = i%fizz_num==0;
        let buzz = i%buzz_num==0;

        if fizz && buzz {print!("Fizzbuzz!");}
        else if fizz {print!("Fizz"); }
        else if buzz {print!("Buzz"); }
        else { print!("{}", i);};
        print!("\n");
    }
}


fn fibonacci(){
    let fib_seed_1 = 0;
    let fib_seed_2 = 1;
    let val:usize=25;

    print!("\nFibonacci until {}: ", val);
    fib::fibonacci_until(fib_seed_1, fib_seed_2, val);

    print!("\n{} Fibonacci numbers: ", val);
    println!("{:?}", fib::fib_seq(fib_seed_1, fib_seed_2, val));

    match fib::nth_fib(fib_seed_1, fib_seed_2, val){
        None => println!("There are no 0th fibonacci number. Try '1' instead."),
        Some(number) => println!("{}-th Fibonacci number: {}", val, number)
    }
}

//TODO quick sort