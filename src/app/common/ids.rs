use super::*;

impl ReceiveIdTypeArg {
    pub(in crate::app) fn resolve(self, id: &str) -> &'static str {
        match self {
            ReceiveIdTypeArg::OpenId => "open_id",
            ReceiveIdTypeArg::UnionId => "union_id",
            ReceiveIdTypeArg::UserId => "user_id",
            ReceiveIdTypeArg::Email => "email",
            ReceiveIdTypeArg::ChatId => "chat_id",
            ReceiveIdTypeArg::Auto => infer_receive_id_type(id),
        }
    }
}

impl UserIdTypeArg {
    pub(in crate::app) fn resolve(self, sample: Option<&str>) -> &'static str {
        match self {
            UserIdTypeArg::OpenId => "open_id",
            UserIdTypeArg::UnionId => "union_id",
            UserIdTypeArg::UserId => "user_id",
            UserIdTypeArg::Auto => sample.map(infer_user_id_type).unwrap_or("open_id"),
        }
    }
}

impl OkrUserIdTypeArg {
    pub(in crate::app) fn as_api_value(self) -> &'static str {
        match self {
            OkrUserIdTypeArg::OpenId => "open_id",
            OkrUserIdTypeArg::UnionId => "union_id",
            OkrUserIdTypeArg::UserId => "user_id",
            OkrUserIdTypeArg::PeopleAdminId => "people_admin_id",
        }
    }
}

impl AttendanceEmployeeTypeArg {
    pub(in crate::app) fn as_api_value(self) -> &'static str {
        match self {
            AttendanceEmployeeTypeArg::EmployeeId => "employee_id",
            AttendanceEmployeeTypeArg::EmployeeNo => "employee_no",
        }
    }
}

impl DepartmentIdTypeArg {
    pub(in crate::app) fn as_api_value(self) -> &'static str {
        match self {
            DepartmentIdTypeArg::OpenDepartmentId => "open_department_id",
            DepartmentIdTypeArg::DepartmentId => "department_id",
        }
    }
}

impl ContentTypeArg {
    pub(in crate::app) fn as_api_value(self) -> &'static str {
        match self {
            ContentTypeArg::Markdown => "markdown",
            ContentTypeArg::Html => "html",
        }
    }
}

pub(in crate::app) fn infer_receive_id_type(id: &str) -> &'static str {
    if id.starts_with("oc_") {
        "chat_id"
    } else if id.starts_with("ou_") {
        "open_id"
    } else if id.starts_with("on_") {
        "union_id"
    } else if id.contains('@') {
        "email"
    } else {
        "user_id"
    }
}

pub(in crate::app) fn infer_user_id_type(id: &str) -> &'static str {
    match infer_receive_id_type(id) {
        "chat_id" | "email" => "open_id",
        other => other,
    }
}
