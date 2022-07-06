#[derive(Debug)]
pub enum PrimeError{
    PrimeListTooShortErr,
    EmptyPrimeListErr
}

pub fn is_prime(val: usize, test_limit:usize, prime_list:&Vec<usize>)
    -> Result<bool, PrimeError>{
    if val<2 {return Ok(false)}
    if prime_list.contains(&val){return Ok(true)}

    for i in prime_list{
        if val % i == 0 {return Ok(false)}
        if i > &test_limit {return Ok(true)}
    }
    return Err(PrimeError::PrimeListTooShortErr)
}

pub fn root_limit(maximum:usize) -> usize{
    ((maximum as f64).sqrt()) as usize
}

pub fn append_next_prime(mut prime_list:Vec<usize>) -> Vec<usize>{
    if prime_list.is_empty(){
        prime_list = vec![2,3]
    }
    let last:usize = *prime_list.last().unwrap();
    let mut candidate = last+2;
    loop{
        if is_prime(candidate,
                    root_limit(last),
                    &prime_list).unwrap(){
            prime_list.push(candidate);
            break
        }
        candidate+=2;
    }
    prime_list
}

pub fn gen_primes(qtd:usize) -> Vec<usize>{
    let mut known_primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23];
    for _ in 1..=qtd{
        known_primes = append_next_prime(known_primes);
    }
    known_primes
}