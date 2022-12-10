use crate::b::utils;

pub fn greet() {
    println!("My name is A (validation)!")
}

pub fn validate_uuid(uuid: &str) -> bool {
    println!("Sample uuid is {}", utils::get_uuid_string());
    uuid.contains("-")
}




#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_validation() {
        
        // given
        let a_uuid = "abc-def";

        // when
        let is_uuid_valid: bool = validate_uuid(a_uuid);

        // then
        assert_eq!(is_uuid_valid, true);
    }

}


