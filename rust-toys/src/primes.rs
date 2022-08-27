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

impl NaivePrimeList{
    pub fn new(known_primes:LinkedList<usize>, last:usize) -> NaivePrimeList{
        NaivePrimeList{known_primes, last}
    }
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

/* tirar o limite do loop economiza processamento,
    Analizando as diferenças entre as raízes de números primos sequenciais,
    essa diferença parece ser sempre menor que 1.
    E tende a diminuir quanto maior ficam os números primos.

    https://en.wikipedia.org/wiki/Prime_gap
*/

pub fn primes_until(qtd: usize) -> LinkedList<usize>{
    let mut prime_list = NaivePrimeList::new(LinkedList::from([3]),3);
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


pub fn sundaram_sieve(n:usize) -> Vec<usize>{
    let k = (n-2)/2 as usize;
    let mut primes = Vec::with_capacity(n);
    primes.push(2);
    let mut composites_seeds:Set<usize> = Set::new();
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
        primes.push(2*m+1)
    }
    primes
} /* Essa implementação está com desempenho ruim, provavelmente devido a operação `difference`*/

pub fn erathostenes_sieve(n:usize) -> Vec<usize> {
    let number_list = 2..n;
    let mut prime_list:Vec<usize>= Vec::with_capacity(n);
    for j in number_list.clone(){
        prime_list.push(j);
        for i in number_list.clone() {
            if j<=i{continue};
            if j%i==0{
                prime_list.pop();
                break
            }
        }
    }
    prime_list
}