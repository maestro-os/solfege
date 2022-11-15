//! This module handles the fstab file, which contains the list of filesystems to mount at boot.

use std::error::Error;
use std::ffi::c_void;
use std::ffi::CString;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::Peekable;
use std::ptr::null;
use std::str::Chars;

/// The path to the fstab file.
const FSTAB_PATH: &str = "/etc/fstab";

/// Enumeration of possible filesystem sources types.
#[derive(Debug, Eq, PartialEq)]
pub enum FSSpec {
	/// Mounting from a file.
	File(String),
	/// Mounting from the given label.
	Label(String),
	/// Mounting from a partition UUID.
	Uuid(String),
}

impl FSSpec {
	/// Returns the value corresponding to the given string.
	pub fn from_str(s: String) -> Self {
		if s.starts_with("LABEL=") {
			Self::Label(String::from(&s[6..]))
		} else if s.starts_with("UUID=") {
			Self::Uuid(String::from(&s[5..]))
		} else {
			Self::File(s)
		}
	}

	/// Returns a string corresponding to the spec.
	pub fn as_str(&self) -> String {
		match self {
			Self::File(s) => s.clone(),
			Self::Label(s) => format!("LABEL={}", s),
			Self::Uuid(s) => format!("UUID={}", s),
		}
	}
}

/// Structure representing an entry in the fstab file.
pub struct FSTabEntry {
	/// The source of the filesystem.
	fs_spec: FSSpec,
	/// The file on which the filesystem will be mounted.
	fs_file: String,
	/// The filesystem type.
	fs_vfstype: String,
	/// The mount options associated with the filesystem.
	fs_mntops: Vec<String>,
	/// Tells whether the filesystem has to be dumped.
	fs_freq: bool,
	/// Tells the order in which fsck checks the filesystems.
	fs_passno: u32,
}

extern "C" {
	/// Mounts the given filesystem.
	fn mount_fs(
		source: *const i8,
		target: *const i8,
		filesystemtype: *const i8,
		mountflags: u32,
		data: *const c_void,
	) -> i32;
}

impl FSTabEntry {
	/// Returns the mountpath.
	pub fn get_path(&self) -> &String {
		&self.fs_file
	}

	/// Mounts the given entry.
	pub fn mount(&self) -> Result<(), Box<dyn Error>> {
		let result = unsafe {
			mount_fs(
				CString::new(self.fs_spec.as_str())?.as_ptr(),
				CString::new(self.fs_file.clone())?.as_ptr(),
				CString::new(self.fs_vfstype.clone())?.as_ptr(),
				0,      // TODO
				null(), // TODO
			)
		};

		if result != 0 {
			Ok(())
		} else {
			Err(format!(
				"Failed to mount `{}` into `{}`!",
				self.fs_spec.as_str(),
				self.fs_file
			)
			.into())
		}
	}
}

/// Skips whitespace on the given iterator.
fn skip_whitespaces(chars: &mut Peekable<Chars>) {
	while let Some(c) = chars.peek() {
		if !c.is_whitespace() {
			break;
		}

		chars.next();
	}
}

/// Consumes a token from the given chars iterator.
/// If the token is invalid, the function returns None.
fn consume_token(chars: &mut Peekable<Chars>) -> Option<String> {
	// The token
	let mut tok = String::new();
	// Tells whether a quote is open
	let mut quote = false;

	while let Some(c) = chars.peek() {
		if !quote && c.is_whitespace() {
			break;
		}

		match c {
			'"' => {
				quote = !quote;
				chars.next();
			}

			'\\' => {
				chars.next();

				if let Some(c) = chars.next() {
					tok.push(c);
					continue;
				} else {
					return None;
				}
			}

			_ => {
				tok.push(*c);
				chars.next();
			}
		}
	}

	if !quote {
		Some(tok)
	} else {
		None
	}
}

/// Parses the given line.
/// If no entry is present on the line or if the entry is invalid, the function returns None.
fn parse_line(line: &str) -> Option<FSTabEntry> {
	if line.is_empty() {
		return None;
	}

	let mut fs_spec = None;
	let mut fs_file = None;
	let mut fs_vfstype = None;
	let mut fs_mntops = None;
	let mut fs_freq = None;
	let mut fs_passno = None;

	// The current index in the entry
	let mut i = 0;

	let mut chars = line.chars().peekable();
	while chars.peek().is_some() {
		skip_whitespaces(&mut chars);

		if let Some(c) = chars.peek() {
			// On comment, stop parsing the line
			if *c == '#' {
				break;
			}
		} else {
			break;
		}

		// Get the next token
		let tok = consume_token(&mut chars)?;

		match i {
			// fs_spec
			0 => fs_spec = Some(FSSpec::from_str(tok)),

			// fs_file
			1 => fs_file = Some(tok),

			// fs_vfstype
			2 => fs_vfstype = Some(tok),

			// fs_mntops
			3 => fs_mntops = Some(tok.split(',').map(|s| s.to_owned()).collect::<Vec<_>>()),

			// fs_freq
			4 => fs_freq = Some(tok != "0"),

			// fs_passno
			5 => fs_passno = Some(tok.parse::<u32>().ok()?),

			// If the line has too much entries, ignore it
			_ => return None,
		}

		i += 1;
	}

	Some(FSTabEntry {
		fs_spec: fs_spec?,
		fs_file: fs_file?,
		fs_vfstype: fs_vfstype?,
		fs_mntops: fs_mntops?,
		fs_freq: fs_freq?,
		fs_passno: fs_passno?,
	})
}

/// Parses the fstab file and returns the list of entries.
/// `path` is the path to the fstab file. If None, the function takes the default path.
/// Invalid entries are ignored.
pub fn parse(path: Option<&str>) -> io::Result<Vec<FSTabEntry>> {
	let mut entries = Vec::new();

	let file = File::open(path.unwrap_or(FSTAB_PATH))?;
	let reader = BufReader::new(file);

	for l in reader.lines() {
		let l = l?;

		if let Some(entry) = parse_line(&l) {
			entries.push(entry);
		}
	}

	Ok(entries)
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn fstab_empty() {
		let entries = parse(Some("tests/fstab/empty")).unwrap();
		assert!(entries.is_empty());
	}

	#[test]
	fn fstab_comments_only() {
		let entries = parse(Some("tests/fstab/comments_only")).unwrap();
		assert!(entries.is_empty());
	}

	#[test]
	fn fstab_single() {
		let entries = parse(Some("tests/fstab/single")).unwrap();
		assert_eq!(entries.len(), 1);

		assert_eq!(entries[0].fs_spec, FSSpec::File("/dev/sda1".to_string()));
		assert_eq!(entries[0].fs_file, "/");
		assert_eq!(entries[0].fs_vfstype, "ext4");
		assert_eq!(entries[0].fs_mntops[0], "rw");
		assert_eq!(entries[0].fs_freq, false);
		assert_eq!(entries[0].fs_passno, 1);
	}

	#[test]
	fn fstab_several() {
		let entries = parse(Some("tests/fstab/several")).unwrap();
		assert_eq!(entries.len(), 3);

		assert_eq!(entries[0].fs_spec, FSSpec::File("/dev/sda1".to_string()));
		assert_eq!(entries[0].fs_file, "/");
		assert_eq!(entries[0].fs_vfstype, "ext4");
		assert_eq!(entries[0].fs_mntops[0], "rw");
		assert_eq!(entries[0].fs_freq, false);
		assert_eq!(entries[0].fs_passno, 1);

		assert_eq!(entries[1].fs_spec, FSSpec::Label("UEFI".to_string()));
		assert_eq!(entries[1].fs_file, "/");
		assert_eq!(entries[1].fs_vfstype, "ext4");
		assert_eq!(entries[1].fs_mntops[0], "defaults");
		assert_eq!(entries[1].fs_mntops[1], "rw");
		assert_eq!(entries[1].fs_freq, false);
		assert_eq!(entries[1].fs_passno, 1);

		assert_eq!(
			entries[2].fs_spec,
			FSSpec::Uuid("5fcd5a6e-a326-43fd-8b39-f6e1238bc54f".to_string())
		);
		assert_eq!(entries[2].fs_file, "/");
		assert_eq!(entries[2].fs_vfstype, "ext4");
		assert_eq!(entries[2].fs_mntops[0], "suid");
		assert_eq!(entries[2].fs_mntops[1], "rw");
		assert_eq!(entries[2].fs_freq, false);
		assert_eq!(entries[2].fs_passno, 1);
	}

	// TODO Test with quotes
	// TODO Test with invalid entries
}
