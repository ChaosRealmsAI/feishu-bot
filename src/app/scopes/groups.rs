pub(in crate::app) const OFFICE_SCOPE_GROUPS: &[&str] =
    &["im", "doc", "wiki", "base", "permission", "search"];

pub(in crate::app) type ScopeGroups = Vec<(&'static str, Vec<&'static str>)>;

#[derive(Clone, Copy)]
pub(in crate::app) struct ScopeGroup {
    pub(in crate::app) name: &'static str,
    pub(in crate::app) scopes: &'static [&'static str],
}

mod content;
mod identity;
mod work;

pub(super) const ALL_SCOPE_GROUPS: &[ScopeGroup] = &[
    identity::USER_TOKEN,
    content::IM,
    identity::CONTACT,
    identity::DIRECTORY,
    content::DOC,
    content::BOARD,
    work::BASE,
    work::TASK,
    content::DRIVE,
    content::PERMISSION,
    work::CALENDAR,
    work::VC,
    work::MINUTES,
    work::SEARCH,
    work::OKR,
    work::ATTENDANCE,
    work::MAIL,
    identity::COREHR,
    work::HELPDESK,
    identity::HIRE,
    content::WIKI,
    content::SHEET,
    work::APPROVAL,
];

pub(super) fn all_scope_groups() -> &'static [ScopeGroup] {
    ALL_SCOPE_GROUPS
}
