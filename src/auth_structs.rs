pub mod auth_structs {
    use serde::{Serialize, Deserialize};

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Payload {
        pub(crate) agent: Agent,
        pub username: String,
        pub password: String,
        pub(crate) request_user: bool,
    }

    ///The Payload struct is a rewrite of the Mojang "Yggdrasil" authentication JSON.
    ///As this project only relates to Minecraft, Only "Minecraft" and version: 1 should be used in the private struct Agent.
    ///Tokens are not implemented yet.
    ///One day, the response will be implemented but that day is not today [ •́ ‸ •̀ ]
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Agent {
        pub(crate) name: String,
        pub(crate) version: i64,
    }

    impl Payload {
        /// This function generates the payload. It is quite simple, as it only works for Minecraft.
        /// #Examples
        /// ```
        /// let example_payload = Payload::from(String::from("CoolName"), String::from("SecretPassword");
        /// assert_eq!(example_payload, Payload {
        ///                             agent: Agent { name: String::from("Minecraft"), version: 1 },
        ///                             String::from("CoolName"),
        ///                             String::from("SecretPassword"),
        ///                             request_user: false})
        /// ```
        pub fn from(username: String, password: String) -> Self {
            Self {
                agent: Agent { name: String::from("Minecraft"), version: 1 },
                username,
                password,
                request_user: false,
            }
        }
        pub fn to_string(&self) -> String {
            return serde_json::to_string(&self).unwrap();
        }
    }
}