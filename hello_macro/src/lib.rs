pub trait HelloMacro {
    fn hello_macro();
}

// the procedural macro crate will implement this trait 
// the procedural macro crate will is the one named 
// hello_macro_derive, since the convention is to append 
// _derive to the name of its crate 

