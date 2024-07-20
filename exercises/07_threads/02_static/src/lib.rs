// Given a static slice of integers, split the slice into two halves and
//  sum each half in a separate thread.
//  Do not allocate any additional memory!
use std::thread;
use std::thread::JoinHandle;

pub fn sum(slice: &'static [i32]) -> i32 {
    let half_size = slice.len() / 2;
    let (v1, v2) = slice.split_at(half_size);
    
    let thread_h1:JoinHandle<i32> = thread::spawn(move || {
        v1.iter().sum::<i32>()
    });
    let thread_h2:JoinHandle<i32> = thread::spawn(move || {
        v2.iter().sum::<i32>()
    });

    thread_h1.join().unwrap() + thread_h2.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        static ARRAY: [i32; 0] = [];
        assert_eq!(sum(&ARRAY), 0);
    }

    #[test]
    fn one() {
        static ARRAY: [i32; 1] = [1];
        assert_eq!(sum(&ARRAY), 1);
    }

    #[test]
    fn five() {
        static ARRAY: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(sum(&ARRAY), 15);
    }

    #[test]
    fn nine() {
        static ARRAY: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(sum(&ARRAY), 45);
    }

    #[test]
    fn ten() {
        static ARRAY: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sum(&ARRAY), 55);
    }
}
