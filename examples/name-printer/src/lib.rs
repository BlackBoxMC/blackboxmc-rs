use blackboxmc_general::JNIInstantiatable;
use std::error::Error;

use blackboxmc_bukkit::{
    event::server::PluginEnableEvent, plugin::Plugin, scheduler::BukkitRunnable,
};
use blackboxmc_general::SharedJNIEnv;
use jni::{objects::JObject, sys::jint, JNIEnv};

pub struct NamePrinter {}

impl NamePrinter {
    pub fn new<'mc>(
        env: &'mc SharedJNIEnv<'mc>,
        plugin: &'mc Plugin<'mc>,
    ) -> Result<BukkitRunnable<'mc>, Box<dyn Error>> {
        BukkitRunnable::from_extendable(
            env,
            plugin,
            0,
            format!("lib{}", std::env!("CARGO_CRATE_NAME")),
            "NamePrinter".to_string(),
        )
    }
    pub fn run(plug: &mut Plugin) -> Result<(), Box<dyn Error>> {
        let server = plug.server()?;
        println!("srever");

        let players = server.online_players()?;
        println!("players");

        for player in players {
            println!("{:?}", player.name()?);
        }
        Ok(())
    }
}

#[no_mangle]
pub extern "system" fn __extends__BukkitRunnable__NamePrinter__run<'mc>(
    env: JNIEnv<'mc>,
    _address: jint,
    plugin: JObject<'mc>,
    _objs: JObject<'mc>,
) {
    println!("executing...?");
    let e = SharedJNIEnv::new(env);
    let mut plug = Plugin::from_raw(&e, plugin).unwrap();
    NamePrinter::run(&mut plug).unwrap();
    println!("executed");
}

#[no_mangle]
pub extern "system" fn __on__PluginEnableEvent(env: JNIEnv<'_>, obj: JObject<'_>) {
    println!("plugin enable?");
    color_eyre::install().expect("Java exception");
    let e = SharedJNIEnv::new(env);
    let ev1 = PluginEnableEvent::from_raw(&e, unsafe { JObject::from_raw(obj.clone()) })
        .expect("Java exception");
    let ev2 = PluginEnableEvent::from_raw(&e, obj).expect("Java exception");

    let plug1 = ev1.plugin().expect("Java exception");
    let plug2 = ev2.plugin().expect("Java exception");
    // New runnable
    println!("run task timer");
    let runnable = NamePrinter::new(&e, &plug1).expect("Java exception");
    runnable
        .run_task_timer(plug2, 0, 20)
        .expect("Java exception");
}
