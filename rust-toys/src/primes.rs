#![allow(dead_code)]

use std::collections::LinkedList;

#[derive(Debug)]
pub enum PrimeError{
    PrimeListTooShortErr,
}

pub fn is_prime(val: usize, test_limit:usize, prime_list:&LinkedList<usize>)
    -> Result<bool, PrimeError>{
    if val<2 {return Ok(false)}
    if val<*prime_list.back().unwrap() {
        if prime_list.contains(&val) { return Ok(true) }
    }
    for i in prime_list{
        if val % i == 0 {return Ok(false)}
        if i > &test_limit {return Ok(true)}
    }
    return Err(PrimeError::PrimeListTooShortErr)
}

pub fn root_limit(maximum:usize) -> usize{
    ((maximum as f64).sqrt()) as usize
}

pub fn append_next_prime(mut prime_list:LinkedList<usize>) -> LinkedList<usize>{
    if prime_list.is_empty(){
        prime_list = LinkedList::from([2,3])
    }
    let last:usize = *prime_list.back().unwrap();
    let mut candidate = last+2;
    loop{
        if is_prime(candidate,
                    root_limit(last),
                    &prime_list).unwrap(){
            prime_list.push_back(candidate);
            break
        }
        candidate+=2;
    }
    prime_list
}

pub fn gen_primes(qtd:usize) -> LinkedList<usize>{
    let mut known_primes = LinkedList::from([2, 3, 5]);
    for _ in 1..=qtd{
        known_primes = append_next_prime(known_primes);
    }
    known_primes
}

#[derive(Debug)]
pub struct PrimeIterator {
    pub known_primes:LinkedList<usize>,
}

impl Iterator for PrimeIterator {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let last = *self.known_primes.back().unwrap();
        let mut candidate:usize = last+2;
        loop{
            if is_prime(candidate,
                        root_limit(last),
                        &self.known_primes).unwrap(){
                self.known_primes.push_back(candidate);
                break
            } else {
                candidate+=2;
            }}
        Some(candidate)
    }}

/*
    Calculo de 10 mil primos em menos de 0.4s
Executed in    1.11 secs      fish           external
   usr time  399.01 millis    0.16 millis  398.84 millis
   sys time  204.57 millis    1.13 millis  203.44 millis
_____________________________________________________
Calculo de 50 mil primos em +/- 0.6s
Executed in    1.73 secs      fish           external
   usr time  538.92 millis  121.00 micros  538.80 millis
   sys time  183.72 millis  734.00 micros  182.98 millis
_____________________________________________________
   Calculo 10 milhões de primos em menos de 7 minutos!
Executed in  397.03 secs    fish           external
   usr time  366.02 secs    0.14 millis  366.02 secs
   sys time    3.33 secs    1.64 millis    3.32 secs
_____________________________________________________
    Calculo 100 milhões de primos em +/- 3 horas e 15min
Executed in  195.45 mins    fish           external
   usr time  192.58 mins    0.13 millis  192.58 mins
   sys time    2.25 mins    1.12 millis    2.25 mins
*/

