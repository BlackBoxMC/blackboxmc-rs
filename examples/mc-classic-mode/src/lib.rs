use std::{sync::Mutex, time::SystemTime};

use blackbox_rs::{
    bukkit::{
        event::server::PluginEnableEvent,
        generator::{ChunkGenerator, ChunkGeneratorChunkData, WorldInfo},
        scheduler::BukkitRunnable,
    },
    JNIRaw, SharedJNIEnv,
};
use chrono::{DateTime, Utc};
use jni::{objects::JObject, sys::jint, JNIEnv};

#[no_mangle]
pub extern "system" fn __extends__ChunkGenerator__BetaGen__generateNoise(
    env: JNIEnv<'_>,
    arg1_raw: JObject<'_>,
    arg2_raw: JObject<'_>,
    arg3: jint,
    arg4: jint,
    arg5_raw: JObject<'_>,
) {
    let e = SharedJNIEnv::new(env);
    let arg1 = WorldInfo::from_raw(&e, arg1_raw);
    let arg5 = ChunkGeneratorChunkData::from_raw(&e, arg5_raw);
}

#[no_mangle]
pub extern "system" fn getDefaultWorldGenerator<'mc>(
    env: JNIEnv<'mc>,
    obj1: JObject<'_>,
    obj2: JObject<'_>,
) -> JObject<'mc> {
    println!("\n\n\n\nreturning new world generator\n\n\n\n");
    let e = SharedJNIEnv::new(env);
    ChunkGenerator::from_extendable(
        &e,
        format!("lib{}", std::env!("CARGO_CRATE_NAME")),
        "BetaGen".into(),
    )
    .unwrap()
    .jni_object()
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
