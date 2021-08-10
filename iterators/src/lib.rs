#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // create an iterator that takes ownership of the vector,
    // then call filter to adapt that iterator into a new iterator
    // that only contains elements for which the closure returns true.
    // Return the result from collect(), a vector containing the matching shoes
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter { // Iterator trait needs a type for Item and a next() implementation
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> { // returns Some(1), Some(2), Some(3), Some(4), Some(5), None, None, ...
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        // calling the next method on an iterator changes internal state that the iterator uses
        // to keep track of where it is in the sequence. In other words, this code consumes, or uses up, the iterator.
        let mut v1_iter = v1.iter(); // therefore it must be mutable

        assert_eq!(v1_iter.next(), Some(&1)); // these are immutable references to items in the vector
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
        // v1.into_iter(): returns owned values
        // v1.iter_mut(): returns mutable references

    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum(); // Methods that call next, like .sum() are called consuming adaptors

        assert_eq!(total, 6);
    }

    #[test]
    fn iterator_map(){
        let v1: Vec<i32> = vec![1, 2, 3];
        // create new iterator that uses a closure to map items and then consume it by using .collect()
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

        assert_eq!(v2, vec![2, 3, 4]);
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10); // shoes_in_size takes ownership

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1)) // pair the two iterators
            .map(|(a, b)| a * b) // multiply items in tuple
            .filter(|x| x % 3 == 0) // take items that are divisible by three
            .sum(); // add them up
        assert_eq!(18, sum); // (2*3) + (3*4), because (1*2) and (4*5) don't match and the theoretical fifth pair (5, None) is never produced because zip returns None when either of its input iterators return None
    }
}