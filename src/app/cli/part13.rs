use super::*;
#[derive(Clone, Copy, Debug, ValueEnum)]
pub(in crate::app) enum DocTemplateKind {
    All,
    SupportMatrix,
    TextChild,
    HeadingChild,
    BulletChild,
    OrderedChild,
    TodoChild,
    QuoteChild,
    CodeChild,
    MermaidCodeChild,
    DividerChild,
    ImageChild,
    FileChild,
    SheetChild,
    BitableChild,
    IframeChild,
    ChatCardChild,
    IsvChild,
    AddOnsChild,
    JiraIssueChild,
    BoardChild,
    LinkPreviewChild,
    SubPageListChild,
    WikiCatalogChild,
    TableDescendant,
    GridDescendant,
    CalloutDescendant,
    QuoteContainerDescendant,
    AgendaDescendant,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(in crate::app) enum BoardSyntaxArg {
    Mermaid,
    Plantuml,
}
