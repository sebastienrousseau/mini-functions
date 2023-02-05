extern crate cmn;
pub use cmn::Constants;
pub use cmn::Words;

fn main() {
    // Constants
    let constants = Constants.constants();
    for constant in constants {
        println!(
            "🦀 Constants::constant(): ✅ Name: {} Value: {}",
            constant.name, constant.value
        );
    }

    // Words
    let words = Words::new();
    let words_list = words.words_list();
    println!("🦀 Words::new():          ✅ {:?}", words_list[0]);
}
