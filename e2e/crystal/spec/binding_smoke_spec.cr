require "./spec_helper"

describe HtmlToMarkdown do
  it "links the generated binding" do
    HtmlToMarkdown::VERSION.should_not be_empty
  end
end
