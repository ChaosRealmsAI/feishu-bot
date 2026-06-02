use super::*;

pub(in crate::app) fn build_vc_reserve_apply_body(args: VcReserveApplyArgs) -> Result<Value> {
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

pub(super) fn build_vc_reserve_update_body(args: VcReserveUpdateArgs) -> Result<Value> {
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

pub(in crate::app) fn build_vc_meeting_invite_body(args: VcMeetingInviteArgs) -> Result<Value> {
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

pub(in crate::app) fn build_vc_meeting_set_host_body(args: VcMeetingSetHostArgs) -> Result<Value> {
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

pub(super) fn build_vc_recording_start_body(args: VcRecordingStartArgs) -> Result<Value> {
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

pub(in crate::app) fn build_vc_recording_permission_body(
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

pub(in crate::app) fn build_vc_room_mget_body(args: VcRoomMgetArgs) -> Result<Value> {
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
