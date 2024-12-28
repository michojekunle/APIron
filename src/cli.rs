use clap::{Arg, Command};

pub fn build_cli() -> Command<'static'> {
    Command::new("apiron")
        .about("APIron: Robust API Query CLI Tool")
        .subcommand(Command::new("github"))
            .about("Interact with the github API")
            .subcommand(Command::new("list-repos"))
                .arg(Arg::new("username"))
                    .required(true)
                    .help("Github username to fetch repos")
        .subcommand(Command::new("openai"))
            .about("Interact with the OpenAI API")
}