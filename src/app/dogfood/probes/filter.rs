use super::*;

const OFFICE_DOGFOOD_MODULES: &[&str] = &[
    "auth", "bot", "message", "contact", "drive", "calendar", "task", "wiki", "search", "minutes",
    "doc", "base",
];

const ENTERPRISE_DOGFOOD_MODULES: &[&str] = &[
    "okr",
    "attendance",
    "vc",
    "hire",
    "corehr",
    "mail",
    "helpdesk",
];

pub(in crate::app) fn dogfood_verify_module_filters(args: &DogfoodVerifyArgs) -> Vec<String> {
    let mut filters = match args.profile {
        Some(DogfoodVerifyProfileArg::Office) => OFFICE_DOGFOOD_MODULES
            .iter()
            .map(|module| (*module).to_string())
            .collect::<Vec<_>>(),
        Some(DogfoodVerifyProfileArg::Enterprise) => ENTERPRISE_DOGFOOD_MODULES
            .iter()
            .map(|module| (*module).to_string())
            .collect::<Vec<_>>(),
        Some(DogfoodVerifyProfileArg::All) | None => Vec::new(),
    };

    for module in &args.module {
        let module = module.trim();
        if module.is_empty() {
            continue;
        }
        if !filters.iter().any(|filter| filter == module) {
            filters.push(module.to_string());
        }
    }

    filters
}

pub(in crate::app) fn dogfood_module_selected(
    filters: &[String],
    module: &str,
    name: &str,
) -> bool {
    if filters.is_empty() {
        return true;
    }
    let module = module.to_ascii_lowercase();
    let name = name.to_ascii_lowercase();
    filters.iter().any(|filter| {
        let filter = filter.trim().to_ascii_lowercase();
        !filter.is_empty() && (module == filter || name == filter || name.starts_with(&filter))
    })
}
