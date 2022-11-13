/* Declarative Macros are the most widely used form of macro

*/

/* the vec![] macro can make a vector of any number of values */
#[macro_export] // indicate that this macro should be made available whenever the crate in which the macro is defined is brought into scope
macro_rules! vecc { // define the macro with the name
    /* the structure is similar to a match expression, with patterns, arms, and code blocks */
    (   // $ declares a variable in the macro system that will contain the code matching the pattern
        $( // capture the internal pattern
            $x:expr // match any expression (expr) and give it the variable name $x
        )
        , // indicate that a literal comma seperator charactor could optionally appear in the code
        * // specify that the pattern matches zero or more of whatever precedes the *
    ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push( // code generated for:
                    $x // each expr matched
                );
            )*
            temp_vec
        }
        /* the generated code wil be:
            {
                let mut temp_vec = Vec::new();
                temp_vec.push(1);
                temp_vec.push(2);
                temp_vec.push(3);
                temp_vec
            }
        */
    }
}
