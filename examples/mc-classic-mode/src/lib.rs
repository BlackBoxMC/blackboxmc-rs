use std::{fmt::Display, sync::Mutex, time::SystemTime};

use blackbox_rs::{
    bukkit::{
        command::{Command, CommandSender},
        event::server::PluginEnableEvent,
        generator::{ChunkGenerator, ChunkGeneratorChunkData, WorldInfo},
        plugin::{self, Plugin},
        scheduler::BukkitRunnable,
    },
    macros::extends_blackbox,
    JNIRaw, SharedJNIEnv,
};
use jni::{objects::JObject, sys::jint, JNIEnv};

pub struct HungerThread {}

#[extends_blackbox(BukkitRunnable)]
impl HungerThread {
    pub fn run(&self, plug: &mut Plugin) {
        //
    }
    fn yeah(&self) {}
}

pub struct ChunkGen {}

#[extends_blackbox(CommandExecutor)]
impl ChunkGen {
    fn on_command(
        &self,
        plug: &mut Plugin,
        sender: &mut CommandSender,
        command: &mut Command,
        arg1: &mut String,
        arg2: &mut [String],
    ) -> bool {
        false
    }
}

/*#[no_mangle]
pub extern "system" fn __on__PluginEnableEvent(env: JNIEnv<'_>, obj: JObject<'_>) {
    let e = SharedJNIEnv::new(env);
    let mut event = PluginEnableEvent::from_raw(&e, obj).unwrap();

    let plugin = event.plugin().unwrap();

    // New runnable
    let mut runnable = BukkitRunnable::from_extendable(
        &e,
        format!("lib{}", std::env!("CARGO_CRATE_NAME")),
        "ClockTest".into(),
    )
    .unwrap();
    println!("run task timer");
    runnable.run_task_timer(plugin, 0, 20).unwrap();
    println!("ok");
}
*/
