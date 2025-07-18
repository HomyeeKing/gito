use crate::utils::*;
use gito_core::utils::run_git;

pub fn run(alias: &str) {
    let git_accounts = safe_get_gito_config("account");
    if git_accounts.section(Some(alias)).is_some() {
        let account = git_accounts.section(Some(alias)).unwrap();
        let git_name = account.get("name").unwrap_or_default();
        let git_email = account.get("email").unwrap_or_default();

        let amend_status = run_git(vec![
            "commit",
            "--amend",
            "--author",
            format!(r#"'{git_name} <{git_email}>'"#).as_str(),
            "--no-edit",
        ]);
        if amend_status.status.success() {
            run_git(vec!["rebase", "--continue"]);
        } else {
            bug_report("gito amend");
        }
        println!("the commit has been amend with {} {}", git_name, git_email);
    } else {
        println!("Invalid alias");
    }
}
