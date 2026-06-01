use super::*;

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

pub(super) fn build_vc_reserve_apply_body(args: VcReserveApplyArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "vc reserve apply body",
        );
    }
    let mut body = Map::new();
    let end_time = args
        .end_time
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("vc reserve apply needs --end-time or raw JSON body"))?;
    body.insert("end_time".to_string(), Value::String(end_time));
    insert_opt_string(&mut body, "owner_id", args.owner_id);
    body.insert(
        "meeting_settings".to_string(),
        build_vc_meeting_settings(args.topic, args.auto_record, args.assign_hosts)?,
    );
    Ok(Value::Object(body))
}

fn build_vc_reserve_update_body(args: VcReserveUpdateArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "vc reserve update body",
        );
    }
    let mut body = Map::new();
    insert_opt_string(&mut body, "end_time", args.end_time);
    let meeting_settings =
        build_vc_meeting_settings(args.topic, args.auto_record, args.assign_hosts)?;
    if let Value::Object(settings) = &meeting_settings {
        if !settings.is_empty() {
            body.insert("meeting_settings".to_string(), meeting_settings);
        }
    }
    if body.is_empty() {
        bail!("vc reserve update needs a field flag or raw JSON body");
    }
    Ok(Value::Object(body))
}

fn build_vc_meeting_settings(
    topic: Option<String>,
    auto_record: Option<bool>,
    assign_hosts: Vec<String>,
) -> Result<Value> {
    let mut settings = Map::new();
    insert_opt_string(&mut settings, "topic", topic);
    if let Some(auto_record) = auto_record {
        settings.insert("auto_record".to_string(), Value::Bool(auto_record));
    }
    let hosts = vc_user_array(assign_hosts, "assign-host")?;
    if !hosts.is_empty() {
        settings.insert("assign_host_list".to_string(), Value::Array(hosts));
    }
    Ok(Value::Object(settings))
}

pub(super) fn build_vc_meeting_invite_body(args: VcMeetingInviteArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "vc meeting invite body",
        );
    }
    let invitees = vc_user_array(args.users, "user")?;
    if invitees.is_empty() {
        bail!("vc meeting invite needs at least one --user or raw JSON body");
    }
    Ok(json!({ "invitees": invitees }))
}

pub(super) fn build_vc_meeting_set_host_body(args: VcMeetingSetHostArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "vc meeting set-host body",
        );
    }
    Ok(json!({
        "host_user": {
            "id": args.user_id,
            "user_type": 1
        }
    }))
}

fn build_vc_recording_start_body(args: VcRecordingStartArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "vc recording start body",
        );
    }
    let mut body = Map::new();
    insert_opt_i64(&mut body, "timezone", args.timezone);
    Ok(Value::Object(body))
}

pub(super) fn build_vc_recording_permission_body(
    args: VcRecordingSetPermissionArgs,
) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "vc recording permission body",
        );
    }
    let mut permission_objects = Vec::new();
    permission_objects.extend(vc_recording_permission_objects(args.users, 1));
    permission_objects.extend(vc_recording_permission_objects(args.chats, 2));
    if args.tenant {
        permission_objects.push(json!({ "type": 3, "permission": 1 }));
    }
    if args.public {
        permission_objects.push(json!({ "type": 4, "permission": 1 }));
    }
    if permission_objects.is_empty() {
        bail!("vc recording set-permission needs --user, --chat, --tenant, --public, or raw JSON body");
    }
    Ok(json!({ "permission_objects": permission_objects }))
}

fn vc_user_array(ids: Vec<String>, label: &str) -> Result<Vec<Value>> {
    let users = ids
        .into_iter()
        .filter(|id| !id.trim().is_empty())
        .map(|id| json!({ "id": id, "user_type": 1 }))
        .collect::<Vec<_>>();
    if users.len() > 10 {
        bail!("vc {label} cannot repeat more than 10 times");
    }
    Ok(users)
}

fn vc_recording_permission_objects(ids: Vec<String>, object_type: i64) -> Vec<Value> {
    ids.into_iter()
        .filter(|id| !id.trim().is_empty())
        .map(|id| json!({ "id": id, "type": object_type, "permission": 1 }))
        .collect()
}

pub(super) fn build_vc_room_mget_body(args: VcRoomMgetArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "vc room mget body",
        );
    }
    if args.room_ids.is_empty() {
        bail!("vc room mget needs at least one --room-id or raw JSON body");
    }
    Ok(json!({ "room_ids": args.room_ids }))
}
