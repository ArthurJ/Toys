#![allow(dead_code)]

use std::collections::LinkedList;
use std::collections::BTreeSet as Set;

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

#[derive(Debug)]
pub struct NaivePrimeList {
    pub known_primes:LinkedList<usize>,
    pub last:usize
}

impl Iterator for NaivePrimeList {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let mut last = self.last;
        let limit= root_limit(last)+1; 
        loop{
            last+=2;
            if is_prime(last,
                        limit,
                        &self.known_primes).unwrap(){
                self.known_primes.push_back(last);
                break
            }}
        self.last=last;
        Some(last)
    }}

pub fn sundaram_sieve(n:usize)->LinkedList<usize>{
    let k = (n-2)/2 as usize;
    let mut composites_seeds:Set<usize> = Set::new();
    let mut primes:LinkedList<usize> = LinkedList::new();
    let mut range = Set::new();
    for i in 1..=k{
        range.insert(i);
        for j in i..=k{
            let l = i + j + 2 * i * j;
            if l>k{continue}
            composites_seeds.insert(l);
        }
    }
    for m in range.difference(&composites_seeds){
        primes.push_back(2*m+1)
    }
    primes
}

pub fn primes_until(qtd: usize) -> LinkedList<usize>{
    let mut prime_list = NaivePrimeList{known_primes:LinkedList::from([3]), last:3};
    for i in 1..qtd{
        prime_list.next().unwrap();
        if prime_list.last >= qtd {break;}
    }
    prime_list.known_primes
}

/* tirar o limite do loop economiza processamento, 
    Analizando as diferenças entre as raízes de números primos sequenciais,
    essa diferença parece ser sempre menor que 1.
    E tende a diminuir quanto maior ficam os números primos.

    https://en.wikipedia.org/wiki/Prime_gap
*/

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

