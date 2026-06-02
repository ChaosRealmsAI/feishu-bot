use super::*;

mod audio;
mod prepare;
mod synth;

use prepare::{prepare_voice_message, TempDirCleanup};

#[cfg(test)]
pub(in crate::app) use audio::{is_opus_path, source_voice_stem};
#[cfg(test)]
pub(in crate::app) use synth::voice_output_candidates;

pub(in crate::app) async fn run_message_send_voice(
    api: &mut FeishuClient,
    args: SendVoiceMessageArgs,
) -> Result<Value> {
    let prepared = prepare_voice_message(&args)?;
    let _cleanup = TempDirCleanup::new(prepared.cleanup_dir.clone());
    let receive_id_type = args.to_type.resolve(&args.to).to_string();
    let uploaded = api
        .upload_im_file(
            &prepared.upload_path,
            prepared.file_name.clone(),
            "opus",
            Some(prepared.duration_ms),
        )
        .await?;
    let file_key = get_string(&uploaded, &["data", "file_key"])
        .ok_or_else(|| anyhow!("upload voice response missing file_key: {uploaded}"))?;
    let content = build_uploaded_file_message_content(
        &file_key,
        &prepared.file_name,
        "audio",
        Some(prepared.duration_ms),
        None,
    );
    let sent = api
        .send_message_json(
            &args.to,
            &receive_id_type,
            "audio",
            content,
            args.uuid.as_deref(),
        )
        .await?;
    let message_id = get_string(&sent, &["data", "message_id"]);
    let message_get = if args.readback {
        let id = message_id
            .as_deref()
            .ok_or_else(|| anyhow!("send voice response missing message_id: {sent}"))?;
        let path = format!("/im/v1/messages/{}", encode_path_segment(id));
        Some(probe_value(
            api.get_json(
                &path,
                &[("user_id_type".to_string(), "open_id".to_string())],
            )
            .await,
        ))
    } else {
        None
    };

    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "receive_id": args.to,
            "receive_id_type": receive_id_type,
            "message_id": message_id,
            "file_key": file_key,
            "file_name": prepared.file_name,
            "duration_ms": prepared.duration_ms,
            "voice": {
                "source_kind": prepared.source_kind,
                "source_path": prepared.source_path.display().to_string(),
                "generated_path": prepared.generated_path.as_ref().map(|path| path.display().to_string()),
                "upload_path": prepared.upload_path.display().to_string(),
                "used_vox": prepared.used_vox,
                "used_ffmpeg": prepared.used_ffmpeg,
                "temp_dir": prepared.temp_dir.as_ref().map(|path| path.display().to_string()),
                "kept_temp_dir": args.keep,
            },
            "upload": uploaded,
            "sent": sent,
            "message_get": message_get,
        }
    }))
}
