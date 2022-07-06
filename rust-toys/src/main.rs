#[allow(unused_variables)]
#[allow(dead_code)]

extern crate core;

mod fib;
mod fbzz;
mod imc;
mod qs;
mod primes;

fn main() {
    //fibonacci();
    //fbzz::fizzbuzz(3, 5, 100);
    //println!("\n{:?}", imc::calc_imc(1.83, 115.0));
    //print!("\n{:?}", qs::quicksort(&[88,1,2,3,99,4,5,0,6,1101]));
    //prime1();
    prime2(100000);
}

fn prime1(){
    let known_primes:Vec<usize> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23];
    let last = 400;
    let limit=primes::root_limit(last);
    print!("\nRoot limit: {limit}");
    for i in 0..last{
        if primes::is_prime(i, limit,&known_primes).unwrap(){
            print!("{i:3.0}, ");
        }
        if i%100==0{
            println!("");
        }
    }
    println!()
}

fn prime2(qtd:usize){
    primes::gen_primes(qtd)
            .iter().enumerate()
            .map(|(i,p)| ((i%20==0), p))
            .for_each(|(i,p)| print!("{p:7.0}, {}",if i{"\n"} else{""}));
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

/* TODO
Torres de Hanoi
Quicksort novamente
Alocação Genética
 */