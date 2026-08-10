from typing import cast

from html_to_markdown import convert
from html_to_markdown.api import HtmlVisitor


class LinkAndImageVisitor:
    """Exercise documented visitor return actions."""

    def visit_link(self, _context: object, href: str, text: str, _title: str | None) -> dict[str, str]:
        """Replace a link with custom output."""
        rewritten_href = href.replace("https://old-cdn.example", "https://new-cdn.example")
        return {"type": "custom", "output": f"[{text}]({rewritten_href})"}

    def visit_image(self, _context: object, src: str, _alt: str, _title: str | None) -> dict[str, str]:
        """Skip tracking images and continue normal images."""
        return {"type": "skip"} if "tracking" in src else {"type": "continue"}


def test_visitor_honors_tagged_link_and_image_actions() -> None:
    html = """
    <a href="https://old-cdn.example/file.pdf">Download</a>
    <img src="https://assets.example/tracking.gif" alt="Tracking">
    <img src="https://assets.example/photo.jpg" alt="Photo">
    """

    result = convert(html, visitor=cast("HtmlVisitor", LinkAndImageVisitor()))

    assert result.content == "[Download](https://new-cdn.example/file.pdf) ![Photo](https://assets.example/photo.jpg)\n"
