# Visitor trait bridge for `HtmlVisitor` — a Crystal object implementing the Rust
# `HtmlVisitor` trait across the C ABI. Layout mirrors the FFI visitor vtable.
require "json"

lib LibHtm
  struct HtmlVisitorContext
    node_type : Int32
    tag_name : LibC::Char*
    depth : LibC::SizeT
    index_in_parent : LibC::SizeT
    parent_tag : LibC::Char*
    is_inline : Int32
  end

  alias HtmlVisitorVisitorHandle = Void*

  struct HtmlVisitorVisitorCallbacks
    user_data : Void*
    visit_text : (HtmlVisitorContext*, Void*, LibC::Char*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_element_start : (HtmlVisitorContext*, Void*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_element_end : (HtmlVisitorContext*, Void*, LibC::Char*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_link : (HtmlVisitorContext*, Void*, LibC::Char*, LibC::Char*, LibC::Char*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_image : (HtmlVisitorContext*, Void*, LibC::Char*, LibC::Char*, LibC::Char*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_heading : (HtmlVisitorContext*, Void*, UInt32, LibC::Char*, LibC::Char*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_code_block : (HtmlVisitorContext*, Void*, LibC::Char*, LibC::Char*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_code_inline : (HtmlVisitorContext*, Void*, LibC::Char*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_list_item : (HtmlVisitorContext*, Void*, Int32, LibC::Char*, LibC::Char*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_list_start : (HtmlVisitorContext*, Void*, Int32, LibC::Char**, LibC::SizeT*) -> Int32
    visit_list_end : (HtmlVisitorContext*, Void*, Int32, LibC::Char*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_table_start : (HtmlVisitorContext*, Void*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_table_end : (HtmlVisitorContext*, Void*, LibC::Char*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_blockquote : (HtmlVisitorContext*, Void*, LibC::Char*, LibC::SizeT, LibC::Char**, LibC::SizeT*) -> Int32
    visit_strong : (HtmlVisitorContext*, Void*, LibC::Char*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_emphasis : (HtmlVisitorContext*, Void*, LibC::Char*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_strikethrough : (HtmlVisitorContext*, Void*, LibC::Char*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_underline : (HtmlVisitorContext*, Void*, LibC::Char*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_subscript : (HtmlVisitorContext*, Void*, LibC::Char*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_superscript : (HtmlVisitorContext*, Void*, LibC::Char*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_mark : (HtmlVisitorContext*, Void*, LibC::Char*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_line_break : (HtmlVisitorContext*, Void*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_horizontal_rule : (HtmlVisitorContext*, Void*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_custom_element : (HtmlVisitorContext*, Void*, LibC::Char*, LibC::Char*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_definition_list_start : (HtmlVisitorContext*, Void*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_definition_term : (HtmlVisitorContext*, Void*, LibC::Char*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_definition_description : (HtmlVisitorContext*, Void*, LibC::Char*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_definition_list_end : (HtmlVisitorContext*, Void*, LibC::Char*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_form : (HtmlVisitorContext*, Void*, LibC::Char*, LibC::Char*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_input : (HtmlVisitorContext*, Void*, LibC::Char*, LibC::Char*, LibC::Char*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_button : (HtmlVisitorContext*, Void*, LibC::Char*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_audio : (HtmlVisitorContext*, Void*, LibC::Char*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_video : (HtmlVisitorContext*, Void*, LibC::Char*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_iframe : (HtmlVisitorContext*, Void*, LibC::Char*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_details : (HtmlVisitorContext*, Void*, Int32, LibC::Char**, LibC::SizeT*) -> Int32
    visit_summary : (HtmlVisitorContext*, Void*, LibC::Char*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_figure_start : (HtmlVisitorContext*, Void*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_figcaption : (HtmlVisitorContext*, Void*, LibC::Char*, LibC::Char**, LibC::SizeT*) -> Int32
    visit_figure_end : (HtmlVisitorContext*, Void*, LibC::Char*, LibC::Char**, LibC::SizeT*) -> Int32
  end

  fun html_visitor_visitor_create = htm_visitor_create(callbacks : HtmlVisitorVisitorCallbacks*) : HtmlVisitorVisitorHandle
  fun html_visitor_visitor_free = htm_visitor_free(handle : HtmlVisitorVisitorHandle) : Void
  fun html_visitor_options_set_visitor = htm_options_set_visitor(options : Void*, handle : HtmlVisitorVisitorHandle) : Void
end

module HtmlToMarkdown

  # Decoded visitor context passed to each callback.
  struct HtmlVisitorVisitorContext
    getter node_type : NodeType?
    getter tag_name : String?
    getter depth : LibC::SizeT
    getter index_in_parent : LibC::SizeT
    getter parent_tag : String?
    getter is_inline : Bool
    def initialize(@node_type, @tag_name, @depth, @index_in_parent, @parent_tag, @is_inline)
    end
  end

  # Subclass and override the methods you care about; each defaults to
  # `VisitResult::Continue`.
  abstract class HtmlVisitorVisitor
    # Visit text nodes (most frequent callback - ~100+ per document).
    def visit_text(ctx : HtmlVisitorVisitorContext, text : String?) : VisitResult
      VisitResult::Continue.new
    end
    # Called before entering any element.
    def visit_element_start(ctx : HtmlVisitorVisitorContext) : VisitResult
      VisitResult::Continue.new
    end
    # Called after exiting any element.
    def visit_element_end(ctx : HtmlVisitorVisitorContext, output : String?) : VisitResult
      VisitResult::Continue.new
    end
    # Visit anchor links `<a href="...">`.
    def visit_link(ctx : HtmlVisitorVisitorContext, href : String?, text : String?, title : String?) : VisitResult
      VisitResult::Continue.new
    end
    # Visit images `<img src="...">`.
    def visit_image(ctx : HtmlVisitorVisitorContext, src : String?, alt : String?, title : String?) : VisitResult
      VisitResult::Continue.new
    end
    # Visit heading elements `<h1>` through `<h6>`.
    def visit_heading(ctx : HtmlVisitorVisitorContext, level : UInt32, text : String?, id : String?) : VisitResult
      VisitResult::Continue.new
    end
    # Visit code blocks `<pre><code>`.
    def visit_code_block(ctx : HtmlVisitorVisitorContext, lang : String?, code : String?) : VisitResult
      VisitResult::Continue.new
    end
    # Visit inline code `<code>`.
    def visit_code_inline(ctx : HtmlVisitorVisitorContext, code : String?) : VisitResult
      VisitResult::Continue.new
    end
    # Visit list items `<li>`.
    def visit_list_item(ctx : HtmlVisitorVisitorContext, ordered : Bool, marker : String?, text : String?) : VisitResult
      VisitResult::Continue.new
    end
    # Called before processing a list `<ul>` or `<ol>`.
    def visit_list_start(ctx : HtmlVisitorVisitorContext, ordered : Bool) : VisitResult
      VisitResult::Continue.new
    end
    # Called after processing a list `</ul>` or `</ol>`.
    def visit_list_end(ctx : HtmlVisitorVisitorContext, ordered : Bool, output : String?) : VisitResult
      VisitResult::Continue.new
    end
    # Called before processing a table `<table>`.
    def visit_table_start(ctx : HtmlVisitorVisitorContext) : VisitResult
      VisitResult::Continue.new
    end
    # Called after processing a table `</table>`.
    def visit_table_end(ctx : HtmlVisitorVisitorContext, output : String?) : VisitResult
      VisitResult::Continue.new
    end
    # Visit blockquote elements `<blockquote>`.
    def visit_blockquote(ctx : HtmlVisitorVisitorContext, content : String?, depth : LibC::SizeT) : VisitResult
      VisitResult::Continue.new
    end
    # Visit strong/bold elements `<strong>`, `<b>`.
    def visit_strong(ctx : HtmlVisitorVisitorContext, text : String?) : VisitResult
      VisitResult::Continue.new
    end
    # Visit emphasis/italic elements `<em>`, `<i>`.
    def visit_emphasis(ctx : HtmlVisitorVisitorContext, text : String?) : VisitResult
      VisitResult::Continue.new
    end
    # Visit strikethrough elements `<s>`, `<del>`, `<strike>`.
    def visit_strikethrough(ctx : HtmlVisitorVisitorContext, text : String?) : VisitResult
      VisitResult::Continue.new
    end
    # Visit underline elements `<u>`, `<ins>`.
    def visit_underline(ctx : HtmlVisitorVisitorContext, text : String?) : VisitResult
      VisitResult::Continue.new
    end
    # Visit subscript elements `<sub>`.
    def visit_subscript(ctx : HtmlVisitorVisitorContext, text : String?) : VisitResult
      VisitResult::Continue.new
    end
    # Visit superscript elements `<sup>`.
    def visit_superscript(ctx : HtmlVisitorVisitorContext, text : String?) : VisitResult
      VisitResult::Continue.new
    end
    # Visit mark/highlight elements `<mark>`.
    def visit_mark(ctx : HtmlVisitorVisitorContext, text : String?) : VisitResult
      VisitResult::Continue.new
    end
    # Visit line break elements `<br>`.
    def visit_line_break(ctx : HtmlVisitorVisitorContext) : VisitResult
      VisitResult::Continue.new
    end
    # Visit horizontal rule elements `<hr>`.
    def visit_horizontal_rule(ctx : HtmlVisitorVisitorContext) : VisitResult
      VisitResult::Continue.new
    end
    # Visit custom elements (web components) or unknown tags.
    def visit_custom_element(ctx : HtmlVisitorVisitorContext, tag_name : String?, html : String?) : VisitResult
      VisitResult::Continue.new
    end
    # Visit definition list `<dl>`.
    def visit_definition_list_start(ctx : HtmlVisitorVisitorContext) : VisitResult
      VisitResult::Continue.new
    end
    # Visit definition term `<dt>`.
    def visit_definition_term(ctx : HtmlVisitorVisitorContext, text : String?) : VisitResult
      VisitResult::Continue.new
    end
    # Visit definition description `<dd>`.
    def visit_definition_description(ctx : HtmlVisitorVisitorContext, text : String?) : VisitResult
      VisitResult::Continue.new
    end
    # Called after processing a definition list `</dl>`.
    def visit_definition_list_end(ctx : HtmlVisitorVisitorContext, output : String?) : VisitResult
      VisitResult::Continue.new
    end
    # Visit form elements `<form>`.
    def visit_form(ctx : HtmlVisitorVisitorContext, action : String?, method : String?) : VisitResult
      VisitResult::Continue.new
    end
    # Visit input elements `<input>`.
    def visit_input(ctx : HtmlVisitorVisitorContext, input_type : String?, name : String?, value : String?) : VisitResult
      VisitResult::Continue.new
    end
    # Visit button elements `<button>`.
    def visit_button(ctx : HtmlVisitorVisitorContext, text : String?) : VisitResult
      VisitResult::Continue.new
    end
    # Visit audio elements `<audio>`.
    def visit_audio(ctx : HtmlVisitorVisitorContext, src : String?) : VisitResult
      VisitResult::Continue.new
    end
    # Visit video elements `<video>`.
    def visit_video(ctx : HtmlVisitorVisitorContext, src : String?) : VisitResult
      VisitResult::Continue.new
    end
    # Visit iframe elements `<iframe>`.
    def visit_iframe(ctx : HtmlVisitorVisitorContext, src : String?) : VisitResult
      VisitResult::Continue.new
    end
    # Visit details elements `<details>`.
    def visit_details(ctx : HtmlVisitorVisitorContext, open : Bool) : VisitResult
      VisitResult::Continue.new
    end
    # Visit summary elements `<summary>`.
    def visit_summary(ctx : HtmlVisitorVisitorContext, text : String?) : VisitResult
      VisitResult::Continue.new
    end
    # Visit figure elements `<figure>`.
    def visit_figure_start(ctx : HtmlVisitorVisitorContext) : VisitResult
      VisitResult::Continue.new
    end
    # Visit figcaption elements `<figcaption>`.
    def visit_figcaption(ctx : HtmlVisitorVisitorContext, text : String?) : VisitResult
      VisitResult::Continue.new
    end
    # Called after processing a figure `</figure>`.
    def visit_figure_end(ctx : HtmlVisitorVisitorContext, output : String?) : VisitResult
      VisitResult::Continue.new
    end
  end

  # Register a Crystal visitor and return an opaque handle. Attach it to a
  # conversion via `LibHtm.html_visitor_options_set_visitor(opts, handle)`,
  # run the conversion, then release it with `free_html_visitor_visitor`.
  def self.register_html_visitor_visitor(impl : HtmlVisitorVisitor) : LibHtm::HtmlVisitorVisitorHandle
    boxed = Box.box(impl)
    callbacks = LibHtm::HtmlVisitorVisitorCallbacks.new
    callbacks.user_data = boxed
    callbacks.visit_text = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, text : LibC::Char*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        text_value = text.null? ? nil : String.new(text)
        decision = visitor.visit_text(context, text_value)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_element_start = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        decision = visitor.visit_element_start(context)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_element_end = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, output : LibC::Char*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        output_value = output.null? ? nil : String.new(output)
        decision = visitor.visit_element_end(context, output_value)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_link = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, href : LibC::Char*, text : LibC::Char*, title : LibC::Char*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        href_value = href.null? ? nil : String.new(href)
        text_value = text.null? ? nil : String.new(text)
        title_value = title.null? ? nil : String.new(title)
        decision = visitor.visit_link(context, href_value, text_value, title_value)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_image = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, src : LibC::Char*, alt : LibC::Char*, title : LibC::Char*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        src_value = src.null? ? nil : String.new(src)
        alt_value = alt.null? ? nil : String.new(alt)
        title_value = title.null? ? nil : String.new(title)
        decision = visitor.visit_image(context, src_value, alt_value, title_value)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_heading = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, level : UInt32, text : LibC::Char*, id : LibC::Char*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        level_value = level
        text_value = text.null? ? nil : String.new(text)
        id_value = id.null? ? nil : String.new(id)
        decision = visitor.visit_heading(context, level_value, text_value, id_value)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_code_block = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, lang : LibC::Char*, code : LibC::Char*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        lang_value = lang.null? ? nil : String.new(lang)
        code_value = code.null? ? nil : String.new(code)
        decision = visitor.visit_code_block(context, lang_value, code_value)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_code_inline = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, code : LibC::Char*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        code_value = code.null? ? nil : String.new(code)
        decision = visitor.visit_code_inline(context, code_value)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_list_item = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, ordered : Int32, marker : LibC::Char*, text : LibC::Char*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        ordered_value = ordered != 0
        marker_value = marker.null? ? nil : String.new(marker)
        text_value = text.null? ? nil : String.new(text)
        decision = visitor.visit_list_item(context, ordered_value, marker_value, text_value)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_list_start = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, ordered : Int32, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        ordered_value = ordered != 0
        decision = visitor.visit_list_start(context, ordered_value)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_list_end = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, ordered : Int32, output : LibC::Char*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        ordered_value = ordered != 0
        output_value = output.null? ? nil : String.new(output)
        decision = visitor.visit_list_end(context, ordered_value, output_value)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_table_start = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        decision = visitor.visit_table_start(context)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_table_end = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, output : LibC::Char*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        output_value = output.null? ? nil : String.new(output)
        decision = visitor.visit_table_end(context, output_value)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_blockquote = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, content : LibC::Char*, depth : LibC::SizeT, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        content_value = content.null? ? nil : String.new(content)
        depth_value = depth
        decision = visitor.visit_blockquote(context, content_value, depth_value)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_strong = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, text : LibC::Char*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        text_value = text.null? ? nil : String.new(text)
        decision = visitor.visit_strong(context, text_value)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_emphasis = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, text : LibC::Char*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        text_value = text.null? ? nil : String.new(text)
        decision = visitor.visit_emphasis(context, text_value)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_strikethrough = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, text : LibC::Char*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        text_value = text.null? ? nil : String.new(text)
        decision = visitor.visit_strikethrough(context, text_value)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_underline = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, text : LibC::Char*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        text_value = text.null? ? nil : String.new(text)
        decision = visitor.visit_underline(context, text_value)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_subscript = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, text : LibC::Char*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        text_value = text.null? ? nil : String.new(text)
        decision = visitor.visit_subscript(context, text_value)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_superscript = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, text : LibC::Char*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        text_value = text.null? ? nil : String.new(text)
        decision = visitor.visit_superscript(context, text_value)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_mark = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, text : LibC::Char*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        text_value = text.null? ? nil : String.new(text)
        decision = visitor.visit_mark(context, text_value)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_line_break = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        decision = visitor.visit_line_break(context)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_horizontal_rule = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        decision = visitor.visit_horizontal_rule(context)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_custom_element = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, tag_name : LibC::Char*, html : LibC::Char*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        tag_name_value = tag_name.null? ? nil : String.new(tag_name)
        html_value = html.null? ? nil : String.new(html)
        decision = visitor.visit_custom_element(context, tag_name_value, html_value)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_definition_list_start = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        decision = visitor.visit_definition_list_start(context)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_definition_term = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, text : LibC::Char*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        text_value = text.null? ? nil : String.new(text)
        decision = visitor.visit_definition_term(context, text_value)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_definition_description = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, text : LibC::Char*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        text_value = text.null? ? nil : String.new(text)
        decision = visitor.visit_definition_description(context, text_value)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_definition_list_end = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, output : LibC::Char*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        output_value = output.null? ? nil : String.new(output)
        decision = visitor.visit_definition_list_end(context, output_value)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_form = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, action : LibC::Char*, method : LibC::Char*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        action_value = action.null? ? nil : String.new(action)
        method_value = method.null? ? nil : String.new(method)
        decision = visitor.visit_form(context, action_value, method_value)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_input = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, input_type : LibC::Char*, name : LibC::Char*, value : LibC::Char*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        input_type_value = input_type.null? ? nil : String.new(input_type)
        name_value = name.null? ? nil : String.new(name)
        value_value = value.null? ? nil : String.new(value)
        decision = visitor.visit_input(context, input_type_value, name_value, value_value)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_button = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, text : LibC::Char*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        text_value = text.null? ? nil : String.new(text)
        decision = visitor.visit_button(context, text_value)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_audio = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, src : LibC::Char*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        src_value = src.null? ? nil : String.new(src)
        decision = visitor.visit_audio(context, src_value)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_video = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, src : LibC::Char*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        src_value = src.null? ? nil : String.new(src)
        decision = visitor.visit_video(context, src_value)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_iframe = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, src : LibC::Char*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        src_value = src.null? ? nil : String.new(src)
        decision = visitor.visit_iframe(context, src_value)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_details = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, open : Int32, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        open_value = open != 0
        decision = visitor.visit_details(context, open_value)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_summary = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, text : LibC::Char*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        text_value = text.null? ? nil : String.new(text)
        decision = visitor.visit_summary(context, text_value)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_figure_start = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        decision = visitor.visit_figure_start(context)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_figcaption = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, text : LibC::Char*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        text_value = text.null? ? nil : String.new(text)
        decision = visitor.visit_figcaption(context, text_value)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    callbacks.visit_figure_end = ->(ctx : LibHtm::HtmlVisitorContext*, user_data : Void*, output : LibC::Char*, out_custom : LibC::Char**, out_len : LibC::SizeT*) do
      begin
        visitor = Box(HtmlVisitorVisitor).unbox(user_data)
        raw = ctx.value
        context = HtmlVisitorVisitorContext.new(NodeType.from_value?(raw.node_type) || NodeType::Text, raw.tag_name.null? ? nil : String.new(raw.tag_name), raw.depth, raw.index_in_parent, raw.parent_tag.null? ? nil : String.new(raw.parent_tag), raw.is_inline != 0)
        output_value = output.null? ? nil : String.new(output)
        decision = visitor.visit_figure_end(context, output_value)
        case decision
        when VisitResult::Continue then 0
        when VisitResult::Skip then 2
        when VisitResult::PreserveHtml then 3
        when VisitResult::Custom
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          1
        when VisitResult::Error
          __payload = decision.value.to_slice
          __buf = LibC.malloc(__payload.size + 1).as(UInt8*)
          __buf.copy_from(__payload.to_unsafe, __payload.size)
          __buf[__payload.size] = 0_u8
          out_custom.value = __buf
          out_len.value = LibC::SizeT.new(__payload.size)
          4
        else 0
        end
      rescue e
        STDERR.puts "[visitor callback error] #{e}"
        STDERR.puts "[visitor callback backtrace] #{e.backtrace.first(3).join("\n")}" if e.backtrace
        out_custom.value = Pointer(LibC::Char).null
        out_len.value = LibC::SizeT.new(0)
        0
      end
    end
    LibHtm.html_visitor_visitor_create(pointerof(callbacks))
  end

  # Release a visitor handle returned by `register_html_visitor_visitor`.
  def self.free_html_visitor_visitor(handle : LibHtm::HtmlVisitorVisitorHandle) : Nil
    LibHtm.html_visitor_visitor_free(handle)
    nil
  end
end
