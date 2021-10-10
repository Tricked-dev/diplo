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
    let config = WatchConfigBuilder::default()
        .clear_screen(false)
        .run_initially(true)
        .paths(vec![".".into()])
        .cmd(vec![command.into()])
        .on_busy_update(OnBusyUpdate::Restart)
        .ignores(default_ignores)
        .build()
        .unwrap();
    return config;
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
