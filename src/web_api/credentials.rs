/// Credentials represent a struct used to hold a user authentication informations
///
/// # Examples
///
/// ```rust
/// use instagram::web_api::Credentials;
///
/// let my_creds: Credentials = Credentials {
///     username: "my_username",
///     password: "my_password",
/// };
/// ```
pub struct Credentials<'a> {
    /// define the credential's username for authentication
    pub username: &'a str,
    /// define the credential's password for authentication
    pub password: &'a str,
}
