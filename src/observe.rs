use std::env;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

mod util;

fn main() {
	let args: Vec<String> = env::args().collect();

	let mut status = "";
	let mut file_path = "";

	let mut i = 0;
	while i + 1 < args.len() {
		if args[i] == "status" {
			status = &args[i + 1];
		}
		if args[i] == "file" {
			file_path = &args[i + 1];
		}
		i += 1;
	}

	if status != "playing" || file_path.is_empty() {
		return;
	}

	let exe_path = env::current_exe().unwrap();
	let base_dir = exe_path.parent().unwrap();
	let covers_dir = base_dir.join(".cover");

	fs::create_dir_all(&covers_dir).ok();
	for entry in fs::read_dir(&covers_dir).unwrap().flatten() {
		fs::remove_file(entry.path()).ok();
	}

	let timestamp = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_secs()
		.to_string();

	let cover_file = covers_dir.join(format!("{}.jpg", timestamp));
	let cover_file_str = cover_file.to_str().unwrap();

	if util::extract_cover(file_path, cover_file_str) {
		return;
	}

	let file_dir = Path::new(file_path).parent().unwrap();
	let cover_jpg = file_dir.join("cover.jpg");
	let folder_jpg = file_dir.join("folder.jpg");

	if cover_jpg.exists() {
		fs::copy(cover_jpg, cover_file).ok();
	} else if folder_jpg.exists() {
		fs::copy(folder_jpg, cover_file).ok();
	}
}
