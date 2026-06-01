use std::fs;

use base64::Engine;

use super::*;

pub(super) async fn run_mail_command(
    api: &mut FeishuClient,
    command: MailCommand,
    raw_json: bool,
) -> Result<()> {
    let data = match command {
        MailCommand::Message(MailMessageCommand::List(args)) => {
            if args.page_size == 0 || args.page_size > 20 {
                bail!("mail message list page_size must be between 1 and 20");
            }
            let mailbox = args.mailbox.mailbox;
            let mut query = vec![("page_size".to_string(), args.page_size.to_string())];
            push_query_opt(&mut query, "page_token", args.page_token);
            push_query_opt(&mut query, "folder_id", args.folder_id);
            push_query_opt(&mut query, "label_id", args.label_id);
            if args.only_unread {
                query.push(("only_unread".to_string(), "true".to_string()));
            }
            let path = format!(
                "/mail/v1/user_mailboxes/{}/messages",
                encode_path_segment(&mailbox)
            );
            mail_get_json(api, &path, &query, args.mailbox.auth, &mailbox).await?
        }
        MailCommand::Message(MailMessageCommand::Get(args)) => {
            let mailbox = args.mailbox.mailbox;
            let path = format!(
                "/mail/v1/user_mailboxes/{}/messages/{}",
                encode_path_segment(&mailbox),
                encode_path_segment(&args.message_id)
            );
            let query = vec![("format".to_string(), args.format)];
            mail_get_json(api, &path, &query, args.mailbox.auth, &mailbox).await?
        }
        MailCommand::Message(MailMessageCommand::Send(args)) => {
            let mailbox = args.mailbox.clone();
            let path = format!(
                "/mail/v1/user_mailboxes/{}/messages/send",
                encode_path_segment(&mailbox)
            );
            let body = build_mail_send_body(args)?;
            api.post_json_user(&path, &[], body).await?
        }
        MailCommand::Message(MailMessageCommand::GetByCard(args)) => {
            let mailbox = args.mailbox.mailbox;
            let path = format!(
                "/mail/v1/user_mailboxes/{}/messages/get_by_card",
                encode_path_segment(&mailbox)
            );
            let query = vec![
                ("card_id".to_string(), args.card_id),
                ("owner_id".to_string(), args.owner_id),
                (
                    "user_id_type".to_string(),
                    args.user_id_type.resolve(None).to_string(),
                ),
            ];
            mail_get_json(api, &path, &query, args.mailbox.auth, &mailbox).await?
        }
        MailCommand::Folder(MailFolderCommand::List(args)) => {
            if args
                .folder_type
                .is_some_and(|kind| !(1..=2).contains(&kind))
            {
                bail!("folder_type must be 1 or 2");
            }
            let mailbox = args.mailbox.mailbox;
            let mut query = Vec::new();
            if let Some(folder_type) = args.folder_type {
                query.push(("folder_type".to_string(), folder_type.to_string()));
            }
            let path = format!(
                "/mail/v1/user_mailboxes/{}/folders",
                encode_path_segment(&mailbox)
            );
            mail_get_json(api, &path, &query, args.mailbox.auth, &mailbox).await?
        }
        MailCommand::Contact(MailContactCommand::List(args)) => {
            if args.page_size == 0 || args.page_size > 20 {
                bail!("mail contact list page_size must be between 1 and 20");
            }
            let mailbox = args.mailbox.mailbox;
            let mut query = vec![("page_size".to_string(), args.page_size.to_string())];
            push_query_opt(&mut query, "page_token", args.page_token);
            let path = format!(
                "/mail/v1/user_mailboxes/{}/mail_contacts",
                encode_path_segment(&mailbox)
            );
            mail_get_json(api, &path, &query, args.mailbox.auth, &mailbox).await?
        }
        MailCommand::Alias(MailAliasCommand::List(args)) => {
            if args.page_size == 0 || args.page_size > 20 {
                bail!("mail alias list page_size must be between 1 and 20");
            }
            let mut query = vec![("page_size".to_string(), args.page_size.to_string())];
            push_query_opt(&mut query, "page_token", args.page_token);
            let path = format!(
                "/mail/v1/user_mailboxes/{}/aliases",
                encode_path_segment(&args.mailbox)
            );
            api.get_json(&path, &query).await?
        }
        MailCommand::Settings(MailSettingsCommand::SendAs(args)) => {
            let mailbox = args.mailbox.mailbox;
            let path = format!(
                "/mail/v1/user_mailboxes/{}/settings/send_as",
                encode_path_segment(&mailbox)
            );
            mail_get_json(api, &path, &[], args.mailbox.auth, &mailbox).await?
        }
        MailCommand::Settings(MailSettingsCommand::Accessible(args)) => {
            let mailbox = args.mailbox.mailbox;
            let path = format!(
                "/mail/v1/user_mailboxes/{}/accessible_mailboxes",
                encode_path_segment(&mailbox)
            );
            mail_get_json(api, &path, &[], args.mailbox.auth, &mailbox).await?
        }
        MailCommand::Rule(MailRuleCommand::List(args)) => {
            let mailbox = args.mailbox.mailbox;
            let path = format!(
                "/mail/v1/user_mailboxes/{}/rules",
                encode_path_segment(&mailbox)
            );
            mail_get_json(api, &path, &[], args.mailbox.auth, &mailbox).await?
        }
        MailCommand::Label(MailLabelCommand::Get(args)) => {
            let mailbox = args.mailbox.mailbox;
            let path = format!(
                "/mail/v1/user_mailboxes/{}/labels/{}",
                encode_path_segment(&mailbox),
                encode_path_segment(&args.label_id)
            );
            mail_get_json(api, &path, &[], args.mailbox.auth, &mailbox).await?
        }
    };
    print_response(raw_json, "mail operation completed", data)
}

async fn mail_get_json(
    api: &mut FeishuClient,
    path: &str,
    query: &[(String, String)],
    auth: MailAuthArg,
    mailbox: &str,
) -> Result<Value> {
    if mail_should_use_user(auth, mailbox)? {
        api.get_json_user(path, query).await
    } else {
        api.get_json(path, query).await
    }
}

pub(super) fn mail_should_use_user(auth: MailAuthArg, mailbox: &str) -> Result<bool> {
    match auth {
        MailAuthArg::User => Ok(true),
        MailAuthArg::Tenant => {
            if mailbox == "me" {
                bail!(
                    "mailbox=me requires --auth user or --auth auto with FEISHU_USER_ACCESS_TOKEN"
                );
            }
            Ok(false)
        }
        MailAuthArg::Auto => Ok(mailbox == "me"),
    }
}

fn mail_address_array(values: Vec<String>) -> Value {
    Value::Array(
        clean_string_values(values)
            .into_iter()
            .map(|mail_address| json!({ "mail_address": mail_address }))
            .collect(),
    )
}

pub(super) fn build_mail_send_body(args: MailMessageSendArgs) -> Result<Value> {
    if has_json_input(&args.body_json, &args.file, args.stdin) {
        return ensure_json_object(
            read_json_value(args.body_json, args.file, args.stdin)?,
            "mail send body",
        );
    }

    let mut body = Map::new();
    insert_opt_string(&mut body, "subject", args.subject);
    let to = clean_string_values(args.to);
    let cc = clean_string_values(args.cc);
    let bcc = clean_string_values(args.bcc);
    if !to.is_empty() {
        body.insert("to".to_string(), mail_address_array(to));
    }
    if !cc.is_empty() {
        body.insert("cc".to_string(), mail_address_array(cc));
    }
    if !bcc.is_empty() {
        body.insert("bcc".to_string(), mail_address_array(bcc));
    }
    insert_opt_string(&mut body, "body_plain_text", args.text);
    insert_opt_string(&mut body, "body_html", args.html);
    insert_opt_string(&mut body, "dedupe_key", args.dedupe_key);

    if let Some(raw) = args.raw_base64url.filter(|value| !value.trim().is_empty()) {
        body.insert("raw".to_string(), Value::String(raw));
    }
    if let Some(path) = args.raw_file {
        let bytes = fs::read(&path).with_context(|| format!("read {}", path.display()))?;
        let encoded = base64::engine::general_purpose::URL_SAFE.encode(bytes);
        body.insert("raw".to_string(), Value::String(encoded));
    }
    if args.from_address.is_some() || args.from_name.is_some() {
        let mut head_from = Map::new();
        insert_opt_string(&mut head_from, "mail_address", args.from_address);
        insert_opt_string(&mut head_from, "name", args.from_name);
        body.insert("head_from".to_string(), Value::Object(head_from));
    }
    if !body.contains_key("raw") && !body.contains_key("to") {
        bail!("mail send requires --to unless --raw-base64url, --raw-file, or raw --body-json is used");
    }
    if !body.contains_key("raw")
        && !body.contains_key("body_plain_text")
        && !body.contains_key("body_html")
    {
        bail!("mail send requires --text, --html, --raw-base64url, --raw-file, or raw --body-json");
    }
    Ok(Value::Object(body))
}
