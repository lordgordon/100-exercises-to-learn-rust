// implement a multi-threaded version of the `sum` function
//  using `spawn` and `join`.
//  Given a vector of integers, split the vector into two halves and
//  sum each half in a separate thread.

// Caveat: We can't test *how* the function is implemented,
// we can only verify that it produces the correct result.
// You _could_ pass this test by just returning `v.iter().sum()`,
// but that would defeat the purpose of the exercise.
//
// Hint: you won't be able to get the spawned threads to _borrow_
// slices of the vector directly. You'll need to allocate new
// vectors for each half of the original vector. We'll see why
// this is necessary in the next exercise.
use std::thread;
use std::thread::JoinHandle;

pub fn sum(v: Vec<i32>) -> i32 {
    let half_size = v.len() / 2;
    let (v1, v2) = v.split_at(half_size);
    let h1 = v1.to_vec();
    let h2 = v2.to_vec();

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
