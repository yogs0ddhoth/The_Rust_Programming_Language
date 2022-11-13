/* Dynamically Sized or unsized types are values whose size can only be known at runtime */
/* 
dynamically sized types MUST be put behind a pointer, such as:
    * & reference,
    * smart pointers
*/ 

/*
Traits are dynamically sized types
which is why trait objects must be put behind a pointer
*/
// generic functions require size be known at compile time. to get aroung this with an unsized type:
fn _generic< // ?Sized makes the default Sized optional
    T: ?Sized
>(
    _t: &T // the unsized type BEHIND A POINTER
) {}