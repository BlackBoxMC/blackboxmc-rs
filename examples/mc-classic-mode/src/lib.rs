use std::{cell::OnceCell, fmt::Display, ops::Deref, sync::Mutex, time::SystemTime};

use blackbox_rs::{
    bukkit::{
        command::{Command, CommandSender},
        event::server::PluginEnableEvent,
        generator::{ChunkGenerator, ChunkGeneratorChunkData, WorldInfo},
        plugin::{self, Plugin},
        scheduler::BukkitRunnable,
    },
    macros::{extends_blackbox, memory::MemoryMap},
    JNIRaw, SharedJNIEnv,
};
use jni::{objects::JObject, sys::jint, JNIEnv};

pub struct HungerThread {}

#[extends_blackbox(BukkitRunnable)]
impl HungerThread {
    pub fn run(plug: &mut Plugin) {
        println!("test");
    }
}

#[no_mangle]
pub extern "system" fn __extends__BukkitRunnable__HungerThread__run<'a>(
    mut env: JNIEnv<'a>,
    address: jint,
    plugin: JObject,
    objs: JObject,
) {
    let se = SharedJNIEnv::new(env);
    let mut plug = Plugin::from_raw(&se, plugin).unwrap();
    HungerThread::run(&mut plug);
}

#[no_mangle]
pub extern "system" fn __on__PluginEnableEvent(env: JNIEnv<'_>, obj: JObject<'_>) {
    let e = SharedJNIEnv::new(env);
    let mut event = PluginEnableEvent::from_raw(&e, obj).unwrap();

    let plugin = event.plugin().unwrap();

    // New runnable
    println!("run task timer");
    let mut runnable = HungerThread::new(e, plugin);
    runnable.run_task_timer(plugin, 0, 20).unwrap();
    println!("ok");
}
