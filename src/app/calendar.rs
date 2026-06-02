use super::*;

mod bodies;

pub(super) use bodies::*;

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
