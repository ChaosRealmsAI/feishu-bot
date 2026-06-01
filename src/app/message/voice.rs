use super::*;

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

struct PreparedVoiceMessage {
    source_kind: &'static str,
    source_path: PathBuf,
    generated_path: Option<PathBuf>,
    upload_path: PathBuf,
    file_name: String,
    duration_ms: u64,
    used_vox: bool,
    used_ffmpeg: bool,
    temp_dir: Option<PathBuf>,
    cleanup_dir: Option<PathBuf>,
}

struct TempDirCleanup {
    path: Option<PathBuf>,
}

impl TempDirCleanup {
    fn new(path: Option<PathBuf>) -> Self {
        Self { path }
    }
}

impl Drop for TempDirCleanup {
    fn drop(&mut self) {
        if let Some(path) = self.path.take() {
            let _ = fs::remove_dir_all(path);
        }
    }
}

fn prepare_voice_message(args: &SendVoiceMessageArgs) -> Result<PreparedVoiceMessage> {
    let input_count = [
        args.file.is_some(),
        args.text.is_some(),
        args.text_file.is_some(),
        args.stdin,
    ]
    .into_iter()
    .filter(|enabled| *enabled)
    .count();
    if input_count != 1 {
        bail!(
            "message send-voice needs exactly one input: --file, --text, --text-file, or --stdin"
        );
    }

    let mut temp_dir = None;
    let (source_kind, source_path, generated_path, used_vox) =
        if let Some(path) = args.file.as_ref() {
            fs::metadata(path).with_context(|| format!("read {}", path.display()))?;
            ("file", path.clone(), None, false)
        } else {
            let text = read_content(args.text.clone(), args.text_file.clone(), args.stdin)?;
            if text.trim().is_empty() {
                bail!("message send-voice synthesis text cannot be empty");
            }
            let workdir = create_voice_temp_dir()?;
            let generated = match run_vox_synth(
                &args.vox_bin,
                args.voice.as_deref(),
                &text,
                &workdir,
                args.vox_timeout_ms,
            ) {
                Ok(path) => path,
                Err(error) => {
                    if !args.keep {
                        let _ = fs::remove_dir_all(&workdir);
                    }
                    return Err(error);
                }
            };
            temp_dir = Some(workdir);
            ("vox", generated.clone(), Some(generated), true)
        };

    let (upload_path, used_ffmpeg) = if is_opus_path(&source_path) {
        (source_path.clone(), false)
    } else {
        let workdir = ensure_voice_temp_dir(&mut temp_dir)?;
        let output = workdir.join(format!("{}.opus", source_voice_stem(&source_path)));
        run_ffmpeg_to_opus(&args.ffmpeg_bin, &source_path, &output)?;
        (output, true)
    };

    let duration_ms = match args.duration {
        Some(duration) if duration > 0 => duration,
        Some(_) => bail!("message send-voice --duration must be greater than 0"),
        None => ffprobe_duration_ms(&args.ffprobe_bin, &upload_path)?,
    };
    let file_name = args
        .name
        .clone()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| {
            upload_path
                .file_name()
                .and_then(|value| value.to_str())
                .filter(|value| !value.trim().is_empty())
                .unwrap_or("feishu-voice.opus")
                .to_string()
        });
    let cleanup_dir = if args.keep { None } else { temp_dir.clone() };

    Ok(PreparedVoiceMessage {
        source_kind,
        source_path,
        generated_path,
        upload_path,
        file_name,
        duration_ms,
        used_vox,
        used_ffmpeg,
        temp_dir,
        cleanup_dir,
    })
}

fn create_voice_temp_dir() -> Result<PathBuf> {
    let path = std::env::temp_dir().join(format!("feishu-bot-voice-{}", Uuid::new_v4()));
    fs::create_dir_all(&path).with_context(|| format!("create {}", path.display()))?;
    Ok(path)
}

fn ensure_voice_temp_dir(temp_dir: &mut Option<PathBuf>) -> Result<PathBuf> {
    if let Some(path) = temp_dir.as_ref() {
        return Ok(path.clone());
    }
    let path = create_voice_temp_dir()?;
    *temp_dir = Some(path.clone());
    Ok(path)
}

pub(in crate::app) fn is_opus_path(path: &Path) -> bool {
    path.extension()
        .and_then(|value| value.to_str())
        .is_some_and(|value| value.eq_ignore_ascii_case("opus"))
}

pub(in crate::app) fn source_voice_stem(path: &Path) -> String {
    path.file_stem()
        .and_then(|value| value.to_str())
        .filter(|value| !value.trim().is_empty())
        .unwrap_or("feishu-voice")
        .to_string()
}

fn run_vox_synth(
    vox_bin: &Path,
    voice: Option<&str>,
    text: &str,
    workdir: &Path,
    timeout_ms: u64,
) -> Result<PathBuf> {
    if timeout_ms == 0 {
        bail!("message send-voice --vox-timeout-ms must be greater than 0");
    }
    let output_name = "feishu-voice.mp3";
    let candidates = voice_output_candidates(workdir, output_name);
    let mut command = ProcessCommand::new(vox_bin);
    command
        .current_dir(workdir)
        .arg("synth")
        .arg(text)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null());
    if let Some(voice) = voice.filter(|value| !value.trim().is_empty()) {
        command.arg("--voice").arg(voice);
    }
    command.arg("-o").arg(output_name);
    let mut child = command
        .spawn()
        .with_context(|| format!("run {}", vox_bin.display()))?;
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(timeout_ms);
    let mut last_seen: Option<(PathBuf, u64, std::time::Instant)> = None;

    loop {
        if let Some((candidate, len)) = first_nonempty_candidate(&candidates)? {
            if let Some((last_path, last_len, since)) = &last_seen {
                if *last_path == candidate
                    && *last_len == len
                    && since.elapsed() >= std::time::Duration::from_millis(400)
                {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Ok(candidate);
                }
            }
            last_seen = Some((candidate, len, std::time::Instant::now()));
        }

        if let Some(status) = child.try_wait().context("poll vox process")? {
            if let Some((candidate, _)) = first_nonempty_candidate(&candidates)? {
                return Ok(candidate);
            }
            if status.success() {
                bail!(
                    "vox exited successfully but did not create {}; checked {}",
                    output_name,
                    candidates
                        .iter()
                        .map(|path| path.display().to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
            bail!("vox exited with status {status}");
        }

        if std::time::Instant::now() >= deadline {
            if let Some((candidate, _)) = first_nonempty_candidate(&candidates)? {
                let _ = child.kill();
                let _ = child.wait();
                return Ok(candidate);
            }
            let _ = child.kill();
            let _ = child.wait();
            bail!("vox synth timed out after {timeout_ms} ms");
        }
        std::thread::sleep(std::time::Duration::from_millis(150));
    }
}

pub(in crate::app) fn voice_output_candidates(workdir: &Path, output_name: &str) -> Vec<PathBuf> {
    let output = Path::new(output_name);
    let stem = output
        .file_stem()
        .and_then(|value| value.to_str())
        .filter(|value| !value.trim().is_empty())
        .unwrap_or("feishu-voice");
    vec![
        workdir.join(output_name),
        workdir.join(stem).join(output_name),
    ]
}

fn first_nonempty_candidate(candidates: &[PathBuf]) -> Result<Option<(PathBuf, u64)>> {
    for candidate in candidates {
        match fs::metadata(candidate) {
            Ok(metadata) if metadata.is_file() && metadata.len() > 0 => {
                return Ok(Some((candidate.clone(), metadata.len())));
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(error).with_context(|| format!("stat {}", candidate.display()))
            }
        }
    }
    Ok(None)
}

fn run_ffmpeg_to_opus(ffmpeg_bin: &Path, input: &Path, output: &Path) -> Result<()> {
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    let status = ProcessCommand::new(ffmpeg_bin)
        .arg("-y")
        .arg("-i")
        .arg(input)
        .arg("-vn")
        .arg("-c:a")
        .arg("libopus")
        .arg("-b:a")
        .arg("32k")
        .arg("-ar")
        .arg("48000")
        .arg(output)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .with_context(|| format!("run {}", ffmpeg_bin.display()))?;
    if !status.success() {
        bail!("ffmpeg exited with status {status}");
    }
    fs::metadata(output).with_context(|| format!("read {}", output.display()))?;
    Ok(())
}

fn ffprobe_duration_ms(ffprobe_bin: &Path, path: &Path) -> Result<u64> {
    let output = ProcessCommand::new(ffprobe_bin)
        .arg("-v")
        .arg("error")
        .arg("-show_entries")
        .arg("format=duration")
        .arg("-of")
        .arg("default=noprint_wrappers=1:nokey=1")
        .arg(path)
        .output()
        .with_context(|| format!("run {}", ffprobe_bin.display()))?;
    if !output.status.success() {
        bail!(
            "ffprobe exited with status {}: {}",
            output.status,
            command_stderr_summary(&output.stderr)
        );
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let seconds: f64 = text
        .trim()
        .parse()
        .with_context(|| format!("parse ffprobe duration from {:?}", text.trim()))?;
    if !seconds.is_finite() || seconds <= 0.0 {
        bail!("ffprobe returned invalid duration: {seconds}");
    }
    Ok((seconds * 1000.0).round() as u64)
}

fn command_stderr_summary(stderr: &[u8]) -> String {
    let text = String::from_utf8_lossy(stderr).trim().to_string();
    if text.is_empty() {
        return "<empty stderr>".to_string();
    }
    text.chars().take(300).collect()
}
