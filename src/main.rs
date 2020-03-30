use anyhow::Result;
use clap::{Arg, App, SubCommand};
use reqwest::blocking as reqb;
use serde_json::Value;
use std::io;
use std::fs;
use std::path;


const GITHUB_LATEST_RELEASES_URL: &'static str = "https://api.github.com/repos/frida/frida/releases/latest";
const APP_USER_AGENT: &'static str = "frida-manager";

#[derive(Clone)]
struct Asset {
    name: String,
    content_type: String,
    download_url: String
}

impl Asset {
    fn download(&self,
                      client: &reqb::Client,
                      download_dir: &path::PathBuf) -> Result<()> {
        let mut response = client.get(&self.download_url).send()?;
        let mut dest = fs::File::create(download_dir.join(&self.name))?;
        io::copy(&mut response, &mut dest)?;

        Ok(())
    }

    fn exists(&self, download_dir: &path::PathBuf) -> bool {
        return download_dir.join(&self.name).exists();
    }
}

struct Release {
    version: String,
    assets: Vec<Asset>
}

impl Release {
    fn get_frida_server_assets(&self) -> Vec<Asset> {
        let mut assets: Vec<Asset> = Vec::new();

        for asset in &self.assets {
            if asset.name.starts_with("frida-server-") {
                assets.push(asset.clone());
            }
        }

        assets
    }
}

fn main() -> Result<()> {
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

    let home_dir = dirs::home_dir()
        .expect("error: unable to determine $HOME.");
    let app_home_dir = home_dir.join(".fridamanager");
    fs::create_dir_all(&app_home_dir)
        .expect("error: unable to create $HOME/.fridamanager");

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::USER_AGENT, APP_USER_AGENT.parse().unwrap());

    let client = reqb::Client::builder()
        .default_headers(headers)
        .build()?;

    let release;
    if let Some(version) = matches.value_of("FRIDA-VERSION") {
        println!("--frida-version option not implemented yet.");
        return Ok(());
    } else {
        release = fetch_latest_release(&client)?;
    }

    println!("version: {}", release.version);
    let assets = release.get_frida_server_assets();

    let version_dir = app_home_dir.join(release.version);
    fs::create_dir_all(&version_dir)
        .expect("error: unable to create $HOME/.fridamanager/$VERSION");

    for asset in assets {
        if !asset.exists(&version_dir) {
            println!("[+] Downloading {}.", asset.name);
            asset.download(&client, &version_dir)?;
        } else {
            println!("[+] {} is cached.", asset.name);
        }
    }

    Ok(())
}

fn fetch_latest_release(client: &reqb::Client) -> Result<Release> {
    let response = client.get(GITHUB_LATEST_RELEASES_URL)
        .send()?
        .error_for_status()?;

    let content: Value = response.json()?;

    let assets = content["assets"].as_array()
        .expect("error: assets is not an array.");

    let mut assets_vec = Vec::new();

    for asset in assets {
        let a = Asset {
            name: asset["name"].as_str()
                .expect("error: name is not a string.")
                .to_string(),
            content_type: asset["content_type"].as_str()
                .expect("error: content_type is not a string.")
                .to_string(),
            download_url: asset["browser_download_url"].as_str()
                .expect("error: browser_download_url is not a string.")
                .to_string()
        };

        assets_vec.push(a);
    }

    Ok(Release {
        version: content["tag_name"].as_str()
            .expect("error: tag_name is not a string.")
            .to_string(),
        assets: assets_vec
    })
}
