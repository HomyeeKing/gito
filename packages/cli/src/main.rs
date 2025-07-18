mod utils;
mod constants; // Add this line
//  command
mod amend_command;
mod branch_command;
mod get_upstream;
mod init_command;
mod open_command;
mod user_command;
// extern
use clap::{Args, Parser, Subcommand};
use gito_core::get_git_info;

#[derive(Parser)]
#[command(name = "gito")]
#[command(about="Git command enhancement CLI", long_about = None, version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "get the upstream remote-url", alias = "gup")]
    GetUpstream {
        #[arg(short = 'n', long = "remote-name", default_value = "upstream")]
        remote_name: String,
    },
    #[command(about = "git account management")]
    User(UserArgs),
    #[command(about = "open websites related to current git repository")]
    Open(OpenArgs),
    #[command(about = "amend the commit's author and email by alias")]
    Amend { alias: String },
    #[command(about = "git init with specific user info by alias")]
    Init { alias: String },
    #[command(about = "branch actions", alias = "br")]
    Branch(BranchArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct BranchArgs {
    #[command(subcommand)]
    command: BranchCmd,
}

#[derive(Debug, Subcommand)]
enum BranchCmd {
    #[command(
        about = "delete both local and remote branch by default",
        alias = "del"
    )]
    Delete {
        branch_name: String,
        #[arg(default_value = "false", short = 'D')]
        force_delete: bool,
    },
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct UserArgs {
    #[command(subcommand)]
    command: UserCmd,
}

#[derive(Debug, Subcommand)]
enum UserCmd {
    #[command(about = "list all git users", alias = "ls")]
    List,
    #[command(about = "change local git user by alias")]
    Use {
        /// alias
        alias: String,
        #[arg(
            short = 'g',
            default_value = "false",
            help = "set git user locally and globally"
        )]
        global: bool,
    },
    #[command(about = "add git user")]
    Add {
        /// alias for this account
        alias: String,
        /// git user
        name: String,
        /// git email
        email: String,
    },
    #[command(about = "delete git user by alias")]
    Del {
        /// alias for this account
        alias: String,
    },
}
#[derive(Debug, Args)]
struct OpenArgs {
    #[arg(help = "alias of the website to open")]
    alias: Option<String>,
    #[command(subcommand)]
    command: Option<OpenCmd>,
}

#[derive(Debug, Subcommand)]
enum OpenCmd {
    #[command(about = "list all registered websites", alias = "ls")]
    List,
    #[command(about = "add target website")]
    Add {
        /// alias for this website
        alias: String,
        /// the base url of the website
        base_url: String,
    },
    #[command(about = "delete website by alias")]
    Del {
        /// alias for this account
        alias: String,
    },
}

#[tokio::main]
async fn main() {
    // Initialize gito config files
    if let Err(e) = utils::init_gito_config() {
        eprintln!("Error initializing gito config: {}", e);
        // Decide how to handle this error. For now, we'll just print and continue.
        // You might want to exit or take other corrective actions.
    }

    let git_info = get_git_info();
    let args = Cli::parse();

    match args.command {
        Commands::GetUpstream { remote_name } => {
            get_upstream::run(&remote_name, &git_info).await;
        }
        Commands::User(user) => match user.command {
            UserCmd::List => {
                user_command::list::run(&git_info);
            }
            UserCmd::Add { alias, name, email } => {
                user_command::add::run(&alias, &name, &email);
            }
            UserCmd::Use { alias, global } => user_command::use_user::run(&alias, global),
            UserCmd::Del { alias } => user_command::del::run(&alias),
        },
        Commands::Open(open) => {
            match open.command {
                Some(OpenCmd::List) => {
                    open_command::list::run(&git_info);
                }
                Some(OpenCmd::Add { alias, base_url }) => {
                    open_command::add::run(&alias, &base_url);
                }
                Some(OpenCmd::Del { alias }) => open_command::del::run(&alias),
                None => {
                    if let Some(alias) = open.alias {
                        open_command::open_website::run(&alias, &git_info);
                    } else {
                        eprintln!("Please provide an alias or a subcommand for 'open'.");
                    }
                }
            }
        }
        Commands::Amend { alias } => amend_command::run(&alias),
        Commands::Init { alias } => init_command::run(&alias),
        Commands::Branch(branch) => match branch.command {
            BranchCmd::Delete {
                branch_name,
                force_delete,
            } => branch_command::delete::run(&branch_name, force_delete),
        },
    }
}
