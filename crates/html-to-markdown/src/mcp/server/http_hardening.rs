//! Production hardening for the MCP Streamable HTTP transport.
//!
//! `start_mcp_server_http` binds a real socket; the [`build_http_router`] entry
//! point here is separated out purely so the layer stack can be exercised with
//! `tower::ServiceExt::oneshot` in tests, without binding a port.
//!
//! Layers, outermost (runs first) to innermost:
//! 1. [`validate_origin_and_host`] — `Origin`/`Host` allowlist (DNS-rebinding protection).
//! 2. [`tower_http::limit::RequestBodyLimitLayer`] — request body size cap.
//! 3. [`axum::error_handling::HandleErrorLayer`] — converts a timeout into a response.
//! 4. [`tower::limit::ConcurrencyLimitLayer`] — global in-flight request cap.
//! 5. [`tower::timeout::TimeoutLayer`] — per-request wall-clock budget.

use std::collections::HashSet;
use std::net::IpAddr;
use std::time::Duration;

use axum::Router;
use axum::error_handling::HandleErrorLayer;
use axum::extract::{Request, State};
use axum::http::{HeaderMap, StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::Response;
use rmcp::transport::streamable_http_server::{StreamableHttpService, session::local::LocalSessionManager};
use tower::ServiceBuilder;
use tower::limit::ConcurrencyLimitLayer;
use tower::timeout::TimeoutLayer;
use tower_http::limit::RequestBodyLimitLayer;

/// Maximum accepted MCP HTTP request body, in bytes.
///
/// Bounds worst-case per-request memory: large HTML documents still fit
/// comfortably, while an oversized (malicious or mistaken) payload is
/// rejected before any of it is buffered or parsed.
const MAX_REQUEST_BODY_BYTES: usize = 10 * 1024 * 1024;

/// Wall-clock budget for one MCP HTTP request/response cycle.
///
/// Bounds how long a single request (and the connection serving it) can run,
/// so one slow or adversarial request cannot hold a session open indefinitely.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

/// Maximum number of MCP HTTP requests processed concurrently.
///
/// Caps memory and CPU fan-out from concurrent conversions; requests beyond
/// this queue for a slot instead of running unbounded.
const MAX_CONCURRENT_REQUESTS: usize = 64;

/// Build the `axum` router for the MCP Streamable HTTP transport, with every
/// hardening layer applied.
pub(super) fn build_http_router(host: &str, port: u16) -> Router {
    let http_service = StreamableHttpService::new(
        || Ok(super::HtmlToMarkdownMcp::new()),
        LocalSessionManager::default().into(),
        Default::default(),
    );

    let allowed_hosts = AllowedHosts::new(host, port);

    Router::new().nest_service("/mcp", http_service).layer(
        ServiceBuilder::new()
            .layer(middleware::from_fn_with_state(allowed_hosts, validate_origin_and_host))
            .layer(RequestBodyLimitLayer::new(MAX_REQUEST_BODY_BYTES))
            .layer(HandleErrorLayer::new(handle_inner_layer_error))
            .layer(ConcurrencyLimitLayer::new(MAX_CONCURRENT_REQUESTS))
            .layer(TimeoutLayer::new(REQUEST_TIMEOUT)),
    )
}

/// Convert an error raised by an inner layer (in practice, only
/// [`TimeoutLayer`] ever produces one — [`tower::timeout::error::Elapsed`])
/// into a response. Anything else would indicate a bug in a lower layer
/// rather than a timeout, so it maps to `500` rather than panicking.
async fn handle_inner_layer_error(error: tower::BoxError) -> StatusCode {
    if error.is::<tower::timeout::error::Elapsed>() {
        StatusCode::REQUEST_TIMEOUT
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

/// Host/Origin allowlist enforced on every MCP HTTP request.
///
/// Rejects requests whose `Host` header does not match the bound address, and
/// requests carrying an `Origin` header (sent by browsers, never by the MCP
/// SDKs or `curl`) that does not match either. This is the DNS-rebinding
/// mitigation the MCP HTTP transport spec calls for: without it, a malicious
/// web page can rebind an attacker-controlled hostname to `127.0.0.1` and
/// reach a server that otherwise only "looks" localhost-bound to the browser.
#[derive(Clone)]
struct AllowedHosts {
    /// Exact `host:port` (or `[ipv6]:port`) authorities that are always accepted.
    authorities: HashSet<String>,
    /// Set when the listener is bound to an unspecified address (`0.0.0.0`,
    /// `::`) — the host part cannot be predicted there, so only the port is
    /// checked. `None` when bound to a concrete address.
    unspecified_bind_port: Option<u16>,
}

impl AllowedHosts {
    fn new(host: &str, port: u16) -> Self {
        let mut authorities = HashSet::new();
        authorities.insert(format!("{host}:{port}"));
        if is_loopback_host(host) {
            authorities.insert(format!("localhost:{port}"));
            authorities.insert(format!("127.0.0.1:{port}"));
            authorities.insert(format!("[::1]:{port}"));
        }
        let unspecified_bind_port = is_unspecified_host(host).then_some(port);
        Self {
            authorities,
            unspecified_bind_port,
        }
    }

    fn allows(&self, authority: &str) -> bool {
        self.authorities.contains(authority)
            || self
                .unspecified_bind_port
                .is_some_and(|port| authority_port(authority) == Some(port))
    }
}

fn is_loopback_host(host: &str) -> bool {
    host.eq_ignore_ascii_case("localhost") || host.parse::<IpAddr>().is_ok_and(|ip| ip.is_loopback())
}

fn is_unspecified_host(host: &str) -> bool {
    host.parse::<IpAddr>().is_ok_and(|ip| ip.is_unspecified())
}

/// Extract the port from a `host:port` (or `[ipv6]:port`) authority string.
fn authority_port(authority: &str) -> Option<u16> {
    authority
        .rsplit_once(':')
        .and_then(|(_, port)| port.trim_end_matches(']').parse().ok())
}

/// `axum` middleware enforcing [`AllowedHosts`] on the `Host` and `Origin` headers.
async fn validate_origin_and_host(
    State(allowed): State<AllowedHosts>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = request.headers();
    if !host_header_allowed(headers, &allowed) || !origin_header_allowed(headers, &allowed) {
        return Err(StatusCode::FORBIDDEN);
    }
    Ok(next.run(request).await)
}

fn host_header_allowed(headers: &HeaderMap, allowed: &AllowedHosts) -> bool {
    headers
        .get(header::HOST)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|host| allowed.allows(host))
}

fn origin_header_allowed(headers: &HeaderMap, allowed: &AllowedHosts) -> bool {
    match headers.get(header::ORIGIN).and_then(|value| value.to_str().ok()) {
        // Non-browser clients (the MCP SDKs, curl) do not send `Origin`.
        None => true,
        Some(origin) => origin
            .rsplit_once("://")
            .is_some_and(|(_, authority)| allowed.allows(authority)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request as HttpRequest;
    use tower::ServiceExt as _;

    fn router() -> Router {
        build_http_router("127.0.0.1", 8001)
    }

    fn request(host: &str, origin: Option<&str>, content_length: Option<usize>) -> HttpRequest<Body> {
        let mut builder = HttpRequest::post("/mcp").header(header::HOST, host);
        if let Some(origin) = origin {
            builder = builder.header(header::ORIGIN, origin);
        }
        if let Some(len) = content_length {
            builder = builder.header(header::CONTENT_LENGTH, len.to_string());
        }
        builder.body(Body::empty()).expect("request builds")
    }

    #[tokio::test]
    async fn should_reject_oversized_body_with_413() {
        let response = router()
            .oneshot(request("127.0.0.1:8001", None, Some(MAX_REQUEST_BODY_BYTES + 1)))
            .await
            .expect("router is infallible");

        assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    }

    #[tokio::test]
    async fn should_accept_body_at_the_limit() {
        let response = router()
            .oneshot(request("127.0.0.1:8001", None, Some(MAX_REQUEST_BODY_BYTES)))
            .await
            .expect("router is infallible");

        assert_ne!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    }

    #[tokio::test]
    async fn should_reject_mismatched_host_header_with_403() {
        let response = router()
            .oneshot(request("evil.example.com:8001", None, None))
            .await
            .expect("router is infallible");

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn should_reject_missing_host_header_with_403() {
        let request = HttpRequest::post("/mcp").body(Body::empty()).expect("request builds");
        let response = router().oneshot(request).await.expect("router is infallible");

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn should_reject_cross_origin_request_with_403() {
        let response = router()
            .oneshot(request("127.0.0.1:8001", Some("https://evil.example.com"), None))
            .await
            .expect("router is infallible");

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn should_accept_same_origin_request_past_the_host_origin_layer() {
        let response = router()
            .oneshot(request("127.0.0.1:8001", Some("http://127.0.0.1:8001"), None))
            .await
            .expect("router is infallible");

        assert_ne!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn should_accept_localhost_host_header_when_bound_to_loopback_ip() {
        let response = router()
            .oneshot(request("localhost:8001", None, None))
            .await
            .expect("router is infallible");

        assert_ne!(response.status(), StatusCode::FORBIDDEN);
    }

    #[test]
    fn allowed_hosts_permits_only_the_bound_authority_for_a_concrete_host() {
        let allowed = AllowedHosts::new("192.168.1.5", 9000);
        assert!(allowed.allows("192.168.1.5:9000"));
        assert!(!allowed.allows("192.168.1.6:9000"), "different host must be rejected");
        assert!(!allowed.allows("192.168.1.5:9001"), "different port must be rejected");
    }

    #[test]
    fn allowed_hosts_checks_only_port_for_unspecified_bind() {
        let allowed = AllowedHosts::new("0.0.0.0", 9000);
        assert!(allowed.allows("anything.example.com:9000"));
        assert!(
            !allowed.allows("anything.example.com:9001"),
            "wrong port must be rejected"
        );
    }

    #[test]
    fn is_loopback_host_recognizes_localhost_and_loopback_ips() {
        assert!(is_loopback_host("localhost"));
        assert!(is_loopback_host("127.0.0.1"));
        assert!(is_loopback_host("::1"));
        assert!(!is_loopback_host("0.0.0.0"));
        assert!(!is_loopback_host("example.com"));
    }
}
