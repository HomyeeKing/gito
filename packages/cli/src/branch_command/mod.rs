pub mod delete {
    use gito_core::utils::{get_stdout, run_git};

    pub fn run(branch_name: &str, local: bool, remote: bool, both: bool) {
        if local || both {
            let output = run_git(vec!["branch", "-d", branch_name]);
            println!("{}", get_stdout(&output));
        }

        if remote || both {
            let output = run_git(vec!["push", "--delete", "origin", branch_name]);
            println!("delete remote branch successfully! {}", get_stdout(&output));
        }
    }
}
