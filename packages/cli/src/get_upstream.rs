use colored::Colorize;
use gito_core::{
    utils::{get_stdout, run_git},
    GitInfo,
};
use reqwest::header::USER_AGENT;
use std::process::Command;
extern crate serde_json;

pub async fn run(name: &str, git_info: &GitInfo) {
    // detect whether given upstream name exists
    let upstream_url = get_stdout(&run_git(vec!["remote", "get-url", name])).unwrap_or_default();
    if upstream_url.trim().len() > 0 {
        eprintln!("`{name}` has existed, please check or input a new name");
    } else {
        println!("{}", format!("🔨 Ready to get upstream").yellow());
        get_repo_meta_info(&git_info.user_repo, name).await.expect(
          "Generate upstream info failed! Please fill an issue at https://github.com/HomyeeKing/gito/issues");
    }
}

async fn get_repo_meta_info(
    user_repo: &str,
    upstream_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let req_client = reqwest::Client::new();
    let res = req_client
        .get(format!("https://api.github.com/repos/{}", user_repo))
        .header(USER_AGENT, "GX")
        .send()
        .await?;
    if res.status().is_success() {
        let body = res.json::<serde_json::Value>().await?;
        let is_forked = serde_json::from_value(body.get("fork").unwrap().to_owned()).unwrap();
        if is_forked {
            let parent_ssh_url: String = serde_json::from_value(
                body.get("parent")
                    .unwrap()
                    .get("ssh_url")
                    .unwrap()
                    .to_owned(),
            )
            .unwrap();

            Command::new("git")
                .args(["remote", "add", upstream_name, &parent_ssh_url])
                .spawn()
                .expect("set upstream url failed")
                .wait()
                .expect("set upstream url failed");

            println!(
                "{}",
                format!("✅ ADD UPSTREAM SUCCESSFULLY. THE LATEST REMOTES ARE:").green()
            );

            Command::new("git")
                .args(["remote", "-v"])
                .spawn()
                .expect("check latest git remote failed");
        } else {
            println!("{} is not a forked repo.", user_repo);
        }
    } else {
        println!("Something else happened. Status: {:?}", res.status());
    }
    Ok(())
}
