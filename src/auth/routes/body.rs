#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct RegisterCredentials {
    pub username: String,
    pub password: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct Session {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct AccessToken {
    pub access_token: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct IsValid {
    pub is_valid: bool,
}
