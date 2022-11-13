/* 
The Newtype Pattern can achieve encapsulation to hide implementation details**
    ** see advanced_traits for more information on the Newtype pattern 
*/
mod aliases;
mod never;
mod dynamically_sized;
mod function_pointers;
use function_pointers::{do_twice, add_one, Status};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_aliases_work() {
        let y: aliases::Kilometers = 5;
        assert_eq!(5, y);

        let f: aliases::Thunk = Box::new(|| println!("Hiya Chuck"));
        f();
    }

    #[test]
    fn function_pointers_work() {
        let answer = do_twice(add_one, 5);
        
        assert_eq!(12, answer);
    
        let list_of_statuses: Vec<Status> = (0u32..20)
            .map(Status::Value) // map takes a closure/function pointer as an arg, like the initializer functions of an enum
            .collect();
        fn to_string(i:&Status) -> String {
            match i {
                Status::Value(x) => x.to_string(),
                Status::Stop => panic!("can't coerce to string!")
            }
        }
        let list_of_strings: Vec<String> = list_of_statuses
            .iter()
            .map(to_string)
            .collect();
    }
}
