use structopt::StructOpt;
use toml::value::Datetime;

#[derive(Debug, StructOpt)]
pub struct BuildConfig {
    pub title: Option<String>,

    #[structopt(short="s")]
    pub section: Option<String>,

    #[structopt(long="date")]
    pub date: Option<Datetime>,

    #[structopt(short="t")]
    pub tags: Option<Vec<String>>,

    #[structopt(short="d")]
    pub draft: bool,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "post")] Post(BuildConfig),
    #[structopt(name = "page")] Page(BuildConfig),
}

#[derive(Debug, StructOpt)]
#[structopt(name = "zola-new")]
pub struct App {
    #[structopt(subcommand)]
    pub command: Command,
    #[structopt(name = "config")]
    pub config_file: Option<String>,
}
