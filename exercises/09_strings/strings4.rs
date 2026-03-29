// Calls of this function should be replaced with calls of `string_slice` or `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
fn main() {
    string_slice("blue");

    string("red".to_string());

    string(String::from("hi"));

    // to_owned() defined in Trait ToOwned
    // fn to_owned(&self) -> Self::Owned
    // Creates owned data from borrowed data, usually by cloning.
    // I don't get it.
    string("rust is fun!".to_owned());

    // Define in Blanket Implementations section of String struct in std
    string("nice weather".into());

    // format! is a concatenation
    string(format!("Interpolation {}", "Station"));

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.
    // lol
    // At the end it is just 'a' and with &, it is &str
    string_slice(&String::from("abc")[0..1]);

    // I don't understand it.
    // I see trim() in both primitive type and String struct.
    string_slice("  hello there ".trim());

    string("Happy Monday!".replace("Mon", "Tues"));

    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
