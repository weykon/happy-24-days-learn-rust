// JSON
use json;
pub fn main() {
    println!("{:?}", json::from(&42));
    println!(
        "{:?}",
        json::stringify(&vec!["to", "be", "or", "not", "to", "be"])
    );
}
