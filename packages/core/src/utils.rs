use std::process::Command;
pub use std::process::Output;

use regex::Regex;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn get_stdout(output: &Output) -> String {
    return (String::from_utf8_lossy(&output.stdout)).trim().to_string();
}

/**
 * git@github.com:HomyeeKing/gito.git -> HomyeeKing/gito
 */
pub fn get_user_repo(remote_url: &str) -> String {
    // r means raw string https://doc.rust-lang.org/stable/reference/tokens.html#raw-string-literals
    let re: Regex = Regex::new(r"^git@github\.com:(.*)\.git$").unwrap();
    let caps = re.captures(remote_url).unwrap();
    return caps[1].to_string();
}

pub fn run_command(program: &str, args: Vec<&str>) -> Output {
    Command::new(program).args(args).output().unwrap()
}

pub fn run_git(args: Vec<&str>) -> Output {
    run_command("git", args)
}
