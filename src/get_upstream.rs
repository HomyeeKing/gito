use std::process::{Command, Output};

use colored::Colorize;
use regex::Regex;
use reqwest::header::USER_AGENT;
extern crate serde_json;

pub async fn run(name: &str) {
    // detect whether given upstream name exists
    let upstream_url = get_stdout(
        &Command::new("git")
            .args(["remote", "get-url", name])
            .output()
            .unwrap(),
    );
    if upstream_url.trim().len() > 0 {
        eprintln!("`{name}` has existed, please check or input a new name");
    } else {
        println!("{}", format!("🔨 Ready to get upstream").yellow());
        let origin_remote = get_stdout(
            &Command::new("git")
                .args(["remote", "get-url", "origin"])
                .output()
                .unwrap(),
        );
        let user_repo = get_user_repo(&origin_remote);
        get_repo_meta_info(&user_repo, name).await.expect(
          "Generate upstream info failed! Please fill an issue at https://github.com/HomyeeKing/gito/issues");
    }
}

fn get_user_repo(remote_url: &str) -> String {
    // r means raw string https://doc.rust-lang.org/stable/reference/tokens.html#raw-string-literals
    let re: Regex = Regex::new(r"^git@github\.com:(.*)\.git$").unwrap();
    let caps = re.captures(remote_url).unwrap();
    return caps[1].to_string();
}

fn get_stdout(output: &Output) -> String {
    return (String::from_utf8(output.stdout.clone()))
        .unwrap()
        .trim()
        .to_string();
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
#[cfg(test)]
mod tests {
    use super::get_user_repo;

    #[test]
    fn user_repo() {
        assert_eq!(
            get_user_repo("git@github.com:HomyeeKing/gx.git"),
            "HomyeeKing/gx"
        );
    }
}
