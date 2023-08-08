use std::{cell::OnceCell, error::Error, fmt::Display, ops::Deref, sync::Mutex, time::SystemTime};

use blackboxmc_bukkit::{
    command::{Command, CommandSender},
    event::server::PluginEnableEvent,
    generator::{ChunkGenerator, ChunkGeneratorChunkData, WorldInfo},
    plugin::Plugin,
    scheduler::BukkitRunnable,
};
use blackboxmc_general::SharedJNIEnv;
use jni::{objects::JObject, sys::jint, JNIEnv};

pub struct HungerThread {}

impl HungerThread {
    pub fn new<'mc>(
        env: &'mc SharedJNIEnv<'mc>,
        plugin: &'mc Plugin<'mc>,
    ) -> Result<BukkitRunnable<'mc>, Box<dyn Error>> {
        BukkitRunnable::from_extendable(
            env,
            plugin,
            0,
            format!("lib{}", std::env!("CARGO_CRATE_NAME")),
            "HungerThread".to_string(),
        )
    }
    pub fn run(plug: &mut Plugin) -> Result<(), Box<dyn Error>> {
        let mut server = plug.server()?;
        let players = server.online_players()?;
        for mut player in players {
            println!("{}", player.name()?);
        }
        Ok(())
    }
}

#[no_mangle]
pub extern "system" fn __extends__BukkitRunnable__HungerThread__run<'mc>(
    mut env: JNIEnv<'mc>,
    address: jint,
    plugin: JObject<'mc>,
    objs: JObject<'mc>,
) {
    let e = SharedJNIEnv::new(env);
    let mut plug = Plugin::from_raw(&e, plugin).unwrap();
    HungerThread::run(&mut plug).unwrap();
}

#[no_mangle]
pub extern "system" fn __on__PluginEnableEvent(env: JNIEnv<'_>, obj: JObject<'_>) {
    color_eyre::install().expect("Java exception");
    let e = SharedJNIEnv::new(env);
    let mut event = PluginEnableEvent::from_raw(&e, obj).expect("Java exception");

    let plugin = event.plugin().expect("Java exception");

    // New runnable
    println!("run task timer");
    let mut runnable = HungerThread::new(&e, &plugin).expect("Java exception");
    runnable
        .run_task_timer(&plugin, 0, 20)
        .expect("Java exception");
}
