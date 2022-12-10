use uuid::Uuid;

const THE_ANSWER: u64 = 42;

pub fn get_uuid_string() -> String {
    Uuid::new_v4().to_string()
}




#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_lenght_non_zero() {
        
        // when
        let uuid = get_uuid_string();

        // then
        assert_eq!(uuid.len() != 0, true);
    }

}


