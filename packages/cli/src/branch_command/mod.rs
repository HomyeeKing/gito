pub mod delete {
    use gito_core::utils::{get_stdout, run_git};

    pub fn run(branch_name: &str, force_delete: bool) {
        println!("start deleting {branch_name}");
        let output = run_git(vec![
            "branch",
            if force_delete { "-D" } else { "-d" },
            branch_name,
        ]);
        println!("{}", get_stdout(&output));
        
        let output = run_git(vec!["push", "--delete", "origin", branch_name]);
        println!("delete remote branch successfully! {}", get_stdout(&output));
    }
}
