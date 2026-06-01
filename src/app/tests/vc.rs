use super::super::*;

#[test]
fn builds_vc_room_mget_body() {
    let body = build_vc_room_mget_body(VcRoomMgetArgs {
        room_ids: vec!["omm_1".to_string(), "omm_2".to_string()],
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
    })
    .unwrap();
    assert_eq!(body["room_ids"][0], "omm_1");
    assert_eq!(body["room_ids"][1], "omm_2");

    let empty = build_vc_room_mget_body(VcRoomMgetArgs {
        room_ids: vec![],
        body_json: None,
        file: None,
        stdin: false,
        user_id_type: UserIdTypeArg::OpenId,
    });
    assert!(empty.is_err());
}

#[test]
fn builds_vc_reserve_meeting_and_recording_bodies() {
    let reserve = build_vc_reserve_apply_body(VcReserveApplyArgs {
        end_time: Some("1780300000".to_string()),
        owner_id: Some("ou_owner".to_string()),
        topic: Some("AI sync".to_string()),
        auto_record: Some(true),
        assign_hosts: vec!["ou_host".to_string()],
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::Tenant,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(reserve["end_time"], "1780300000");
    assert_eq!(reserve["owner_id"], "ou_owner");
    assert_eq!(reserve["meeting_settings"]["topic"], "AI sync");
    assert_eq!(reserve["meeting_settings"]["auto_record"], true);
    assert_eq!(
        reserve["meeting_settings"]["assign_host_list"][0]["id"],
        "ou_host"
    );

    let invite = build_vc_meeting_invite_body(VcMeetingInviteArgs {
        meeting_id: "mtg_1".to_string(),
        users: vec!["ou_1".to_string(), "ou_2".to_string()],
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::User,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(invite["invitees"][1]["id"], "ou_2");
    assert_eq!(invite["invitees"][1]["user_type"], 1);

    let host = build_vc_meeting_set_host_body(VcMeetingSetHostArgs {
        meeting_id: "mtg_1".to_string(),
        user_id: "ou_host".to_string(),
        user_id_type: UserIdTypeArg::OpenId,
        auth: ApiAuthArg::Tenant,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(host["host_user"]["id"], "ou_host");

    let recording = build_vc_recording_permission_body(VcRecordingSetPermissionArgs {
        meeting_id: "mtg_1".to_string(),
        users: vec!["ou_1".to_string()],
        chats: vec!["oc_1".to_string()],
        tenant: true,
        public: false,
        auth: ApiAuthArg::User,
        body_json: None,
        file: None,
        stdin: false,
    })
    .unwrap();
    assert_eq!(recording["permission_objects"][0]["type"], 1);
    assert_eq!(recording["permission_objects"][1]["type"], 2);
    assert_eq!(recording["permission_objects"][2]["type"], 3);
}
