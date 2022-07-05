#[allow(unused_variables)]
#[allow(dead_code)]

mod fib;
mod fbzz;
mod imc;
mod quicksort;

fn main() {
    //fibonacci();
    //fbzz::fizzbuzz(3, 5, 100);
    //println!("\n{:?}", imc::calc_imc(1.83, 115.0));
    print!("\n{:?}", quicksort::quicksort(vec![88,1,2,3,4,5,6]));

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
