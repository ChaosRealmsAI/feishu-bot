use super::*;

mod corehr;
mod directory;
mod hire;

pub(super) use corehr::*;
pub(super) use directory::*;
pub(super) use hire::*;

pub(super) async fn run_contact_command(
    api: &mut FeishuClient,
    command: ContactCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        ContactCommand::User(ContactUserCommand::Get(args)) => {
            let path = format!("/contact/v3/users/{}", args.user_id);
            api.get_json(
                &path,
                &contact_query(args.user_id_type, args.department_id_type),
            )
            .await?
        }
        ContactCommand::User(ContactUserCommand::List(args)) => {
            let mut query = contact_query(args.user_id_type, args.department_id_type);
            query.push(("page_size".to_string(), args.page_size.to_string()));
            push_query_opt(&mut query, "department_id", args.department_id);
            push_query_opt(&mut query, "page_token", args.page_token);
            api.get_json("/contact/v3/users", &query).await?
        }
        ContactCommand::Department(ContactDepartmentCommand::Get(args)) => {
            let path = format!("/contact/v3/departments/{}", args.department_id);
            api.get_json(
                &path,
                &contact_query(args.user_id_type, args.department_id_type),
            )
            .await?
        }
        ContactCommand::Department(ContactDepartmentCommand::List(args)) => {
            let mut query = contact_query(args.user_id_type, args.department_id_type);
            query.push(("page_size".to_string(), args.page_size.to_string()));
            push_query_opt(&mut query, "page_token", args.page_token);
            push_query_opt(
                &mut query,
                "parent_department_id",
                args.parent_department_id,
            );
            if args.fetch_child {
                query.push(("fetch_child".to_string(), "true".to_string()));
            }
            api.get_json("/contact/v3/departments", &query).await?
        }
        ContactCommand::Department(ContactDepartmentCommand::Children(args)) => {
            let path = format!("/contact/v3/departments/{}/children", args.department_id);
            let mut query = contact_query(args.user_id_type, args.department_id_type);
            query.push(("page_size".to_string(), args.page_size.to_string()));
            push_query_opt(&mut query, "page_token", args.page_token);
            if args.fetch_child {
                query.push(("fetch_child".to_string(), "true".to_string()));
            }
            api.get_json(&path, &query).await?
        }
        ContactCommand::Department(ContactDepartmentCommand::Search(args)) => {
            let mut query = contact_query(args.user_id_type, args.department_id_type);
            query.push(("query".to_string(), args.query));
            query.push(("page_size".to_string(), args.page_size.to_string()));
            push_query_opt(&mut query, "page_token", args.page_token);
            api.get_json("/contact/v3/departments/search", &query)
                .await?
        }
    };
    print_response(raw_json, "contact operation completed", data)
}

fn contact_query(
    user_id_type: UserIdTypeArg,
    department_id_type: DepartmentIdTypeArg,
) -> Vec<(String, String)> {
    vec![
        (
            "user_id_type".to_string(),
            user_id_type.resolve(None).to_string(),
        ),
        (
            "department_id_type".to_string(),
            department_id_type.as_api_value().to_string(),
        ),
    ]
}
