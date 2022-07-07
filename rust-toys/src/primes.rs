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
        //else{return Ok(false);}
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
    let mut known_primes = LinkedList::from([2, 3, 5, 7, 11, 13, 17, 19, 23]);
    for _ in 1..=qtd{
        known_primes = append_next_prime(known_primes);
    }
    known_primes
}

#[derive(Debug)]
pub struct Primes{
    pub known_primes:LinkedList<usize>,
}

impl Iterator for Primes {
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

