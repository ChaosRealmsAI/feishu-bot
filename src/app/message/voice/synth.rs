use super::*;

pub(super) fn run_vox_synth(
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
                return Err(error).with_context(|| format!("stat {}", candidate.display()));
            }
        }
    }
    Ok(None)
}
