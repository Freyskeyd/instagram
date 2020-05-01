#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate assert_impl;

mod web_api_client {
    mod authenticated_client;
    mod create_client;
    mod unauthenticated_client;
}
