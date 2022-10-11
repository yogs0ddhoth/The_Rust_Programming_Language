pub mod shoes;
use shoes::Shoe;

fn iterator_ex() -> Vec<i32> {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter(); // create an iterator over v1
                             // .into_iter() moves ownership
    for val in v1_iter {
        // consume the iterator in a for loop
        println!("Got: {}", val);
    }
    /* iterators are implicitly created in for loops
     * this moves ownership
     */
    let v2 = vec![4, 5, 6];

    for v in &v2 {
        println!("Got: {}", v);
    }
    // v2 can no longer be used
    v1
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consumer_adaptors() {
        /* consumer adaptors use up the iterators */
        let v1 = iterator_ex();
        let mut v1_iter = v1.iter();

        // next() will consume the iterator
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);

        // a new iterator will need to be reassigned
        let v1_iter = v1.iter();
        // also consumes the iterator
        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }

    #[test]
    fn iterator_adaptors() {
        let v1 = iterator_ex();

        let v1_iter: Vec<_> = v1
            .iter() // create iterator
            .map(|x| x + 1)
            .collect(); // consumer adaptor
                        // iterators have to be consumed
        assert_eq!(v1_iter, vec![2, 3, 4]);
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
        let in_my_size = shoes_in_size(shoes, 10);

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
                }
            ]
        );
    }
}
