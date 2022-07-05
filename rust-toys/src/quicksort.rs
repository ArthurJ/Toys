use core::cmp::max;
use core::cmp::min;

pub fn quicksort<T>(list:Vec<T>) -> Vec<T>
    where T:Ord + Clone {
    let len = list.len();
    match len{
        0|1 => list,
        /*2 {
            let head = list.get(0).unwrap();
            let tail = list.get(1).unwrap();
            let mut output:Vec<T> = Vec::new();

            output.push(min(head.clone(), tail.clone()));
            output.push(max(head.clone(), tail.clone()));
            output
        },*/
        _ => {
            let mut list_without_pivot:Vec<T> = list.clone();
            let pivot = list_without_pivot.pop().unwrap();

            let mut output:Vec<T> = Vec::new();

            let lesser:Vec<T>;
            let greater:Vec<T>;
            (lesser, greater) = list_without_pivot.into_iter().partition(|x| *x < pivot);

            output.extend(quicksort(Vec::from(lesser)));
            output.push(pivot);
            output.extend(quicksort(Vec::from(greater)));
            output
        }
    }
}