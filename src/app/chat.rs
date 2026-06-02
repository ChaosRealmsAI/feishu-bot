use super::*;

mod bodies;

pub(super) use bodies::*;

pub(super) async fn run_chat_command(
    api: &mut FeishuClient,
    command: ChatCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        ChatCommand::List(args) => {
            let mut query = vec![
                (
                    "user_id_type".to_string(),
                    args.user_id_type.resolve(None).to_string(),
                ),
                ("page_size".to_string(), args.page_size.to_string()),
            ];
            push_query_opt(&mut query, "page_token", args.page_token);
            push_query_opt(&mut query, "sort_type", args.sort_type);
            api.get_json("/im/v1/chats", &query).await?
        }
        ChatCommand::Search(args) => {
            let mut query = vec![
                (
                    "user_id_type".to_string(),
                    args.user_id_type.resolve(None).to_string(),
                ),
                ("page_size".to_string(), args.page_size.to_string()),
                ("query".to_string(), args.query),
            ];
            push_query_opt(&mut query, "page_token", args.page_token);
            api.get_json("/im/v1/chats/search", &query).await?
        }
        ChatCommand::Get(args) => {
            let path = format!("/im/v1/chats/{}", args.chat_id);
            api.get_json(
                &path,
                &[(
                    "user_id_type".to_string(),
                    args.user_id_type.resolve(None).to_string(),
                )],
            )
            .await?
        }
        ChatCommand::Create(args) => {
            let user_type = args.user_id_type.resolve(
                args.users
                    .first()
                    .or(args.owner_id.as_ref())
                    .map(String::as_str),
            );
            let mut body = build_chat_create_body(&args)?;
            insert_uploaded_avatar(api, args.avatar_file.as_ref(), &mut body).await?;
            let mut query = vec![("user_id_type".to_string(), user_type.to_string())];
            if args.set_bot_manager {
                query.push(("set_bot_manager".to_string(), "true".to_string()));
            }
            push_query_opt(&mut query, "uuid", args.uuid);
            api.post_json("/im/v1/chats", &query, body).await?
        }
        ChatCommand::Update(args) => {
            let mut body = build_chat_update_body(&args)?;
            insert_uploaded_avatar(api, args.avatar_file.as_ref(), &mut body).await?;
            if body.as_object().is_none_or(Map::is_empty) {
                bail!("chat update needs at least one field, --avatar-file, or raw body JSON");
            }
            let path = format!("/im/v1/chats/{}", args.chat_id);
            api.put_json(
                &path,
                &[(
                    "user_id_type".to_string(),
                    args.user_id_type
                        .resolve(args.owner_id.as_deref())
                        .to_string(),
                )],
                body,
            )
            .await?
        }
        ChatCommand::Delete(args) => {
            let path = format!("/im/v1/chats/{}", args.chat_id);
            api.delete_json(&path, &[], None).await?
        }
        ChatCommand::Member(ChatMemberCommand::List(args)) => {
            let path = format!("/im/v1/chats/{}/members", args.chat_id);
            let mut query = vec![
                (
                    "member_id_type".to_string(),
                    args.member_id_type.resolve(None).to_string(),
                ),
                ("page_size".to_string(), args.page_size.to_string()),
            ];
            push_query_opt(&mut query, "page_token", args.page_token);
            api.get_json(&path, &query).await?
        }
        ChatCommand::Member(ChatMemberCommand::Add(args)) => {
            let path = format!("/im/v1/chats/{}/members", args.chat_id);
            let query = chat_member_query(args.member_id_type, args.succeed_type);
            let body = build_chat_members_body(args.ids, args.body_json, args.file, args.stdin)?;
            api.post_json(&path, &query, body).await?
        }
        ChatCommand::Member(ChatMemberCommand::Delete(args)) => {
            let path = format!("/im/v1/chats/{}/members", args.chat_id);
            let query = chat_member_query(args.member_id_type, args.succeed_type);
            let body = build_chat_members_body(args.ids, args.body_json, args.file, args.stdin)?;
            api.delete_json(&path, &query, Some(body)).await?
        }
        ChatCommand::Member(ChatMemberCommand::IsInChat(args)) => {
            let path = format!("/im/v1/chats/{}/members/is_in_chat", args.chat_id);
            api.get_json(&path, &[]).await?
        }
        ChatCommand::Tab(ChatTabCommand::List(args)) => {
            let path = format!("/im/v1/chats/{}/chat_tabs/list_tabs", args.chat_id);
            api.get_json(&path, &[]).await?
        }
        ChatCommand::Tab(ChatTabCommand::Add(args)) => {
            let icon_key = upload_chat_tab_icon(api, args.icon_file.as_ref()).await?;
            let body = build_chat_tab_body(&args, false, icon_key)?;
            let path = format!("/im/v1/chats/{}/chat_tabs", args.chat_id);
            api.post_json(&path, &[], body).await?
        }
        ChatCommand::Tab(ChatTabCommand::Update(args)) => {
            let icon_key = upload_chat_tab_icon(api, args.icon_file.as_ref()).await?;
            let body = build_chat_tab_body(&args, true, icon_key)?;
            let path = format!("/im/v1/chats/{}/chat_tabs/update_tabs", args.chat_id);
            api.post_json(&path, &[], body).await?
        }
        ChatCommand::Tab(ChatTabCommand::Delete(args)) => {
            let body = build_repeated_ids_body(
                args.tab_ids,
                args.body_json,
                args.body_file,
                args.stdin,
                "tab_ids",
                "chat tab delete body",
            )?;
            let path = format!("/im/v1/chats/{}/chat_tabs/delete_tabs", args.chat_id);
            api.delete_json(&path, &[], Some(body)).await?
        }
        ChatCommand::Tab(ChatTabCommand::Sort(args)) => {
            let body = build_repeated_ids_body(
                args.tab_ids,
                args.body_json,
                args.body_file,
                args.stdin,
                "tab_ids",
                "chat tab sort body",
            )?;
            let path = format!("/im/v1/chats/{}/chat_tabs/sort_tabs", args.chat_id);
            api.post_json(&path, &[], body).await?
        }
        ChatCommand::Menu(ChatMenuCommand::Get(args)) => {
            let path = format!("/im/v1/chats/{}/menu_tree", args.chat_id);
            api.get_json(&path, &[]).await?
        }
        ChatCommand::Menu(ChatMenuCommand::Add(args)) => {
            let body = build_chat_menu_add_body(args.body_json, args.body_file, args.stdin)?;
            let path = format!("/im/v1/chats/{}/menu_tree", args.chat_id);
            api.post_json(&path, &[], body).await?
        }
        ChatCommand::Menu(ChatMenuCommand::Delete(args)) => {
            let body = build_repeated_ids_body(
                args.ids,
                args.body_json,
                args.body_file,
                args.stdin,
                "chat_menu_top_level_ids",
                "chat menu delete body",
            )?;
            let path = format!("/im/v1/chats/{}/menu_tree", args.chat_id);
            api.delete_json(&path, &[], Some(body)).await?
        }
        ChatCommand::Menu(ChatMenuCommand::Sort(args)) => {
            let body = build_repeated_ids_body(
                args.ids,
                args.body_json,
                args.body_file,
                args.stdin,
                "chat_menu_top_level_ids",
                "chat menu sort body",
            )?;
            let path = format!("/im/v1/chats/{}/menu_tree/sort", args.chat_id);
            api.post_json(&path, &[], body).await?
        }
    };
    print_response(raw_json, "chat operation completed", data)
}
