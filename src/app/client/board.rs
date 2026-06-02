use super::*;

impl FeishuClient {
    pub(in crate::app) async fn import_board_syntax(
        &mut self,
        whiteboard_id: &str,
        syntax: BoardSyntaxArg,
        code: &str,
        style_type: u8,
        diagram_type: u8,
        client_token: Option<String>,
    ) -> Result<Value> {
        let path = format!("/board/v1/whiteboards/{whiteboard_id}/nodes/plantuml");
        let mut query = Vec::new();
        push_query_opt(&mut query, "client_token", client_token);
        self.post_json(
            &path,
            &query,
            json!({
                "plant_uml_code": code,
                "style_type": style_type,
                "syntax_type": syntax.as_api_value(),
                "diagram_type": diagram_type,
            }),
        )
        .await
    }

    pub(in crate::app) async fn create_board_nodes(
        &mut self,
        whiteboard_id: &str,
        body: Value,
        user_id_type: UserIdTypeArg,
        client_token: Option<String>,
    ) -> Result<Value> {
        let path = format!("/board/v1/whiteboards/{whiteboard_id}/nodes");
        let mut query = vec![(
            "user_id_type".to_string(),
            user_id_type.resolve(None).to_string(),
        )];
        push_query_opt(&mut query, "client_token", client_token);
        self.post_json(&path, &query, body).await
    }
}
