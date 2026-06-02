use super::audio::{ffprobe_duration_ms, is_opus_path, run_ffmpeg_to_opus, source_voice_stem};
use super::synth::run_vox_synth;
use super::*;

pub(super) struct PreparedVoiceMessage {
    pub(super) source_kind: &'static str,
    pub(super) source_path: PathBuf,
    pub(super) generated_path: Option<PathBuf>,
    pub(super) upload_path: PathBuf,
    pub(super) file_name: String,
    pub(super) duration_ms: u64,
    pub(super) used_vox: bool,
    pub(super) used_ffmpeg: bool,
    pub(super) temp_dir: Option<PathBuf>,
    pub(super) cleanup_dir: Option<PathBuf>,
}

pub(super) struct TempDirCleanup {
    path: Option<PathBuf>,
}

impl TempDirCleanup {
    pub(super) fn new(path: Option<PathBuf>) -> Self {
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

pub(super) fn prepare_voice_message(args: &SendVoiceMessageArgs) -> Result<PreparedVoiceMessage> {
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
