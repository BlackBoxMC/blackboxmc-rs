use std::error::Error;

use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::SharedJNIEnv;
use jni::{objects::JObject, JNIEnv};

#[no_mangle]
pub extern "system" fn __on__BlockBreakEvent(env: JNIEnv<'_>, obj: JObject<'_>) {
    let res = || -> Result<(), Box<dyn Error>> {
        let e = SharedJNIEnv::new(env);
        let event = blackboxmc_bukkit::event::block::BlockBreakEvent::from_raw(&e, obj)?;

        // Cancel the event.
        event.set_cancelled(true)?;

        let player = event.player()?;
        if let Some(player) = player {
            println!("{}", player.inventory()?.item_in_main_hand()?.get_type()?);
        };
        Ok(())
    }();
    if let Err(err) = res {
        println!("BlockBreakEvent error: {}", err);
    }
}
