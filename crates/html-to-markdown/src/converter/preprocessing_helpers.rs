//! HTML preprocessing and validation helpers.
//!
//! This module contains helper functions for preprocessing HTML before conversion,
//! including validation and normalization checks.

use crate::converter::dom_context::DomContext;
use crate::converter::main_helpers::is_inline_element;
use crate::converter::utility::attributes::{attribute_matches_any, element_has_navigation_hint};
use crate::options::ConversionOptions;

/// Check if an inline ancestor element is allowed to contain block-level elements.
pub fn inline_ancestor_allows_block(tag_name: &str) -> bool {
    matches!(tag_name, "a" | "ins" | "del")
}

/// Ancestor state inherited top-down while scanning for misnested elements.
///
/// ~keep Each field mirrors one of the three independent ancestor-chain scans the
/// ~keep original implementation ran separately per node (O(depth) each); carrying
/// ~keep the already-computed parent result down to children makes every node O(1)
/// ~keep instead, since each scan only ever needs the *nearest* qualifying ancestor.
#[derive(Clone, Copy)]
struct MisnestState {
    /// True if this node or any strict ancestor is `<pre>`/`<code>`.
    inside_preformatted: bool,
    /// True if any strict ancestor is an inline element that disallows block children.
    blocked_by_inline_ancestor: bool,
    /// Result of a `has_p_ancestor` scan started at this node (used by its children).
    p_ancestor_state: bool,
}

impl MisnestState {
    const ROOT: Self = Self {
        inside_preformatted: false,
        blocked_by_inline_ancestor: false,
        p_ancestor_state: false,
    };
}

/// Detect block elements that were incorrectly nested under inline ancestors.
///
/// Excludes elements inside `<pre>` or `<code>` blocks, as they have special
/// whitespace preservation rules and should not be repaired.
///
/// Also detects table structural elements (`td`, `tr`, `th`) nested under `<p>` —
/// a structural impossibility in valid HTML that signals the `tl` parser absorbed
/// a table into a paragraph because of an unclosed `<p>` (common in Word/Outlook
/// HTML such as `<p class='MsoNormal'>` cells). Issue #336.
///
/// ~keep Walks the tree top-down exactly once, carrying inherited ancestor state
/// ~keep (see [`MisnestState`]) instead of re-walking every node's ancestor chain.
/// ~keep The original per-node ancestor walk was O(depth) per node — O(n²) total on
/// ~keep a deeply nested chain (e.g. 20k nested `<div>`s took ~30s; this pass alone
/// ~keep accounted for essentially all of it, confirmed via phase timing in
/// ~keep `tools/benchmark-harness/examples/profile_deep_nesting_phases.rs`).
pub fn has_inline_block_misnest(dom_ctx: &DomContext, parser: &tl::Parser) -> bool {
    let mut stack: Vec<(tl::NodeHandle, MisnestState)> = dom_ctx
        .root_children
        .iter()
        .map(|handle| (*handle, MisnestState::ROOT))
        .collect();

    while let Some((handle, state)) = stack.pop() {
        let node_id = handle.get_inner();
        if !matches!(handle.get(parser), Some(tl::Node::Tag(_))) {
            continue;
        }
        let Some(info) = dom_ctx.tag_info(node_id, parser) else {
            continue;
        };

        // ~keep Table elements under <p>: tl misparsed an unclosed <p> in <td>.
        if matches!(info.name.as_str(), "td" | "tr" | "th") && state.p_ancestor_state {
            return true;
        }

        let self_inside_preformatted = state.inside_preformatted || matches!(info.name.as_str(), "pre" | "code");
        if info.is_block && !self_inside_preformatted && state.blocked_by_inline_ancestor {
            return true;
        }

        if let Some(children) = dom_ctx.children_of(node_id) {
            let child_state = MisnestState {
                inside_preformatted: self_inside_preformatted,
                blocked_by_inline_ancestor: state.blocked_by_inline_ancestor
                    || (is_inline_element(&info.name) && !inline_ancestor_allows_block(&info.name)),
                p_ancestor_state: p_ancestor_state_for(&info.name, state.p_ancestor_state),
            };
            stack.extend(children.iter().map(|child| (*child, child_state)));
        }
    }

    false
}

/// Compute the `has_p_ancestor` scan result to hand down to a node's children,
/// given the node's own tag name and the state its own parent handed down.
///
/// Mirrors the original ancestor walk's stopping rule: a `<p>` ancestor found
/// before crossing a `table`/`body`/`html` boundary counts; hitting the boundary
/// first resets the search to "no `<p>` ancestor".
fn p_ancestor_state_for(tag_name: &str, inherited: bool) -> bool {
    if tag_name == "p" {
        true
    } else if matches!(tag_name, "table" | "body" | "html") {
        false
    } else {
        inherited
    }
}

/// Determine if a node should be dropped during preprocessing.
///
/// Behavior depends on the [`PreprocessingPreset`]:
///
/// - **Minimal**: Only scripts/styles are stripped (handled elsewhere). This function
///   drops nothing — all structural elements are preserved.
/// - **Standard** (default): Drops `<nav>` unconditionally. Drops `<header>`, `<footer>`,
///   and `<aside>` only when they have navigation hints (class/role/aria attributes
///   indicating site chrome). Drops `<form>` when `remove_forms` is enabled.
/// - **Aggressive**: All of Standard, plus: drops `<footer>`, `<aside>`, `<noscript>`
///   unconditionally. Drops ANY element with navigation hints in class/id/role
///   (e.g. `<div class="sidebar">`). Drops elements with noise-related classes/roles.
pub fn should_drop_for_preprocessing(tag_name: &str, tag: &tl::HTMLTag, options: &ConversionOptions) -> bool {
    use crate::options::PreprocessingPreset;

    if !options.preprocessing.enabled {
        return false;
    }

    let preset = options.preprocessing.preset;

    if preset == PreprocessingPreset::Minimal {
        return false;
    }

    if options.preprocessing.remove_forms && tag_name == "form" {
        return true;
    }

    let is_aggressive = preset == PreprocessingPreset::Aggressive;

    // ~keep Aggressive: drop <noscript> — its content is fallback for no-JS browsers.
    if is_aggressive && tag_name == "noscript" {
        return true;
    }

    if !options.preprocessing.remove_navigation {
        return false;
    }

    let has_nav_hint = element_has_navigation_hint(tag);

    if tag_name == "nav" {
        return true;
    }

    if tag_name == "header" {
        return has_nav_hint;
    }

    if tag_name == "footer" || tag_name == "aside" {
        return is_aggressive || has_nav_hint;
    }

    if is_aggressive && has_nav_hint {
        return true;
    }

    if is_aggressive {
        if element_has_noise_hint(tag) {
            return true;
        }
    }

    false
}

/// Check if an element has noise-related hints (ads, cookie banners, social sharing).
fn element_has_noise_hint(tag: &tl::HTMLTag) -> bool {
    const NOISE_KEYWORDS: &[&str] = &[
        "cookie",
        "consent",
        "gdpr",
        "banner",
        "advertisement",
        "ad-container",
        "advert",
        "social-share",
        "share-buttons",
        "popup",
        "modal-overlay",
        "newsletter-signup",
    ];

    attribute_matches_any(tag, "class", NOISE_KEYWORDS) || attribute_matches_any(tag, "id", NOISE_KEYWORDS)
}
