mod config;
mod remote;
use clap::Clap;
use shellexpand::tilde;
use std::path::Path;

#[derive(Clap)]
#[clap(version = "0.1", author = "wgjak47 ak47m61@gmail.com")]
struct Opts {
    #[clap(short, long, default_value = "~/.pv_checker.yaml")]
    config: String,
}

#[tokio::main]
async fn request_package(config: &config::PackageConfig) {
    println!("{}-{}", config.name, config.url);

    match config.get_version().await {
        Ok(info) => {
            println!("{:#?}", info);
        }
        Err(e) => {
            println!("{}", e);
        }
    };
}

pub fn main() {
    let opts: Opts = Opts::parse();
    let file_path = tilde(&opts.config).into_owned();
    println!("{}", file_path);
    let path = Path::new(&file_path);
    match config::load_package_config(path) {
        Err(e) => println!("{}", e),
        Ok(configs) => configs.iter().for_each(|config| {
            request_package(config);
        }),
    }
}
