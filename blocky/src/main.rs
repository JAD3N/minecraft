#[macro_use]
extern crate clap;

use java_props::Properties;
use blocky::net::{AsJson, FromJson};
use blocky::net::protocol::{ProtocolWrite, Test};
use blocky::net::chat::{Style, ClickEvent, ClickAction, TextColor, TextComponent, Component};

fn main() {
    let matches = clap_app!(blocky =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())

        (@arg initSettings: --initSettings "Initializes 'server.properties' then quits")
        (@arg demo: --demo)
        (@arg bonusChest: --bonusChest)
        (@arg forceUpgrade: --forceUpgrade)
        (@arg eraseCache: --eraseCache)
        (@arg safeMode: --safeMode "Loads level with vanilla datapack only")

        (@arg singleplayer: --singleplayer +takes_value)
        (@arg universe: --universe +takes_value)
        (@arg world: --world +takes_value)
        (@arg port: --port +takes_value)
        (@arg serverId: --serverId +takes_value)
    ).get_matches();

    let test = Test {
        text: TextComponent::new("this is a fun test!"),
    };
}