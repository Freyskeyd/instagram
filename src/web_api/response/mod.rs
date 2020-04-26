use crate::web_api::domain::LoginInfos;
use crate::web_api::domain::UserInfos;
use serde::Deserialize;

impl std::convert::From<reqwest::Error> for UserInfosError {
    fn from(_error: reqwest::Error) -> Self {
        Self::Other
    }
}

#[derive(Debug)]
pub enum UserInfosError {
    NotFound,
    Other,
}

#[derive(Deserialize)]
pub struct ApiResponse<T> {
    pub graphql: T,
}

#[derive(Deserialize)]
pub struct UserInfosResponse {
    pub user: UserInfos,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum LoginResponse {
    Success(LoginInfos),
    TwoFactorNeeded(TwoFactorRequiredResponse),
}

#[derive(Debug, Deserialize)]
pub struct TwoFactorRequiredResponse {
    pub message: String,
    pub two_factor_required: bool,
    pub two_factor_info: TwoFactorInfo,
    pub phone_verification_settings: PhoneVerificationSettings,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct TwoFactorInfo {
    pub username: String,
    pub sms_two_factor_on: bool,
    pub totp_two_factor_on: bool,
    pub obfuscated_phone_number: String,
    pub two_factor_identifier: String,
    pub show_messenger_code_option: bool,
    pub show_new_login_screen: bool,
    pub show_trusted_device_option: bool,
    pub phone_verification_settings: PhoneVerificationSettings,
}

#[derive(Debug, Deserialize)]
pub struct PhoneVerificationSettings {
    pub max_sms_count: i8,
    pub resend_sms_delay_sec: i32,
    pub robocall_count_down_time_sec: i32,
    pub robocall_after_max_sms: bool,
}
