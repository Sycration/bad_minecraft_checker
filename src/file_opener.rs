pub mod file_opener {
    use std::io::{BufRead, BufReader};
    use std::fs::File;
    use crate::acct_checker::acct_checker::{check_account, AccStatus};
    use crate::auth_structs::auth_structs::Payload;
    use rayon::prelude::*;
    use std::borrow::Borrow;

    ///Wrapper around the acct_checker
    ///Pass a string that is the path
    pub fn check_list(path: String) {

        let mut list_of_payloads: Vec<String> = Vec::new();
        let reader = BufReader::new(File::open(path).expect("Cannot open accts.txt"));
        for line in reader.lines() {
            list_of_payloads.push(line.unwrap());
        }

        list_of_payloads.par_iter().for_each(|string| {
            match string.matches(":").count() {
                1 | 2 => {
                    let tuple: Vec<&str> = string.split(":").collect();
                    let payload = Payload::from(tuple[0].parse().unwrap(), tuple[1].parse().unwrap());
                    match check_account(payload.borrow()) {
                        AccStatus::Works => println!("Account found! {} {}", payload.username, payload.password),
                        _ => println!("User {} with password {} is not a valid account", payload.username, payload.password)
                    };
                }
                _ => println!("There are accounts in the wrong format!"),
            }
        });
    }
}