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
