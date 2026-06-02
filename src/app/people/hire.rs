use super::*;

mod bodies;
mod id_types;
mod queries;

pub(in crate::app) use bodies::{
    build_hire_job_open_body, build_hire_location_query_body, build_hire_talent_create_body,
};
use queries::hire_job_detail_query;
pub(in crate::app) use queries::{
    hire_application_detail_query, hire_application_list_query, hire_job_list_query,
    hire_page_query, hire_talent_list_query,
};

pub(in crate::app) async fn run_hire_command(
    api: &mut FeishuClient,
    command: HireCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        HireCommand::Job(HireJobCommand::List(args)) => {
            let query = hire_job_list_query(args)?;
            api.get_json("/hire/v1/jobs", &query).await?
        }
        HireCommand::Job(HireJobCommand::Get(args)) => {
            let path = format!("/hire/v1/jobs/{}", encode_path_segment(&args.job_id));
            let query = hire_job_detail_query(args);
            api.get_json(&path, &query).await?
        }
        HireCommand::Job(HireJobCommand::Detail(args)) => {
            let path = format!(
                "/hire/v1/jobs/{}/get_detail",
                encode_path_segment(&args.job_id)
            );
            let query = hire_job_detail_query(args);
            api.get_json(&path, &query).await?
        }
        HireCommand::Job(HireJobCommand::Schemas(args)) => {
            let mut query = hire_page_query(args.page_size, 100, args.page_token)?;
            push_query_opt_u8(&mut query, "scenario", args.scenario);
            api.get_json("/hire/v1/job_schemas", &query).await?
        }
        HireCommand::Job(HireJobCommand::Open(args)) => {
            let path = format!("/hire/v1/jobs/{}/open", encode_path_segment(&args.job_id));
            let body = build_hire_job_open_body(args)?;
            api.post_json(&path, &[], body).await?
        }
        HireCommand::Talent(HireTalentCommand::List(args)) => {
            let query = hire_talent_list_query(args)?;
            api.get_json("/hire/v1/talents", &query).await?
        }
        HireCommand::Talent(HireTalentCommand::Get(args)) => {
            let path = format!("/hire/v1/talents/{}", encode_path_segment(&args.talent_id));
            let query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.as_api_value().to_string(),
            )];
            api.get_json(&path, &query).await?
        }
        HireCommand::Talent(HireTalentCommand::Create(args)) => {
            let query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.as_api_value().to_string(),
            )];
            let body = build_hire_talent_create_body(args)?;
            api.post_json("/hire/v1/talents/combined_create", &query, body)
                .await?
        }
        HireCommand::Application(HireApplicationCommand::List(args)) => {
            let query = hire_application_list_query(args)?;
            api.get_json("/hire/v1/applications", &query).await?
        }
        HireCommand::Application(HireApplicationCommand::Get(args)) => {
            let path = format!(
                "/hire/v1/applications/{}",
                encode_path_segment(&args.application_id)
            );
            let mut query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.resolve(None).to_string(),
            )];
            push_query_repeated(&mut query, "options", args.options);
            api.get_json(&path, &query).await?
        }
        HireCommand::Application(HireApplicationCommand::Detail(args)) => {
            let path = format!(
                "/hire/v1/applications/{}/get_detail",
                encode_path_segment(&args.application_id)
            );
            let query = hire_application_detail_query(args);
            api.get_json(&path, &query).await?
        }
        HireCommand::Interview(HireInterviewCommand::ByTalent(args)) => {
            let query = vec![
                ("talent_id".to_string(), args.talent_id),
                (
                    "user_id_type".to_string(),
                    args.user_id_type.as_api_value().to_string(),
                ),
                (
                    "job_level_id_type".to_string(),
                    args.job_level_id_type.as_api_value().to_string(),
                ),
            ];
            api.get_json("/hire/v1/interviews/get_by_talent", &query)
                .await?
        }
        HireCommand::Process(HireProcessCommand::List(args)) => {
            let query = hire_page_query(args.page_size, 100, args.page_token)?;
            api.get_json("/hire/v1/job_processes", &query).await?
        }
        HireCommand::Requirement(HireRequirementCommand::Schemas(args)) => {
            let query = hire_page_query(args.page_size, 100, args.page_token)?;
            api.get_json("/hire/v1/job_requirement_schemas", &query)
                .await?
        }
        HireCommand::Metadata(HireMetadataCommand::ResumeSources(args)) => {
            let query = hire_page_query(args.page_size, 100, args.page_token)?;
            api.get_json("/hire/v1/resume_sources", &query).await?
        }
        HireCommand::Metadata(HireMetadataCommand::JobTypes(args)) => {
            let query = hire_page_query(args.page_size, 100, args.page_token)?;
            api.get_json("/hire/v1/job_types", &query).await?
        }
        HireCommand::Metadata(HireMetadataCommand::JobFunctions(args)) => {
            let query = hire_page_query(args.page_size, 50, args.page_token)?;
            api.get_json("/hire/v1/job_functions", &query).await?
        }
        HireCommand::Metadata(HireMetadataCommand::Subjects(args)) => {
            let mut query = hire_page_query(args.page_size, 200, args.page_token)?;
            query.push((
                "user_id_type".to_string(),
                args.user_id_type.resolve(None).to_string(),
            ));
            api.get_json("/hire/v1/subjects", &query).await?
        }
        HireCommand::Metadata(HireMetadataCommand::Websites(args)) => {
            let query = hire_page_query(args.page_size, 10, args.page_token)?;
            api.get_json("/hire/v1/websites", &query).await?
        }
        HireCommand::Attachment(HireAttachmentCommand::Get(args)) => {
            let path = format!(
                "/hire/v1/attachments/{}",
                encode_path_segment(&args.attachment_id)
            );
            let mut query = Vec::new();
            push_query_opt_u8(&mut query, "type", args.attachment_type);
            api.get_json(&path, &query).await?
        }
        HireCommand::Location(HireLocationCommand::Query(args)) => {
            let query = hire_page_query(args.page_size, 100, args.page_token.clone())?;
            let body = build_hire_location_query_body(args)?;
            api.post_json("/hire/v1/locations/query", &query, body)
                .await?
        }
    };
    print_response(raw_json, "hire operation completed", data)
}
