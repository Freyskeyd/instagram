extern crate instagram;

use mockito::mock;
use mockito::Matcher;

use instagram::web_api::prelude::*;

#[test]
fn test_create_client() {
    let client: Client = Client::new();

    assert_eq!(client.version, 0);
    assert_eq!(client.is_authenticated(), false);
    assert_eq!(client.authenticated_user_id(), "");
    assert_eq!(client.authenticated_user_name(), "");
}

#[tokio::test]
async fn test_get_user_info() {
    let fixture: String =
        ::std::fs::read_to_string("tests/web_api_client/response_user_info.json").unwrap();
    let m = mock("GET", "/Freyskeyd")
        .match_query(Matcher::UrlEncoded("__a".into(), "1".into()))
        .with_status(200)
        .with_body(&fixture)
        .expect(1)
        .create();

    let client: Client = Client::new_with_url(&mockito::server_url());
    let freyskeyd_info = client.user_info("Freyskeyd").await.unwrap();

    m.assert();

    assert_eq!(freyskeyd_info.username, "freyskeyd");
    assert_eq!(freyskeyd_info.full_name, "FREYSKEYD");

    let m = mock("GET", "/Freyskeyd")
        .match_query(Matcher::UrlEncoded("__a".into(), "1".into()))
        .with_status(404)
        .expect(1)
        .create();

    let client: Client = Client::new_with_url(&mockito::server_url());
    let freyskeyd_info = client.user_info("Freyskeyd").await;

    assert!(freyskeyd_info.is_err());
    m.assert();
}
