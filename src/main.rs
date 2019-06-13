#![allow(unused)]

pub mod target_os;
pub mod utils;

use self::target_os::{TargetOs, TargetOsSpecific as _};
use cursive::{align, traits::*, views, Cursive};
use std::path::PathBuf;

struct SteamDir(PathBuf);
struct SrcdsDir(PathBuf);

fn main() {
    Cursive::default().with(menu).run()
}

fn menu(siv: &mut Cursive) {
    let contents = views::LinearLayout::vertical()
        .child(text(include_str!("../txt/menu")))
        .child(views::DummyView)
        .child(btn("Run Install Wizard", wizard_s0))
        .child(btn("Quit", Cursive::quit));

    siv.pop_layer();
    siv.add_layer(dialog(contents));
}

fn wizard_s0(siv: &mut Cursive) {
    if siv.user_data::<SteamDir>().is_some() {
        return wizard_s1(siv);
    }

    if let Some(steam_dir) = TargetOs::steam_dir() {
        siv.set_user_data(SteamDir(steam_dir));
        return wizard_s1(siv);
    }

    let contents = views::LinearLayout::vertical()
        .child(text(include_str!("../txt/wizard_s0")))
        .child(views::DummyView)
        .child(btn("Restart Install Wizard", menu))
        .child(btn("Quit", Cursive::quit));

    siv.pop_layer();
    siv.add_layer(dialog(contents))
}

fn wizard_s1(siv: &mut Cursive) {
    if siv.user_data::<SrcdsDir>().is_some() {
        return wizard_s2(siv);
    }

    let steam_dir = siv.user_data::<SteamDir>().unwrap();

    if let Ok(srcds_dir) = utils::srcds_dir(&steam_dir.0, 232250) {
        siv.set_user_data(SrcdsDir(srcds_dir));
        return wizard_s2(siv);
    }

    let contents = views::LinearLayout::vertical()
        .child(text(include_str!("../txt/wizard_s1")))
        .child(views::DummyView)
        .child(btn("Log out of Steam", |siv| {
            utils::exit_steam(&siv.user_data::<SteamDir>().unwrap().0);
        }))
        .child(btn("Launch Steam as anonymous and start download", |siv| {
            utils::install_srcds(&siv.user_data::<SteamDir>().unwrap().0, 232250);

            let contents = views::LinearLayout::vertical()
                .child(text(include_str!("../txt/wizard_s1_download_started")))
                .child(views::DummyView)
                .child(btn("Restart Install Wizard", menu))
                .child(btn("Quit", Cursive::quit));

            siv.pop_layer();
            siv.add_layer(dialog(contents))
        }))
        .child(btn("Restart Install Wizard", menu))
        .child(btn("Quit", Cursive::quit));

    siv.pop_layer();
    siv.add_layer(dialog(contents))
}

fn wizard_s2(siv: &mut Cursive) {}

// helpers

fn dialog<V: View + 'static>(view: V) -> views::Dialog {
    views::Dialog::around(view)
        .title("TF2 Dedicated Server Install Wizard")
        .padding_top(1)
        .padding_bottom(1)
}

fn text(contents: &str) -> views::TextView {
    views::TextView::new(contents)
}

fn btn(label: &str, callback: impl 'static + Fn(&mut Cursive)) -> views::Button {
    views::Button::new_raw(label, callback)
}
