use relm4::prelude::*;

use anime_launcher_sdk::wincompatlib::prelude::*;

use crate::*;

use super::{App, AppMsg};

pub fn install_vcrun2015(sender: ComponentSender<App>) {
    sender.input(AppMsg::DisableButtons(true));

    let config = Config::get().unwrap();

    std::thread::spawn(move || {
        // I honestly don't care anymore
        let wine = config.get_selected_wine().unwrap().unwrap();

        let wine = wine
            .to_wine(config.components.path, Some(config.game.wine.builds.join(&wine.name)))
            .with_loader(WineLoader::Current)
            .with_arch(WineArch::Win64)
            .with_prefix(&config.game.wine.prefix);

        if let Err(err) = vcrun2015::install(wine, config.game.wine.prefix, config.launcher.temp) {
            tracing::error!("Failed to install vcrun2015");

            // TODO: wouldn't hurt to translate
            sender.input(AppMsg::Toast {
                title: String::from("Failed to install vcrun2015"),
                description: Some(err.to_string())
            });
        }

        sender.input(AppMsg::DisableButtons(false));
        sender.input(AppMsg::UpdateLauncherState {
            perform_on_download_needed: false,
            show_status_page: true
        });
    });
}
