//! Regression tests for three Tier-1 gaps found by audit review:
//!
//! - **A**: `scanner.rs` independently duplicated two bugs just fixed in Tier-2
//!   (`converter/handlers/code_block.rs`): a code fence hardcoded to exactly three
//!   backticks (never widened for content containing a matching run), and a
//!   backwards inline-code-span delimiter formula. Exercised here by calling
//!   `tier1::run` directly so a silent bail-to-Tier-2 fallback cannot mask a
//!   scanner-level regression.
//! - **B** (audit #57): the scanner had no depth ceiling matching Tier-2's
//!   `effective_max_depth`. Tier-2's recursive `walk_node` truncates past that
//!   depth; the byte scanner, having no native recursion of its own, would
//!   otherwise scan pathologically deep input to completion and diverge from
//!   Tier-2's truncated (authoritative) output.
//! - **C** (audit #58): `result.metadata` was silently `HtmlMetadata::default()`
//!   on every Tier-1 success path regardless of `extract_metadata`, because
//!   `tier1::run` only produces YAML frontmatter *text*, never the structured
//!   struct. Fixed by gating `classify()` on `extract_metadata` so Tier-2 (which
//!   already builds the struct) is authoritative whenever it's requested.
//! - **D** (audit #12 follow-up): Tier-2's `strip_hidden_elements`
//!   (`converter/utility/preprocessing.rs`) unconditionally removes elements
//!   carrying the `hidden` attribute or a `style="display:none"` /
//!   `style="visibility:hidden"` declaration, as a raw-string pass before
//!   either tier sees the document. The scanner had no equivalent and would
//!   emit the hidden element's content verbatim.
//! - **E** (markdown-injection security fix mirror): the scanner emitted two
//!   unescaped labels into Markdown link/image syntax — an SVG `<title>` as
//!   `![title](data:...)`, and an `<a>` element's text as `[label](href)`.
//!   Inert HTML content containing `](https://evil.example)` in either spot
//!   closes the label early and opens a second, attacker-controlled Markdown
//!   link/image. Fixed by calling the same `escape_link_label` helper
//!   Tier-2's `<a>` handler (`converter/handlers/link.rs`) already uses.

#![cfg(feature = "testkit")]

use html_to_markdown_rs::prescan::PrescanReport;
use html_to_markdown_rs::{CodeBlockStyle, ConversionOptions, HighlightStyle, TierStrategy, convert, tier1};

/// Baseline options that clear every classifier gate so `TierStrategy::Auto`
/// genuinely attempts the Tier-1 scanner rather than routing straight to
/// Tier-2 for an unrelated reason (`highlight_style`'s non-`None` default is
/// itself a router gate — see `tier1::router::classify`). Tests that assert
/// Auto-routed output matches Tier-2 must start from these so a passing
/// assertion actually exercises the scanner fix under test, not a
/// classifier bail that never reaches Tier-1 in the first place.
fn base_options() -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        highlight_style: HighlightStyle::None,
        ..ConversionOptions::default()
    }
}

// ~keep ── A. Code fence widening (scanner.rs close_pre, Backticks style) ────────────

#[test]
fn should_widen_tier1_code_fence_to_four_backticks_when_pre_content_has_a_triple_backtick_run() {
    let html = "<pre><code>a\n```\nb</code></pre>";
    let options = ConversionOptions {
        code_block_style: CodeBlockStyle::Backticks,
        ..base_options()
    };
    let report = PrescanReport::default();
    let result = tier1::run(html, &report, &options).expect("tier1 scanner should not bail on this input");
    assert_eq!(result, "````\na\n```\nb\n````\n");
}

#[test]
fn should_use_three_backtick_tier1_fence_when_pre_content_has_no_backticks() {
    let html = "<pre><code>plain content</code></pre>";
    let options = ConversionOptions {
        code_block_style: CodeBlockStyle::Backticks,
        ..base_options()
    };
    let report = PrescanReport::default();
    let result = tier1::run(html, &report, &options).expect("tier1 scanner should not bail on this input");
    assert_eq!(result, "```\nplain content\n```\n");
}

// ~keep ── A. Inline code span delimiter (scanner.rs close_code) ────────────────────

#[test]
fn should_use_three_backtick_tier1_delimiter_when_inline_code_mixes_single_and_double_backtick_runs() {
    // ~keep the genuine latent bug: run lengths {1, 2} are both present, so the
    // ~keep smallest safe delimiter is 3. The old `if max_consecutive == 1 { 2 } else
    // ~keep { 1 } ` formula returned 1 here (max_consecutive == 2), which collides
    // ~keep with the length-1 run already in the content and corrupts the span.
    let html = "<p><code>x`y``z</code></p>";
    let report = PrescanReport::default();
    let result = tier1::run(html, &report, &base_options()).expect("tier1 scanner should not bail on this input");
    assert_eq!(result, "```x`y``z```\n");
}

#[test]
fn should_use_single_backtick_tier1_delimiter_when_inline_code_has_only_a_double_backtick_run() {
    // ~keep CommonMark closes a code span at a run of the SAME length as the opener
    // ~keep (6.1); a run of 2 never collides with a length-1 delimiter.
    let html = "<p><code>x``y</code></p>";
    let report = PrescanReport::default();
    let result = tier1::run(html, &report, &base_options()).expect("tier1 scanner should not bail on this input");
    assert_eq!(result, "`x``y`\n");
}

// ~keep ── B. Depth ceiling (audit #57) ──────────────────────────────────────────────

/// Mirrors `NATIVE_STACK_SAFE_DEPTH` (options/conversion.rs) — the default
/// `effective_max_depth` when the caller does not set `max_depth`.
const NATIVE_STACK_SAFE_DEPTH: usize = 64;

fn nested_divs(count: usize) -> String {
    let mut html = "<div>".repeat(count);
    html.push('x');
    html.push_str(&"</div>".repeat(count));
    html
}

#[test]
fn should_bail_with_depth_limit_exceeded_when_tier1_nesting_reaches_the_native_stack_safe_depth() {
    // ~keep One level past the limit: the (NATIVE_STACK_SAFE_DEPTH + 1)-th `<div>`
    // ~keep opens at stack depth == NATIVE_STACK_SAFE_DEPTH, matching Tier-2's
    // ~keep `depth >= effective_max_depth` skip condition exactly.
    let html = nested_divs(NATIVE_STACK_SAFE_DEPTH + 1);
    let report = PrescanReport::default();
    let result = tier1::run(&html, &report, &base_options());
    match result {
        Err(tier1::BailReason::DepthLimitExceeded { depth, max_depth }) => {
            assert_eq!(depth, NATIVE_STACK_SAFE_DEPTH);
            assert_eq!(max_depth, NATIVE_STACK_SAFE_DEPTH);
        }
        other => panic!("expected Err(BailReason::DepthLimitExceeded {{ .. }}), got {other:?}"),
    }
}

#[test]
fn should_not_bail_when_tier1_nesting_is_exactly_at_the_native_stack_safe_depth() {
    let html = nested_divs(NATIVE_STACK_SAFE_DEPTH);
    let report = PrescanReport::default();
    let result = tier1::run(&html, &report, &base_options());
    assert!(
        result.is_ok(),
        "nesting exactly at the limit must not bail: {:?}",
        result.err()
    );
}

#[test]
fn should_respect_custom_max_depth_when_tier1_nesting_reaches_it() {
    let options = ConversionOptions {
        max_depth: Some(5),
        ..base_options()
    };
    let html = nested_divs(6);
    let report = PrescanReport::default();
    let result = tier1::run(&html, &report, &options);
    match result {
        Err(tier1::BailReason::DepthLimitExceeded { depth, max_depth }) => {
            assert_eq!(depth, 5);
            assert_eq!(max_depth, 5);
        }
        other => panic!("expected Err(BailReason::DepthLimitExceeded {{ .. }}), got {other:?}"),
    }
}

#[test]
fn should_match_tier2_output_when_auto_routing_hits_a_pathologically_deep_dom() {
    // ~keep Before the fix, Tier-1 had no ceiling and would fully render all
    // ~keep NATIVE_STACK_SAFE_DEPTH + 16 levels, while Tier-2 truncates at
    // ~keep NATIVE_STACK_SAFE_DEPTH — a silent byte-equality violation on any
    // ~keep pathologically deep document. After the fix, Tier-1 bails and the
    // ~keep Tier-2 fallback (truncated, authoritative) output is returned instead.
    let html = nested_divs(NATIVE_STACK_SAFE_DEPTH + 16);

    let tier2_options = ConversionOptions {
        tier_strategy: TierStrategy::Tier2,
        ..base_options()
    };
    let tier2_output = convert(&html, Some(tier2_options))
        .expect("tier2 conversion must succeed")
        .content
        .unwrap_or_default();

    let auto_options = ConversionOptions {
        tier_strategy: TierStrategy::Auto,
        ..base_options()
    };
    let auto_output = convert(&html, Some(auto_options))
        .expect("auto conversion must succeed")
        .content
        .unwrap_or_default();

    assert_eq!(
        auto_output, tier2_output,
        "Auto routing must match Tier-2's truncated output for pathologically deep input"
    );
}

// ~keep ── C. `result.metadata` parity (audit #58) ──────────────────────────────────

#[cfg(feature = "metadata")]
#[test]
fn should_populate_identical_metadata_when_auto_routing_and_forced_tier2_run_on_the_same_input() {
    let html = concat!(
        "<html><head><title>My Article</title>",
        "<meta name=\"description\" content=\"An interesting read\"></head>",
        "<body><h1 id=\"main\">Title</h1>",
        "<a href=\"https://example.com\">External Link</a></body></html>",
    );

    let tier2_options = ConversionOptions {
        tier_strategy: TierStrategy::Tier2,
        extract_metadata: true,
        ..ConversionOptions::default()
    };
    let tier2_result = convert(html, Some(tier2_options)).expect("tier2 conversion must succeed");

    let auto_options = ConversionOptions {
        tier_strategy: TierStrategy::Auto,
        extract_metadata: true,
        ..ConversionOptions::default()
    };
    let auto_result = convert(html, Some(auto_options)).expect("auto conversion must succeed");

    // ~keep `HtmlMetadata` has no `PartialEq`; compare via its `Serialize` impl for an
    // ~keep exact structural equality check rather than spot-checking a few fields.
    let tier2_json = serde_json::to_string(&tier2_result.metadata).expect("tier2 metadata must serialize");
    let auto_json = serde_json::to_string(&auto_result.metadata).expect("auto metadata must serialize");
    assert_eq!(
        auto_json, tier2_json,
        "Auto-routed result.metadata must equal Tier-2's structured metadata byte-for-byte"
    );

    // ~keep Sanity: prove the compared metadata is actually populated, not two empty
    // ~keep defaults trivially matching each other.
    assert_eq!(tier2_result.metadata.document.title.as_deref(), Some("My Article"));
    assert_eq!(auto_result.metadata.document.title.as_deref(), Some("My Article"));
}

#[cfg(feature = "metadata")]
#[test]
fn should_leave_tier1_metadata_default_when_extract_metadata_is_false() {
    // ~keep When metadata is not requested, both tiers agree it stays default —
    // ~keep confirms the gate is specific to `extract_metadata`, not a blanket
    // ~keep Tier-2 detour for every `metadata`-feature build. Starts from
    // ~keep `base_options()` (not `ConversionOptions::default()`) so Auto
    // ~keep genuinely reaches Tier-1 instead of bailing on the unrelated
    // ~keep default `highlight_style` gate.
    let html = "<html><head><title>Ignored</title></head><body><p>hi</p></body></html>";
    let options = ConversionOptions {
        tier_strategy: TierStrategy::Auto,
        ..base_options()
    };
    let result = convert(html, Some(options)).expect("conversion must succeed");
    assert_eq!(result.metadata.document.title, None);
}

// ~keep ── D. Hidden-element parity (audit #12 follow-up) ───────────────────────────

#[test]
fn should_bail_with_hidden_element_when_tier1_open_tag_has_hidden_attribute() {
    let html = "<p>before</p><div hidden>secret</div><p>after</p>";
    let report = PrescanReport::default();
    let result = tier1::run(html, &report, &base_options());
    match result {
        Err(tier1::BailReason::HiddenElement { offset }) => {
            assert_eq!(offset, html.find("<div").expect("fixture must contain <div"));
        }
        other => panic!("expected Err(BailReason::HiddenElement {{ .. }}), got {other:?}"),
    }
}

#[test]
fn should_bail_with_hidden_element_when_tier1_open_tag_has_display_none_style() {
    let html = r#"<p>before</p><div style="display: none">secret</div><p>after</p>"#;
    let report = PrescanReport::default();
    let result = tier1::run(html, &report, &base_options());
    assert!(
        matches!(result, Err(tier1::BailReason::HiddenElement { .. })),
        "expected Err(BailReason::HiddenElement {{ .. }}), got {result:?}"
    );
}

#[test]
fn should_bail_with_hidden_element_when_tier1_open_tag_has_visibility_hidden_style() {
    let html = r#"<p>before</p><span style="visibility:hidden !important">secret</span><p>after</p>"#;
    let report = PrescanReport::default();
    let result = tier1::run(html, &report, &base_options());
    assert!(
        matches!(result, Err(tier1::BailReason::HiddenElement { .. })),
        "expected Err(BailReason::HiddenElement {{ .. }}), got {result:?}"
    );
}

#[test]
fn should_bail_with_hidden_element_when_a_hidden_svg_is_encountered() {
    // ~keep The hidden-element check runs before the scanner's dedicated `<svg>`
    // ~keep base64-emission branch (which has no hidden-element awareness of its
    // ~keep own), so a hidden SVG must bail rather than emit a data URI.
    let html = r#"<svg hidden><circle r="1"/></svg>"#;
    let report = PrescanReport::default();
    let result = tier1::run(html, &report, &base_options());
    assert!(
        matches!(result, Err(tier1::BailReason::HiddenElement { .. })),
        "expected Err(BailReason::HiddenElement {{ .. }}), got {result:?}"
    );
}

#[test]
fn should_not_bail_when_tier1_open_tag_has_a_visible_style_declaration() {
    // ~keep Regression guard against over-triggering: an unrelated `style`
    // ~keep declaration (or `data-hidden` / `aria-hidden`, which are distinct
    // ~keep attributes) must not be treated as a hidden-element marker.
    let html = r#"<p style="color: red" data-hidden="not-real" aria-hidden="true">visible</p>"#;
    let report = PrescanReport::default();
    let result = tier1::run(html, &report, &base_options()).expect("visible content must not bail");
    assert_eq!(result, "visible\n");
}

#[test]
fn should_match_tier2_output_when_auto_routing_hits_a_hidden_element() {
    // ~keep Before the fix: Tier-1 emitted "secret" verbatim while Tier-2's
    // ~keep `strip_hidden_elements` removed it — a silent byte-equality
    // ~keep violation. After the fix: Tier-1 bails and the Tier-2 fallback
    // ~keep (which strips the hidden element) is returned instead.
    let html = "<p>before</p><div hidden>secret</div><div style=\"display:none\">also secret</div><p>after</p>";

    let tier2_options = ConversionOptions {
        tier_strategy: TierStrategy::Tier2,
        ..base_options()
    };
    let tier2_output = convert(html, Some(tier2_options))
        .expect("tier2 conversion must succeed")
        .content
        .unwrap_or_default();
    assert!(
        !tier2_output.contains("secret"),
        "sanity check: tier2 must strip hidden content, got {tier2_output:?}"
    );

    let auto_options = ConversionOptions {
        tier_strategy: TierStrategy::Auto,
        ..base_options()
    };
    let auto_output = convert(html, Some(auto_options))
        .expect("auto conversion must succeed")
        .content
        .unwrap_or_default();

    assert_eq!(
        auto_output, tier2_output,
        "Auto routing must match Tier-2's hidden-element-stripped output"
    );
}

// ~keep ── E. Markdown-injection label escaping (security fix mirror) ───────────────

#[test]
fn should_escape_unbalanced_bracket_when_svg_title_attempts_markdown_image_injection() {
    let html = r"<svg><title>x](https://evil.example)y</title><rect/></svg>";
    let report = PrescanReport::default();
    let result = tier1::run(html, &report, &base_options()).expect("tier1 scanner should not bail on this input");
    assert!(
        result.starts_with("![x\\](https://evil.example)y]("),
        "expected the SVG title's unbalanced `]` to be escaped, got: {result:?}"
    );
    // ~keep Sanity: the escaped label must not parse as two separate Markdown
    // ~keep images/links — only one `![` opener should appear in the output.
    assert_eq!(result.matches("![").count(), 1, "got: {result:?}");
}

#[test]
fn should_escape_unbalanced_bracket_when_link_text_attempts_markdown_link_injection() {
    let html = r#"<p><a href="/x">a] (https://evil.example) b</a></p>"#;
    let report = PrescanReport::default();
    let result = tier1::run(html, &report, &base_options()).expect("tier1 scanner should not bail on this input");
    assert_eq!(result, "[a\\] (https://evil.example) b](/x)\n");
}

#[test]
fn should_not_escape_when_link_text_brackets_are_already_balanced() {
    // ~keep Regression guard against over-escaping: a balanced `[inner]` inside
    // ~keep the label must pass through untouched (matches CommonMark — nested
    // ~keep balanced brackets are legal inside a link label).
    let html = r#"<p><a href="/x">outer [inner] text</a></p>"#;
    let report = PrescanReport::default();
    let result = tier1::run(html, &report, &base_options()).expect("tier1 scanner should not bail on this input");
    assert_eq!(result, "[outer [inner] text](/x)\n");
}

#[test]
fn should_not_escape_brackets_when_link_has_no_href() {
    // ~keep Tier-2's href-less branch never calls `escape_link_label` — it emits
    // ~keep the text with no bracket wrapping at all. An href-less `<a>` must not
    // ~keep gain a stray backslash escape it would never have received from
    // ~keep Tier-2.
    let html = r"<p><a>a] (https://evil.example) b</a></p>";
    let report = PrescanReport::default();
    let result = tier1::run(html, &report, &base_options()).expect("tier1 scanner should not bail on this input");
    assert_eq!(result, "a] (https://evil.example) b\n");
}

#[test]
fn should_match_tier2_output_when_auto_routing_hits_a_link_injection_attempt() {
    let html = r#"<p><a href="/x">a] (https://evil.example) b</a></p>"#;

    let tier2_options = ConversionOptions {
        tier_strategy: TierStrategy::Tier2,
        ..base_options()
    };
    let tier2_output = convert(html, Some(tier2_options))
        .expect("tier2 conversion must succeed")
        .content
        .unwrap_or_default();

    let auto_options = ConversionOptions {
        tier_strategy: TierStrategy::Auto,
        ..base_options()
    };
    let auto_output = convert(html, Some(auto_options))
        .expect("auto conversion must succeed")
        .content
        .unwrap_or_default();

    assert_eq!(
        auto_output, tier2_output,
        "Auto routing must match Tier-2's escaped-label output"
    );
}

#[test]
fn should_match_tier2_output_when_auto_routing_hits_an_svg_title_injection_attempt() {
    let html = r"<p>before</p><svg><title>x](https://evil.example)y</title><rect/></svg>";

    let tier2_options = ConversionOptions {
        tier_strategy: TierStrategy::Tier2,
        ..base_options()
    };
    let tier2_output = convert(html, Some(tier2_options))
        .expect("tier2 conversion must succeed")
        .content
        .unwrap_or_default();

    let auto_options = ConversionOptions {
        tier_strategy: TierStrategy::Auto,
        ..base_options()
    };
    let auto_output = convert(html, Some(auto_options))
        .expect("auto conversion must succeed")
        .content
        .unwrap_or_default();

    assert_eq!(
        auto_output, tier2_output,
        "Auto routing must match Tier-2's escaped SVG-title output"
    );
}

#[test]
fn should_escape_unbalanced_bracket_when_img_alt_text_attempts_markdown_image_injection() {
    let html = r#"<p><img src="/x.png" alt="x] (https://evil.example) y"></p>"#;
    let report = PrescanReport::default();
    let result = tier1::run(html, &report, &base_options()).expect("tier1 scanner should not bail on this input");
    assert_eq!(result, "![x\\] (https://evil.example) y](/x.png)\n");
}

#[test]
fn should_not_escape_alt_text_when_image_is_stripped_to_alt_only() {
    // ~keep Mirrors `format_image_markdown`'s `use_alt_only` branch (outside
    // ~keep tier1/), which returns `alt.to_string()` with no `escape_link_label`
    // ~keep call at all — there is no `![...]` wrapper for a `]` to prematurely
    // ~keep close, so escaping here would be wrong (a caller would see a stray
    // ~keep backslash Tier-2 never produces). `keep_inline_images_in` must be
    // ~keep non-empty AND exclude the actual heading ancestor (`h1`) for
    // ~keep `should_keep_image_as_markdown` to take the alt-only path — an
    // ~keep empty list (the default) always keeps markdown regardless of
    // ~keep heading ancestry. Requires the `inline-images` feature, which is
    // ~keep the only build where this path exists at all.
    #[cfg(feature = "inline-images")]
    {
        let html = r#"<h1><img src="/x.png" alt="x] (https://evil.example) y"></h1>"#;
        let options = ConversionOptions {
            keep_inline_images_in: vec!["h2".to_string()],
            ..base_options()
        };
        let report = PrescanReport::default();
        let result = tier1::run(html, &report, &options).expect("tier1 scanner should not bail on this input");
        assert_eq!(result, "# x] (https://evil.example) y\n");
    }
}

#[test]
fn should_match_tier2_output_when_auto_routing_hits_an_image_alt_injection_attempt() {
    let html = r#"<p><img src="/x.png" alt="x] (https://evil.example) y"></p>"#;

    let tier2_options = ConversionOptions {
        tier_strategy: TierStrategy::Tier2,
        ..base_options()
    };
    let tier2_output = convert(html, Some(tier2_options))
        .expect("tier2 conversion must succeed")
        .content
        .unwrap_or_default();

    let auto_options = ConversionOptions {
        tier_strategy: TierStrategy::Auto,
        ..base_options()
    };
    let auto_output = convert(html, Some(auto_options))
        .expect("auto conversion must succeed")
        .content
        .unwrap_or_default();

    assert_eq!(
        auto_output, tier2_output,
        "Auto routing must match Tier-2's escaped image-alt output"
    );
}
