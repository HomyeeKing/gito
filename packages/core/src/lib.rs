pub mod utils;
use napi_derive::napi;
pub use utils::*;

#[derive(Debug)]
#[napi(object)]
pub struct GitInfo {
  pub username: String,
  pub email: String,
  pub ssh_url: String,
  pub user_repo: String,
  pub current_branch: String,
}

#[napi]
pub fn get_git_info() -> GitInfo {
  let ssh_url = get_stdout(&run_git(vec!["config", "remote.origin.url"])).unwrap();
  let username = get_stdout(&run_git(vec!["config", "user.name"])).unwrap();
  let email = get_stdout(&run_git(vec!["config", "user.email"])).unwrap();
  let current_branch = get_stdout(&run_git(vec!["branch", "--show-current"])).unwrap();

  GitInfo {
    user_repo: get_user_repo(&ssh_url),
    ssh_url,
    username,
    email,
    current_branch,
  }
}
