/* UNSAFE RUST allows:
 *
 * - dereferencing a raw pointer
 * - calling an unsafe function or method
 * - accessing or modifying a mutable static (global) variable
 * - implementing an unsafe trait
 * - accessing fields of unions
 *
*/

// unsafe rust should be declared in an unsafe block
mod safe_abstractions;
mod static_variables;

unsafe trait Foo {}
// implementing unsafe traits requires an unsafe block
unsafe impl Foo for i32 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /* Raw Pointers: *const T and *mut T
     * - are allowed to ignore borrowing rules
     * - aren't guaranteed to point to valid memory
     * - are allowed to be null
     * - don't implement any automatic cleanup
     */
    fn raw_pointers_work() {
        let mut num = 5;
        // raw pointers can be created in safe code, but can only be dereferenced in unsafe code
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;
        unsafe {
            assert_eq!(5, *r1);
            assert_eq!(5, *r2);
        }
    }

    #[test]
    /* Unsafe Funtions/Methods
     */
    fn unsafe_functions_work() {
        unsafe fn dangerous() {}

        // unsafe functions/methods can only be called in an unsafe block
        unsafe { dangerous() }
    }

    #[test]
    /* unsafe code blocks can be enclosed in safe abstractions
     */
    fn safe_abstraction_works() {
        let mut v = vec![1, 2, 3, 4, 5, 6];

        let r = &mut v[..];

        let (a, b) = safe_abstractions::split_at_mut(r, 3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
        safe_abstractions::c_abs();
    }

    #[test]
    fn static_variables_work() {
        static_variables::works();
    }
}
