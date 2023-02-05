#[derive(Debug)]
struct MyStruct {
    num: i32,
}

fn print_my_struct(my_struct: &MyStruct) {
    println!("{:?}", my_struct)
}

fn change_my_struct(my_struct: &mut MyStruct) {
    my_struct.num = my_struct.num + my_struct.num;
}

fn main() {
    let mut my_struct = MyStruct { num: 10 };
    let my_struct_ref = &my_struct;
    print_my_struct(&my_struct);
    // change_my_struct(&mut my_struct);
    print_my_struct(my_struct_ref);
}

// Write a function that takes in a string of one or more words,
// and returns the same string, but with all five or more letter
// words reversed (Just like the name of this Kata).
// Strings passed in will consist of only letters and spaces.
// Spaces will be included only when more than one word is present.
fn spin_words(words: &str) -> String {
    if words.len() > 5 {
        println!("HELLO");
    } else {
        println!("WORLD");
    }
    words.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(spin_words("Welcome"), "emocleW");
        assert_eq!(spin_words("Hey fellow warriors"), "Hey wollef sroirraw");
        assert_eq!(spin_words("This is a test"), "This is a test");
        assert_eq!(spin_words("This is another test"), "This is rehtona test");
        assert_eq!(
            spin_words("You are almost to the last test"),
            "You are tsomla to the last test"
        );
        assert_eq!(
            spin_words("Just kidding there is still one more"),
            "Just gniddik ereht is llits one more"
        );
        assert_eq!(
            spin_words("Seriously this is the last one"),
            "ylsuoireS this is the last one"
        );
    }
}
