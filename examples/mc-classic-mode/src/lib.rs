use std::time::SystemTime;

use blackbox_rs::bukkit::{event::server::PluginEnableEvent, scheduler::BukkitRunnable};
use chrono::{DateTime, Utc};
use jni::{objects::JObject, JNIEnv};

#[no_mangle]
pub extern "system" fn __extends__BukkitRunnable__HungerThread__run(
    env: JNIEnv<'_>,
    obj: JObject<'_>,
) {
    let system_time = SystemTime::now();
    let datetime: DateTime<Utc> = system_time.into();
    println!("{}", datetime.format("%d/%m/%Y %T"));
}

#[no_mangle]
pub extern "system" fn __on__PluginEnableEvent(env: JNIEnv<'_>, obj: JObject<'_>) {
    let mut event = PluginEnableEvent::from_raw(env, obj);

    let plugin = event.plugin().unwrap();

    // New runnable
    let mut runnable = BukkitRunnable::from_extendable(
        &plugin,
        &event,
        format!("lib{}", std::env!("CARGO_CRATE_NAME")),
        "HungerThread".into(),
    )
    .unwrap();
    runnable.run_task_timer(plugin, 0, 20).unwrap();
}
