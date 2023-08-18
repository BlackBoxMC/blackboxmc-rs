use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::SharedJNIEnv;
use jni::{objects::JObject, JNIEnv};

#[no_mangle]
pub extern "system" fn __on__BlockBreakEvent(env: JNIEnv<'_>, obj: JObject<'_>) {
    let e = SharedJNIEnv::new(env);
    let event = blackboxmc_bukkit::event::block::BlockBreakEvent::from_raw(&e, obj).unwrap();

    // Cancel the event.
    event.set_cancelled(true).expect("Couldn't cancel event");

    let player = event.player().expect("Couldn't get player");
    if let Some(player) = player {
        println!(
            "{}",
            *player
                .inventory()
                .expect("Couldn't get inventory")
                .item_in_hand()
                .expect("Couldn't get item in hand")
                .get_type()
                .expect("Couldn't get type")
        );
    }
}
