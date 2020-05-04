extern crate instagram;

lazy_static! {
    static ref INSTAGRAM_USERNAME: String = {
        std::env::var("INSTAGRAM_USERNAME")
            .expect("You need to define INSTAGRAM_USERNAME environment variable.")
    };
    static ref INSTAGRAM_PASSWORD: String = {
        std::env::var("INSTAGRAM_PASSWORD")
            .expect("You need to define INSTAGRAM_PASSWORD environment variable.")
    };
}

use mockito::mock;
use mockito::Matcher;

use instagram::web_api::behaviour::*;
use instagram::web_api::Client;
use instagram::web_api::ClientError;
use instagram::web_api::Credentials;

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

    let client = Client::new_with_url(&mockito::server_url(), "");
    let freyskeyd_infos = client.fetch_user_infos("Freyskeyd").await.unwrap();

    m.assert();

    assert_eq!(freyskeyd_infos.username, "freyskeyd");
    assert_eq!(freyskeyd_infos.full_name, "FREYSKEYD");

    let m = mock("GET", "/Freyskeyd")
        .match_query(Matcher::UrlEncoded("__a".into(), "1".into()))
        .with_status(404)
        .expect(1)
        .create();

    let client = Client::new_with_url(&mockito::server_url(), "");
    let freyskeyd_infos = client.fetch_user_infos("Freyskeyd").await;

    assert!(freyskeyd_infos.is_err());
    m.assert();
}

#[tokio::test]
async fn test_logged_in_2_fa() {
    let fixture: String =
        ::std::fs::read_to_string("tests/web_api_client/response_login_2FA.json").unwrap();

    let fixture_init_rollout_hash: String =
        ::std::fs::read_to_string("tests/web_api_client/response_init_rollout.html").unwrap();

    let m_root = mock("GET", "/")
        .with_body(fixture_init_rollout_hash)
        .with_status(200)
        .expect(1)
        .create();

    let m_login = mock("POST", "/accounts/login/ajax/")
        .with_status(400)
        .with_body(&fixture)
        .expect(1)
        .create();

    let x = Client::new_with_url(&mockito::server_url(), "")
        .login(&get_credentials())
        .await;

    assert_eq!(Err(ClientError::UnableToPerform2FA), x);

    m_root.assert();
    m_login.assert();
}

#[tokio::test]
async fn test_logged_in() {
    let fixture = "{\"authenticated\": true, \"user\": true, \"userId\": \"8343444274\", \"oneTapPrompt\": false, \"status\": \"ok\"}";

    let fixture_init_rollout_hash: String =
        ::std::fs::read_to_string("tests/web_api_client/response_init_rollout.html").unwrap();

    let fixture_infos: String =
        ::std::fs::read_to_string("tests/web_api_client/response_user_info.json").unwrap();

    let m_root = mock("GET", "/")
        .with_body(fixture_init_rollout_hash)
        .with_status(200)
        .expect(1)
        .create();

    let m_login = mock("POST", "/accounts/login/ajax/")
        .with_status(200)
        .with_body(fixture)
        .expect(1)
        .create();

    let m_user_info = mock("GET", "/Freyskeyd")
        .match_query(Matcher::UrlEncoded("__a".into(), "1".into()))
        .with_status(200)
        .with_body(&fixture_infos)
        .expect(1)
        .create();

    let client = Client::new_with_url(&mockito::server_url(), "")
        .login(&get_credentials())
        .await
        .unwrap();
    let freyskeyd_infos = client.fetch_user_infos("Freyskeyd").await.unwrap();

    assert_eq!(freyskeyd_infos.username, "freyskeyd");
    assert_eq!(freyskeyd_infos.full_name, "FREYSKEYD");

    m_root.assert();
    m_login.assert();
    m_user_info.assert();
}

#[tokio::test]
async fn test_user_feed() {
    let _ = env_logger::try_init();
    let fixture: String =
        ::std::fs::read_to_string("tests/web_api_client/response_user_feed.json").unwrap();

    let m_user_feed = mock("GET", "/")
        .match_query(Matcher::Regex("query_hash=.*variables=.*".into()))
        .with_status(200)
        .with_body(&fixture)
        .expect(1)
        .create();

    let user_feed = Client::new_with_url("", &mockito::server_url())
        .fetch_user_feed("1234", None)
        .await
        .unwrap();

    assert_eq!(user_feed.count, 147);

    m_user_feed.assert();
}

fn get_credentials() -> Credentials<'static> {
    Credentials {
        username: &INSTAGRAM_USERNAME,
        password: &INSTAGRAM_PASSWORD,
    }
}
