use errors::common::ErrorType;

fn main() {
    let error_type = ErrorType::new("illegal_argument");
    let error_type_new_subtype = error_type.new_subtype("subtype");

    println!(
        "🦀 Error::error_type_new():             ✅ {:?}\n",
        error_type
    );
    println!(
        "🦀 Error::error_type_new_subtype():        ✅ {:?}\n",
        error_type_new_subtype
    );
}
