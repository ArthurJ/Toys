#![allow(dead_code)]

use std::collections::LinkedList;
use bit_vec::BitVec;

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

impl NaivePrimeList{
    pub fn new() -> NaivePrimeList{
        let known_primes = LinkedList::from([2, 3, 5, 7, 11]);
        NaivePrimeList{known_primes, last: 5 }
    }
}

impl Iterator for NaivePrimeList {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let mut last = self.last;
        let limit= root_limit(last)+1;
        let mut cycle = [2,4,2,2].iter().cycle();
        match last%10 { /* Evita multiplos de 2 e de 5 */
            3 => {cycle.next();},
            7 => {cycle.next(); cycle.next();},
            9 => {cycle.next(); cycle.next();cycle.next();},
            _ => ()
        }
        loop{
            last += cycle.next().unwrap();
            if is_prime(last,
                    limit,
                    &self.known_primes).unwrap(){
                self.known_primes.push_back(last);
                break
            }}
        self.last=last;
        Some(last)
    }}

/* tirar o limite de dentro do loop economiza processamento,
    Analizando as diferenças entre as raízes de números primos sequenciais,
    essa diferença parece ser sempre menor que 1.
    E tende a diminuir quanto maior ficam os números primos.

    https://en.wikipedia.org/wiki/Prime_gap
*/

pub fn primes_until(qtd: usize) -> LinkedList<usize>{
    let mut prime_list = NaivePrimeList::new();
    for i in 1..qtd{
        prime_list.next().unwrap();
        if prime_list.last >= qtd {break;}
    }
    prime_list.known_primes
}

/* Testando o desempenho desse Iterador, comparando Vectors e LinkedLists,
vetores consistentemente foram piores que listas ligadas,
apesar dos resultados terem sempre a mesma ordem de grandeza.
Assim, se a lógica a ser aplicada se manter simples,
listas ligadas tem desempenho melhor.

Se, por outro lado, for necessário acesso a um valor intermediário
(que não seja a cabeça ou a cauda da lista),
vetores serão mais fáceis de usar sem afetarem o desempenho significativamente.

Obs.: SOB A CONDIÇÃO DE O VETOR JÁ SER CRIADO COM SUA CAPACIDADE FINAL DEFINIDA:
Existe um custo de crescimento relacionado aos vetores,
a cada exento de crescimento todos os itens precisam ser copiados para o novo espaço na memória.
Esses eventos são raros (cada vez que o vetor enche, sua capacidade deve crescer para o dobro da atual)

Na maioria dos casos, use vetores; se precisar do máximo de desempenho, considere usar listas ligadas.
*/

/* https://primes.utm.edu/howmany.html
Estimando a quantidade de números primos menores do que x
*/

fn pi(x:usize) -> usize{
    let n = x as f64;
    (n /(n-1.0).ln()) as usize
}

pub fn sundaram_sieve(n:usize) -> Vec<usize>{
    let k = (n-2)/2;
    //println!("{} estimated prime numbers.", pi(n));
    let mut seeds = BitVec::from_elem(k, true);
    for i in 1..k{
        for j in i..k{
            let l = i + j + 2 * i * j;
            if l>=k{continue}
            seeds.set(l, false);
        }
    }
    let mut primes = vec![2usize];
    primes.extend(
    seeds.iter().enumerate().filter(|(i,v)| *v==true).map(|(i,v)| 2*i+1).skip(1)
    );
    primes
}

pub fn erathostenes_sieve(n:usize) -> Vec<usize>{
    let limit = root_limit(n)+1;
    let mut number_list: BitVec = BitVec::from_elem(n, true);

    number_list.set(0, false);
    number_list.set(1, false);
    for cursor in 2..limit {
        if number_list[cursor] {
            for i in ((cursor*cursor)..n).step_by(cursor) {
                number_list.set(i, false);
            }
        }
    }
    number_list.iter().enumerate().filter(|(i,v)| *v==true).map(|(i,v)| i).collect()
}