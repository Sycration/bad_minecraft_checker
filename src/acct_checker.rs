pub mod acct_checker {
    use crate::auth_structs::auth_structs::Payload;

    pub enum AccStatus {
        Works,
        Broke,
    }


    pub fn check_account(payload: &Payload) -> AccStatus {
        let json_string = payload.to_string();
        let response = minreq::post("https://authserver.mojang.com/authenticate")
            .with_header("content-type", "application/json")
            .with_body(json_string)
            .send();
        if response.unwrap().as_str().unwrap().contains("error") {
            return AccStatus::Broke;
        }
        return AccStatus::Works;
    }
}
