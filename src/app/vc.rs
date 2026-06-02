use super::*;

mod bodies;

pub(super) use bodies::{
    build_vc_meeting_invite_body, build_vc_meeting_set_host_body,
    build_vc_recording_permission_body, build_vc_reserve_apply_body, build_vc_room_mget_body,
};
use bodies::{build_vc_recording_start_body, build_vc_reserve_update_body};

pub(super) async fn run_vc_command(
    api: &mut FeishuClient,
    command: VcCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        VcCommand::Reserve(VcReserveCommand::Apply(args)) => {
            let query = vec![(
                "user_id_type".to_string(),
                args.user_id_type
                    .resolve(args.owner_id.as_deref())
                    .to_string(),
            )];
            let auth = args.auth;
            let body = build_vc_reserve_apply_body(args)?;
            api.request_json_with_auth(
                Method::POST,
                "/vc/v1/reserves/apply",
                &query,
                Some(body),
                auth,
                &[],
            )
            .await?
        }
        VcCommand::Reserve(VcReserveCommand::Get(args)) => {
            let path = format!("/vc/v1/reserves/{}", encode_path_segment(&args.reserve_id));
            let query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.resolve(None).to_string(),
            )];
            api.request_json_with_auth(Method::GET, &path, &query, None, args.auth, &[])
                .await?
        }
        VcCommand::Reserve(VcReserveCommand::Update(args)) => {
            let path = format!("/vc/v1/reserves/{}", encode_path_segment(&args.reserve_id));
            let query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.resolve(None).to_string(),
            )];
            let auth = args.auth;
            let body = build_vc_reserve_update_body(args)?;
            api.request_json_with_auth(Method::PUT, &path, &query, Some(body), auth, &[])
                .await?
        }
        VcCommand::Reserve(VcReserveCommand::Delete(args)) => {
            let path = format!("/vc/v1/reserves/{}", encode_path_segment(&args.reserve_id));
            api.request_json_with_auth(Method::DELETE, &path, &[], None, args.auth, &[])
                .await?
        }
        VcCommand::Reserve(VcReserveCommand::ActiveMeeting(args)) => {
            let path = format!(
                "/vc/v1/reserves/{}/get_active_meeting",
                encode_path_segment(&args.reserve_id)
            );
            let query = vec![
                (
                    "with_participants".to_string(),
                    args.with_participants.to_string(),
                ),
                (
                    "user_id_type".to_string(),
                    args.user_id_type.resolve(None).to_string(),
                ),
            ];
            api.request_json_with_auth(Method::GET, &path, &query, None, args.auth, &[])
                .await?
        }
        VcCommand::Meeting(VcMeetingCommand::Get(args)) => {
            let path = format!("/vc/v1/meetings/{}", args.meeting_id);
            api.get_json(
                &path,
                &[(
                    "user_id_type".to_string(),
                    args.user_id_type.resolve(None).to_string(),
                )],
            )
            .await?
        }
        VcCommand::Meeting(VcMeetingCommand::ListByNo(args)) => {
            let mut query = vec![
                ("meeting_no".to_string(), args.meeting_no),
                ("start_time".to_string(), args.start_time),
                ("end_time".to_string(), args.end_time),
                ("page_size".to_string(), args.page_size.to_string()),
            ];
            push_query_opt(&mut query, "page_token", args.page_token);
            api.get_json("/vc/v1/meetings/list_by_no", &query).await?
        }
        VcCommand::Meeting(VcMeetingCommand::Invite(args)) => {
            let path = format!(
                "/vc/v1/meetings/{}/invite",
                encode_path_segment(&args.meeting_id)
            );
            let query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.resolve(None).to_string(),
            )];
            let auth = args.auth;
            let body = build_vc_meeting_invite_body(args)?;
            api.request_json_with_auth(Method::PATCH, &path, &query, Some(body), auth, &[])
                .await?
        }
        VcCommand::Meeting(VcMeetingCommand::SetHost(args)) => {
            let path = format!(
                "/vc/v1/meetings/{}/set_host",
                encode_path_segment(&args.meeting_id)
            );
            let query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.resolve(Some(&args.user_id)).to_string(),
            )];
            let auth = args.auth;
            let body = build_vc_meeting_set_host_body(args)?;
            api.request_json_with_auth(Method::PATCH, &path, &query, Some(body), auth, &[])
                .await?
        }
        VcCommand::Meeting(VcMeetingCommand::End(args)) => {
            let path = format!(
                "/vc/v1/meetings/{}/end",
                encode_path_segment(&args.meeting_id)
            );
            api.request_json_with_auth(Method::PATCH, &path, &[], None, args.auth, &[])
                .await?
        }
        VcCommand::Recording(VcRecordingCommand::Get(args)) => {
            let path = format!("/vc/v1/meetings/{}/recording", args.meeting_id);
            api.request_json_with_auth(Method::GET, &path, &[], None, args.auth, &[])
                .await?
        }
        VcCommand::Recording(VcRecordingCommand::Start(args)) => {
            let path = format!(
                "/vc/v1/meetings/{}/recording/start",
                encode_path_segment(&args.meeting_id)
            );
            let auth = args.auth;
            let body = build_vc_recording_start_body(args)?;
            api.request_json_with_auth(Method::PATCH, &path, &[], Some(body), auth, &[])
                .await?
        }
        VcCommand::Recording(VcRecordingCommand::Stop(args)) => {
            let path = format!(
                "/vc/v1/meetings/{}/recording/stop",
                encode_path_segment(&args.meeting_id)
            );
            api.request_json_with_auth(Method::PATCH, &path, &[], None, args.auth, &[])
                .await?
        }
        VcCommand::Recording(VcRecordingCommand::SetPermission(args)) => {
            let path = format!(
                "/vc/v1/meetings/{}/recording/set_permission",
                encode_path_segment(&args.meeting_id)
            );
            let auth = args.auth;
            let body = build_vc_recording_permission_body(args)?;
            api.request_json_with_auth(Method::PATCH, &path, &[], Some(body), auth, &[])
                .await?
        }
        VcCommand::Report(VcReportCommand::Daily(args)) => {
            api.get_json(
                "/vc/v1/reports/get_daily",
                &[
                    ("start_time".to_string(), args.start_time),
                    ("end_time".to_string(), args.end_time),
                ],
            )
            .await?
        }
        VcCommand::Report(VcReportCommand::TopUser(args)) => {
            api.get_json(
                "/vc/v1/reports/get_top_user",
                &[
                    ("start_time".to_string(), args.start_time),
                    ("end_time".to_string(), args.end_time),
                    ("limit".to_string(), args.limit.to_string()),
                    ("order_by".to_string(), args.order_by.to_string()),
                    (
                        "user_id_type".to_string(),
                        args.user_id_type.resolve(None).to_string(),
                    ),
                ],
            )
            .await?
        }
        VcCommand::Room(VcRoomCommand::List(args)) => {
            let mut query = vec![
                ("page_size".to_string(), args.page_size.to_string()),
                (
                    "user_id_type".to_string(),
                    args.user_id_type.resolve(None).to_string(),
                ),
            ];
            push_query_opt(&mut query, "page_token", args.page_token);
            push_query_opt(&mut query, "room_level_id", args.room_level_id);
            api.get_json("/vc/v1/rooms", &query).await?
        }
        VcCommand::Room(VcRoomCommand::Get(args)) => {
            let path = format!("/vc/v1/rooms/{}", args.room_id);
            api.get_json(
                &path,
                &[(
                    "user_id_type".to_string(),
                    args.user_id_type.resolve(None).to_string(),
                )],
            )
            .await?
        }
        VcCommand::Room(VcRoomCommand::Mget(args)) => {
            let query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.resolve(None).to_string(),
            )];
            let body = build_vc_room_mget_body(args)?;
            api.post_json("/vc/v1/rooms/mget", &query, body).await?
        }
        VcCommand::RoomLevel(VcRoomLevelCommand::List(args)) => {
            let mut query = vec![("page_size".to_string(), args.page_size.to_string())];
            push_query_opt(&mut query, "page_token", args.page_token);
            push_query_opt(&mut query, "room_level_id", args.room_level_id);
            api.get_json("/vc/v1/room_levels", &query).await?
        }
    };
    print_response(raw_json, "vc operation completed", data)
}
