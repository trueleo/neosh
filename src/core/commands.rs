//! All built-in commands available in neosh

use std::{env, path::{Path, PathBuf}, str::SplitWhitespace};
type EEditor = rustyline::Editor<()>;

/// Exit shell
///
/// Actually, does nothing but saving last cmd in history
pub fn exit<'l>(editor: &mut EEditor, line: &'l String) {
    editor.add_history_entry(line);
}

/// Change current working directory
// https://unix.stackexchange.com/a/38809
pub fn cd<'l>(
    editor: &mut EEditor,
    line: &'l String,
    args: SplitWhitespace,
) {
    editor.add_history_entry(line);
    let home_dir = dirs::home_dir().unwrap();

    let next_dir = args.peekable().peek().map_or(home_dir, PathBuf::from);
    let next_dir = Path::new(&next_dir);

    if let Err(err) = env::set_current_dir(next_dir) {
        eprintln!("{}", err);
    }
}

/// Print current working directory
// NOTE: I am not importing cwd from main.rs because we might change structure (Shift)
pub fn pwd<'l>(
    editor: &mut EEditor,
    line: &'l String,
) {
    editor.add_history_entry(line);
    println!("{}", env::current_dir().unwrap().into_os_string().into_string().unwrap());
}