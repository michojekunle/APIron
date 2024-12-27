use clap::{App, Arg};

fn main() {
    let matches = App::new("API Query Tool")
        .version("1.0")
        .author("Michael Ojekunle")
        .about("A CLI tool to interact with APIs")
        .arg(Arg::new("api")
            .short('a')
            .long("api")
            .value_name("API")
            .about("Specify the API to use (e.g., GitHub, Twitter)")
            .takes_value(true))
        .arg(Arg::new("action")
            .short('c')
            .long("action")
            .value_name("ACTION")
            .about("Specify the action to perform (e.g., fetch, post)")
            .takes_value(true))
        .get_matches();

    let api = matches.value_of("api").unwrap_or("GitHub");
    let action = matches.value_of("action").unwrap_or("fetch");

    println!("API: {}, Action: {}", api, action);
}
