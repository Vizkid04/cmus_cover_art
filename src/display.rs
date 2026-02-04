use std::{
	fs,
	path::Path,
	process::{Command, Stdio},
	thread,
	time::Duration,
};

const COVERS_DIR: &str =
	"/home/vizkid/.config/cmus/cmus-cover-art/target/release/.cover";

const FALLBACK_IMAGE: &str =
	"/home/vizkid/.config/cmus/cmus-cover-art/Violet_Evergarden.jpg";

const IMAGE_VIEWER: &str =
	"kitten icat --place=65x60@3x1 --scale-up";

fn kill_child_processes() {
	let _ = Command::new("pkill")
		.arg("-TERM")
		.arg("-P")
		.arg(std::process::id().to_string())
		.status();
}

fn clear_screen() {
	let _ = Command::new("clear").status();
}

fn list_cover(covers: &Path) -> Option<String> {
	let mut entries = fs::read_dir(covers).ok()?;
	let entry = entries.next()?.ok()?;
	Some(entry.file_name().to_string_lossy().to_string())
}

fn show_image(path: &str) {
	let mut parts = IMAGE_VIEWER.split_whitespace();
	let cmd = parts.next().unwrap();
	let args: Vec<_> = parts.collect();

	let _ = Command::new(cmd)
		.args(args)
		.arg(path)
		.stdin(Stdio::inherit())
		.stdout(Stdio::inherit())
		.stderr(Stdio::inherit())
		.status();
}

fn main() {
	let covers = Path::new(COVERS_DIR);

	let mut previous = String::new();
	let mut first = false;

	clear_screen();

	loop {
		let current = list_cover(covers).unwrap_or_default();

		if current != previous {
			kill_child_processes();
			clear_screen();

			if !current.is_empty() {
				show_image(&format!("{}/{}", COVERS_DIR, current));
				previous = current.clone();
			} else {
				show_image(FALLBACK_IMAGE);
				previous.clear();
			}
		}

		if !first && current.is_empty() {
			kill_child_processes();
			clear_screen();
			show_image(FALLBACK_IMAGE);
			first = true;
		}

		thread::sleep(Duration::from_secs(1));
	}
}
