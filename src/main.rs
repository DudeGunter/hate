#![allow(unused)]
use bevy::prelude::*;

fn main() -> AppExit {
    let mut app = App::new();

    #[cfg(feature = "client")]
    app.add_plugins(client::plugin);

    #[cfg(feature = "server")]
    app.add_plugins(server::plugin);

    app.run()
}
