use super::*;

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

pub(super) fn run_ffmpeg_to_opus(ffmpeg_bin: &Path, input: &Path, output: &Path) -> Result<()> {
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

pub(super) fn ffprobe_duration_ms(ffprobe_bin: &Path, path: &Path) -> Result<u64> {
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
