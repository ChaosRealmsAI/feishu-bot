use super::*;

impl HireUserIdTypeArg {
    pub(in crate::app) fn as_api_value(self) -> &'static str {
        match self {
            HireUserIdTypeArg::OpenId => "open_id",
            HireUserIdTypeArg::UnionId => "union_id",
            HireUserIdTypeArg::UserId => "user_id",
            HireUserIdTypeArg::PeopleAdminId => "people_admin_id",
        }
    }
}

impl HireJobLevelIdTypeArg {
    pub(in crate::app) fn as_api_value(self) -> &'static str {
        match self {
            HireJobLevelIdTypeArg::PeopleAdminJobLevelId => "people_admin_job_level_id",
            HireJobLevelIdTypeArg::JobLevelId => "job_level_id",
        }
    }
}

impl HireJobFamilyIdTypeArg {
    pub(in crate::app) fn as_api_value(self) -> &'static str {
        match self {
            HireJobFamilyIdTypeArg::PeopleAdminJobCategoryId => "people_admin_job_category_id",
            HireJobFamilyIdTypeArg::JobFamilyId => "job_family_id",
        }
    }
}

impl HireEmployeeTypeIdTypeArg {
    pub(in crate::app) fn as_api_value(self) -> &'static str {
        match self {
            HireEmployeeTypeIdTypeArg::PeopleAdminEmployeeTypeId => "people_admin_employee_type_id",
            HireEmployeeTypeIdTypeArg::EmployeeTypeEnumId => "employee_type_enum_id",
        }
    }
}
