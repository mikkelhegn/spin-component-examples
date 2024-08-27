use std::intrinsics::mir::Field;

use std::str::FromStr;
use super::OAuth2;
use cookie::{Cookie, SameSite};
use oauth2::{AuthorizationCode, CsrfToken, RedirectUrl, TokenResponse};
use spin_sdk::http::{Headers, OutgoingResponse, ResponseOutparam, SendError};
use url::Url;

pub async fn callback(url: Url, output: ResponseOutparam) {
    let callback_url = match std::env::var("AUTH_CALLBACK_URL") {
        Ok(runtime_env) => runtime_env,
        Err(_) => crate::api::AUTH_CALLBACK_URL
            .unwrap_or("http://127.0.0.1:3000/login/callback")
            .to_string(),
    };
    let client = match OAuth2::try_init() {
        Ok(config) => {
            let redirect_url =
                RedirectUrl::new(callback_url.clone()).expect("Invalid redirect URL");
            config
                .into_client()
                .set_auth_type(oauth2::AuthType::RequestBody)
                .set_redirect_uri(redirect_url)
        }
        Err(error) => {
            eprintln!("failed to initialize oauth client: {error}");
            let response = OutgoingResponse::new(Headers::new());
            response.set_status_code(500);
            output.set(response);
            return;
        }
    };

    let (code, _state) = match get_code_and_state_param(&url) {
        Ok((code, state)) => (code, state),
        Err(error) => {
            eprintln!("error retrieving required query parameters: {error}");
            let response = OutgoingResponse::new(Headers::new());
            response.set_status_code(500);
            output.set(response);
            return;
        }
    };

    // TODO: check state with cached state and ensure equality

    let result = client
        .exchange_code(code)
        .request_async(oauth_http_client)
        .await;

    let mut location = url::Url::parse(&callback_url).unwrap();
    location.set_path("");
    match result {
        Ok(result) => {
            let access_token = serde_json::to_string(result.access_token())
                .unwrap()
                .replace("\"", "");

            let mut oauth_cookie = Cookie::new("access-token", access_token);
            oauth_cookie.set_same_site(Some(SameSite::Lax));
            oauth_cookie.set_http_only(true);
            oauth_cookie.set_path("/");

            let headers = Headers::from_list(&[
                ("Content-Type".to_string(), b"text/plain".to_vec()),
                (
                    "Location".to_string(),
                    location.to_string().as_bytes().to_vec(),
                ),
                (
                    "Set-Cookie".to_string(),
                    oauth_cookie.to_string().as_bytes().to_vec(),
                ),
            ])
            .expect("could not set header from list");

            let response = OutgoingResponse::new(headers);
            response.set_status_code(301);
            output.set(response);
        }
        Err(error) => {
            eprintln!("error exchanging code for token with github: {error}");
            let response = OutgoingResponse::new(Headers::new());
            response.set_status_code(403);
            output.set(response);
        }
    }
}

fn get_code_and_state_param(url: &Url) -> anyhow::Result<(AuthorizationCode, CsrfToken)> {
    fn get_query_param(url: &Url, param: &str) -> Option<String> {
        url.query_pairs()
            .find(|(key, _)| key == param)
            .map(|(_, value)| value.into_owned())
    }

    const STATE_QUERY_PARAM_NAME: &str = "state";
    const CODE_QUERY_PARAM_NAME: &str = "code";

    let Some(param) = get_query_param(url, STATE_QUERY_PARAM_NAME) else {
        anyhow::bail!("missing '{STATE_QUERY_PARAM_NAME}' query parameter");
    };

    let state = CsrfToken::new(param);

    let Some(param) = get_query_param(url, CODE_QUERY_PARAM_NAME) else {
        anyhow::bail!("missing '{CODE_QUERY_PARAM_NAME}' query parameter");
    };

    let code = AuthorizationCode::new(param);

    Ok((code, state))
}

async fn oauth_http_client(req: oauth2::HttpRequest) -> Result<oauth2::HttpResponse, SendError> {

    let mut builder = http::Request::builder()
        .method(req.method)
        .uri(req.url.as_str());

    for (name, value) in &req.headers {
        builder = builder.header(name, value);
    }

    let res = spin_sdk::http::send::<_, http::Response<String>>(builder.body(req.body)).await?;

    let (parts, body) = res.into_parts();

    Ok(oauth2::HttpResponse {
        status_code: parts.status,
        headers: parts.headers,
        body: body.into_bytes(),
    })
}
