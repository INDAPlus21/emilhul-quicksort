use std::{io::{self, Read}};

fn qsort(arr: &mut Vec::<isize>, low: usize, high: usize) {
    if low < high {
        match (low..high).len() {
            x if x <= 24 => insertion_sort(arr, low, high), // Looking around it seems like the optimal cut-off is somewhere between 4-16.
            _ => {
                let pivot = partition(arr, low, high);

                qsort(arr, low, pivot); // Smaller
                qsort(arr, pivot+1, high); // Above
            }
        }
    }
}

fn partition(arr: &mut Vec::<isize>, low: usize, high: usize) -> usize {
    let pivot: isize = arr[(low + high) / 2];

    let mut a: isize = low as isize - 1;
    let mut b = high + 1;
    loop {
        // Do while loop implementation
        loop {
            a += 1;
            if !(arr[a as usize] < pivot) {
                break;
            }
        }
        loop {
            b -= 1;
            if !(arr[b] > pivot){
                break;
            }
        }

        if a as usize >= b { return b }

        arr.swap(a as usize, b);
    }
}

fn insertion_sort(arr: &mut Vec<isize>, low: usize, high: usize) {
    for i in low + 1..=high {
        let key = arr[i]; 
        let mut j: isize = i as isize - 1;
        while j >= 0 && key < arr[j as usize] {
            arr[(j + 1) as usize] = arr[j as usize];
            j -= 1;
        }
        arr[(j + 1) as usize] = key;
    }
}

fn main() {
    let mut line = String::with_capacity(500_000);
    io::stdin()
        .lock()
        .read_to_string(&mut line)
        .expect("Failed to read string");

    let mut values: Vec<isize> = line
        .split_whitespace()
        .skip(1)
        .map(|_value| _value.parse::<isize>().unwrap())
        .collect();

    let length = values.len();
    if length != 0 {
        qsort(&mut values, 0, length-1);
    }

    let mut line = String::with_capacity(length);
    for value in values {
        line.push_str(&value.to_string());
        line.push(' '); // A teeny tiny bit faster according to https://github.com/hoodie/concatenation_benchmarks-rs
    }
    print!("{}", line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }

    #[test]
    fn qsort_only_duplicate() {
        let answer = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        let mut arr = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        
        let length = arr.len();
        qsort(&mut arr, 0, length-1);

        assert_eq!(answer, arr);
    }

    #[test]
    fn qsort_duplicate() {
        let answer = vec![1, 2, 2, 4, 5, 6, 7, 7, 9, 10];
        let mut arr =vec![2, 1, 5, 7, 2, 6, 10, 9, 7, 4];

        let length = arr.len();
        qsort(&mut arr, 0, length-1);

        assert_eq!(answer, arr);
    }

    #[test]
    fn qsort_sorted() {
        let answer = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let length = arr.len();
        qsort(&mut arr, 0, length-1);

        assert_eq!(answer, arr);
    }

    #[test]
    fn qsort_reverse() {
        let answer = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut arr = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];

        let length = arr.len();
        qsort(&mut arr, 0, length-1);

        assert_eq!(answer, arr);
    }

    #[test]
    fn qsort_random() {
        let answer = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut arr = vec![2, 7, 5, 4, 1, 3, 6, 8, 10, 9];

        let length = arr.len();
        qsort(&mut arr, 0, length-1);

        assert_eq!(answer, arr);
    }
}