use super::*;

pub(super) async fn run_calendar_command(
    api: &mut FeishuClient,
    command: CalendarCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        CalendarCommand::Primary => api.get_json("/calendar/v4/calendars/primary", &[]).await?,
        CalendarCommand::List(args) => {
            let mut query = vec![("page_size".to_string(), args.page_size.to_string())];
            push_query_opt(&mut query, "page_token", args.page_token);
            api.get_json("/calendar/v4/calendars", &query).await?
        }
        CalendarCommand::Create(args) => {
            let body = if args.body_json.is_some() || args.file.is_some() || args.stdin {
                read_json_value(args.body_json, args.file, args.stdin)?
            } else {
                let summary = args
                    .summary
                    .ok_or_else(|| anyhow!("calendar create needs --summary or raw body"))?;
                let mut body = Map::new();
                body.insert("summary".to_string(), Value::String(summary));
                if let Some(description) = args.description {
                    body.insert("description".to_string(), Value::String(description));
                }
                Value::Object(body)
            };
            api.post_json("/calendar/v4/calendars", &[], body).await?
        }
        CalendarCommand::Event(CalendarEventCommand::List(args)) => {
            let path = format!("/calendar/v4/calendars/{}/events", args.calendar_id);
            let mut query = vec![
                ("page_size".to_string(), args.page_size.to_string()),
                (
                    "user_id_type".to_string(),
                    args.user_id_type.resolve(None).to_string(),
                ),
            ];
            push_query_opt(&mut query, "page_token", args.page_token);
            push_query_opt(&mut query, "start_time", args.start_ts);
            push_query_opt(&mut query, "end_time", args.end_ts);
            api.get_json(&path, &query).await?
        }
        CalendarCommand::Event(CalendarEventCommand::Get(args)) => {
            let path = format!(
                "/calendar/v4/calendars/{}/events/{}",
                args.calendar_id, args.event_id
            );
            api.get_json(
                &path,
                &[(
                    "user_id_type".to_string(),
                    args.user_id_type.resolve(None).to_string(),
                )],
            )
            .await?
        }
        CalendarCommand::Event(CalendarEventCommand::Create(args)) => {
            let path = format!("/calendar/v4/calendars/{}/events", args.calendar_id);
            let mut query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.resolve(None).to_string(),
            )];
            push_query_opt(&mut query, "idempotency_key", args.idempotency_key.clone());
            let body = build_calendar_event_create_body(args)?;
            api.post_json(&path, &query, body).await?
        }
        CalendarCommand::Event(CalendarEventCommand::Update(args)) => {
            let path = format!(
                "/calendar/v4/calendars/{}/events/{}",
                args.calendar_id, args.event_id
            );
            let query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.resolve(None).to_string(),
            )];
            let body = build_calendar_event_update_body(args)?;
            api.patch_json(&path, &query, body).await?
        }
        CalendarCommand::Event(CalendarEventCommand::Delete(args)) => {
            let path = format!(
                "/calendar/v4/calendars/{}/events/{}",
                args.calendar_id, args.event_id
            );
            api.delete_json(&path, &[], None).await?
        }
        CalendarCommand::Attendee(CalendarAttendeeCommand::List(args)) => {
            let path = format!(
                "/calendar/v4/calendars/{}/events/{}/attendees",
                args.calendar_id, args.event_id
            );
            let mut query = vec![
                ("page_size".to_string(), args.page_size.to_string()),
                (
                    "user_id_type".to_string(),
                    args.user_id_type.resolve(None).to_string(),
                ),
            ];
            push_query_opt(&mut query, "page_token", args.page_token);
            api.get_json(&path, &query).await?
        }
        CalendarCommand::Attendee(CalendarAttendeeCommand::Add(args)) => {
            let path = format!(
                "/calendar/v4/calendars/{}/events/{}/attendees",
                args.calendar_id, args.event_id
            );
            let query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.resolve(None).to_string(),
            )];
            let body = build_calendar_attendee_add_body(args)?;
            api.post_json(&path, &query, body).await?
        }
        CalendarCommand::Attendee(CalendarAttendeeCommand::Delete(args)) => {
            let path = format!(
                "/calendar/v4/calendars/{}/events/{}/attendees/batch_delete",
                args.calendar_id, args.event_id
            );
            let query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.resolve(None).to_string(),
            )];
            let body = build_calendar_attendee_delete_body(args)?;
            api.post_json(&path, &query, body).await?
        }
        CalendarCommand::Attendee(CalendarAttendeeCommand::ChatMembers(args)) => {
            let path = format!(
                "/calendar/v4/calendars/{}/events/{}/attendees/{}/chat_members",
                args.calendar_id, args.event_id, args.attendee_id
            );
            let mut query = vec![
                ("page_size".to_string(), args.page_size.to_string()),
                (
                    "user_id_type".to_string(),
                    args.user_id_type.resolve(None).to_string(),
                ),
            ];
            push_query_opt(&mut query, "page_token", args.page_token);
            api.get_json(&path, &query).await?
        }
        CalendarCommand::Freebusy(CalendarFreebusyCommand::List(args)) => {
            let query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.resolve(None).to_string(),
            )];
            let body = build_calendar_freebusy_list_body(args)?;
            api.post_json("/calendar/v4/freebusy/list", &query, body)
                .await?
        }
        CalendarCommand::Freebusy(CalendarFreebusyCommand::Batch(args)) => {
            let query = vec![(
                "user_id_type".to_string(),
                args.user_id_type.resolve(None).to_string(),
            )];
            let body = build_calendar_freebusy_batch_body(args)?;
            api.post_json("/calendar/v4/freebusy/batch", &query, body)
                .await?
        }
    };
    print_response(raw_json, "calendar operation completed", data)
}

pub(super) fn build_calendar_event_create_body(args: CalendarEventCreateArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return read_json_value(args.body_json, args.file, args.stdin);
    }

    let summary = args
        .summary
        .ok_or_else(|| anyhow!("event create needs --summary or raw body"))?;
    let start_ts = args
        .start_ts
        .ok_or_else(|| anyhow!("event create needs --start-ts or raw body"))?;
    let end_ts = args
        .end_ts
        .ok_or_else(|| anyhow!("event create needs --end-ts or raw body"))?;
    let mut body = Map::new();
    body.insert("summary".to_string(), Value::String(summary));
    if let Some(description) = args.description {
        body.insert("description".to_string(), Value::String(description));
    }
    body.insert(
        "start_time".to_string(),
        json!({ "timestamp": start_ts, "timezone": args.time_zone.clone() }),
    );
    body.insert(
        "end_time".to_string(),
        json!({ "timestamp": end_ts, "timezone": args.time_zone }),
    );
    Ok(Value::Object(body))
}

pub(super) fn build_calendar_event_update_body(args: CalendarEventUpdateArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return read_json_value(args.body_json, args.file, args.stdin);
    }

    let mut body = Map::new();
    if let Some(summary) = args.summary {
        body.insert("summary".to_string(), Value::String(summary));
    }
    if let Some(description) = args.description {
        body.insert("description".to_string(), Value::String(description));
    }
    if body.is_empty() {
        bail!("event update needs field flags or raw body");
    }
    Ok(Value::Object(body))
}

pub(super) fn build_calendar_attendee_add_body(args: CalendarAttendeeAddArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "calendar attendee add body",
        );
    }
    if let Some(attendees_json) = args.attendees_json {
        let value = parse_json_value(&attendees_json, "attendees-json")?;
        if value.get("attendees").is_some() {
            return ensure_json_object(value, "calendar attendee add body");
        }
        return Ok(json!({ "attendees": ensure_json_array(value, "attendees")? }));
    }
    let mut attendees = Vec::new();
    for user_id in args
        .users
        .into_iter()
        .filter(|value| !value.trim().is_empty())
    {
        attendees.push(json!({
            "type": "user",
            "user_id": user_id,
            "is_optional": args.optional,
        }));
    }
    for chat_id in args
        .chats
        .into_iter()
        .filter(|value| !value.trim().is_empty())
    {
        attendees.push(json!({
            "type": "chat",
            "chat_id": chat_id,
            "is_optional": args.optional,
        }));
    }
    if attendees.is_empty() {
        bail!("calendar attendee add needs --user/--chat, --attendees-json, or raw body");
    }
    Ok(json!({ "attendees": attendees }))
}

pub(super) fn build_calendar_attendee_delete_body(
    args: CalendarAttendeeDeleteArgs,
) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "calendar attendee delete body",
        );
    }
    let mut body = Map::new();
    let attendee_ids =
        collect_json_string_array(args.attendee_ids, args.attendee_ids_json, "attendee_ids")?;
    if let Some(attendee_ids) = attendee_ids {
        body.insert("attendee_ids".to_string(), attendee_ids);
    }
    let delete_ids =
        collect_json_string_array(args.delete_ids, args.delete_ids_json, "delete_ids")?;
    if let Some(delete_ids) = delete_ids {
        body.insert("delete_ids".to_string(), delete_ids);
    }
    if body.is_empty() {
        bail!("calendar attendee delete needs --attendee-id/--delete-id, JSON arrays, or raw body");
    }
    Ok(Value::Object(body))
}

pub(super) fn build_calendar_freebusy_list_body(args: CalendarFreebusyListArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "calendar freebusy/list body",
        );
    }
    if args.user_id.is_some() && args.room_id.is_some() {
        bail!("calendar freebusy list accepts only one of --user-id or --room-id");
    }
    let mut body = Map::new();
    body.insert("time_min".to_string(), Value::String(args.time_min));
    body.insert("time_max".to_string(), Value::String(args.time_max));
    insert_opt_string(&mut body, "user_id", args.user_id);
    insert_opt_string(&mut body, "room_id", args.room_id);
    if body.get("user_id").is_none() && body.get("room_id").is_none() {
        bail!("calendar freebusy list needs --user-id, --room-id, or raw body");
    }
    if let Some(include_external_calendar) = args.include_external_calendar {
        body.insert(
            "include_external_calendar".to_string(),
            Value::Bool(include_external_calendar),
        );
    }
    if let Some(only_busy) = args.only_busy {
        body.insert("only_busy".to_string(), Value::Bool(only_busy));
    }
    Ok(Value::Object(body))
}

pub(super) fn build_calendar_freebusy_batch_body(args: CalendarFreebusyBatchArgs) -> Result<Value> {
    if args.body_json.is_some() || args.file.is_some() || args.stdin {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "calendar freebusy/batch body",
        );
    }
    let user_ids = collect_json_string_array(args.user_ids, args.user_ids_json, "user_ids")?
        .ok_or_else(|| {
            anyhow!("calendar freebusy batch needs --user-id, --user-ids-json, or raw body")
        })?;
    Ok(json!({
        "time_min": args.time_min,
        "time_max": args.time_max,
        "user_ids": user_ids,
    }))
}
