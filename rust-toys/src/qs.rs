pub fn quicksort<T>(list:&[T]) -> Vec<T>
    where T:Ord + Clone + Copy {
    let len:usize = list.len();
    match len{
        0|1 => list.to_vec(),
        _ => {
            let (head, tail) = list.split_first().unwrap();

            let mut output:Vec<T> = Vec::new();
            let lesser:Vec<T>;
            let greater:Vec<T>;

            (lesser, greater) = tail.iter().partition(|x| *x < head);

            output.extend(quicksort(&lesser));
            output.push(*head);
            output.extend(quicksort(&greater));
            output
        }}}