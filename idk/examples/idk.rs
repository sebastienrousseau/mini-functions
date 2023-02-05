extern crate idk;
use idk::common::ErrorType;

fn main() {
    let error_type = ErrorType::new("illegal_argument");
    let error_type_new_subtype = error_type.new_subtype("subtype");

    println!("🦀 Error::error_type_new():             ✅ {error_type:?}\n",);
    println!("🦀 Error::error_type_new_subtype():        ✅ {error_type_new_subtype:?}\n",);
}
