use super::*;

impl FeishuClient {
    pub(in crate::app) async fn send_text(
        &mut self,
        receive_id: &str,
        receive_id_type: &str,
        text: &str,
        uuid: Option<&str>,
    ) -> Result<Value> {
        let mut body = json!({
            "receive_id": receive_id,
            "msg_type": "text",
            "content": json!({ "text": text }).to_string(),
        });
        body["uuid"] = Value::String(uuid.map(ToString::to_string).unwrap_or_else(random_uuid));
        self.post_json(
            "/im/v1/messages",
            &[("receive_id_type".to_string(), receive_id_type.to_string())],
            body,
        )
        .await
    }

    pub(in crate::app) async fn send_interactive(
        &mut self,
        receive_id: &str,
        receive_id_type: &str,
        card: Value,
        uuid: Option<&str>,
    ) -> Result<Value> {
        let mut body = json!({
            "receive_id": receive_id,
            "msg_type": "interactive",
            "content": card.to_string(),
        });
        body["uuid"] = Value::String(uuid.map(ToString::to_string).unwrap_or_else(random_uuid));
        self.post_json(
            "/im/v1/messages",
            &[("receive_id_type".to_string(), receive_id_type.to_string())],
            body,
        )
        .await
    }

    pub(in crate::app) async fn send_message_json(
        &mut self,
        receive_id: &str,
        receive_id_type: &str,
        msg_type: &str,
        content: Value,
        uuid: Option<&str>,
    ) -> Result<Value> {
        let mut body = json!({
            "receive_id": receive_id,
            "msg_type": msg_type,
            "content": content.to_string(),
        });
        body["uuid"] = Value::String(uuid.map(ToString::to_string).unwrap_or_else(random_uuid));
        self.post_json(
            "/im/v1/messages",
            &[("receive_id_type".to_string(), receive_id_type.to_string())],
            body,
        )
        .await
    }

    pub(in crate::app) async fn reply_message_json(
        &mut self,
        message_id: &str,
        msg_type: &str,
        content: Value,
        uuid: Option<&str>,
    ) -> Result<Value> {
        let path = format!("/im/v1/messages/{message_id}/reply");
        let mut body = json!({
            "msg_type": msg_type,
            "content": content.to_string(),
        });
        body["uuid"] = Value::String(uuid.map(ToString::to_string).unwrap_or_else(random_uuid));
        self.post_json(&path, &[], body).await
    }

    pub(in crate::app) async fn edit_message_json(
        &mut self,
        message_id: &str,
        msg_type: &str,
        content: Value,
    ) -> Result<Value> {
        let path = format!("/im/v1/messages/{message_id}");
        self.put_json(
            &path,
            &[],
            json!({
                "msg_type": msg_type,
                "content": content.to_string(),
            }),
        )
        .await
    }

    pub(in crate::app) async fn delete_message(&mut self, message_id: &str) -> Result<Value> {
        let path = format!("/im/v1/messages/{message_id}");
        self.delete_json(&path, &[], None).await
    }

    pub(in crate::app) async fn create_chat(
        &mut self,
        name: &str,
        description: Option<&str>,
        users: &[String],
        user_id_type: &str,
    ) -> Result<Value> {
        let mut body = json!({
            "name": name,
            "chat_mode": "group",
            "chat_type": "private",
            "group_message_type": "chat",
        });
        if let Some(description) = description {
            body["description"] = Value::String(description.to_string());
        }
        if !users.is_empty() {
            body["user_id_list"] =
                Value::Array(users.iter().map(|u| Value::String(u.clone())).collect());
        }
        self.post_json(
            "/im/v1/chats",
            &[("user_id_type".to_string(), user_id_type.to_string())],
            body,
        )
        .await
    }
}
