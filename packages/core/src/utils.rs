use std::process::Command;
pub use std::process::Output;

use regex::Regex;

/**
 * return stdout when success
 * or return stderr
 */
pub fn get_stdout(output: &Output) -> Option<String> {
  if output.status.success() {
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
  } else {
    eprintln!("Command failed: {}", String::from_utf8_lossy(&output.stderr).trim().to_string());
    None
  }
}

/**
 * git@github.com:HomyeeKing/gito.git -> HomyeeKing/gito
 */
pub fn get_user_repo(remote_url: &str) -> String {
  // r means raw string https://doc.rust-lang.org/stable/reference/tokens.html#raw-string-literals
  let re: Regex = Regex::new(r"^git@.*\.com:(.*)\.git$").unwrap();
  let caps = re.captures(remote_url).unwrap();
  return caps[1].to_string();
}

pub fn run_command(program: &str, args: Vec<&str>) -> Output {
  Command::new(program).args(args).output().unwrap()
}

pub fn run_git(args: Vec<&str>) -> Output {
  run_command("git", args)
}
