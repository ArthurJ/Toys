#![feature(int_log)]
#![allow(unused_variables)]
#![allow(dead_code)]

extern crate core;
extern crate bit_vec;

use std::collections::LinkedList;

mod fib;
mod fbzz;
mod imc;
mod qs;
mod primes;

/* TODO
    Torres de Hanoi
    Quicksort novamente
    Alocação Genética
 */

fn main() {
    //fibonacci();
    //fbzz::fizzbuzz(3, 5, 100);
    //println!("\n{:?}", imc::calc_imc(1.83, 115.0));
    //print!("\n{:?}", qs::quicksort(&[88,1,2,3,99,4,5,0,6,1101]));
    print_number_list( primes::erathostenes_sieve(1_0_000_000));
    // print_number_list( primes::sundaram_sieve(500_000));
    // for _ in 0..100{
    //     primes::erathostenes_sieve(1_0_000_000);
    // }
    //println!();
}

fn print_number_list(prime_list: Vec<usize>){
    println!();
    prime_list.iter().enumerate()
            .map(|(i,p)| (((i+1)%20==0), p))
            .for_each(|(i,p)| print!("{p:8.0}, {}", if i {"\n"} else {""}));
    println!("\n\n\t {} primes found.", prime_list.len())
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