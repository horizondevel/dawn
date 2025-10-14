use axum::extract::Request;
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use openidconnect::core::{
    CoreAuthenticationFlow,
    CoreClient,
    CoreProviderMetadata

    ,
};
use openidconnect::reqwest;
use openidconnect::{
    ClientId,
    ClientSecret,
    CsrfToken,
    IssuerUrl,
    Nonce
    ,
    PkceCodeChallenge,
    RedirectUrl,
    Scope
    ,
};
use std::task::Poll;
use tower::{Layer, Service};

#[derive(Clone)]
pub struct SecurityLayer {
    pub client: CoreClient,
}

impl SecurityLayer {
    pub const fn new(client: CoreClient) -> Self {
        
        Self {
            client: create_client()
        }
    }
}
async fn create_client() -> CoreClient {
    let http_client = reqwest::ClientBuilder::new()
        // Following redirects opens the client up to SSRF vulnerabilities.
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");

    // Use OpenID Connect Discovery to fetch the provider metadata.
    let provider_metadata = CoreProviderMetadata::discover_async(
        IssuerUrl::new(
            "https://cognito-idp.ca-west-1.amazonaws.com/ca-west-1_zdCraYDUL".to_string(),
        )
            .unwrap(),
        &http_client,
    )
        .await
        .unwrap();
    CoreClient::from_provider_metadata(
        provider_metadata,
        ClientId::new("3fcqpqv0e1ha4ceslb5fnp625n".to_string()),
        Some(ClientSecret::new(
            "6hmt1cg8lr6ra7p1o22hevubugh22g33g4n5ft79fs9qdf4nse5".to_string(),
        )),
    )
}
impl<S> Layer<S> for SecurityLayer {
    type Service = SecurityMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SecurityMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct SecurityMiddleware<S> {
    inner: S,
}

impl<S> Service<Request> for SecurityMiddleware<S>
where
    S: Service<Request, Response=Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = futures_util::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        println!("In Security middleware {:?}", req);
        let response = Response::builder()
            .status(StatusCode::FORBIDDEN)
            .header(http::header::CONTENT_TYPE, "text/plain")
            .body("403 Forbidden: Access Denied".to_string())
            .unwrap();
        if !response.status().is_success() {
            return Box::pin(async move { Ok(response.into_response()) });
        }
        let future = self.inner.call(req);
        Box::pin(async move {
            let response: Response = future.await?;
            println!("{:#?}", response);
            Ok(response)
        })
    }
}
