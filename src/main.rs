mod config;
mod remote;
mod version;

use clap::Clap;
use futures::{stream, StreamExt};
use shellexpand::tilde;
use std::path::Path;

#[derive(Clap)]
#[clap(version = "0.1", author = "wgjak47 ak47m61@gmail.com")]
struct Opts {
    #[clap(short, long, default_value = "~/.pv_checker.yaml")]
    config: String,
}

async fn request_package(configs: &[config::PackageConfig]) {
    let result = stream::iter(configs).for_each_concurrent(4, |config| async move {
        match config.get_version().await {
            Ok(info) => {
                println!("package: {}, url: {}:", config.name, config.url);
                println!("{}", info);
            }
            Err(e) => {
                println!("package: {}, url: {}:", config.name, config.url);
                println!("{}", e);
            }
        };
    });

    result.await;
}

#[tokio::main]
async fn main() {
    let opts: Opts = Opts::parse();
    let file_path = tilde(&opts.config).into_owned();
    let path = Path::new(&file_path);
    match config::load_package_config(path) {
        Err(e) => println!("{}", e),
        Ok(configs) => {
            request_package(&configs).await;
        }
    }
}
