//! Handler for line break elements (br).
//!
//! Converts HTML line break tags to Markdown line breaks using the configured
//! newline style (spaces, backslash, or plain newline).

use crate::converter::main_helpers::{emit_table_cell_break, trim_trailing_whitespace};
use crate::options::{ConversionOptions, NewlineStyle};
#[cfg(feature = "visitor")]
use std::borrow::Cow;
use tl::{NodeHandle, Parser};

type Context = crate::converter::Context;
type DomContext = crate::converter::DomContext;

/// Handle line break elements (br).
///
/// Converts to appropriate Markdown line break syntax based on the configured
/// newline style and current context (e.g., in headings).
#[cfg_attr(not(feature = "visitor"), allow(unused_variables))]
pub fn handle(
    node_handle: &NodeHandle,
    parser: &Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    depth: usize,
    dom_ctx: &DomContext,
) {
    #[cfg(feature = "visitor")]
    if let Some(ref visitor_handle) = ctx.visitor {
        use crate::visitor::EMPTY_ATTRS;
        use crate::visitor::{NodeContext, NodeType, VisitResult};

        let node_id = node_handle.get_inner();
        let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
        let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);
        let node_ctx = if let Some(tl::Node::Tag(t)) = node_handle.get(parser) {
            NodeContext::with_lazy_attributes(
                NodeType::Br,
                Cow::Borrowed("br"),
                t,
                depth,
                index_in_parent,
                parent_tag.map(Cow::Borrowed),
                true,
            )
        } else {
            NodeContext::with_borrowed_attributes(
                NodeType::Br,
                Cow::Borrowed("br"),
                &EMPTY_ATTRS,
                depth,
                index_in_parent,
                parent_tag.map(Cow::Borrowed),
                true,
            )
        };
        let visit_result = {
            let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
            visitor.visit_line_break(&node_ctx)
        };
        match visit_result {
            VisitResult::Continue => {}
            VisitResult::Skip => return,
            VisitResult::Custom(custom) => {
                output.push_str(&custom);
                return;
            }
            VisitResult::PreserveHtml => {
                use crate::converter::utility::serialization::serialize_node;
                output.push_str(&serialize_node(node_handle, parser));
                return;
            }
            VisitResult::Error(err) => {
                if ctx.visitor_error.borrow().is_none() {
                    *ctx.visitor_error.borrow_mut() = Some(err);
                }
                return;
            }
        }
    }

    if ctx.in_heading {
        // ~keep A single-line ATX heading cannot carry a hard break at all, so any marker
        // ~keep here is inherently lossy. A single space is the only choice that is
        // ~keep round-trip stable: a renderer's own HTML whitespace collapsing already
        // ~keep reduces a run of literal spaces to one on the next parse, so a
        // ~keep two-space marker here only survives the FIRST conversion before
        // ~keep collapsing on the second, and never reaches a fixpoint. Matches
        // ~keep Tier-1: `tier1/scanner.rs`'s `TagKind::LineBreak` handling emits the same
        // ~keep "  \n" marker regardless of heading context, but `close_heading` then
        // ~keep folds every whitespace run (including that marker) in the finished
        // ~keep heading body down to one space.
        trim_trailing_whitespace(output);
        output.push(' ');
    } else if ctx.in_code {
        // ~keep A code span reproduces its content literally, so a newline_style marker is
        // ~keep not syntax here -- it is a character in the user's code. `CommonMark` gives a
        // ~keep line ending inside a code span no hard-break meaning and renders it as a
        // ~keep space (<https://spec.commonmark.org/spec#code-spans>), and inside a fenced
        // ~keep block the marker would land in the code itself. Same reasoning as the
        // ~keep table-cell branch below: the context cannot carry a hard break, so
        // ~keep newline_style is never consulted and both styles agree byte for byte.
        output.push('\n');
    } else if ctx.in_table_cell {
        // ~keep Shared with div/p continuations inside a cell (issue #453, #454): a cell
        // ~keep cannot contain a hard line break, so newline_style is never consulted and
        // ~keep source whitespace before the <br> is trimmed rather than leaked.
        emit_table_cell_break(output, options.br_in_tables);
    } else if output.len() == ctx.block_content_start {
        // ~keep A <br> with nothing before it on the current line has no prior line to
        // ~keep break: emitting a style marker here would leave a leading artifact instead
        // ~keep of being invisible. A leading run of <br> therefore collapses to no output
        // ~keep at all (issue #464), rather than the previous "swallow every break after the
        // ~keep first" check (`output.ends_with('\n')`), which also matched — and silently
        // ~keep collapsed — a run of *consecutive* breaks with real content before them.
        // ~keep Unguarded by `ctx.in_paragraph` (unlike `text_node.rs`'s identical-looking
        // ~keep check): a bare top-level <br> with no enclosing paragraph/div must also
        // ~keep no-op here, matching Tier-1's explicit "bare <br> at top level emits
        // ~keep nothing" contract (`tier1/scanner.rs`'s `TagKind::LineBreak` arm) — the
        // ~keep default `block_content_start: 0` from a fresh `Context` still equals
        // ~keep `output.len()` at true document start, so this stays correct there.
        //
        // ~keep The bare `\n` (rather than no output at all) is load-bearing and predates
        // ~keep #464: `integration_test.rs::test_breaks_and_newlines_issue_112` pins that a
        // ~keep leading top-level `<br>` still opens a line. Only the CONDITION changed for
        // ~keep #464 -- the old `output.ends_with('\n')` also matched a break that followed
        // ~keep another break's marker, which is what swallowed consecutive runs.
        output.push('\n');
    } else {
        match options.newline_style {
            NewlineStyle::Spaces => output.push_str("  \n"),
            NewlineStyle::Backslash => output.push_str("\\\n"),
        }
    }
}
