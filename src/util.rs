use std::process::Command;

pub fn extract_cover(input: &str, output: &str) -> bool {
	Command::new("ffmpeg")
		.args([
			"-loglevel",
			"error",
			"-y",
			"-i",
			input,
			"-map",
			"0:v:0",
			"-frames:v",
			"1",
			output,
		])
		.status()
		.map(|s| s.success())
		.unwrap_or(false)
}
