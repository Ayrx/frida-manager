use clap::{Arg, ArgGroup, App, SubCommand};

fn main() {
    let matches = App::new(clap::crate_name!())
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .subcommand(SubCommand::with_name("fetch")
            .about("Download Frida artifacts.")
            .arg(Arg::with_name("FRIDA-VERSION")
                 .help("Download the specified Frida version.")
                 .long("frida-version")
                 .takes_value(true)))
    .get_matches();

    if let Some(version) = matches.value_of("FRIDA-VERSION") {
        println!("--frida-version option not implemented yet.");
    } else {
        fetch_latest_release();
    }
}

fn fetch_latest_release() {
    ()
}
