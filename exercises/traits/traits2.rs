// traits2.rs
//
// Your task is to implement the trait
// `AppendBar' for a vector of strings.
//
// To implement this trait, consider for
// a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time,
// you can do this!

// I AM NOT DONE

trait AppendBar {
    fn append_bar(self) -> Self;
}

//TODO: Add your code here
impl AppendBar for Vec<String> {
    //Add your code here
    fn append_bar(self) -> Vec<String> {
        let mut buf = self;
        buf.push(String::from("Bar"));
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
