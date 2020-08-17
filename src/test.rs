#[cfg(test)]
mod test {
    use super::*;
    use crate::auth_structs::auth_structs::{Payload, Agent};


    #[test]
    fn make_payload() {
        let example_payload = Payload::from(String::from("CoolName"), String::from("SecretPassword"));
        assert_eq!(example_payload, Payload {
            agent: Agent { name: String::from("Minecraft"), version: 1 },
            username: String::from("CoolName"),
            password: String::from("SecretPassword"),
            request_user: false,
        })
    }

    #[test]
    fn make_string() {
        let example_payload = Payload::from(String::from("CoolName"), String::from("SecretPassword"));
        assert_eq!((example_payload.to_string()), String::from(r#"{"agent":{"name":"Minecraft","version":1},"username":"CoolName","password":"SecretPassword","requestUser":false}"#))
    }
}