use super::super::*;

#[test]
fn builds_calendar_event_body() {
    let body = build_calendar_event_create_body(CalendarEventCreateArgs {
        calendar_id: "cal_1".to_string(),
        summary: Some("sync".to_string()),
        description: Some("notes".to_string()),
        start_ts: Some("1760000000".to_string()),
        end_ts: Some("1760003600".to_string()),
        time_zone: "Asia/Shanghai".to_string(),
        idempotency_key: None,
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
    })
    .unwrap();
    assert_eq!(body["summary"], "sync");
    assert_eq!(body["start_time"]["timestamp"], "1760000000");
    assert_eq!(body["end_time"]["timezone"], "Asia/Shanghai");

    let attendees = build_calendar_attendee_add_body(CalendarAttendeeAddArgs {
        calendar_id: "cal_1".to_string(),
        event_id: "evt_1".to_string(),
        users: vec!["ou_1".to_string()],
        chats: vec!["oc_1".to_string()],
        optional: true,
        attendees_json: None,
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
    })
    .unwrap();
    assert_eq!(attendees["attendees"][0]["type"], "user");
    assert_eq!(attendees["attendees"][0]["is_optional"], true);
    assert_eq!(attendees["attendees"][1]["chat_id"], "oc_1");

    let delete = build_calendar_attendee_delete_body(CalendarAttendeeDeleteArgs {
        calendar_id: "cal_1".to_string(),
        event_id: "evt_1".to_string(),
        attendee_ids: vec!["att_1".to_string()],
        delete_ids: vec!["ou_1".to_string()],
        attendee_ids_json: None,
        delete_ids_json: None,
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
    })
    .unwrap();
    assert_eq!(delete["attendee_ids"][0], "att_1");
    assert_eq!(delete["delete_ids"][0], "ou_1");

    let freebusy = build_calendar_freebusy_list_body(CalendarFreebusyListArgs {
        time_min: "2026-06-01T09:00:00+08:00".to_string(),
        time_max: "2026-06-01T18:00:00+08:00".to_string(),
        user_id: Some("ou_1".to_string()),
        room_id: None,
        include_external_calendar: Some(false),
        only_busy: Some(true),
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
    })
    .unwrap();
    assert_eq!(freebusy["user_id"], "ou_1");
    assert_eq!(freebusy["include_external_calendar"], false);
    assert_eq!(freebusy["only_busy"], true);

    let batch = build_calendar_freebusy_batch_body(CalendarFreebusyBatchArgs {
        time_min: "2026-06-01T09:00:00+08:00".to_string(),
        time_max: "2026-06-01T18:00:00+08:00".to_string(),
        user_ids: vec!["ou_1".to_string(), "ou_2".to_string()],
        user_ids_json: None,
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
    })
    .unwrap();
    assert_eq!(batch["user_ids"][0], "ou_1");
    assert_eq!(batch["user_ids"][1], "ou_2");
}
