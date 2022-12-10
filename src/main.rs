mod a;
mod b;
mod c;
mod d;


fn main() {
    
    a::greet();

    a::validation::greet();

    let uuid = b::utils::get_uuid_string();
    println!("Generate an uuid: {uuid}");

    c::greet();

    d::greet();

    let is_uuid_valid = a::validation::validate_uuid(&uuid);

    println!("{uuid} is valid: {is_uuid_valid}");

}
