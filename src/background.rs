use std::process::Command;

use anime_launcher_sdk::anime_game_core::installer::downloader::Downloader;

use md5::{Md5, Digest};

#[derive(Debug, Clone)]
pub struct Background {
    pub uri: String,
    pub hash: String
}

pub fn get_uri() -> String {
    concat!("https://wuther", "ingwav", "es.kur", "ogames.com/website-preface/video/bg/bg-poster.png").to_owned()
}

#[cached::proc_macro::cached(result)]
pub fn get_background_info() -> anyhow::Result<Background> {
    let uri = get_uri();

    let hash = uri.split('/')
        .last()
        .unwrap_or_default()
        .split('_')
        .next()
        .unwrap_or_default()
        .to_owned();

    Ok(Background {
        uri,
        hash
    })
}

pub fn download_background() -> anyhow::Result<()> {
    tracing::debug!("Downloading background picture");

    let info = get_background_info()?;

    let mut download_image = true;
    let mut replace_cached_image = false;

    if crate::BACKGROUND_FILE.exists() {
        let hash = Md5::digest(std::fs::read(crate::BACKGROUND_FILE.as_path())?);

        if format!("{:x}", hash).to_lowercase() == info.hash {
            tracing::debug!("Background picture is already downloaded. Skipping");

            download_image = false;
        }
    }

    if download_image {
        let mut downloader = Downloader::new(&info.uri)?;

        downloader.continue_downloading = false;

        if let Err(err) = downloader.download(crate::BACKGROUND_FILE.as_path(), |_, _| {}) {
            anyhow::bail!(err);
        }

        replace_cached_image = true;
    }

    // Workaround for GTK weakness
    if info.uri.ends_with(".webp") {
        Command::new("dwebp")
            .arg(crate::BACKGROUND_FILE.as_path())
            .arg("-o")
            .arg(crate::PROCESSED_BACKGROUND_FILE.as_path())
            .spawn()?
            .wait()?;

        // If it failed to re-code the file - just copy it
        // Will happen with HSR because devs apparently named
        // their background image ".webp" while it's JPEG
        if replace_cached_image || !crate::PROCESSED_BACKGROUND_FILE.exists() {
            std::fs::copy(crate::BACKGROUND_FILE.as_path(), crate::PROCESSED_BACKGROUND_FILE.as_path())?;
        }
    }

    else {
        std::fs::copy(crate::BACKGROUND_FILE.as_path(), crate::PROCESSED_BACKGROUND_FILE.as_path())?;
    }

    Ok(())
}
