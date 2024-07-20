//  Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;
use std::thread::JoinHandle;

pub fn sum(v: Vec<i32>) -> i32 {
    let sliced = v.leak();

    let half = sliced.len() / 2;
    let (h1, h2) = sliced.split_at(half);

    let thread_h1:JoinHandle<i32> = thread::spawn(move || {
        h1.iter().sum::<i32>()
    });
    let thread_h2:JoinHandle<i32> = thread::spawn(move || {
        h2.iter().sum::<i32>()
    });

    thread_h1.join().unwrap() + thread_h2.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
