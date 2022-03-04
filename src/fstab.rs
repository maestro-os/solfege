//! This module handles the fstab file, which contains the list of filesystems to mount at boot.

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io;
use std::iter::Peekable;
use std::str::Chars;

/// The path to the fstab file.
const FSTAB_PATH: &str = "/etc/fstab";

/// Enumeration of possible filesystem sources types.
pub enum FSSpec {
    /// Mounting from a file.
    File(String),
    /// Mounting from the given label.
    Label(String),
    /// mountiing from a partition UUID.
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
            },

            '\\' => {
                chars.next();

                if let Some(c) = chars.next() {
                    tok.push(c);
                    continue;
                } else {
                    return None;
                }
            },

            _ => {
                tok.push(*c);
                chars.next();
            },
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
            3 => fs_mntops = Some(tok.split(',').map(| s | s.to_owned()).collect::<Vec<_>>()),

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
/// Invalid entries are ignored.
pub fn parse() -> io::Result<Vec<FSTabEntry>> {
    let mut entries = Vec::new();

    let file = File::open(FSTAB_PATH)?;
    let reader = BufReader::new(file);

    for l in reader.lines() {
        let l = l?;

        if let Some(entry) = parse_line(&l) {
            entries.push(entry);
        }
    }

    Ok(entries)
}
