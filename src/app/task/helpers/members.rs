use super::*;

pub(super) fn task_members(ids: Vec<String>, role: &str) -> Vec<Value> {
    task_members_typed(ids, role, "user")
}

pub(super) fn task_members_typed(ids: Vec<String>, role: &str, member_type: &str) -> Vec<Value> {
    ids.into_iter()
        .map(|id| {
            json!({
                "type": member_type,
                "id": id,
                "role": role,
            })
        })
        .collect()
}
