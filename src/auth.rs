//! Shared-secret gate for the routes that mutate state or send messages.
//!
//! Two routes are dangerous to expose: `POST /trigger` runs a whole tick and can
//! push Discord cards plus advance KV state, and `POST /test-webhook` sends a
//! diagnostic card on demand. Both used to be reachable by anyone who knew the
//! hostname, so a stranger could spam the channel and consume episode state.
//!
//! Read-only routes stay open on purpose: they are what makes the service
//! diagnosable, and they expose nothing sensitive.

use worker::*;

/// Header carrying the shared secret.
pub const TOKEN_HEADER: &str = "X-Trigger-Token";

/// Name of the Worker secret holding the shared secret.
pub const SECRET_NAME: &str = "TRIGGER_TOKEN";

/// Compares two secrets without leaking where they differ.
///
/// A plain `==` returns as soon as two bytes differ, which turns the endpoint
/// into a timing oracle for guessing the token byte by byte. This folds the
/// length difference in and always scans the longer of the two.
pub fn tokens_match(expected: &str, presented: &str) -> bool {
    let a = expected.as_bytes();
    let b = presented.as_bytes();

    let mut diff = (a.len() ^ b.len()) as u16;
    let longest = a.len().max(b.len());
    for i in 0..longest {
        let x = a.get(i).copied().unwrap_or(0);
        let y = b.get(i).copied().unwrap_or(0);
        diff |= (x ^ y) as u16;
    }
    diff == 0
}

/// Extracts a token from an `Authorization` header value.
///
/// Accepts `Bearer <token>` with any casing of the scheme, because clients
/// differ and a rejected-but-correct token would be blamed on the client.
pub fn parse_authorization(value: &str) -> Option<String> {
    let value = value.trim();
    let (scheme, rest) = value.split_once(' ')?;
    if !scheme.eq_ignore_ascii_case("bearer") {
        return None;
    }
    let token = rest.trim();
    if token.is_empty() {
        None
    } else {
        Some(token.to_string())
    }
}

/// Reads the presented token from either accepted header form.
pub fn presented_token(req: &Request) -> Option<String> {
    let headers = req.headers();

    if let Ok(Some(value)) = headers.get(TOKEN_HEADER) {
        let owned = value.to_string();
        let trimmed = owned.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_string());
        }
    }

    match headers.get("Authorization") {
        Ok(Some(value)) => parse_authorization(&value.to_string()),
        _ => None,
    }
}

/// Gate for mutating routes. `Err` carries the response to return instead.
///
/// Fails closed: an unset or empty `TRIGGER_TOKEN` returns 503 rather than
/// letting the request through. A missing secret must never mean "no auth".
pub fn require(req: &Request, env: &Env) -> std::result::Result<(), Response> {
    let expected = match env.secret(SECRET_NAME) {
        Ok(secret) => secret.to_string(),
        Err(e) => {
            worker::console_error!(
                "Secret {} is not configured, refusing mutating route: {:?}",
                SECRET_NAME,
                e
            );
            return Err(unavailable());
        }
    };

    if expected.trim().is_empty() {
        worker::console_error!(
            "Secret {} is empty, refusing mutating route so it cannot fail open",
            SECRET_NAME
        );
        return Err(unavailable());
    }

    match presented_token(req) {
        Some(presented) if tokens_match(expected.trim(), &presented) => Ok(()),
        Some(_) => {
            worker::console_warn!("Rejected mutating route: wrong token");
            Err(Response::error("Unauthorized", 401).unwrap_or_else(|_| fallback(401)))
        }
        None => {
            worker::console_warn!("Rejected mutating route: no token presented");
            Err(Response::error("Unauthorized", 401).unwrap_or_else(|_| fallback(401)))
        }
    }
}

fn unavailable() -> Response {
    Response::error("Endpoint not configured", 503).unwrap_or_else(|_| fallback(503))
}

fn fallback(status: u16) -> Response {
    let mut resp = Response::ok(format!("status {status}")).unwrap();
    let _ = resp.headers_mut().set("Content-Type", "text/plain");
    resp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_tokens_match() {
        assert!(tokens_match("s3cret-token", "s3cret-token"));
    }

    #[test]
    fn different_tokens_do_not_match() {
        assert!(!tokens_match("s3cret-token", "s3cret-tokeX"));
        assert!(!tokens_match("s3cret-token", "S3CRET-TOKEN"));
        assert!(!tokens_match("s3cret-token", ""));
    }

    #[test]
    fn prefix_and_suffix_do_not_match() {
        // A prefix must not be accepted, which is what an early-return compare
        // can be tricked into on some implementations.
        assert!(!tokens_match("s3cret-token", "s3cret"));
        assert!(!tokens_match("s3cret", "s3cret-token"));
    }

    #[test]
    fn empty_expected_never_matches_a_presented_value() {
        // `require` rejects an empty secret before comparing, but the comparison
        // itself must not treat two empty strings as a match that matters.
        assert!(tokens_match("", ""));
        assert!(!tokens_match("", "x"));
    }

    #[test]
    fn authorization_accepts_bearer_case_insensitively() {
        assert_eq!(parse_authorization("Bearer abc").as_deref(), Some("abc"));
        assert_eq!(parse_authorization("bearer abc").as_deref(), Some("abc"));
        assert_eq!(parse_authorization("BEARER abc").as_deref(), Some("abc"));
        assert_eq!(
            parse_authorization("  Bearer   abc  ").as_deref(),
            Some("abc")
        );
    }

    #[test]
    fn authorization_rejects_other_schemes_and_empty_tokens() {
        assert_eq!(parse_authorization("Basic abc"), None);
        assert_eq!(parse_authorization("Bearer"), None);
        assert_eq!(parse_authorization("Bearer "), None);
        assert_eq!(parse_authorization(""), None);
        assert_eq!(parse_authorization("abc"), None);
    }
}
