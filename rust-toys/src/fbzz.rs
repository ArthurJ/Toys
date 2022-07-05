pub fn fizzbuzz(fizz_num: usize, buzz_num: usize, limit: usize){
    for i in 1..=limit{
        let fizz = i%fizz_num==0;
        let buzz = i%buzz_num==0;

        match true{
            _ if fizz && buzz => print!("Fizzbuzz!"),
            _ if fizz => print!("Fizz"),
            _ if buzz => print!("Buzz"),
            _ => print!("{}", i)
        }

        print!("\n");
    }
}