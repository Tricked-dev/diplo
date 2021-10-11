use crate::{load_config::WatcherClass, CONFIG};
use std::path::MAIN_SEPARATOR;
use watchexec::{
    config::{Config as WatchConfig, ConfigBuilder as WatchConfigBuilder},
    error::Result as WatchResult,
    pathop::PathOp,
    run::{ExecHandler, Handler, OnBusyUpdate},
};
pub fn get_config(command: &str) -> WatchConfig {
    let default_ignores = vec![
        format!("**{}.DS_Store", MAIN_SEPARATOR),
        String::from("*.py[co]"),
        String::from("#*#"),
        String::from(".#*"),
        String::from(".*.kate-swp"),
        String::from(".*.sw?"),
        String::from(".*.sw?x"),
        format!("node_modules{}*", MAIN_SEPARATOR),
        format!("**{}.git{}**", MAIN_SEPARATOR, MAIN_SEPARATOR),
        format!("**{}.hg{}**", MAIN_SEPARATOR, MAIN_SEPARATOR),
        format!("**{}.svn{}**", MAIN_SEPARATOR, MAIN_SEPARATOR),
    ];
    let default: WatcherClass = serde_json::from_str("{}").unwrap();
    let watch_config = &*CONFIG.watcher.as_ref().unwrap_or(&default);
    let config = WatchConfigBuilder::default()
        .clear_screen(watch_config.clear.unwrap_or(false))
        .run_initially(true)
        .paths(vec![watch_config
            .directory
            .as_ref()
            .unwrap_or(&".".to_string())
            .into()])
        .cmd(vec![command.into()])
        .on_busy_update(OnBusyUpdate::Restart)
        .ignores(
            if watch_config.no_ignore.unwrap_or(false)
                && watch_config.default_ignores.unwrap_or(true)
            {
                default_ignores
            } else {
                vec![]
            },
        )
        .build()
        .unwrap();
    config
}

pub struct DiploHandler(pub ExecHandler);

impl Handler for DiploHandler {
    fn args(&self) -> WatchConfig {
        self.0.args()
    }

    fn on_manual(&self) -> WatchResult<bool> {
        // println!("Running manually!");
        self.0.on_manual()
    }

    fn on_update(&self, ops: &[PathOp]) -> WatchResult<bool> {
        // println!("Running manually {:?}", ops);
        println!("Noticed file change, restarting");
        self.0.on_update(ops)
    }
}
