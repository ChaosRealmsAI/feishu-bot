use super::*;

mod child;
mod descendant;
mod support;

use child::child_template;
use descendant::descendant_template;
use support::doc_support_matrix;

pub(in crate::app) fn doc_template(kind: DocTemplateKind) -> Value {
    match kind {
        DocTemplateKind::All => all_templates(),
        DocTemplateKind::SupportMatrix => doc_support_matrix(),
        DocTemplateKind::TextChild
        | DocTemplateKind::HeadingChild
        | DocTemplateKind::BulletChild
        | DocTemplateKind::OrderedChild
        | DocTemplateKind::TodoChild
        | DocTemplateKind::QuoteChild
        | DocTemplateKind::CodeChild
        | DocTemplateKind::MermaidCodeChild
        | DocTemplateKind::DividerChild
        | DocTemplateKind::ImageChild
        | DocTemplateKind::FileChild
        | DocTemplateKind::SheetChild
        | DocTemplateKind::BitableChild
        | DocTemplateKind::IframeChild
        | DocTemplateKind::ChatCardChild
        | DocTemplateKind::IsvChild
        | DocTemplateKind::AddOnsChild
        | DocTemplateKind::JiraIssueChild
        | DocTemplateKind::BoardChild
        | DocTemplateKind::LinkPreviewChild
        | DocTemplateKind::SubPageListChild
        | DocTemplateKind::WikiCatalogChild => child_template(kind),
        DocTemplateKind::TableDescendant
        | DocTemplateKind::GridDescendant
        | DocTemplateKind::CalloutDescendant
        | DocTemplateKind::QuoteContainerDescendant
        | DocTemplateKind::AgendaDescendant => descendant_template(kind),
    }
}

fn all_templates() -> Value {
    json!({
        "support-matrix": doc_template(DocTemplateKind::SupportMatrix),
        "text-child": doc_template(DocTemplateKind::TextChild),
        "heading-child": doc_template(DocTemplateKind::HeadingChild),
        "bullet-child": doc_template(DocTemplateKind::BulletChild),
        "ordered-child": doc_template(DocTemplateKind::OrderedChild),
        "todo-child": doc_template(DocTemplateKind::TodoChild),
        "quote-child": doc_template(DocTemplateKind::QuoteChild),
        "code-child": doc_template(DocTemplateKind::CodeChild),
        "mermaid-code-child": doc_template(DocTemplateKind::MermaidCodeChild),
        "divider-child": doc_template(DocTemplateKind::DividerChild),
        "image-child": doc_template(DocTemplateKind::ImageChild),
        "file-child": doc_template(DocTemplateKind::FileChild),
        "sheet-child": doc_template(DocTemplateKind::SheetChild),
        "bitable-child": doc_template(DocTemplateKind::BitableChild),
        "iframe-child": doc_template(DocTemplateKind::IframeChild),
        "chat-card-child": doc_template(DocTemplateKind::ChatCardChild),
        "isv-child": doc_template(DocTemplateKind::IsvChild),
        "add-ons-child": doc_template(DocTemplateKind::AddOnsChild),
        "jira-issue-child": doc_template(DocTemplateKind::JiraIssueChild),
        "board-child": doc_template(DocTemplateKind::BoardChild),
        "link-preview-child": doc_template(DocTemplateKind::LinkPreviewChild),
        "sub-page-list-child": doc_template(DocTemplateKind::SubPageListChild),
        "wiki-catalog-child": doc_template(DocTemplateKind::WikiCatalogChild),
        "table-descendant": doc_template(DocTemplateKind::TableDescendant),
        "grid-descendant": doc_template(DocTemplateKind::GridDescendant),
        "callout-descendant": doc_template(DocTemplateKind::CalloutDescendant),
        "quote-container-descendant": doc_template(DocTemplateKind::QuoteContainerDescendant),
        "agenda-descendant": doc_template(DocTemplateKind::AgendaDescendant),
    })
}
