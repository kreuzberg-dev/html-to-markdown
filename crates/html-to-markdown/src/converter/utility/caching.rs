//! Performance caching utilities.
//!
//! Caching mechanisms for expensive operations during conversion, including
//! DOM context building.

use crate::converter::DomContext;

/// Build a DOM context with hierarchical node information.
///
/// Pre-computes parent-child relationships, sibling indices, and caches
/// tag information for efficient DOM navigation during conversion.
#[must_use]
pub fn build_dom_context(dom: &tl::VDom, parser: &tl::Parser, _input_len: usize) -> DomContext {
    let mut ctx = DomContext {
        parent_map: Vec::new(),
        children_map: Vec::new(),
        sibling_index_map: Vec::new(),
        root_children: dom.children().to_vec(),
        node_map: Vec::new(),
        tag_info_map: Vec::new(),
        prev_inline_like_map: Vec::new(),
        next_inline_like_map: Vec::new(),
        next_tag_map: Vec::new(),
        next_whitespace_map: Vec::new(),
        table_content_summary_cache: std::cell::RefCell::new(std::collections::HashMap::new()),
    };

    // ~keep Node ids index `dom.nodes()`, so the arena length is an exact upper bound on them.
    // ~keep Sizing every map once here replaces one incremental grow per map per newly-seen id
    // ~keep (nine maps x one grow per node) with nine allocations for the whole document.
    if let Some(highest_node_id) = dom.nodes().len().checked_sub(1).and_then(|id| u32::try_from(id).ok()) {
        ctx.ensure_capacity(highest_node_id);
    }

    for (index, child_handle) in dom.children().iter().enumerate() {
        let id = child_handle.get_inner();
        ctx.ensure_capacity(id);
        ctx.sibling_index_map[id as usize] = Some(index);
        record_node_hierarchy(*child_handle, None, parser, &mut ctx);
    }

    ctx
}

/// Record node hierarchy into DOM context.
///
/// Builds the complete parent-child relationship map for efficient tree traversal.
pub fn record_node_hierarchy(
    node_handle: tl::NodeHandle,
    parent: Option<u32>,
    parser: &tl::Parser,
    ctx: &mut DomContext,
) {
    let mut work = vec![(node_handle, parent)];
    while let Some((node_handle, parent)) = work.pop() {
        let id = node_handle.get_inner();
        ctx.ensure_capacity(id);
        ctx.parent_map[id as usize] = parent;
        ctx.node_map[id as usize] = Some(node_handle);

        if let Some(tl::Node::Tag(tag)) = node_handle.get(parser) {
            let tag_children = tag.children();
            // ~keep `RawChildren::iter` reports no size hint, so collecting through it regrows the
            // ~keep Vec from empty (and over-allocates to the growth step). Copying the slice sizes
            // ~keep the allocation exactly once; the Vec itself must be owned because `children_map`
            // ~keep outlives this borrow of the parser.
            let children = tag_children.top().as_slice();
            for (index, child) in children.iter().enumerate() {
                let child_id = child.get_inner();
                ctx.ensure_capacity(child_id);
                ctx.sibling_index_map[child_id as usize] = Some(index);
                work.push((*child, Some(id)));
            }
            ctx.children_map[id as usize] = Some(children.to_vec());
        }
    }
}
