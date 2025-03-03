use relm4::prelude::*;
use gtk::prelude::*;

use anime_launcher_sdk::wincompatlib::prelude::*;

use anime_launcher_sdk::config::ConfigExt;
use anime_launcher_sdk::wuwa::config::Config;
use anime_launcher_sdk::wuwa::config::schema::prelude::LauncherBehavior;

use crate::*;

use super::{App, AppMsg};

pub fn launch(sender: ComponentSender<App>) {
    let config = Config::get().unwrap();

    match config.launcher.behavior {
        // Disable launch button and show kill game button if behavior set to "Nothing" to prevent sussy actions
        LauncherBehavior::Nothing => {
            sender.input(AppMsg::DisableButtons(true));
            sender.input(AppMsg::SetKillGameButton(true));
        }

        // Hide launcher window if behavior set to "Hide" or "Close"
        LauncherBehavior::Hide | LauncherBehavior::Close => sender.input(AppMsg::HideWindow)
    }

    std::thread::spawn(move || {
        // I honestly don't care anymore
        let wine = config.get_selected_wine().unwrap().unwrap();

        let wine = wine
            .to_wine(config.components.path, Some(config.game.wine.builds.join(&wine.name)))
            .with_loader(WineLoader::Current)
            .with_arch(WineArch::Win64)
            .with_prefix(&config.game.wine.prefix);

        // Could definitely make this better
        if config.game.enhancements.fix_launch_dialog {
            let command = ["reg", "add", "HKCU\\Software\\Wine\\DllOverrides", "/v", "krsdkexternal.exe", "/f"];
            wine.run_args_with_env(command, config.game.wine.sync.get_env_vars())
                .expect("Failed to run wine server")
                .wait()
                .expect(&format!("Failed to run command {:?}", command));
        } else {
            let command = ["reg", "delete", "HKCU\\Software\\Wine\\DllOverrides", "/v", "krsdkexternal.exe", "/f"];
            wine.run_args_with_env(command, config.game.wine.sync.get_env_vars())
                .expect("Failed to run wine server")
                .wait()
                .expect(&format!("Failed to run command {:?}", command));
        }

        // Fix for the in-game browser being a black window
        wine.run_args_with_env(["winecfg", "-v", "win7"], config.game.wine.sync.get_env_vars())
            .expect("Failed to run wine server")
            .wait()
            .expect("Failed to run winecfg -v win7");

        wine.shutdown().expect("Failed to shutdown wineserver");

        if let Err(err) = anime_launcher_sdk::wuwa::game::run() {
            tracing::error!("Failed to launch game: {err}");

            sender.input(AppMsg::Toast {
                title: tr!("game-launching-failed"),
                description: Some(err.to_string())
            });
        }

        match config.launcher.behavior {
            // Enable launch button and hide kill game button if behavior set to "Nothing" after the game has closed
            LauncherBehavior::Nothing => {
                sender.input(AppMsg::DisableButtons(false));
                sender.input(AppMsg::SetKillGameButton(false));
            }

            // Show back launcher window if behavior set to "Hide" and the game has closed
            LauncherBehavior::Hide => sender.input(AppMsg::ShowWindow),

            // Otherwise close the launcher if behavior set to "Close" and the game has closed
            // We're calling quit method from the main context here because otherwise app won't be closed properly
            LauncherBehavior::Close => gtk::glib::MainContext::default().invoke(|| {
                relm4::main_application().quit();
            })
        }
    });
}
