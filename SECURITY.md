# Security Policy

## Output Is Not Sanitized

`convert()` and its bindings do not sanitize the Markdown they produce. HTML content — including
`<script>`, inline event handlers, `javascript:`/`data:` URLs, and raw tags passed through via
`preserve_tags` — can end up carried into the output verbatim. **Treat conversion output as
untrusted input** and sanitize it in whatever downstream system renders it (a browser, a
Markdown-to-HTML renderer, a chat UI, etc.) before displaying content derived from
attacker-controlled HTML. The library performs no XSS filtering, URL scheme allow-listing, or
attribute stripping.

## Resource Limits

- **No input size cap.** The library does not enforce a maximum HTML input length. Callers that
  accept HTML from untrusted or public sources (e.g. a server-side conversion endpoint) are
  responsible for enforcing their own size limits — unbounded input is an availability risk.
- **DOM depth is limited.** `ConversionOptions::max_depth` bounds recursive DOM traversal
  (default: an internal native-stack-safe limit; explicit values are clamped to an absolute
  ceiling) to prevent stack overflow on pathologically deep or malicious input. This limit cannot
  be disabled.
- Input is scanned for binary/corrupted data (compressed formats, excess NUL or control bytes,
  undeclared UTF-16) and rejected before parsing; this is a data-integrity check, not a
  sanitization pass.

## Reporting a Vulnerability

**Do not open a public issue for security reports.**

Preferred channel: open a private advisory at
<https://github.com/xberg-io/html-to-markdown/security/advisories/new>.

Alternative: email **<security@xberg.io>**.

Please include a description of the issue, steps to reproduce, affected versions, and your
preferred credit (or none). We acknowledge reports within **2 business days** and aim to
publish a fix within **14 days** for critical issues and **30 days** for others.

## Supported Versions

Security fixes target the latest release on `main`. Older versions are not back-ported.

## Scope

In scope: the html-to-markdown library and its bindings.
Out of scope: third-party dependencies (report upstream and notify us).
