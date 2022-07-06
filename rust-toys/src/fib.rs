#[allow(unused_variables)]
#[allow(dead_code)]

pub fn fibonacci_until(mut x:usize, mut y:usize, limit:usize) {
    match limit{
        0 => {print!(""); return},
        1 => {print!("{}", x); return},
        2 => {print!("{}, {}", x, y); return},
        _ => {
            print!("{}, {}, ", x, y);
            loop {
                (x, y) = (y, y + x);
                if y > limit { break };
                print!("{}, ", y);
            }
        }
    }
}

pub fn fib_seq(mut x: usize, mut y: usize, qtd: usize) -> Vec<usize> {
    let mut seq = vec![];
    match qtd{
        0 => seq,
        1 => {seq.push(x); seq},
        2 => {seq.push(x); seq.push(y); seq},
        _ => {
            seq.push(x); seq.push(y);
            for _ in 1..=qtd {
                (x, y) = (y, y+x);
                seq.push(y);
            }
            seq
        }
    }
}

pub fn nth_fib(mut x:usize, mut y:usize, mut n:usize) -> Option<usize>{
    match n {
        0 => Option::None,
        1 => Some(x),
        2 => Some(y),
        3 => Some(x+y),
        _ =>{
            {   n-=2;
                while n != 0 {
                    (x, y) = (y, y + x);
                    n -= 1;
                }
                Some(y)
            }}}}