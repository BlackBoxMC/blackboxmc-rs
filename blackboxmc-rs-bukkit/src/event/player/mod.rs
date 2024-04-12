#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;/*org/bukkit/event/player/mod.rs*/

#[repr(C)]
pub struct PlayerCommandSendEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerCommandSendEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerCommandSendEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerCommandSendEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerCommandSendEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerCommandSendEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerCommandSendEventTrait<'mc> for PlayerCommandSendEvent<'mc> {}
pub trait PlayerCommandSendEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Returns a mutable collection of all top level commands to be sent.
/// 
/// It is not legal to add entries to this collection, only remove them.
/// Behaviour of adding entries is undefined.
	fn commands(&self) 
-> Result<Vec<String>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Collection;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCommands",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(),res.l()?)?;let iter = col.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?.to_string_lossy().to_string());
};
Ok(
new_vec
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerCommandSendEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerCommandSendEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerCommandSendEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerCommandSendEvent<'mc> {}
#[repr(C)]
pub struct PlayerToggleSneakEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerToggleSneakEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerToggleSneakEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerToggleSneakEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerToggleSneakEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerToggleSneakEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerToggleSneakEventTrait<'mc> for PlayerToggleSneakEvent<'mc> {}
pub trait PlayerToggleSneakEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,is_sneaking: bool) 
-> Result<crate::event::player::PlayerToggleSneakEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Z)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Bool(is_sneaking.into());
let cls = jni.find_class("org/bukkit/event/player/PlayerToggleSneakEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerToggleSneakEvent::from_raw(&jni,res
)}
/// Returns whether the player is now sneaking or not.
	fn is_sneaking(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isSneaking",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerToggleSneakEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerToggleSneakEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerToggleSneakEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for PlayerToggleSneakEvent<'mc> {}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerToggleSneakEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerToggleSneakEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerToggleSneakEvent<'mc> {}
#[repr(C)]
pub struct PlayerMoveEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerMoveEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerMoveEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerMoveEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerMoveEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerMoveEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerMoveEventTrait<'mc> for PlayerMoveEvent<'mc> {}
pub trait PlayerMoveEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,from: impl Into<crate::Location<'mc>>,to: impl Into<crate::Location<'mc>>) 
-> Result<crate::event::player::PlayerMoveEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Lorg/bukkit/Location;Lorg/bukkit/Location;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(from.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(to.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/player/PlayerMoveEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerMoveEvent::from_raw(&jni,res
)}
/// Gets the cancellation state of this event. A cancelled event will not
/// be executed in the server, but will still pass to other plugins
/// 
/// If a move or teleport event is cancelled, the player will be moved or
/// teleported back to the Location as defined by getFrom(). This will not
/// fire an event
	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Sets the cancellation state of this event. A cancelled event will not
/// be executed in the server, but will still pass to other plugins
/// 
/// If a move or teleport event is cancelled, the player will be moved or
/// teleported back to the Location as defined by getFrom(). This will not
/// fire an event
	fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the location this player moved from
	fn from(&self) 
-> Result<crate::Location<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Location;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFrom",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the location to mark as where the player moved from
	fn set_from(&self,from: impl Into<crate::Location<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Location;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(from.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFrom",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the location this player moved to
	fn to(&self) 
-> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Location;");
let res = self.jni_ref().call_method(&self.jni_object(),"getTo",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Sets the location that this player will move to
	fn set_to(&self,to: impl Into<crate::Location<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Location;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(to.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setTo",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerMoveEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerMoveEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerMoveEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for PlayerMoveEvent<'mc> {}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerMoveEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerMoveEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerMoveEvent<'mc> {}
pub enum PlayerSpawnChangeEventCause<'mc> {
	Command {inner: PlayerSpawnChangeEventCauseStruct<'mc>},
	Bed {inner: PlayerSpawnChangeEventCauseStruct<'mc>},
	RespawnAnchor {inner: PlayerSpawnChangeEventCauseStruct<'mc>},
	Plugin {inner: PlayerSpawnChangeEventCauseStruct<'mc>},
	Reset {inner: PlayerSpawnChangeEventCauseStruct<'mc>},
	Unknown {inner: PlayerSpawnChangeEventCauseStruct<'mc>},
}
impl<'mc> std::fmt::Display for PlayerSpawnChangeEventCause<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           PlayerSpawnChangeEventCause::Command { .. } => f.write_str("COMMAND"),
           PlayerSpawnChangeEventCause::Bed { .. } => f.write_str("BED"),
           PlayerSpawnChangeEventCause::RespawnAnchor { .. } => f.write_str("RESPAWN_ANCHOR"),
           PlayerSpawnChangeEventCause::Plugin { .. } => f.write_str("PLUGIN"),
           PlayerSpawnChangeEventCause::Reset { .. } => f.write_str("RESET"),
           PlayerSpawnChangeEventCause::Unknown { .. } => f.write_str("UNKNOWN"),
       }
   }
}

        impl<'mc> PlayerSpawnChangeEventCauseTrait<'mc> for PlayerSpawnChangeEventCause<'mc> {}
        
        pub trait PlayerSpawnChangeEventCauseTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<PlayerSpawnChangeEventCause<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/event/player/PlayerSpawnChangeEvent/Cause");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/event/player/PlayerSpawnChangeEvent/Cause;",
                    vec![jni::objects::JValueGen::from(val_1)],
                );
                let res = env.translate_error(res)?;
                let obj = res.l()?;
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    
"COMMAND" => Ok(PlayerSpawnChangeEventCause::Command { inner: PlayerSpawnChangeEventCauseStruct::from_raw(env,obj)?}),
"BED" => Ok(PlayerSpawnChangeEventCause::Bed { inner: PlayerSpawnChangeEventCauseStruct::from_raw(env,obj)?}),
"RESPAWN_ANCHOR" => Ok(PlayerSpawnChangeEventCause::RespawnAnchor { inner: PlayerSpawnChangeEventCauseStruct::from_raw(env,obj)?}),
"PLUGIN" => Ok(PlayerSpawnChangeEventCause::Plugin { inner: PlayerSpawnChangeEventCauseStruct::from_raw(env,obj)?}),
"RESET" => Ok(PlayerSpawnChangeEventCause::Reset { inner: PlayerSpawnChangeEventCauseStruct::from_raw(env,obj)?}),
"UNKNOWN" => Ok(PlayerSpawnChangeEventCause::Unknown { inner: PlayerSpawnChangeEventCauseStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct PlayerSpawnChangeEventCauseStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerSpawnChangeEventCause<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::Command { inner } => inner.0.clone(),
Self::Bed { inner } => inner.0.clone(),
Self::RespawnAnchor { inner } => inner.0.clone(),
Self::Plugin { inner } => inner.0.clone(),
Self::Reset { inner } => inner.0.clone(),
Self::Unknown { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::Command { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Bed { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::RespawnAnchor { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Plugin { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Reset { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Unknown { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerSpawnChangeEventCause<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerSpawnChangeEventCause from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerSpawnChangeEvent/Cause")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerSpawnChangeEventCause object, got {}",
                    name
                )
                .into())
            } else {
    
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    "COMMAND" => Ok(PlayerSpawnChangeEventCause::Command { inner: PlayerSpawnChangeEventCauseStruct::from_raw(env,obj)?}),"BED" => Ok(PlayerSpawnChangeEventCause::Bed { inner: PlayerSpawnChangeEventCauseStruct::from_raw(env,obj)?}),"RESPAWN_ANCHOR" => Ok(PlayerSpawnChangeEventCause::RespawnAnchor { inner: PlayerSpawnChangeEventCauseStruct::from_raw(env,obj)?}),"PLUGIN" => Ok(PlayerSpawnChangeEventCause::Plugin { inner: PlayerSpawnChangeEventCauseStruct::from_raw(env,obj)?}),"RESET" => Ok(PlayerSpawnChangeEventCause::Reset { inner: PlayerSpawnChangeEventCauseStruct::from_raw(env,obj)?}),"UNKNOWN" => Ok(PlayerSpawnChangeEventCause::Unknown { inner: PlayerSpawnChangeEventCauseStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for PlayerSpawnChangeEventCauseStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerSpawnChangeEventCauseStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerSpawnChangeEventCauseStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerSpawnChangeEvent/Cause")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerSpawnChangeEventCauseStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerSpawnChangeEventCauseStruct<'mc> {

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::player::PlayerSpawnChangeEventCause<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/player/PlayerSpawnChangeEvent/Cause;");
let cls = jni.find_class("org/bukkit/event/player/PlayerSpawnChangeEvent/Cause"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::player::PlayerSpawnChangeEventCause::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct PlayerBucketEntityEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerBucketEntityEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerBucketEntityEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerBucketEntityEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerBucketEntityEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerBucketEntityEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerBucketEntityEventTrait<'mc> for PlayerBucketEntityEvent<'mc> {}
pub trait PlayerBucketEntityEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,entity: impl Into<crate::entity::Entity<'mc>>,original_bucket: impl Into<crate::inventory::ItemStack<'mc>>,entity_bucket: impl Into<crate::inventory::ItemStack<'mc>>,hand: impl Into<crate::inventory::EquipmentSlot<'mc>>) 
-> Result<crate::event::player::PlayerBucketEntityEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Lorg/bukkit/entity/Entity;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/EquipmentSlot;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(entity.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(original_bucket.into().jni_object().clone())});
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(entity_bucket.into().jni_object().clone())});
let val_5 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(hand.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/player/PlayerBucketEntityEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4),jni::objects::JValueGen::from(val_5)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerBucketEntityEvent::from_raw(&jni,res
)}
/// Gets the {@link Entity} being put into the bucket.
	fn entity(&self) 
-> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Entity;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEntity",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Entity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the bucket used to capture the {@link Entity}.
/// This refers to the bucket clicked with, eg {@link Material#WATER_BUCKET}.
	fn original_bucket(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getOriginalBucket",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the bucket that the {@link Entity} will be put into.
/// This refers to the bucket with the entity, eg
/// {@link Material#PUFFERFISH_BUCKET}.
	fn entity_bucket(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEntityBucket",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Get the hand that was used to bucket the entity.
	fn hand(&self) 
-> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/EquipmentSlot;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHand",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerBucketEntityEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerBucketEntityEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerBucketEntityEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for PlayerBucketEntityEvent<'mc> {}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerBucketEntityEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerBucketEntityEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerBucketEntityEvent<'mc> {}
#[repr(C)]
pub struct PlayerBucketEmptyEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerBucketEmptyEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerBucketEmptyEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerBucketEmptyEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerBucketEmptyEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerBucketEmptyEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerBucketEmptyEventTrait<'mc> for PlayerBucketEmptyEvent<'mc> {}
pub trait PlayerBucketEmptyEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,who: impl Into<crate::entity::Player<'mc>>,block: impl Into<crate::block::Block<'mc>>,block_clicked: impl Into<crate::block::Block<'mc>>,block_face: impl Into<crate::block::BlockFace<'mc>>,bucket: impl Into<crate::Material<'mc>>,item_in_hand: std::option::Option<impl Into<crate::inventory::ItemStack<'mc>>>,hand: std::option::Option<impl Into<crate::inventory::EquipmentSlot<'mc>>>) 
-> Result<crate::event::player::PlayerBucketEmptyEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/entity/Player;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(who.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/block/Block;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
args.push(val_2);
sig += "Lorg/bukkit/block/Block;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block_clicked.into().jni_object().clone())});
args.push(val_3);
sig += "Lorg/bukkit/block/BlockFace;";
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block_face.into().jni_object().clone())});
args.push(val_4);
sig += "Lorg/bukkit/Material;";
let val_5 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(bucket.into().jni_object().clone())});
args.push(val_5);
if let Some(a) = item_in_hand {
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_6 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_6);
}
if let Some(a) = hand {
sig += "Lorg/bukkit/inventory/EquipmentSlot;";
let val_7 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_7);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/player/PlayerBucketEmptyEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerBucketEmptyEvent::from_raw(&jni,res
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerBucketEmptyEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::player::PlayerBucketEvent<'mc>> for PlayerBucketEmptyEvent<'mc>{

fn into(self) -> crate::event::player::PlayerBucketEvent<'mc> {

crate::event::player::PlayerBucketEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerBucketEmptyEvent into crate::event::player::PlayerBucketEvent")

   }
}
impl<'mc> crate::event::player::PlayerBucketEventTrait<'mc> for PlayerBucketEmptyEvent<'mc> {}
#[repr(C)]
pub struct PlayerEggThrowEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerEggThrowEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerEggThrowEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerEggThrowEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerEggThrowEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerEggThrowEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerEggThrowEventTrait<'mc> for PlayerEggThrowEvent<'mc> {}
pub trait PlayerEggThrowEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,egg: impl Into<crate::entity::Egg<'mc>>,hatching: bool,num_hatches: i8,hatching_type: impl Into<crate::entity::EntityType<'mc>>) 
-> Result<crate::event::player::PlayerEggThrowEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Lorg/bukkit/entity/Egg;ZBLorg/bukkit/entity/EntityType;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(egg.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Bool(hatching.into());
let val_4 = jni::objects::JValueGen::Byte(num_hatches);
let val_5 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(hatching_type.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/player/PlayerEggThrowEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4),jni::objects::JValueGen::from(val_5)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerEggThrowEvent::from_raw(&jni,res
)}
/// Gets the egg involved in this event.
	fn egg(&self) 
-> Result<crate::entity::Egg<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Egg;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEgg",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Egg::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets whether the egg is hatching or not. Will be what the server
/// would've done without interaction.
	fn is_hatching(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isHatching",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Sets whether the egg will hatch or not.
	fn set_hatching(&self,hatching: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(hatching.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setHatching",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the type of the mob being hatched (EntityType.CHICKEN by default)
	fn hatching_type(&self) 
-> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/EntityType;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHatchingType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::EntityType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Change the type of mob being hatched by the egg
	fn set_hatching_type(&self,hatch_type: impl Into<crate::entity::EntityType<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/EntityType;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(hatch_type.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setHatchingType",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the number of mob hatches from the egg. By default the number will
/// be the number the server would've done
/// <ul>
/// <li>7/8 chance of being 0
/// <li>31/256 ~= 1/8 chance to be 1
/// <li>1/256 chance to be 4
/// </ul>
	fn num_hatches(&self) 
-> Result<i8, Box<dyn std::error::Error>>

{let sig = String::from("()B");
let res = self.jni_ref().call_method(&self.jni_object(),"getNumHatches",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.b()?
)}
/// Change the number of mobs coming out of the hatched egg
/// 
/// The boolean hatching will override this number. Ie. If hatching =
/// false, this number will not matter
	fn set_num_hatches(&self,num_hatches: i8) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(B)V");
let val_1 = jni::objects::JValueGen::Byte(num_hatches);
let res = self.jni_ref().call_method(&self.jni_object(),"setNumHatches",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerEggThrowEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerEggThrowEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerEggThrowEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerEggThrowEvent<'mc> {}
#[repr(C)]
pub struct PlayerToggleSprintEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerToggleSprintEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerToggleSprintEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerToggleSprintEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerToggleSprintEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerToggleSprintEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerToggleSprintEventTrait<'mc> for PlayerToggleSprintEvent<'mc> {}
pub trait PlayerToggleSprintEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,is_sprinting: bool) 
-> Result<crate::event::player::PlayerToggleSprintEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Z)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Bool(is_sprinting.into());
let cls = jni.find_class("org/bukkit/event/player/PlayerToggleSprintEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerToggleSprintEvent::from_raw(&jni,res
)}
/// Gets whether the player is now sprinting or not.
	fn is_sprinting(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isSprinting",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerToggleSprintEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerToggleSprintEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerToggleSprintEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for PlayerToggleSprintEvent<'mc> {}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerToggleSprintEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerToggleSprintEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerToggleSprintEvent<'mc> {}
#[repr(C)]
pub struct PlayerBucketFishEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerBucketFishEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerBucketFishEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerBucketFishEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerBucketFishEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerBucketFishEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerBucketFishEventTrait<'mc> for PlayerBucketFishEvent<'mc> {}
pub trait PlayerBucketFishEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,fish: impl Into<crate::entity::Fish<'mc>>,water_bucket: impl Into<crate::inventory::ItemStack<'mc>>,fish_bucket: impl Into<crate::inventory::ItemStack<'mc>>,hand: impl Into<crate::inventory::EquipmentSlot<'mc>>) 
-> Result<crate::event::player::PlayerBucketFishEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Lorg/bukkit/entity/Fish;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/EquipmentSlot;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(fish.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(water_bucket.into().jni_object().clone())});
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(fish_bucket.into().jni_object().clone())});
let val_5 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(hand.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/player/PlayerBucketFishEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4),jni::objects::JValueGen::from(val_5)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerBucketFishEvent::from_raw(&jni,res
)}
/// Gets the fish involved with this event.
	fn entity(&self) 
-> Result<crate::entity::Fish<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Fish;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEntity",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Fish::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]
/// Gets the bucket used. This refers to the bucket clicked with, ie {@link Material#WATER_BUCKET}.
	fn water_bucket(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getWaterBucket",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]
/// Gets the bucket that the fish will be put into. This refers to the bucket with the fish, ie {@link Material#PUFFERFISH_BUCKET}.
	fn fish_bucket(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFishBucket",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::player::PlayerBucketEntityEvent<'mc>> for PlayerBucketFishEvent<'mc>{

fn into(self) -> crate::event::player::PlayerBucketEntityEvent<'mc> {

crate::event::player::PlayerBucketEntityEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerBucketFishEvent into crate::event::player::PlayerBucketEntityEvent")

   }
}
impl<'mc> crate::event::player::PlayerBucketEntityEventTrait<'mc> for PlayerBucketFishEvent<'mc> {}
#[repr(C)]
pub struct PlayerRecipeBookClickEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerRecipeBookClickEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerRecipeBookClickEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerRecipeBookClickEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerRecipeBookClickEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerRecipeBookClickEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerRecipeBookClickEventTrait<'mc> for PlayerRecipeBookClickEvent<'mc> {}
pub trait PlayerRecipeBookClickEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,recipe: impl Into<crate::inventory::Recipe<'mc>>,shift_click: bool) 
-> Result<crate::event::player::PlayerRecipeBookClickEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Lorg/bukkit/inventory/Recipe;Z)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(recipe.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Bool(shift_click.into());
let cls = jni.find_class("org/bukkit/event/player/PlayerRecipeBookClickEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerRecipeBookClickEvent::from_raw(&jni,res
)}
/// Gets the original recipe the player was trying to craft.
/// 
/// This <em>will not</em> reflect any changes made with {@link setRecipe}.
	fn original_recipe(&self) 
-> Result<crate::inventory::Recipe<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/Recipe;");
let res = self.jni_ref().call_method(&self.jni_object(),"getOriginalRecipe",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Recipe::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the recipe the player is trying to craft.
/// 
/// This <em>will</em> reflect changes made with {@link setRecipe}.
	fn recipe(&self) 
-> Result<crate::inventory::Recipe<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/Recipe;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRecipe",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Recipe::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Set the recipe that will be used.
/// 
/// The game will attempt to move the ingredients for this recipe into the
/// appropriate slots.
/// 
/// If the original recipe is a {@link CraftingRecipe} the provided recipe
/// must also be a {@link CraftingRecipe}, otherwise the provided recipe must
/// be of the same type as the original recipe.
	fn set_recipe(&self,recipe: impl Into<crate::inventory::Recipe<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/Recipe;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(recipe.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setRecipe",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// If true the game will attempt to move the ingredients for as many copies
/// of this recipe as possible into the appropriate slots, otherwise only 1
/// copy will be moved.
	fn is_shift_click(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isShiftClick",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Sets if the game will attempt to move the ingredients for as many copies
/// of this recipe as possible into the appropriate slots.
	fn set_shift_click(&self,shift_click: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(shift_click.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setShiftClick",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerRecipeBookClickEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerRecipeBookClickEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerRecipeBookClickEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerRecipeBookClickEvent<'mc> {}
#[repr(C)]
pub struct PlayerResourcePackStatusEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerResourcePackStatusEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerResourcePackStatusEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerResourcePackStatusEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerResourcePackStatusEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerResourcePackStatusEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerResourcePackStatusEventTrait<'mc> for PlayerResourcePackStatusEvent<'mc> {}
pub trait PlayerResourcePackStatusEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,who: impl Into<crate::entity::Player<'mc>>,id: impl Into<blackboxmc_java::util::JavaUUID<'mc>>,resource_pack_status: impl Into<crate::event::player::PlayerResourcePackStatusEventStatus<'mc>>) 
-> Result<crate::event::player::PlayerResourcePackStatusEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Ljava/util/UUID;Lorg/bukkit/event/player/PlayerResourcePackStatusEvent/Status;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(who.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(id.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(resource_pack_status.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/player/PlayerResourcePackStatusEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerResourcePackStatusEvent::from_raw(&jni,res
)}
/// Gets the unique ID of this pack.
	fn id(&self) 
-> Result<blackboxmc_java::util::JavaUUID<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/UUID;");
let res = self.jni_ref().call_method(&self.jni_object(),"getID",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaUUID::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the status of this pack.
	fn status(&self) 
-> Result<crate::event::player::PlayerResourcePackStatusEventStatus<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/player/PlayerResourcePackStatusEvent/Status;");
let res = self.jni_ref().call_method(&self.jni_object(),"getStatus",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::player::PlayerResourcePackStatusEventStatus::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerResourcePackStatusEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerResourcePackStatusEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerResourcePackStatusEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerResourcePackStatusEvent<'mc> {}
pub enum PlayerLoginEventResult<'mc> {
	Allowed {inner: PlayerLoginEventResultStruct<'mc>},
	KickFull {inner: PlayerLoginEventResultStruct<'mc>},
	KickBanned {inner: PlayerLoginEventResultStruct<'mc>},
	KickWhitelist {inner: PlayerLoginEventResultStruct<'mc>},
	KickOther {inner: PlayerLoginEventResultStruct<'mc>},
}
impl<'mc> std::fmt::Display for PlayerLoginEventResult<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           PlayerLoginEventResult::Allowed { .. } => f.write_str("ALLOWED"),
           PlayerLoginEventResult::KickFull { .. } => f.write_str("KICK_FULL"),
           PlayerLoginEventResult::KickBanned { .. } => f.write_str("KICK_BANNED"),
           PlayerLoginEventResult::KickWhitelist { .. } => f.write_str("KICK_WHITELIST"),
           PlayerLoginEventResult::KickOther { .. } => f.write_str("KICK_OTHER"),
       }
   }
}

        impl<'mc> PlayerLoginEventResultTrait<'mc> for PlayerLoginEventResult<'mc> {}
        
        pub trait PlayerLoginEventResultTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<PlayerLoginEventResult<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/event/player/PlayerLoginEvent/Result");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/event/player/PlayerLoginEvent/Result;",
                    vec![jni::objects::JValueGen::from(val_1)],
                );
                let res = env.translate_error(res)?;
                let obj = res.l()?;
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    
"ALLOWED" => Ok(PlayerLoginEventResult::Allowed { inner: PlayerLoginEventResultStruct::from_raw(env,obj)?}),
"KICK_FULL" => Ok(PlayerLoginEventResult::KickFull { inner: PlayerLoginEventResultStruct::from_raw(env,obj)?}),
"KICK_BANNED" => Ok(PlayerLoginEventResult::KickBanned { inner: PlayerLoginEventResultStruct::from_raw(env,obj)?}),
"KICK_WHITELIST" => Ok(PlayerLoginEventResult::KickWhitelist { inner: PlayerLoginEventResultStruct::from_raw(env,obj)?}),
"KICK_OTHER" => Ok(PlayerLoginEventResult::KickOther { inner: PlayerLoginEventResultStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct PlayerLoginEventResultStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerLoginEventResult<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::Allowed { inner } => inner.0.clone(),
Self::KickFull { inner } => inner.0.clone(),
Self::KickBanned { inner } => inner.0.clone(),
Self::KickWhitelist { inner } => inner.0.clone(),
Self::KickOther { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::Allowed { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::KickFull { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::KickBanned { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::KickWhitelist { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::KickOther { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerLoginEventResult<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerLoginEventResult from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerLoginEvent/Result")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerLoginEventResult object, got {}",
                    name
                )
                .into())
            } else {
    
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    "ALLOWED" => Ok(PlayerLoginEventResult::Allowed { inner: PlayerLoginEventResultStruct::from_raw(env,obj)?}),"KICK_FULL" => Ok(PlayerLoginEventResult::KickFull { inner: PlayerLoginEventResultStruct::from_raw(env,obj)?}),"KICK_BANNED" => Ok(PlayerLoginEventResult::KickBanned { inner: PlayerLoginEventResultStruct::from_raw(env,obj)?}),"KICK_WHITELIST" => Ok(PlayerLoginEventResult::KickWhitelist { inner: PlayerLoginEventResultStruct::from_raw(env,obj)?}),"KICK_OTHER" => Ok(PlayerLoginEventResult::KickOther { inner: PlayerLoginEventResultStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for PlayerLoginEventResultStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerLoginEventResultStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerLoginEventResultStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerLoginEvent/Result")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerLoginEventResultStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerLoginEventResultStruct<'mc> {

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::player::PlayerLoginEventResult<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/player/PlayerLoginEvent/Result;");
let cls = jni.find_class("org/bukkit/event/player/PlayerLoginEvent/Result"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::player::PlayerLoginEventResult::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct PlayerAdvancementDoneEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerAdvancementDoneEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerAdvancementDoneEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerAdvancementDoneEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerAdvancementDoneEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerAdvancementDoneEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerAdvancementDoneEventTrait<'mc> for PlayerAdvancementDoneEvent<'mc> {}
pub trait PlayerAdvancementDoneEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,who: impl Into<crate::entity::Player<'mc>>,advancement: impl Into<crate::advancement::Advancement<'mc>>) 
-> Result<crate::event::player::PlayerAdvancementDoneEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Lorg/bukkit/advancement/Advancement;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(who.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(advancement.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/player/PlayerAdvancementDoneEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerAdvancementDoneEvent::from_raw(&jni,res
)}
/// Get the advancement which has been completed.
	fn advancement(&self) 
-> Result<crate::advancement::Advancement<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/advancement/Advancement;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAdvancement",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::advancement::Advancement::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerAdvancementDoneEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerAdvancementDoneEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerAdvancementDoneEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerAdvancementDoneEvent<'mc> {}
#[repr(C)]
pub struct PlayerArmorStandManipulateEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerArmorStandManipulateEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerArmorStandManipulateEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerArmorStandManipulateEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerArmorStandManipulateEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerArmorStandManipulateEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerArmorStandManipulateEventTrait<'mc> for PlayerArmorStandManipulateEvent<'mc> {}
pub trait PlayerArmorStandManipulateEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,who: impl Into<crate::entity::Player<'mc>>,clicked_entity: impl Into<crate::entity::ArmorStand<'mc>>,player_item: impl Into<crate::inventory::ItemStack<'mc>>,armor_stand_item: impl Into<crate::inventory::ItemStack<'mc>>,slot: impl Into<crate::inventory::EquipmentSlot<'mc>>,hand: std::option::Option<impl Into<crate::inventory::EquipmentSlot<'mc>>>) 
-> Result<crate::event::player::PlayerArmorStandManipulateEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/entity/Player;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(who.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/entity/ArmorStand;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(clicked_entity.into().jni_object().clone())});
args.push(val_2);
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player_item.into().jni_object().clone())});
args.push(val_3);
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(armor_stand_item.into().jni_object().clone())});
args.push(val_4);
sig += "Lorg/bukkit/inventory/EquipmentSlot;";
let val_5 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(slot.into().jni_object().clone())});
args.push(val_5);
if let Some(a) = hand {
sig += "Lorg/bukkit/inventory/EquipmentSlot;";
let val_6 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_6);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/player/PlayerArmorStandManipulateEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerArmorStandManipulateEvent::from_raw(&jni,res
)}
/// Returns the item held by the player.
/// 
/// If this item is empty and the armor stand item is also empty, there will be no
/// transaction between the player and the armor stand. If the player's item is empty
/// but the armor stand item is not, the player's item will be placed on the armor
/// stand. If both items are not empty, the items will be swapped.
/// 
/// In the case that this event is cancelled, the original items will remain the same.
	fn player_item(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPlayerItem",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Returns the item held by the armor stand.
/// 
/// If this item is empty and the player's item is also empty, there will be no
/// transaction between the player and the armor stand. If the player's item is empty
/// but the armor stand item is not, then the player will obtain the armor stand item.
/// In the case that the player's item is not empty but the armor stand item is empty,
/// the player's item will be placed on the armor stand. If both items are not empty,
/// the items will be swapped.
/// 
/// In the case that the event is cancelled the original items will remain the same.
	fn armor_stand_item(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getArmorStandItem",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Returns the raw item slot of the armor stand in this event.
	fn slot(&self) 
-> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/EquipmentSlot;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSlot",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// {@inheritDoc}
/// 
/// Note that this is not the hand of the armor stand that was changed, but rather
/// the hand used by the player to swap items with the armor stand.
	fn hand(&self) 
-> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/EquipmentSlot;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHand",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn right_clicked(&self) 
-> Result<crate::entity::ArmorStand<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/ArmorStand;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRightClicked",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::ArmorStand::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerArmorStandManipulateEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::player::PlayerInteractEntityEvent<'mc>> for PlayerArmorStandManipulateEvent<'mc>{

fn into(self) -> crate::event::player::PlayerInteractEntityEvent<'mc> {

crate::event::player::PlayerInteractEntityEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerArmorStandManipulateEvent into crate::event::player::PlayerInteractEntityEvent")

   }
}
impl<'mc> crate::event::player::PlayerInteractEntityEventTrait<'mc> for PlayerArmorStandManipulateEvent<'mc> {}
#[repr(C)]
pub struct PlayerShowEntityEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerShowEntityEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerShowEntityEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerShowEntityEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerShowEntityEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerShowEntityEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerShowEntityEventTrait<'mc> for PlayerShowEntityEvent<'mc> {}
pub trait PlayerShowEntityEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,who: impl Into<crate::entity::Player<'mc>>,entity: impl Into<crate::entity::Entity<'mc>>) 
-> Result<crate::event::player::PlayerShowEntityEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Lorg/bukkit/entity/Entity;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(who.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(entity.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/player/PlayerShowEntityEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerShowEntityEvent::from_raw(&jni,res
)}
/// Gets the entity which has been shown to the player.
	fn entity(&self) 
-> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Entity;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEntity",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Entity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerShowEntityEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerShowEntityEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerShowEntityEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerShowEntityEvent<'mc> {}
#[repr(C)]
pub struct PlayerInteractEntityEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerInteractEntityEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerInteractEntityEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerInteractEntityEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerInteractEntityEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerInteractEntityEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerInteractEntityEventTrait<'mc> for PlayerInteractEntityEvent<'mc> {}
pub trait PlayerInteractEntityEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,who: impl Into<crate::entity::Player<'mc>>,clicked_entity: impl Into<crate::entity::Entity<'mc>>,hand: std::option::Option<impl Into<crate::inventory::EquipmentSlot<'mc>>>) 
-> Result<crate::event::player::PlayerInteractEntityEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/entity/Player;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(who.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/entity/Entity;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(clicked_entity.into().jni_object().clone())});
args.push(val_2);
if let Some(a) = hand {
sig += "Lorg/bukkit/inventory/EquipmentSlot;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_3);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/player/PlayerInteractEntityEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerInteractEntityEvent::from_raw(&jni,res
)}

	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the entity that was right-clicked by the player.
	fn right_clicked(&self) 
-> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Entity;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRightClicked",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Entity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// The hand used to perform this interaction.
	fn hand(&self) 
-> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/EquipmentSlot;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHand",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerInteractEntityEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerInteractEntityEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerInteractEntityEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for PlayerInteractEntityEvent<'mc> {}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerInteractEntityEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerInteractEntityEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerInteractEntityEvent<'mc> {}
#[repr(C)]
pub struct PlayerChangedWorldEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerChangedWorldEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerChangedWorldEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerChangedWorldEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerChangedWorldEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerChangedWorldEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerChangedWorldEventTrait<'mc> for PlayerChangedWorldEvent<'mc> {}
pub trait PlayerChangedWorldEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,from: impl Into<crate::World<'mc>>) 
-> Result<crate::event::player::PlayerChangedWorldEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Lorg/bukkit/World;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(from.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/player/PlayerChangedWorldEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerChangedWorldEvent::from_raw(&jni,res
)}
/// Gets the world the player is switching from.
	fn from(&self) 
-> Result<crate::World<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/World;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFrom",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::World::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerChangedWorldEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerChangedWorldEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerChangedWorldEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerChangedWorldEvent<'mc> {}
#[repr(C)]
pub struct PlayerEditBookEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerEditBookEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerEditBookEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerEditBookEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerEditBookEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerEditBookEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerEditBookEventTrait<'mc> for PlayerEditBookEvent<'mc> {}
pub trait PlayerEditBookEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,who: impl Into<crate::entity::Player<'mc>>,slot: i32,previous_book_meta: impl Into<crate::inventory::meta::BookMeta<'mc>>,new_book_meta: impl Into<crate::inventory::meta::BookMeta<'mc>>,is_signing: bool) 
-> Result<crate::event::player::PlayerEditBookEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;ILorg/bukkit/inventory/meta/BookMeta;Lorg/bukkit/inventory/meta/BookMeta;Z)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(who.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Int(slot);
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(previous_book_meta.into().jni_object().clone())});
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(new_book_meta.into().jni_object().clone())});
let val_5 = jni::objects::JValueGen::Bool(is_signing.into());
let cls = jni.find_class("org/bukkit/event/player/PlayerEditBookEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4),jni::objects::JValueGen::from(val_5)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerEditBookEvent::from_raw(&jni,res
)}
/// Gets the book meta currently on the book.
/// 
/// Note: this is a copy of the book meta. You cannot use this object to
/// change the existing book meta.
	fn previous_book_meta(&self) 
-> Result<crate::inventory::meta::BookMeta<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/meta/BookMeta;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPreviousBookMeta",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::meta::BookMeta::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the book meta that the player is attempting to add to the book.
/// 
/// Note: this is a copy of the proposed new book meta. Use {@link
/// #setNewBookMeta(BookMeta)} to change what will actually be added to the
/// book.
	fn new_book_meta(&self) 
-> Result<crate::inventory::meta::BookMeta<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/meta/BookMeta;");
let res = self.jni_ref().call_method(&self.jni_object(),"getNewBookMeta",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::meta::BookMeta::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]
/// Gets the inventory slot number for the book item that triggered this event.This is a slot number on the player's hotbar in the range 0-8, or -1 for off hand.
	fn slot(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getSlot",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Sets the book meta that will actually be added to the book.
	fn set_new_book_meta(&self,new_book_meta: impl Into<crate::inventory::meta::BookMeta<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/meta/BookMeta;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(new_book_meta.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setNewBookMeta",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets whether or not the book is being signed. If a book is signed the
/// Material changes from BOOK_AND_QUILL to WRITTEN_BOOK.
	fn is_signing(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isSigning",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Sets whether or not the book is being signed. If a book is signed the
/// Material changes from BOOK_AND_QUILL to WRITTEN_BOOK.
	fn set_signing(&self,signing: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(signing.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setSigning",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerEditBookEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerEditBookEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerEditBookEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for PlayerEditBookEvent<'mc> {}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerEditBookEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerEditBookEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerEditBookEvent<'mc> {}
#[repr(C)]
pub struct PlayerRespawnEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerRespawnEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerRespawnEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerRespawnEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerRespawnEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerRespawnEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerRespawnEventTrait<'mc> for PlayerRespawnEvent<'mc> {}
pub trait PlayerRespawnEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,respawn_player: impl Into<crate::entity::Player<'mc>>,respawn_location: impl Into<crate::Location<'mc>>,is_bed_spawn: bool,is_anchor_spawn: std::option::Option<bool>,respawn_reason: std::option::Option<impl Into<crate::event::player::PlayerRespawnEventRespawnReason<'mc>>>) 
-> Result<crate::event::player::PlayerRespawnEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/entity/Player;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(respawn_player.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/Location;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(respawn_location.into().jni_object().clone())});
args.push(val_2);
sig += "Z";
let val_3 = jni::objects::JValueGen::Bool(is_bed_spawn.into());
args.push(val_3);
if let Some(a) = is_anchor_spawn {
sig += "Z";
let val_4 = jni::objects::JValueGen::Bool(a.into());
args.push(val_4);
}
if let Some(a) = respawn_reason {
sig += "Lorg/bukkit/event/player/PlayerRespawnEvent/RespawnReason;";
let val_5 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_5);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/player/PlayerRespawnEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerRespawnEvent::from_raw(&jni,res
)}
/// Gets the current respawn location
	fn respawn_location(&self) 
-> Result<crate::Location<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Location;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRespawnLocation",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the new respawn location
	fn set_respawn_location(&self,respawn_location: impl Into<crate::Location<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Location;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(respawn_location.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setRespawnLocation",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets whether the respawn location is the player's bed.
	fn is_bed_spawn(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isBedSpawn",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets whether the respawn location is the player's respawn anchor.
	fn is_anchor_spawn(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isAnchorSpawn",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets the reason this respawn event was called.
	fn respawn_reason(&self) 
-> Result<crate::event::player::PlayerRespawnEventRespawnReason<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/player/PlayerRespawnEvent/RespawnReason;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRespawnReason",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::player::PlayerRespawnEventRespawnReason::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerRespawnEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerRespawnEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerRespawnEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerRespawnEvent<'mc> {}
#[repr(C)]
pub struct PlayerItemBreakEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerItemBreakEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerItemBreakEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerItemBreakEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerItemBreakEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerItemBreakEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerItemBreakEventTrait<'mc> for PlayerItemBreakEvent<'mc> {}
pub trait PlayerItemBreakEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,broken_item: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<crate::event::player::PlayerItemBreakEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(broken_item.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/player/PlayerItemBreakEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerItemBreakEvent::from_raw(&jni,res
)}
/// Gets the item that broke
	fn broken_item(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBrokenItem",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerItemBreakEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerItemBreakEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerItemBreakEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerItemBreakEvent<'mc> {}
#[repr(C)]
pub struct PlayerBedLeaveEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerBedLeaveEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerBedLeaveEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerBedLeaveEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerBedLeaveEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerBedLeaveEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerBedLeaveEventTrait<'mc> for PlayerBedLeaveEvent<'mc> {}
pub trait PlayerBedLeaveEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,who: impl Into<crate::entity::Player<'mc>>,bed: impl Into<crate::block::Block<'mc>>,set_bed_spawn: bool) 
-> Result<crate::event::player::PlayerBedLeaveEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Lorg/bukkit/block/Block;Z)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(who.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(bed.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Bool(set_bed_spawn.into());
let cls = jni.find_class("org/bukkit/event/player/PlayerBedLeaveEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerBedLeaveEvent::from_raw(&jni,res
)}
/// Returns the bed block involved in this event.
	fn bed(&self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/Block;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBed",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Get if this event should set the new spawn location for the
/// {@link Player}.
/// 
/// This does not remove any existing spawn location, only prevent it from
/// being changed (if true).
/// 
/// To change a {@link Player}'s spawn location, use
/// {@link Player#setBedSpawnLocation(Location)}.
	fn should_set_spawn_location(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"shouldSetSpawnLocation",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Set if this event should set the new spawn location for the
/// {@link Player}.
/// 
/// This will not remove any existing spawn location, only prevent it from
/// being changed (if true).
/// 
/// To change a {@link Player}'s spawn location, use
/// {@link Player#setBedSpawnLocation(Location)}.
	fn set_spawn_location(&self,set_bed_spawn: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(set_bed_spawn.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setSpawnLocation",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_cancelled(&self,cancelled: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancelled.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerBedLeaveEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerBedLeaveEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerBedLeaveEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for PlayerBedLeaveEvent<'mc> {}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerBedLeaveEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerBedLeaveEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerBedLeaveEvent<'mc> {}
#[repr(C)]
pub struct PlayerSwapHandItemsEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerSwapHandItemsEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerSwapHandItemsEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerSwapHandItemsEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerSwapHandItemsEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerSwapHandItemsEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerSwapHandItemsEventTrait<'mc> for PlayerSwapHandItemsEvent<'mc> {}
pub trait PlayerSwapHandItemsEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,main_hand_item: impl Into<crate::inventory::ItemStack<'mc>>,off_hand_item: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<crate::event::player::PlayerSwapHandItemsEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(main_hand_item.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(off_hand_item.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/player/PlayerSwapHandItemsEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerSwapHandItemsEvent::from_raw(&jni,res
)}
/// Gets the item switched to the main hand.
	fn main_hand_item(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMainHandItem",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Sets the item in the main hand.
	fn set_main_hand_item(&self,main_hand_item: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(main_hand_item.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setMainHandItem",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the item switched to the off hand.
	fn off_hand_item(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getOffHandItem",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Sets the item in the off hand.
	fn set_off_hand_item(&self,off_hand_item: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(off_hand_item.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setOffHandItem",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerSwapHandItemsEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerSwapHandItemsEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerSwapHandItemsEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for PlayerSwapHandItemsEvent<'mc> {}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerSwapHandItemsEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerSwapHandItemsEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerSwapHandItemsEvent<'mc> {}
#[repr(C)]
pub struct PlayerVelocityEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerVelocityEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerVelocityEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerVelocityEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerVelocityEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerVelocityEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerVelocityEventTrait<'mc> for PlayerVelocityEvent<'mc> {}
pub trait PlayerVelocityEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,velocity: impl Into<crate::util::Vector<'mc>>) 
-> Result<crate::event::player::PlayerVelocityEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Lorg/bukkit/util/Vector;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(velocity.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/player/PlayerVelocityEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerVelocityEvent::from_raw(&jni,res
)}

	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the velocity vector that will be sent to the player
	fn velocity(&self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/util/Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"getVelocity",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the velocity vector in meters per tick that will be sent to the player
	fn set_velocity(&self,velocity: impl Into<crate::util::Vector<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(velocity.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setVelocity",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerVelocityEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerVelocityEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerVelocityEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for PlayerVelocityEvent<'mc> {}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerVelocityEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerVelocityEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerVelocityEvent<'mc> {}
#[repr(C)]
pub struct PlayerLoginEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerLoginEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerLoginEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerLoginEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerLoginEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerLoginEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerLoginEventTrait<'mc> for PlayerLoginEvent<'mc> {}
pub trait PlayerLoginEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// This constructor pre-configures the event with a result and message
	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,hostname: impl Into<String>,address: jni::objects::JObject<'mc>,result: std::option::Option<impl Into<crate::event::player::PlayerLoginEventResult<'mc>>>,message: std::option::Option<impl Into<String>>,real_address: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<crate::event::player::PlayerLoginEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/entity/Player;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
args.push(val_1);
sig += "Ljava/lang/String;";
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(hostname.into())?));
args.push(val_2);
sig += "Ljava/net/InetAddress;";
let val_3 = jni::objects::JValueGen::Object(address);
args.push(val_3);
if let Some(a) = result {
sig += "Lorg/bukkit/event/player/PlayerLoginEvent/Result;";
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_4);
}
if let Some(a) = message {
sig += "Ljava/lang/String;";
let val_5 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(a.into())?));
args.push(val_5);
}
if let Some(a) = real_address {
sig += "Ljava/net/InetAddress;";
let val_6 = jni::objects::JValueGen::Object(a);
args.push(val_6);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/player/PlayerLoginEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerLoginEvent::from_raw(&jni,res
)}
/// Gets the current result of the login, as an enum
	fn result(&self) 
-> Result<crate::event::player::PlayerLoginEventResult<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/player/PlayerLoginEvent/Result;");
let res = self.jni_ref().call_method(&self.jni_object(),"getResult",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::player::PlayerLoginEventResult::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the new result of the login, as an enum
	fn set_result(&self,result: impl Into<crate::event::player::PlayerLoginEventResult<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/event/player/PlayerLoginEvent/Result;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(result.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setResult",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the current kick message that will be used if getResult() !=
/// Result.ALLOWED
	fn kick_message(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getKickMessage",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Sets the kick message to display if getResult() != Result.ALLOWED
	fn set_kick_message(&self,message: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(message.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setKickMessage",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the hostname that the player used to connect to the server, or
/// blank if unknown
	fn hostname(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHostname",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Allows the player to log in
	fn allow(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"allow",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Disallows the player from logging in, with the given reason
	fn disallow(&self,result: impl Into<crate::event::player::PlayerLoginEventResult<'mc>>,message: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/event/player/PlayerLoginEvent/Result;Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(result.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(message.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"disallow",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the {@link InetAddress} for the Player associated with this event.
/// This method is provided as a workaround for player.getAddress()
/// returning null during PlayerLoginEvent.
	fn address(&self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/net/InetAddress;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAddress",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.l()?
)}
/// Gets the connection address of this player, regardless of whether it has
/// been spoofed or not.
	fn real_address(&self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/net/InetAddress;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRealAddress",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.l()?
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerLoginEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerLoginEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerLoginEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerLoginEvent<'mc> {}
#[repr(C)]
pub struct PlayerRecipeDiscoverEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerRecipeDiscoverEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerRecipeDiscoverEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerRecipeDiscoverEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerRecipeDiscoverEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerRecipeDiscoverEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerRecipeDiscoverEventTrait<'mc> for PlayerRecipeDiscoverEvent<'mc> {}
pub trait PlayerRecipeDiscoverEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,who: impl Into<crate::entity::Player<'mc>>,recipe: impl Into<crate::NamespacedKey<'mc>>) 
-> Result<crate::event::player::PlayerRecipeDiscoverEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Lorg/bukkit/NamespacedKey;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(who.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(recipe.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/player/PlayerRecipeDiscoverEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerRecipeDiscoverEvent::from_raw(&jni,res
)}
/// Get the namespaced key of the discovered recipe.
	fn recipe(&self) 
-> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/NamespacedKey;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRecipe",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::NamespacedKey::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerRecipeDiscoverEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerRecipeDiscoverEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerRecipeDiscoverEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for PlayerRecipeDiscoverEvent<'mc> {}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerRecipeDiscoverEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerRecipeDiscoverEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerRecipeDiscoverEvent<'mc> {}
#[repr(C)]
pub struct PlayerExpCooldownChangeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerExpCooldownChangeEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerExpCooldownChangeEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerExpCooldownChangeEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerExpCooldownChangeEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerExpCooldownChangeEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerExpCooldownChangeEventTrait<'mc> for PlayerExpCooldownChangeEvent<'mc> {}
pub trait PlayerExpCooldownChangeEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,newcooldown: i32,reason: impl Into<crate::event::player::PlayerExpCooldownChangeEventChangeReason<'mc>>) 
-> Result<crate::event::player::PlayerExpCooldownChangeEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;ILorg/bukkit/event/player/PlayerExpCooldownChangeEvent/ChangeReason;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Int(newcooldown);
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(reason.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/player/PlayerExpCooldownChangeEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerExpCooldownChangeEvent::from_raw(&jni,res
)}
/// Gets the reason for the change.
	fn reason(&self) 
-> Result<crate::event::player::PlayerExpCooldownChangeEventChangeReason<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/player/PlayerExpCooldownChangeEvent/ChangeReason;");
let res = self.jni_ref().call_method(&self.jni_object(),"getReason",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::player::PlayerExpCooldownChangeEventChangeReason::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the new cooldown for the player.
	fn new_cooldown(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getNewCooldown",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Sets the new cooldown for the player.
	fn set_new_cooldown(&self,new_cooldown: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(new_cooldown);
let res = self.jni_ref().call_method(&self.jni_object(),"setNewCooldown",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerExpCooldownChangeEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerExpCooldownChangeEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerExpCooldownChangeEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerExpCooldownChangeEvent<'mc> {}
#[repr(C)]
pub struct PlayerPickupArrowEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerPickupArrowEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerPickupArrowEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerPickupArrowEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerPickupArrowEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerPickupArrowEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerPickupArrowEventTrait<'mc> for PlayerPickupArrowEvent<'mc> {}
pub trait PlayerPickupArrowEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,item: impl Into<crate::entity::Item<'mc>>,arrow: impl Into<crate::entity::AbstractArrow<'mc>>) 
-> Result<crate::event::player::PlayerPickupArrowEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Lorg/bukkit/entity/Item;Lorg/bukkit/entity/AbstractArrow;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(arrow.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/player/PlayerPickupArrowEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerPickupArrowEvent::from_raw(&jni,res
)}
/// Get the arrow being picked up by the player
	fn arrow(&self) 
-> Result<crate::entity::AbstractArrow<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/AbstractArrow;");
let res = self.jni_ref().call_method(&self.jni_object(),"getArrow",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::AbstractArrow::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::player::PlayerPickupItemEvent<'mc>> for PlayerPickupArrowEvent<'mc>{

fn into(self) -> crate::event::player::PlayerPickupItemEvent<'mc> {

crate::event::player::PlayerPickupItemEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerPickupArrowEvent into crate::event::player::PlayerPickupItemEvent")

   }
}
impl<'mc> crate::event::player::PlayerPickupItemEventTrait<'mc> for PlayerPickupArrowEvent<'mc> {}
#[repr(C)]
pub struct PlayerRegisterChannelEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerRegisterChannelEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerRegisterChannelEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerRegisterChannelEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerRegisterChannelEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerRegisterChannelEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerRegisterChannelEventTrait<'mc> for PlayerRegisterChannelEvent<'mc> {}
pub trait PlayerRegisterChannelEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,channel: impl Into<String>) 
-> Result<crate::event::player::PlayerRegisterChannelEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(channel.into())?));
let cls = jni.find_class("org/bukkit/event/player/PlayerRegisterChannelEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerRegisterChannelEvent::from_raw(&jni,res
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::player::PlayerChannelEvent<'mc>> for PlayerRegisterChannelEvent<'mc>{

fn into(self) -> crate::event::player::PlayerChannelEvent<'mc> {

crate::event::player::PlayerChannelEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerRegisterChannelEvent into crate::event::player::PlayerChannelEvent")

   }
}
impl<'mc> crate::event::player::PlayerChannelEventTrait<'mc> for PlayerRegisterChannelEvent<'mc> {}
pub enum PlayerAnimationType<'mc> {
	ArmSwing {inner: PlayerAnimationTypeStruct<'mc>},
	OffArmSwing {inner: PlayerAnimationTypeStruct<'mc>},
}
impl<'mc> std::fmt::Display for PlayerAnimationType<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           PlayerAnimationType::ArmSwing { .. } => f.write_str("ARM_SWING"),
           PlayerAnimationType::OffArmSwing { .. } => f.write_str("OFF_ARM_SWING"),
       }
   }
}

        impl<'mc> PlayerAnimationTypeTrait<'mc> for PlayerAnimationType<'mc> {}
        
        pub trait PlayerAnimationTypeTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<PlayerAnimationType<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/event/player/PlayerAnimationType");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/event/player/PlayerAnimationType;",
                    vec![jni::objects::JValueGen::from(val_1)],
                );
                let res = env.translate_error(res)?;
                let obj = res.l()?;
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    
"ARM_SWING" => Ok(PlayerAnimationType::ArmSwing { inner: PlayerAnimationTypeStruct::from_raw(env,obj)?}),
"OFF_ARM_SWING" => Ok(PlayerAnimationType::OffArmSwing { inner: PlayerAnimationTypeStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct PlayerAnimationTypeStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerAnimationType<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::ArmSwing { inner } => inner.0.clone(),
Self::OffArmSwing { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::ArmSwing { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::OffArmSwing { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerAnimationType<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerAnimationType from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerAnimationType")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerAnimationType object, got {}",
                    name
                )
                .into())
            } else {
    
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    "ARM_SWING" => Ok(PlayerAnimationType::ArmSwing { inner: PlayerAnimationTypeStruct::from_raw(env,obj)?}),"OFF_ARM_SWING" => Ok(PlayerAnimationType::OffArmSwing { inner: PlayerAnimationTypeStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for PlayerAnimationTypeStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerAnimationTypeStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerAnimationTypeStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerAnimationType")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerAnimationTypeStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerAnimationTypeStruct<'mc> {

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::player::PlayerAnimationType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/player/PlayerAnimationType;");
let cls = jni.find_class("org/bukkit/event/player/PlayerAnimationType"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::player::PlayerAnimationType::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct PlayerJoinEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerJoinEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerJoinEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerJoinEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerJoinEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerJoinEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerJoinEventTrait<'mc> for PlayerJoinEvent<'mc> {}
pub trait PlayerJoinEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player_joined: impl Into<crate::entity::Player<'mc>>,join_message: impl Into<String>) 
-> Result<crate::event::player::PlayerJoinEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player_joined.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(join_message.into())?));
let cls = jni.find_class("org/bukkit/event/player/PlayerJoinEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerJoinEvent::from_raw(&jni,res
)}
/// Gets the join message to send to all online players
	fn join_message(&self) 
-> Result<Option<String>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getJoinMessage",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)
)}
/// Sets the join message to send to all online players
	fn set_join_message(&self,join_message: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(join_message.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setJoinMessage",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerJoinEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerJoinEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerJoinEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerJoinEvent<'mc> {}
#[repr(C)]
pub struct PlayerDropItemEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerDropItemEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerDropItemEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerDropItemEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerDropItemEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerDropItemEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerDropItemEventTrait<'mc> for PlayerDropItemEvent<'mc> {}
pub trait PlayerDropItemEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,drop: impl Into<crate::entity::Item<'mc>>) 
-> Result<crate::event::player::PlayerDropItemEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Lorg/bukkit/entity/Item;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(drop.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/player/PlayerDropItemEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerDropItemEvent::from_raw(&jni,res
)}
/// Gets the ItemDrop created by the player
	fn item_drop(&self) 
-> Result<crate::entity::Item<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Item;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItemDrop",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Item::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerDropItemEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerDropItemEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerDropItemEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for PlayerDropItemEvent<'mc> {}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerDropItemEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerDropItemEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerDropItemEvent<'mc> {}
pub enum PlayerBedEnterEventBedEnterResult<'mc> {
	Ok {inner: PlayerBedEnterEventBedEnterResultStruct<'mc>},
	NotPossibleHere {inner: PlayerBedEnterEventBedEnterResultStruct<'mc>},
	NotPossibleNow {inner: PlayerBedEnterEventBedEnterResultStruct<'mc>},
	TooFarAway {inner: PlayerBedEnterEventBedEnterResultStruct<'mc>},
	NotSafe {inner: PlayerBedEnterEventBedEnterResultStruct<'mc>},
	OtherProblem {inner: PlayerBedEnterEventBedEnterResultStruct<'mc>},
}
impl<'mc> std::fmt::Display for PlayerBedEnterEventBedEnterResult<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           PlayerBedEnterEventBedEnterResult::Ok { .. } => f.write_str("OK"),
           PlayerBedEnterEventBedEnterResult::NotPossibleHere { .. } => f.write_str("NOT_POSSIBLE_HERE"),
           PlayerBedEnterEventBedEnterResult::NotPossibleNow { .. } => f.write_str("NOT_POSSIBLE_NOW"),
           PlayerBedEnterEventBedEnterResult::TooFarAway { .. } => f.write_str("TOO_FAR_AWAY"),
           PlayerBedEnterEventBedEnterResult::NotSafe { .. } => f.write_str("NOT_SAFE"),
           PlayerBedEnterEventBedEnterResult::OtherProblem { .. } => f.write_str("OTHER_PROBLEM"),
       }
   }
}

        impl<'mc> PlayerBedEnterEventBedEnterResultTrait<'mc> for PlayerBedEnterEventBedEnterResult<'mc> {}
        
        pub trait PlayerBedEnterEventBedEnterResultTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<PlayerBedEnterEventBedEnterResult<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/event/player/PlayerBedEnterEvent/BedEnterResult");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/event/player/PlayerBedEnterEvent/BedEnterResult;",
                    vec![jni::objects::JValueGen::from(val_1)],
                );
                let res = env.translate_error(res)?;
                let obj = res.l()?;
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    
"OK" => Ok(PlayerBedEnterEventBedEnterResult::Ok { inner: PlayerBedEnterEventBedEnterResultStruct::from_raw(env,obj)?}),
"NOT_POSSIBLE_HERE" => Ok(PlayerBedEnterEventBedEnterResult::NotPossibleHere { inner: PlayerBedEnterEventBedEnterResultStruct::from_raw(env,obj)?}),
"NOT_POSSIBLE_NOW" => Ok(PlayerBedEnterEventBedEnterResult::NotPossibleNow { inner: PlayerBedEnterEventBedEnterResultStruct::from_raw(env,obj)?}),
"TOO_FAR_AWAY" => Ok(PlayerBedEnterEventBedEnterResult::TooFarAway { inner: PlayerBedEnterEventBedEnterResultStruct::from_raw(env,obj)?}),
"NOT_SAFE" => Ok(PlayerBedEnterEventBedEnterResult::NotSafe { inner: PlayerBedEnterEventBedEnterResultStruct::from_raw(env,obj)?}),
"OTHER_PROBLEM" => Ok(PlayerBedEnterEventBedEnterResult::OtherProblem { inner: PlayerBedEnterEventBedEnterResultStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct PlayerBedEnterEventBedEnterResultStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerBedEnterEventBedEnterResult<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::Ok { inner } => inner.0.clone(),
Self::NotPossibleHere { inner } => inner.0.clone(),
Self::NotPossibleNow { inner } => inner.0.clone(),
Self::TooFarAway { inner } => inner.0.clone(),
Self::NotSafe { inner } => inner.0.clone(),
Self::OtherProblem { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::Ok { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::NotPossibleHere { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::NotPossibleNow { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::TooFarAway { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::NotSafe { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::OtherProblem { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerBedEnterEventBedEnterResult<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerBedEnterEventBedEnterResult from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerBedEnterEvent/BedEnterResult")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerBedEnterEventBedEnterResult object, got {}",
                    name
                )
                .into())
            } else {
    
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    "OK" => Ok(PlayerBedEnterEventBedEnterResult::Ok { inner: PlayerBedEnterEventBedEnterResultStruct::from_raw(env,obj)?}),"NOT_POSSIBLE_HERE" => Ok(PlayerBedEnterEventBedEnterResult::NotPossibleHere { inner: PlayerBedEnterEventBedEnterResultStruct::from_raw(env,obj)?}),"NOT_POSSIBLE_NOW" => Ok(PlayerBedEnterEventBedEnterResult::NotPossibleNow { inner: PlayerBedEnterEventBedEnterResultStruct::from_raw(env,obj)?}),"TOO_FAR_AWAY" => Ok(PlayerBedEnterEventBedEnterResult::TooFarAway { inner: PlayerBedEnterEventBedEnterResultStruct::from_raw(env,obj)?}),"NOT_SAFE" => Ok(PlayerBedEnterEventBedEnterResult::NotSafe { inner: PlayerBedEnterEventBedEnterResultStruct::from_raw(env,obj)?}),"OTHER_PROBLEM" => Ok(PlayerBedEnterEventBedEnterResult::OtherProblem { inner: PlayerBedEnterEventBedEnterResultStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for PlayerBedEnterEventBedEnterResultStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerBedEnterEventBedEnterResultStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerBedEnterEventBedEnterResultStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerBedEnterEvent/BedEnterResult")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerBedEnterEventBedEnterResultStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerBedEnterEventBedEnterResultStruct<'mc> {

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::player::PlayerBedEnterEventBedEnterResult<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/player/PlayerBedEnterEvent/BedEnterResult;");
let cls = jni.find_class("org/bukkit/event/player/PlayerBedEnterEvent/BedEnterResult"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::player::PlayerBedEnterEventBedEnterResult::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct PlayerGameModeChangeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerGameModeChangeEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerGameModeChangeEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerGameModeChangeEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerGameModeChangeEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerGameModeChangeEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerGameModeChangeEventTrait<'mc> for PlayerGameModeChangeEvent<'mc> {}
pub trait PlayerGameModeChangeEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,new_game_mode: impl Into<crate::GameMode<'mc>>) 
-> Result<crate::event::player::PlayerGameModeChangeEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Lorg/bukkit/GameMode;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(new_game_mode.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/player/PlayerGameModeChangeEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerGameModeChangeEvent::from_raw(&jni,res
)}

	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the GameMode the player is switched to.
	fn new_game_mode(&self) 
-> Result<crate::GameMode<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/GameMode;");
let res = self.jni_ref().call_method(&self.jni_object(),"getNewGameMode",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::GameMode::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerGameModeChangeEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerGameModeChangeEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerGameModeChangeEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for PlayerGameModeChangeEvent<'mc> {}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerGameModeChangeEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerGameModeChangeEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerGameModeChangeEvent<'mc> {}
#[repr(C)]
pub struct PlayerItemConsumeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerItemConsumeEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerItemConsumeEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerItemConsumeEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerItemConsumeEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerItemConsumeEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerItemConsumeEventTrait<'mc> for PlayerItemConsumeEvent<'mc> {}
pub trait PlayerItemConsumeEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,item: impl Into<crate::inventory::ItemStack<'mc>>,hand: std::option::Option<impl Into<crate::inventory::EquipmentSlot<'mc>>>) 
-> Result<crate::event::player::PlayerItemConsumeEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/entity/Player;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
args.push(val_2);
if let Some(a) = hand {
sig += "Lorg/bukkit/inventory/EquipmentSlot;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_3);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/player/PlayerItemConsumeEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerItemConsumeEvent::from_raw(&jni,res
)}
/// Gets the item that is being consumed. Modifying the returned item will
/// have no effect, you must use {@link
/// #setItem(org.bukkit.inventory.ItemStack)} instead.
	fn item(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItem",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Set the item being consumed
	fn set_item(&self,item: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setItem",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the hand used to consume the item.
	fn hand(&self) 
-> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/EquipmentSlot;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHand",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerItemConsumeEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerItemConsumeEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerItemConsumeEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for PlayerItemConsumeEvent<'mc> {}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerItemConsumeEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerItemConsumeEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerItemConsumeEvent<'mc> {}
pub enum PlayerResourcePackStatusEventStatus<'mc> {
	SuccessfullyLoaded {inner: PlayerResourcePackStatusEventStatusStruct<'mc>},
	Declined {inner: PlayerResourcePackStatusEventStatusStruct<'mc>},
	FailedDownload {inner: PlayerResourcePackStatusEventStatusStruct<'mc>},
	Accepted {inner: PlayerResourcePackStatusEventStatusStruct<'mc>},
	Downloaded {inner: PlayerResourcePackStatusEventStatusStruct<'mc>},
	InvalidUrl {inner: PlayerResourcePackStatusEventStatusStruct<'mc>},
	FailedReload {inner: PlayerResourcePackStatusEventStatusStruct<'mc>},
	Discarded {inner: PlayerResourcePackStatusEventStatusStruct<'mc>},
}
impl<'mc> std::fmt::Display for PlayerResourcePackStatusEventStatus<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           PlayerResourcePackStatusEventStatus::SuccessfullyLoaded { .. } => f.write_str("SUCCESSFULLY_LOADED"),
           PlayerResourcePackStatusEventStatus::Declined { .. } => f.write_str("DECLINED"),
           PlayerResourcePackStatusEventStatus::FailedDownload { .. } => f.write_str("FAILED_DOWNLOAD"),
           PlayerResourcePackStatusEventStatus::Accepted { .. } => f.write_str("ACCEPTED"),
           PlayerResourcePackStatusEventStatus::Downloaded { .. } => f.write_str("DOWNLOADED"),
           PlayerResourcePackStatusEventStatus::InvalidUrl { .. } => f.write_str("INVALID_URL"),
           PlayerResourcePackStatusEventStatus::FailedReload { .. } => f.write_str("FAILED_RELOAD"),
           PlayerResourcePackStatusEventStatus::Discarded { .. } => f.write_str("DISCARDED"),
       }
   }
}

        impl<'mc> PlayerResourcePackStatusEventStatusTrait<'mc> for PlayerResourcePackStatusEventStatus<'mc> {}
        
        pub trait PlayerResourcePackStatusEventStatusTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<PlayerResourcePackStatusEventStatus<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/event/player/PlayerResourcePackStatusEvent/Status");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/event/player/PlayerResourcePackStatusEvent/Status;",
                    vec![jni::objects::JValueGen::from(val_1)],
                );
                let res = env.translate_error(res)?;
                let obj = res.l()?;
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    
"SUCCESSFULLY_LOADED" => Ok(PlayerResourcePackStatusEventStatus::SuccessfullyLoaded { inner: PlayerResourcePackStatusEventStatusStruct::from_raw(env,obj)?}),
"DECLINED" => Ok(PlayerResourcePackStatusEventStatus::Declined { inner: PlayerResourcePackStatusEventStatusStruct::from_raw(env,obj)?}),
"FAILED_DOWNLOAD" => Ok(PlayerResourcePackStatusEventStatus::FailedDownload { inner: PlayerResourcePackStatusEventStatusStruct::from_raw(env,obj)?}),
"ACCEPTED" => Ok(PlayerResourcePackStatusEventStatus::Accepted { inner: PlayerResourcePackStatusEventStatusStruct::from_raw(env,obj)?}),
"DOWNLOADED" => Ok(PlayerResourcePackStatusEventStatus::Downloaded { inner: PlayerResourcePackStatusEventStatusStruct::from_raw(env,obj)?}),
"INVALID_URL" => Ok(PlayerResourcePackStatusEventStatus::InvalidUrl { inner: PlayerResourcePackStatusEventStatusStruct::from_raw(env,obj)?}),
"FAILED_RELOAD" => Ok(PlayerResourcePackStatusEventStatus::FailedReload { inner: PlayerResourcePackStatusEventStatusStruct::from_raw(env,obj)?}),
"DISCARDED" => Ok(PlayerResourcePackStatusEventStatus::Discarded { inner: PlayerResourcePackStatusEventStatusStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct PlayerResourcePackStatusEventStatusStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerResourcePackStatusEventStatus<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::SuccessfullyLoaded { inner } => inner.0.clone(),
Self::Declined { inner } => inner.0.clone(),
Self::FailedDownload { inner } => inner.0.clone(),
Self::Accepted { inner } => inner.0.clone(),
Self::Downloaded { inner } => inner.0.clone(),
Self::InvalidUrl { inner } => inner.0.clone(),
Self::FailedReload { inner } => inner.0.clone(),
Self::Discarded { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::SuccessfullyLoaded { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Declined { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::FailedDownload { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Accepted { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Downloaded { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::InvalidUrl { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::FailedReload { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Discarded { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerResourcePackStatusEventStatus<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerResourcePackStatusEventStatus from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerResourcePackStatusEvent/Status")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerResourcePackStatusEventStatus object, got {}",
                    name
                )
                .into())
            } else {
    
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    "SUCCESSFULLY_LOADED" => Ok(PlayerResourcePackStatusEventStatus::SuccessfullyLoaded { inner: PlayerResourcePackStatusEventStatusStruct::from_raw(env,obj)?}),"DECLINED" => Ok(PlayerResourcePackStatusEventStatus::Declined { inner: PlayerResourcePackStatusEventStatusStruct::from_raw(env,obj)?}),"FAILED_DOWNLOAD" => Ok(PlayerResourcePackStatusEventStatus::FailedDownload { inner: PlayerResourcePackStatusEventStatusStruct::from_raw(env,obj)?}),"ACCEPTED" => Ok(PlayerResourcePackStatusEventStatus::Accepted { inner: PlayerResourcePackStatusEventStatusStruct::from_raw(env,obj)?}),"DOWNLOADED" => Ok(PlayerResourcePackStatusEventStatus::Downloaded { inner: PlayerResourcePackStatusEventStatusStruct::from_raw(env,obj)?}),"INVALID_URL" => Ok(PlayerResourcePackStatusEventStatus::InvalidUrl { inner: PlayerResourcePackStatusEventStatusStruct::from_raw(env,obj)?}),"FAILED_RELOAD" => Ok(PlayerResourcePackStatusEventStatus::FailedReload { inner: PlayerResourcePackStatusEventStatusStruct::from_raw(env,obj)?}),"DISCARDED" => Ok(PlayerResourcePackStatusEventStatus::Discarded { inner: PlayerResourcePackStatusEventStatusStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for PlayerResourcePackStatusEventStatusStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerResourcePackStatusEventStatusStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerResourcePackStatusEventStatusStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerResourcePackStatusEvent/Status")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerResourcePackStatusEventStatusStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerResourcePackStatusEventStatusStruct<'mc> {

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::player::PlayerResourcePackStatusEventStatus<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/player/PlayerResourcePackStatusEvent/Status;");
let cls = jni.find_class("org/bukkit/event/player/PlayerResourcePackStatusEvent/Status"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::player::PlayerResourcePackStatusEventStatus::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
pub enum PlayerPreLoginEventResult<'mc> {
	Allowed {inner: PlayerPreLoginEventResultStruct<'mc>},
	KickFull {inner: PlayerPreLoginEventResultStruct<'mc>},
	KickBanned {inner: PlayerPreLoginEventResultStruct<'mc>},
	KickWhitelist {inner: PlayerPreLoginEventResultStruct<'mc>},
	KickOther {inner: PlayerPreLoginEventResultStruct<'mc>},
}
impl<'mc> std::fmt::Display for PlayerPreLoginEventResult<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           PlayerPreLoginEventResult::Allowed { .. } => f.write_str("ALLOWED"),
           PlayerPreLoginEventResult::KickFull { .. } => f.write_str("KICK_FULL"),
           PlayerPreLoginEventResult::KickBanned { .. } => f.write_str("KICK_BANNED"),
           PlayerPreLoginEventResult::KickWhitelist { .. } => f.write_str("KICK_WHITELIST"),
           PlayerPreLoginEventResult::KickOther { .. } => f.write_str("KICK_OTHER"),
       }
   }
}

        impl<'mc> PlayerPreLoginEventResultTrait<'mc> for PlayerPreLoginEventResult<'mc> {}
        
        pub trait PlayerPreLoginEventResultTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<PlayerPreLoginEventResult<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/event/player/PlayerPreLoginEvent/Result");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/event/player/PlayerPreLoginEvent/Result;",
                    vec![jni::objects::JValueGen::from(val_1)],
                );
                let res = env.translate_error(res)?;
                let obj = res.l()?;
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    
"ALLOWED" => Ok(PlayerPreLoginEventResult::Allowed { inner: PlayerPreLoginEventResultStruct::from_raw(env,obj)?}),
"KICK_FULL" => Ok(PlayerPreLoginEventResult::KickFull { inner: PlayerPreLoginEventResultStruct::from_raw(env,obj)?}),
"KICK_BANNED" => Ok(PlayerPreLoginEventResult::KickBanned { inner: PlayerPreLoginEventResultStruct::from_raw(env,obj)?}),
"KICK_WHITELIST" => Ok(PlayerPreLoginEventResult::KickWhitelist { inner: PlayerPreLoginEventResultStruct::from_raw(env,obj)?}),
"KICK_OTHER" => Ok(PlayerPreLoginEventResult::KickOther { inner: PlayerPreLoginEventResultStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct PlayerPreLoginEventResultStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerPreLoginEventResult<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::Allowed { inner } => inner.0.clone(),
Self::KickFull { inner } => inner.0.clone(),
Self::KickBanned { inner } => inner.0.clone(),
Self::KickWhitelist { inner } => inner.0.clone(),
Self::KickOther { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::Allowed { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::KickFull { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::KickBanned { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::KickWhitelist { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::KickOther { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerPreLoginEventResult<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerPreLoginEventResult from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerPreLoginEvent/Result")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerPreLoginEventResult object, got {}",
                    name
                )
                .into())
            } else {
    
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    "ALLOWED" => Ok(PlayerPreLoginEventResult::Allowed { inner: PlayerPreLoginEventResultStruct::from_raw(env,obj)?}),"KICK_FULL" => Ok(PlayerPreLoginEventResult::KickFull { inner: PlayerPreLoginEventResultStruct::from_raw(env,obj)?}),"KICK_BANNED" => Ok(PlayerPreLoginEventResult::KickBanned { inner: PlayerPreLoginEventResultStruct::from_raw(env,obj)?}),"KICK_WHITELIST" => Ok(PlayerPreLoginEventResult::KickWhitelist { inner: PlayerPreLoginEventResultStruct::from_raw(env,obj)?}),"KICK_OTHER" => Ok(PlayerPreLoginEventResult::KickOther { inner: PlayerPreLoginEventResultStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for PlayerPreLoginEventResultStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerPreLoginEventResultStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerPreLoginEventResultStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerPreLoginEvent/Result")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerPreLoginEventResultStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerPreLoginEventResultStruct<'mc> {

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::player::PlayerPreLoginEventResult<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/player/PlayerPreLoginEvent/Result;");
let cls = jni.find_class("org/bukkit/event/player/PlayerPreLoginEvent/Result"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::player::PlayerPreLoginEventResult::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct PlayerHideEntityEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerHideEntityEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerHideEntityEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerHideEntityEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerHideEntityEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerHideEntityEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerHideEntityEventTrait<'mc> for PlayerHideEntityEvent<'mc> {}
pub trait PlayerHideEntityEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,who: impl Into<crate::entity::Player<'mc>>,entity: impl Into<crate::entity::Entity<'mc>>) 
-> Result<crate::event::player::PlayerHideEntityEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Lorg/bukkit/entity/Entity;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(who.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(entity.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/player/PlayerHideEntityEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerHideEntityEvent::from_raw(&jni,res
)}
/// Gets the entity which has been hidden from the player.
	fn entity(&self) 
-> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Entity;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEntity",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Entity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerHideEntityEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerHideEntityEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerHideEntityEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerHideEntityEvent<'mc> {}
pub enum PlayerFishEventState<'mc> {
	Fishing {inner: PlayerFishEventStateStruct<'mc>},
	CaughtFish {inner: PlayerFishEventStateStruct<'mc>},
	CaughtEntity {inner: PlayerFishEventStateStruct<'mc>},
	InGround {inner: PlayerFishEventStateStruct<'mc>},
	FailedAttempt {inner: PlayerFishEventStateStruct<'mc>},
	ReelIn {inner: PlayerFishEventStateStruct<'mc>},
	Bite {inner: PlayerFishEventStateStruct<'mc>},
}
impl<'mc> std::fmt::Display for PlayerFishEventState<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           PlayerFishEventState::Fishing { .. } => f.write_str("FISHING"),
           PlayerFishEventState::CaughtFish { .. } => f.write_str("CAUGHT_FISH"),
           PlayerFishEventState::CaughtEntity { .. } => f.write_str("CAUGHT_ENTITY"),
           PlayerFishEventState::InGround { .. } => f.write_str("IN_GROUND"),
           PlayerFishEventState::FailedAttempt { .. } => f.write_str("FAILED_ATTEMPT"),
           PlayerFishEventState::ReelIn { .. } => f.write_str("REEL_IN"),
           PlayerFishEventState::Bite { .. } => f.write_str("BITE"),
       }
   }
}

        impl<'mc> PlayerFishEventStateTrait<'mc> for PlayerFishEventState<'mc> {}
        
        pub trait PlayerFishEventStateTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<PlayerFishEventState<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/event/player/PlayerFishEvent/State");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/event/player/PlayerFishEvent/State;",
                    vec![jni::objects::JValueGen::from(val_1)],
                );
                let res = env.translate_error(res)?;
                let obj = res.l()?;
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    
"FISHING" => Ok(PlayerFishEventState::Fishing { inner: PlayerFishEventStateStruct::from_raw(env,obj)?}),
"CAUGHT_FISH" => Ok(PlayerFishEventState::CaughtFish { inner: PlayerFishEventStateStruct::from_raw(env,obj)?}),
"CAUGHT_ENTITY" => Ok(PlayerFishEventState::CaughtEntity { inner: PlayerFishEventStateStruct::from_raw(env,obj)?}),
"IN_GROUND" => Ok(PlayerFishEventState::InGround { inner: PlayerFishEventStateStruct::from_raw(env,obj)?}),
"FAILED_ATTEMPT" => Ok(PlayerFishEventState::FailedAttempt { inner: PlayerFishEventStateStruct::from_raw(env,obj)?}),
"REEL_IN" => Ok(PlayerFishEventState::ReelIn { inner: PlayerFishEventStateStruct::from_raw(env,obj)?}),
"BITE" => Ok(PlayerFishEventState::Bite { inner: PlayerFishEventStateStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct PlayerFishEventStateStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerFishEventState<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::Fishing { inner } => inner.0.clone(),
Self::CaughtFish { inner } => inner.0.clone(),
Self::CaughtEntity { inner } => inner.0.clone(),
Self::InGround { inner } => inner.0.clone(),
Self::FailedAttempt { inner } => inner.0.clone(),
Self::ReelIn { inner } => inner.0.clone(),
Self::Bite { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::Fishing { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::CaughtFish { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::CaughtEntity { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::InGround { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::FailedAttempt { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::ReelIn { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Bite { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerFishEventState<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerFishEventState from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerFishEvent/State")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerFishEventState object, got {}",
                    name
                )
                .into())
            } else {
    
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    "FISHING" => Ok(PlayerFishEventState::Fishing { inner: PlayerFishEventStateStruct::from_raw(env,obj)?}),"CAUGHT_FISH" => Ok(PlayerFishEventState::CaughtFish { inner: PlayerFishEventStateStruct::from_raw(env,obj)?}),"CAUGHT_ENTITY" => Ok(PlayerFishEventState::CaughtEntity { inner: PlayerFishEventStateStruct::from_raw(env,obj)?}),"IN_GROUND" => Ok(PlayerFishEventState::InGround { inner: PlayerFishEventStateStruct::from_raw(env,obj)?}),"FAILED_ATTEMPT" => Ok(PlayerFishEventState::FailedAttempt { inner: PlayerFishEventStateStruct::from_raw(env,obj)?}),"REEL_IN" => Ok(PlayerFishEventState::ReelIn { inner: PlayerFishEventStateStruct::from_raw(env,obj)?}),"BITE" => Ok(PlayerFishEventState::Bite { inner: PlayerFishEventStateStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for PlayerFishEventStateStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerFishEventStateStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerFishEventStateStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerFishEvent/State")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerFishEventStateStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerFishEventStateStruct<'mc> {

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::player::PlayerFishEventState<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/player/PlayerFishEvent/State;");
let cls = jni.find_class("org/bukkit/event/player/PlayerFishEvent/State"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::player::PlayerFishEventState::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct PlayerSpawnChangeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerSpawnChangeEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerSpawnChangeEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerSpawnChangeEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerSpawnChangeEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerSpawnChangeEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerSpawnChangeEventTrait<'mc> for PlayerSpawnChangeEvent<'mc> {}
pub trait PlayerSpawnChangeEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,new_spawn: impl Into<crate::Location<'mc>>,forced: bool,cause: impl Into<crate::event::player::PlayerSpawnChangeEventCause<'mc>>) 
-> Result<crate::event::player::PlayerSpawnChangeEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Lorg/bukkit/Location;ZLorg/bukkit/event/player/PlayerSpawnChangeEvent/Cause;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(new_spawn.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Bool(forced.into());
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(cause.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/player/PlayerSpawnChangeEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerSpawnChangeEvent::from_raw(&jni,res
)}
/// Gets the cause of spawn change.
	fn cause(&self) 
-> Result<crate::event::player::PlayerSpawnChangeEventCause<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/player/PlayerSpawnChangeEvent/Cause;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCause",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::player::PlayerSpawnChangeEventCause::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets if the spawn position will be used regardless of bed obstruction
/// rules.
	fn is_forced(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isForced",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Sets if the spawn position will be used regardless of bed obstruction
/// rules.
	fn set_forced(&self,forced: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(forced.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setForced",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the new spawn to be set.
	fn new_spawn(&self) 
-> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Location;");
let res = self.jni_ref().call_method(&self.jni_object(),"getNewSpawn",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Sets the new spawn location.
	fn set_new_spawn(&self,new_spawn: impl Into<crate::Location<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Location;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(new_spawn.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setNewSpawn",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerSpawnChangeEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerSpawnChangeEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerSpawnChangeEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for PlayerSpawnChangeEvent<'mc> {}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerSpawnChangeEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerSpawnChangeEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerSpawnChangeEvent<'mc> {}
#[repr(C)]
pub struct AsyncPlayerPreLoginEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for AsyncPlayerPreLoginEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for AsyncPlayerPreLoginEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate AsyncPlayerPreLoginEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/AsyncPlayerPreLoginEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a AsyncPlayerPreLoginEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> AsyncPlayerPreLoginEventTrait<'mc> for AsyncPlayerPreLoginEvent<'mc> {}
pub trait AsyncPlayerPreLoginEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,name: impl Into<String>,ip_address: jni::objects::JObject<'mc>,unique_id: std::option::Option<impl Into<blackboxmc_java::util::JavaUUID<'mc>>>) 
-> Result<crate::event::player::AsyncPlayerPreLoginEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(name.into())?));
args.push(val_1);
sig += "Ljava/net/InetAddress;";
let val_2 = jni::objects::JValueGen::Object(ip_address);
args.push(val_2);
if let Some(a) = unique_id {
sig += "Ljava/util/UUID;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_3);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/player/AsyncPlayerPreLoginEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::AsyncPlayerPreLoginEvent::from_raw(&jni,res
)}
/// Gets the current result of the login, as an enum
	fn login_result(&self) 
-> Result<crate::event::player::AsyncPlayerPreLoginEventResult<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/player/AsyncPlayerPreLoginEvent/Result;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLoginResult",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::player::AsyncPlayerPreLoginEventResult::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]
/// Gets the current result of the login, as an enum
	fn result(&self) 
-> Result<crate::event::player::PlayerPreLoginEventResult<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/player/PlayerPreLoginEvent/Result;");
let res = self.jni_ref().call_method(&self.jni_object(),"getResult",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::player::PlayerPreLoginEventResult::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the new result of the login, as an enum
	fn set_login_result(&self,result: impl Into<crate::event::player::AsyncPlayerPreLoginEventResult<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/event/player/AsyncPlayerPreLoginEvent/Result;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(result.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setLoginResult",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[deprecated]
/// Sets the new result of the login, as an enum
	fn set_result(&self,result: impl Into<crate::event::player::PlayerPreLoginEventResult<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/event/player/PlayerPreLoginEvent/Result;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(result.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setResult",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the current kick message that will be used if getResult() !=
/// Result.ALLOWED
	fn kick_message(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getKickMessage",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Sets the kick message to display if getResult() != Result.ALLOWED
	fn set_kick_message(&self,message: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(message.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setKickMessage",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Allows the player to log in
	fn allow(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"allow",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[deprecated]
/// Disallows the player from logging in, with the given reason
	fn disallow(&self,result: impl Into<crate::event::player::PlayerPreLoginEventResult<'mc>>,message: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/event/player/PlayerPreLoginEvent/Result;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(result.into().jni_object().clone())});
args.push(val_1);
sig += "Ljava/lang/String;";
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(message.into())?));
args.push(val_2);
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"disallow",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the player's name.
	fn name(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Gets the player IP address.
	fn address(&self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/net/InetAddress;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAddress",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.l()?
)}
/// Gets the player's unique ID.
	fn unique_id(&self) 
-> Result<blackboxmc_java::util::JavaUUID<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/UUID;");
let res = self.jni_ref().call_method(&self.jni_object(),"getUniqueId",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaUUID::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/AsyncPlayerPreLoginEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Event<'mc>> for AsyncPlayerPreLoginEvent<'mc>{

fn into(self) -> crate::event::Event<'mc> {

crate::event::Event::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting AsyncPlayerPreLoginEvent into crate::event::Event")

   }
}
impl<'mc> crate::event::EventTrait<'mc> for AsyncPlayerPreLoginEvent<'mc> {}
#[repr(C)]
pub struct PlayerTeleportEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerTeleportEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerTeleportEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerTeleportEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerTeleportEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerTeleportEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerTeleportEventTrait<'mc> for PlayerTeleportEvent<'mc> {}
pub trait PlayerTeleportEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,from: impl Into<crate::Location<'mc>>,to: impl Into<crate::Location<'mc>>,cause: std::option::Option<impl Into<crate::event::player::PlayerTeleportEventTeleportCause<'mc>>>) 
-> Result<crate::event::player::PlayerTeleportEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/entity/Player;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/Location;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(from.into().jni_object().clone())});
args.push(val_2);
sig += "Lorg/bukkit/Location;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(to.into().jni_object().clone())});
args.push(val_3);
if let Some(a) = cause {
sig += "Lorg/bukkit/event/player/PlayerTeleportEvent/TeleportCause;";
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_4);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/player/PlayerTeleportEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerTeleportEvent::from_raw(&jni,res
)}
/// Gets the cause of this teleportation event
	fn cause(&self) 
-> Result<crate::event::player::PlayerTeleportEventTeleportCause<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/player/PlayerTeleportEvent/TeleportCause;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCause",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::player::PlayerTeleportEventTeleportCause::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerTeleportEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::player::PlayerMoveEvent<'mc>> for PlayerTeleportEvent<'mc>{

fn into(self) -> crate::event::player::PlayerMoveEvent<'mc> {

crate::event::player::PlayerMoveEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerTeleportEvent into crate::event::player::PlayerMoveEvent")

   }
}
impl<'mc> crate::event::player::PlayerMoveEventTrait<'mc> for PlayerTeleportEvent<'mc> {}
#[repr(C)]
pub struct PlayerRiptideEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerRiptideEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerRiptideEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerRiptideEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerRiptideEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerRiptideEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerRiptideEventTrait<'mc> for PlayerRiptideEvent<'mc> {}
pub trait PlayerRiptideEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,who: impl Into<crate::entity::Player<'mc>>,item: impl Into<crate::inventory::ItemStack<'mc>>,velocity: std::option::Option<impl Into<crate::util::Vector<'mc>>>) 
-> Result<crate::event::player::PlayerRiptideEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/entity/Player;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(who.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
args.push(val_2);
if let Some(a) = velocity {
sig += "Lorg/bukkit/util/Vector;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_3);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/player/PlayerRiptideEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerRiptideEvent::from_raw(&jni,res
)}
/// Gets the item containing the used enchantment.
	fn item(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItem",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Get the velocity applied to the player as a result of this riptide.
	fn velocity(&self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/util/Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"getVelocity",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerRiptideEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerRiptideEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerRiptideEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerRiptideEvent<'mc> {}
#[repr(C)]
pub struct PlayerCommandPreprocessEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerCommandPreprocessEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerCommandPreprocessEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerCommandPreprocessEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerCommandPreprocessEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerCommandPreprocessEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerCommandPreprocessEventTrait<'mc> for PlayerCommandPreprocessEvent<'mc> {}
pub trait PlayerCommandPreprocessEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,message: impl Into<String>,recipients: std::option::Option<impl Into<blackboxmc_java::util::JavaSet<'mc>>>) 
-> Result<crate::event::player::PlayerCommandPreprocessEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/entity/Player;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
args.push(val_1);
sig += "Ljava/lang/String;";
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(message.into())?));
args.push(val_2);
if let Some(a) = recipients {
sig += "Ljava/util/Set;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_3);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/player/PlayerCommandPreprocessEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerCommandPreprocessEvent::from_raw(&jni,res
)}

	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the command that the player is attempting to send.
/// 
/// All commands begin with a special character; implementations do not
/// consider the first character when executing the content.
	fn message(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMessage",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Sets the command that the player will send.
/// 
/// All commands begin with a special character; implementations do not
/// consider the first character when executing the content.
	fn set_message(&self,command: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(command.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setMessage",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Sets the player that this command will be executed as.
	fn set_player(&self,player: impl Into<crate::entity::Player<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setPlayer",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[deprecated]
/// Gets a set of recipients that this chat message will be displayed to.The set returned is not guaranteed to be mutable and may auto-populate on access. Any listener accessing the returned set should be aware that it may reduce performance for a lazy set implementation. Listeners should be aware that modifying the list may throw {@link UnsupportedOperationException} if the event caller provides an unmodifiable set.
	fn recipients(&self) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Set;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRecipients",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerCommandPreprocessEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerCommandPreprocessEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerCommandPreprocessEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for PlayerCommandPreprocessEvent<'mc> {}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerCommandPreprocessEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerCommandPreprocessEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerCommandPreprocessEvent<'mc> {}
#[repr(C)]
pub struct PlayerItemMendEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerItemMendEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerItemMendEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerItemMendEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerItemMendEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerItemMendEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerItemMendEventTrait<'mc> for PlayerItemMendEvent<'mc> {}
pub trait PlayerItemMendEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,who: impl Into<crate::entity::Player<'mc>>,item: impl Into<crate::inventory::ItemStack<'mc>>,slot: impl Into<crate::inventory::EquipmentSlot<'mc>>,experience_orb: impl Into<crate::entity::ExperienceOrb<'mc>>,repair_amount: std::option::Option<i32>) 
-> Result<crate::event::player::PlayerItemMendEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/entity/Player;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(who.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
args.push(val_2);
sig += "Lorg/bukkit/inventory/EquipmentSlot;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(slot.into().jni_object().clone())});
args.push(val_3);
sig += "Lorg/bukkit/entity/ExperienceOrb;";
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(experience_orb.into().jni_object().clone())});
args.push(val_4);
if let Some(a) = repair_amount {
sig += "I";
let val_5 = jni::objects::JValueGen::Int(a);
args.push(val_5);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/player/PlayerItemMendEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerItemMendEvent::from_raw(&jni,res
)}
/// Get the {@link ItemStack} to be repaired.
/// This is not necessarily the item the player is holding.
	fn item(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItem",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Get the {@link EquipmentSlot} in which the repaired {@link ItemStack}
/// may be found.
	fn slot(&self) 
-> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/EquipmentSlot;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSlot",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Get the experience orb triggering the event.
	fn experience_orb(&self) 
-> Result<crate::entity::ExperienceOrb<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/ExperienceOrb;");
let res = self.jni_ref().call_method(&self.jni_object(),"getExperienceOrb",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::ExperienceOrb::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Get the amount the item is to be repaired.
/// The default value is twice the value of the consumed experience orb
/// or the remaining damage left on the item, whichever is smaller.
	fn repair_amount(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getRepairAmount",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Set the amount the item will be repaired.
/// Half of this value will be subtracted from the experience orb which initiated this event.
	fn set_repair_amount(&self,amount: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(amount);
let res = self.jni_ref().call_method(&self.jni_object(),"setRepairAmount",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_cancelled(&self,cancelled: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancelled.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerItemMendEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerItemMendEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerItemMendEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for PlayerItemMendEvent<'mc> {}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerItemMendEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerItemMendEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerItemMendEvent<'mc> {}
#[repr(C)]
pub struct PlayerChatTabCompleteEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerChatTabCompleteEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerChatTabCompleteEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerChatTabCompleteEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerChatTabCompleteEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerChatTabCompleteEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerChatTabCompleteEventTrait<'mc> for PlayerChatTabCompleteEvent<'mc> {}
pub trait PlayerChatTabCompleteEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Gets the chat message being tab-completed.
	fn chat_message(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getChatMessage",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Gets the last 'token' of the message being tab-completed.
/// 
/// The token is the substring starting with the character after the last
/// space in the message.
	fn last_token(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLastToken",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// This is the collection of completions for this event.
	fn tab_completions(&self) 
-> Result<Vec<String>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Collection;");
let res = self.jni_ref().call_method(&self.jni_object(),"getTabCompletions",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(),res.l()?)?;let iter = col.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?.to_string_lossy().to_string());
};
Ok(
new_vec
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerChatTabCompleteEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerChatTabCompleteEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerChatTabCompleteEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerChatTabCompleteEvent<'mc> {}
#[repr(C)]
pub struct PlayerLocaleChangeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerLocaleChangeEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerLocaleChangeEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerLocaleChangeEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerLocaleChangeEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerLocaleChangeEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerLocaleChangeEventTrait<'mc> for PlayerLocaleChangeEvent<'mc> {}
pub trait PlayerLocaleChangeEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,who: impl Into<crate::entity::Player<'mc>>,locale: impl Into<String>) 
-> Result<crate::event::player::PlayerLocaleChangeEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(who.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(locale.into())?));
let cls = jni.find_class("org/bukkit/event/player/PlayerLocaleChangeEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerLocaleChangeEvent::from_raw(&jni,res
)}

	fn locale(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLocale",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerLocaleChangeEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerLocaleChangeEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerLocaleChangeEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerLocaleChangeEvent<'mc> {}
#[repr(C)]
pub struct PlayerKickEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerKickEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerKickEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerKickEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerKickEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerKickEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerKickEventTrait<'mc> for PlayerKickEvent<'mc> {}
pub trait PlayerKickEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player_kicked: impl Into<crate::entity::Player<'mc>>,kick_reason: impl Into<String>,leave_message: impl Into<String>) 
-> Result<crate::event::player::PlayerKickEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Ljava/lang/String;Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player_kicked.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(kick_reason.into())?));
let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(leave_message.into())?));
let cls = jni.find_class("org/bukkit/event/player/PlayerKickEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerKickEvent::from_raw(&jni,res
)}
/// Gets the reason why the player is getting kicked
	fn reason(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getReason",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Gets the leave message send to all online players
	fn leave_message(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLeaveMessage",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Sets the reason why the player is getting kicked
	fn set_reason(&self,kick_reason: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(kick_reason.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setReason",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Sets the leave message send to all online players
	fn set_leave_message(&self,leave_message: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(leave_message.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setLeaveMessage",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerKickEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerKickEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerKickEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for PlayerKickEvent<'mc> {}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerKickEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerKickEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerKickEvent<'mc> {}
#[repr(C)]
pub struct AsyncPlayerChatPreviewEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for AsyncPlayerChatPreviewEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for AsyncPlayerChatPreviewEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate AsyncPlayerChatPreviewEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/AsyncPlayerChatPreviewEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a AsyncPlayerChatPreviewEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> AsyncPlayerChatPreviewEventTrait<'mc> for AsyncPlayerChatPreviewEvent<'mc> {}
pub trait AsyncPlayerChatPreviewEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_async: bool,who: impl Into<crate::entity::Player<'mc>>,message: impl Into<String>,players: impl Into<blackboxmc_java::util::JavaSet<'mc>>) 
-> Result<crate::event::player::AsyncPlayerChatPreviewEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(ZLorg/bukkit/entity/Player;Ljava/lang/String;Ljava/util/Set;)V");
let val_1 = jni::objects::JValueGen::Bool(val_async.into());
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(who.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(message.into())?));
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(players.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/player/AsyncPlayerChatPreviewEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::AsyncPlayerChatPreviewEvent::from_raw(&jni,res
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/AsyncPlayerChatPreviewEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::player::AsyncPlayerChatEvent<'mc>> for AsyncPlayerChatPreviewEvent<'mc>{

fn into(self) -> crate::event::player::AsyncPlayerChatEvent<'mc> {

crate::event::player::AsyncPlayerChatEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting AsyncPlayerChatPreviewEvent into crate::event::player::AsyncPlayerChatEvent")

   }
}
impl<'mc> crate::event::player::AsyncPlayerChatEventTrait<'mc> for AsyncPlayerChatPreviewEvent<'mc> {}
pub enum AsyncPlayerPreLoginEventResult<'mc> {
	Allowed {inner: AsyncPlayerPreLoginEventResultStruct<'mc>},
	KickFull {inner: AsyncPlayerPreLoginEventResultStruct<'mc>},
	KickBanned {inner: AsyncPlayerPreLoginEventResultStruct<'mc>},
	KickWhitelist {inner: AsyncPlayerPreLoginEventResultStruct<'mc>},
	KickOther {inner: AsyncPlayerPreLoginEventResultStruct<'mc>},
}
impl<'mc> std::fmt::Display for AsyncPlayerPreLoginEventResult<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           AsyncPlayerPreLoginEventResult::Allowed { .. } => f.write_str("ALLOWED"),
           AsyncPlayerPreLoginEventResult::KickFull { .. } => f.write_str("KICK_FULL"),
           AsyncPlayerPreLoginEventResult::KickBanned { .. } => f.write_str("KICK_BANNED"),
           AsyncPlayerPreLoginEventResult::KickWhitelist { .. } => f.write_str("KICK_WHITELIST"),
           AsyncPlayerPreLoginEventResult::KickOther { .. } => f.write_str("KICK_OTHER"),
       }
   }
}

        impl<'mc> AsyncPlayerPreLoginEventResultTrait<'mc> for AsyncPlayerPreLoginEventResult<'mc> {}
        
        pub trait AsyncPlayerPreLoginEventResultTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<AsyncPlayerPreLoginEventResult<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/event/player/AsyncPlayerPreLoginEvent/Result");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/event/player/AsyncPlayerPreLoginEvent/Result;",
                    vec![jni::objects::JValueGen::from(val_1)],
                );
                let res = env.translate_error(res)?;
                let obj = res.l()?;
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    
"ALLOWED" => Ok(AsyncPlayerPreLoginEventResult::Allowed { inner: AsyncPlayerPreLoginEventResultStruct::from_raw(env,obj)?}),
"KICK_FULL" => Ok(AsyncPlayerPreLoginEventResult::KickFull { inner: AsyncPlayerPreLoginEventResultStruct::from_raw(env,obj)?}),
"KICK_BANNED" => Ok(AsyncPlayerPreLoginEventResult::KickBanned { inner: AsyncPlayerPreLoginEventResultStruct::from_raw(env,obj)?}),
"KICK_WHITELIST" => Ok(AsyncPlayerPreLoginEventResult::KickWhitelist { inner: AsyncPlayerPreLoginEventResultStruct::from_raw(env,obj)?}),
"KICK_OTHER" => Ok(AsyncPlayerPreLoginEventResult::KickOther { inner: AsyncPlayerPreLoginEventResultStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct AsyncPlayerPreLoginEventResultStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for AsyncPlayerPreLoginEventResult<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::Allowed { inner } => inner.0.clone(),
Self::KickFull { inner } => inner.0.clone(),
Self::KickBanned { inner } => inner.0.clone(),
Self::KickWhitelist { inner } => inner.0.clone(),
Self::KickOther { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::Allowed { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::KickFull { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::KickBanned { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::KickWhitelist { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::KickOther { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for AsyncPlayerPreLoginEventResult<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate AsyncPlayerPreLoginEventResult from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/AsyncPlayerPreLoginEvent/Result")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a AsyncPlayerPreLoginEventResult object, got {}",
                    name
                )
                .into())
            } else {
    
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    "ALLOWED" => Ok(AsyncPlayerPreLoginEventResult::Allowed { inner: AsyncPlayerPreLoginEventResultStruct::from_raw(env,obj)?}),"KICK_FULL" => Ok(AsyncPlayerPreLoginEventResult::KickFull { inner: AsyncPlayerPreLoginEventResultStruct::from_raw(env,obj)?}),"KICK_BANNED" => Ok(AsyncPlayerPreLoginEventResult::KickBanned { inner: AsyncPlayerPreLoginEventResultStruct::from_raw(env,obj)?}),"KICK_WHITELIST" => Ok(AsyncPlayerPreLoginEventResult::KickWhitelist { inner: AsyncPlayerPreLoginEventResultStruct::from_raw(env,obj)?}),"KICK_OTHER" => Ok(AsyncPlayerPreLoginEventResult::KickOther { inner: AsyncPlayerPreLoginEventResultStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for AsyncPlayerPreLoginEventResultStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for AsyncPlayerPreLoginEventResultStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate AsyncPlayerPreLoginEventResultStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/AsyncPlayerPreLoginEvent/Result")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a AsyncPlayerPreLoginEventResultStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> AsyncPlayerPreLoginEventResultStruct<'mc> {

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::player::AsyncPlayerPreLoginEventResult<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/player/AsyncPlayerPreLoginEvent/Result;");
let cls = jni.find_class("org/bukkit/event/player/AsyncPlayerPreLoginEvent/Result"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::player::AsyncPlayerPreLoginEventResult::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct PlayerInteractAtEntityEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerInteractAtEntityEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerInteractAtEntityEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerInteractAtEntityEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerInteractAtEntityEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerInteractAtEntityEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerInteractAtEntityEventTrait<'mc> for PlayerInteractAtEntityEvent<'mc> {}
pub trait PlayerInteractAtEntityEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,who: impl Into<crate::entity::Player<'mc>>,clicked_entity: impl Into<crate::entity::Entity<'mc>>,position: impl Into<crate::util::Vector<'mc>>,hand: std::option::Option<impl Into<crate::inventory::EquipmentSlot<'mc>>>) 
-> Result<crate::event::player::PlayerInteractAtEntityEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/entity/Player;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(who.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/entity/Entity;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(clicked_entity.into().jni_object().clone())});
args.push(val_2);
sig += "Lorg/bukkit/util/Vector;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(position.into().jni_object().clone())});
args.push(val_3);
if let Some(a) = hand {
sig += "Lorg/bukkit/inventory/EquipmentSlot;";
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_4);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/player/PlayerInteractAtEntityEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerInteractAtEntityEvent::from_raw(&jni,res
)}

	fn clicked_position(&self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/util/Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"getClickedPosition",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerInteractAtEntityEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::player::PlayerInteractEntityEvent<'mc>> for PlayerInteractAtEntityEvent<'mc>{

fn into(self) -> crate::event::player::PlayerInteractEntityEvent<'mc> {

crate::event::player::PlayerInteractEntityEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerInteractAtEntityEvent into crate::event::player::PlayerInteractEntityEvent")

   }
}
impl<'mc> crate::event::player::PlayerInteractEntityEventTrait<'mc> for PlayerInteractAtEntityEvent<'mc> {}
#[repr(C)]
pub struct PlayerChannelEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerChannelEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerChannelEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerChannelEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerChannelEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerChannelEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerChannelEventTrait<'mc> for PlayerChannelEvent<'mc> {}
pub trait PlayerChannelEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,channel: impl Into<String>) 
-> Result<crate::event::player::PlayerChannelEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(channel.into())?));
let cls = jni.find_class("org/bukkit/event/player/PlayerChannelEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerChannelEvent::from_raw(&jni,res
)}

	fn channel(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getChannel",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerChannelEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerChannelEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerChannelEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerChannelEvent<'mc> {}
#[repr(C)]
pub struct PlayerLevelChangeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerLevelChangeEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerLevelChangeEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerLevelChangeEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerLevelChangeEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerLevelChangeEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerLevelChangeEventTrait<'mc> for PlayerLevelChangeEvent<'mc> {}
pub trait PlayerLevelChangeEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,old_level: i32,new_level: i32) 
-> Result<crate::event::player::PlayerLevelChangeEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;II)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Int(old_level);
let val_3 = jni::objects::JValueGen::Int(new_level);
let cls = jni.find_class("org/bukkit/event/player/PlayerLevelChangeEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerLevelChangeEvent::from_raw(&jni,res
)}
/// Gets the old level of the player
	fn old_level(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getOldLevel",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Gets the new level of the player
	fn new_level(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getNewLevel",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerLevelChangeEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerLevelChangeEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerLevelChangeEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerLevelChangeEvent<'mc> {}
#[repr(C)]
pub struct PlayerChatEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerChatEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerChatEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerChatEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerChatEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerChatEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerChatEventTrait<'mc> for PlayerChatEvent<'mc> {}
pub trait PlayerChatEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,message: impl Into<String>,format: std::option::Option<impl Into<String>>,recipients: std::option::Option<impl Into<blackboxmc_java::util::JavaSet<'mc>>>) 
-> Result<crate::event::player::PlayerChatEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/entity/Player;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
args.push(val_1);
sig += "Ljava/lang/String;";
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(message.into())?));
args.push(val_2);
if let Some(a) = format {
sig += "Ljava/lang/String;";
let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(a.into())?));
args.push(val_3);
}
if let Some(a) = recipients {
sig += "Ljava/util/Set;";
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_4);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/player/PlayerChatEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerChatEvent::from_raw(&jni,res
)}

	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the message that the player is attempting to send
	fn message(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMessage",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Sets the message that the player will send
	fn set_message(&self,message: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(message.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setMessage",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Sets the player that this message will display as, or command will be
/// executed as
	fn set_player(&self,player: impl Into<crate::entity::Player<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setPlayer",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the format to use to display this chat message
	fn format(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFormat",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Sets the format to use to display this chat message
	fn set_format(&self,format: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(format.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setFormat",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets a set of recipients that this chat message will be displayed to
	fn recipients(&self) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Set;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRecipients",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerChatEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerChatEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerChatEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for PlayerChatEvent<'mc> {}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerChatEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerChatEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerChatEvent<'mc> {}
#[repr(C)]
pub struct PlayerStatisticIncrementEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerStatisticIncrementEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerStatisticIncrementEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerStatisticIncrementEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerStatisticIncrementEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerStatisticIncrementEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerStatisticIncrementEventTrait<'mc> for PlayerStatisticIncrementEvent<'mc> {}
pub trait PlayerStatisticIncrementEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,statistic: impl Into<crate::Statistic<'mc>>,initial_value: i32,new_value: i32,material: std::option::Option<impl Into<crate::Material<'mc>>>) 
-> Result<crate::event::player::PlayerStatisticIncrementEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/entity/Player;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/Statistic;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(statistic.into().jni_object().clone())});
args.push(val_2);
sig += "I";
let val_3 = jni::objects::JValueGen::Int(initial_value);
args.push(val_3);
sig += "I";
let val_4 = jni::objects::JValueGen::Int(new_value);
args.push(val_4);
if let Some(a) = material {
sig += "Lorg/bukkit/Material;";
let val_5 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_5);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/player/PlayerStatisticIncrementEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerStatisticIncrementEvent::from_raw(&jni,res
)}
/// Gets the statistic that is being incremented.
	fn statistic(&self) 
-> Result<crate::Statistic<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Statistic;");
let res = self.jni_ref().call_method(&self.jni_object(),"getStatistic",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Statistic::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the previous value of the statistic.
	fn previous_value(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getPreviousValue",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Gets the new value of the statistic.
	fn new_value(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getNewValue",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Gets the EntityType if {@link #getStatistic() getStatistic()} is an
/// entity statistic otherwise returns null.
	fn entity_type(&self) 
-> Result<Option<crate::entity::EntityType<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/EntityType;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEntityType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::entity::EntityType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Gets the Material if {@link #getStatistic() getStatistic()} is a block
/// or item statistic otherwise returns null.
	fn material(&self) 
-> Result<Option<crate::Material<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Material;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMaterial",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::Material::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerStatisticIncrementEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerStatisticIncrementEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerStatisticIncrementEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for PlayerStatisticIncrementEvent<'mc> {}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerStatisticIncrementEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerStatisticIncrementEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerStatisticIncrementEvent<'mc> {}
pub enum PlayerRecipeBookSettingsChangeEventRecipeBookType<'mc> {
	Crafting {inner: PlayerRecipeBookSettingsChangeEventRecipeBookTypeStruct<'mc>},
	Furnace {inner: PlayerRecipeBookSettingsChangeEventRecipeBookTypeStruct<'mc>},
	BlastFurnace {inner: PlayerRecipeBookSettingsChangeEventRecipeBookTypeStruct<'mc>},
	Smoker {inner: PlayerRecipeBookSettingsChangeEventRecipeBookTypeStruct<'mc>},
}
impl<'mc> std::fmt::Display for PlayerRecipeBookSettingsChangeEventRecipeBookType<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           PlayerRecipeBookSettingsChangeEventRecipeBookType::Crafting { .. } => f.write_str("CRAFTING"),
           PlayerRecipeBookSettingsChangeEventRecipeBookType::Furnace { .. } => f.write_str("FURNACE"),
           PlayerRecipeBookSettingsChangeEventRecipeBookType::BlastFurnace { .. } => f.write_str("BLAST_FURNACE"),
           PlayerRecipeBookSettingsChangeEventRecipeBookType::Smoker { .. } => f.write_str("SMOKER"),
       }
   }
}

        impl<'mc> PlayerRecipeBookSettingsChangeEventRecipeBookTypeTrait<'mc> for PlayerRecipeBookSettingsChangeEventRecipeBookType<'mc> {}
        
        pub trait PlayerRecipeBookSettingsChangeEventRecipeBookTypeTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<PlayerRecipeBookSettingsChangeEventRecipeBookType<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/event/player/PlayerRecipeBookSettingsChangeEvent/RecipeBookType");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/event/player/PlayerRecipeBookSettingsChangeEvent/RecipeBookType;",
                    vec![jni::objects::JValueGen::from(val_1)],
                );
                let res = env.translate_error(res)?;
                let obj = res.l()?;
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    
"CRAFTING" => Ok(PlayerRecipeBookSettingsChangeEventRecipeBookType::Crafting { inner: PlayerRecipeBookSettingsChangeEventRecipeBookTypeStruct::from_raw(env,obj)?}),
"FURNACE" => Ok(PlayerRecipeBookSettingsChangeEventRecipeBookType::Furnace { inner: PlayerRecipeBookSettingsChangeEventRecipeBookTypeStruct::from_raw(env,obj)?}),
"BLAST_FURNACE" => Ok(PlayerRecipeBookSettingsChangeEventRecipeBookType::BlastFurnace { inner: PlayerRecipeBookSettingsChangeEventRecipeBookTypeStruct::from_raw(env,obj)?}),
"SMOKER" => Ok(PlayerRecipeBookSettingsChangeEventRecipeBookType::Smoker { inner: PlayerRecipeBookSettingsChangeEventRecipeBookTypeStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct PlayerRecipeBookSettingsChangeEventRecipeBookTypeStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerRecipeBookSettingsChangeEventRecipeBookType<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::Crafting { inner } => inner.0.clone(),
Self::Furnace { inner } => inner.0.clone(),
Self::BlastFurnace { inner } => inner.0.clone(),
Self::Smoker { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::Crafting { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Furnace { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::BlastFurnace { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Smoker { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerRecipeBookSettingsChangeEventRecipeBookType<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerRecipeBookSettingsChangeEventRecipeBookType from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerRecipeBookSettingsChangeEvent/RecipeBookType")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerRecipeBookSettingsChangeEventRecipeBookType object, got {}",
                    name
                )
                .into())
            } else {
    
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    "CRAFTING" => Ok(PlayerRecipeBookSettingsChangeEventRecipeBookType::Crafting { inner: PlayerRecipeBookSettingsChangeEventRecipeBookTypeStruct::from_raw(env,obj)?}),"FURNACE" => Ok(PlayerRecipeBookSettingsChangeEventRecipeBookType::Furnace { inner: PlayerRecipeBookSettingsChangeEventRecipeBookTypeStruct::from_raw(env,obj)?}),"BLAST_FURNACE" => Ok(PlayerRecipeBookSettingsChangeEventRecipeBookType::BlastFurnace { inner: PlayerRecipeBookSettingsChangeEventRecipeBookTypeStruct::from_raw(env,obj)?}),"SMOKER" => Ok(PlayerRecipeBookSettingsChangeEventRecipeBookType::Smoker { inner: PlayerRecipeBookSettingsChangeEventRecipeBookTypeStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for PlayerRecipeBookSettingsChangeEventRecipeBookTypeStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerRecipeBookSettingsChangeEventRecipeBookTypeStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerRecipeBookSettingsChangeEventRecipeBookTypeStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerRecipeBookSettingsChangeEvent/RecipeBookType")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerRecipeBookSettingsChangeEventRecipeBookTypeStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerRecipeBookSettingsChangeEventRecipeBookTypeStruct<'mc> {

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::player::PlayerRecipeBookSettingsChangeEventRecipeBookType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/player/PlayerRecipeBookSettingsChangeEvent/RecipeBookType;");
let cls = jni.find_class("org/bukkit/event/player/PlayerRecipeBookSettingsChangeEvent/RecipeBookType"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::player::PlayerRecipeBookSettingsChangeEventRecipeBookType::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct PlayerExpChangeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerExpChangeEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerExpChangeEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerExpChangeEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerExpChangeEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerExpChangeEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerExpChangeEventTrait<'mc> for PlayerExpChangeEvent<'mc> {}
pub trait PlayerExpChangeEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,exp_amount: i32) 
-> Result<crate::event::player::PlayerExpChangeEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;I)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Int(exp_amount);
let cls = jni.find_class("org/bukkit/event/player/PlayerExpChangeEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerExpChangeEvent::from_raw(&jni,res
)}
/// Get the amount of experience the player will receive
	fn amount(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getAmount",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Set the amount of experience the player will receive
	fn set_amount(&self,amount: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(amount);
let res = self.jni_ref().call_method(&self.jni_object(),"setAmount",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerExpChangeEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerExpChangeEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerExpChangeEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerExpChangeEvent<'mc> {}
pub enum PlayerRespawnEventRespawnReason<'mc> {
	Death {inner: PlayerRespawnEventRespawnReasonStruct<'mc>},
	EndPortal {inner: PlayerRespawnEventRespawnReasonStruct<'mc>},
	Plugin {inner: PlayerRespawnEventRespawnReasonStruct<'mc>},
}
impl<'mc> std::fmt::Display for PlayerRespawnEventRespawnReason<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           PlayerRespawnEventRespawnReason::Death { .. } => f.write_str("DEATH"),
           PlayerRespawnEventRespawnReason::EndPortal { .. } => f.write_str("END_PORTAL"),
           PlayerRespawnEventRespawnReason::Plugin { .. } => f.write_str("PLUGIN"),
       }
   }
}

        impl<'mc> PlayerRespawnEventRespawnReasonTrait<'mc> for PlayerRespawnEventRespawnReason<'mc> {}
        
        pub trait PlayerRespawnEventRespawnReasonTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<PlayerRespawnEventRespawnReason<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/event/player/PlayerRespawnEvent/RespawnReason");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/event/player/PlayerRespawnEvent/RespawnReason;",
                    vec![jni::objects::JValueGen::from(val_1)],
                );
                let res = env.translate_error(res)?;
                let obj = res.l()?;
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    
"DEATH" => Ok(PlayerRespawnEventRespawnReason::Death { inner: PlayerRespawnEventRespawnReasonStruct::from_raw(env,obj)?}),
"END_PORTAL" => Ok(PlayerRespawnEventRespawnReason::EndPortal { inner: PlayerRespawnEventRespawnReasonStruct::from_raw(env,obj)?}),
"PLUGIN" => Ok(PlayerRespawnEventRespawnReason::Plugin { inner: PlayerRespawnEventRespawnReasonStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct PlayerRespawnEventRespawnReasonStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerRespawnEventRespawnReason<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::Death { inner } => inner.0.clone(),
Self::EndPortal { inner } => inner.0.clone(),
Self::Plugin { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::Death { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::EndPortal { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Plugin { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerRespawnEventRespawnReason<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerRespawnEventRespawnReason from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerRespawnEvent/RespawnReason")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerRespawnEventRespawnReason object, got {}",
                    name
                )
                .into())
            } else {
    
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    "DEATH" => Ok(PlayerRespawnEventRespawnReason::Death { inner: PlayerRespawnEventRespawnReasonStruct::from_raw(env,obj)?}),"END_PORTAL" => Ok(PlayerRespawnEventRespawnReason::EndPortal { inner: PlayerRespawnEventRespawnReasonStruct::from_raw(env,obj)?}),"PLUGIN" => Ok(PlayerRespawnEventRespawnReason::Plugin { inner: PlayerRespawnEventRespawnReasonStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for PlayerRespawnEventRespawnReasonStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerRespawnEventRespawnReasonStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerRespawnEventRespawnReasonStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerRespawnEvent/RespawnReason")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerRespawnEventRespawnReasonStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerRespawnEventRespawnReasonStruct<'mc> {

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::player::PlayerRespawnEventRespawnReason<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/player/PlayerRespawnEvent/RespawnReason;");
let cls = jni.find_class("org/bukkit/event/player/PlayerRespawnEvent/RespawnReason"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::player::PlayerRespawnEventRespawnReason::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct PlayerToggleFlightEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerToggleFlightEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerToggleFlightEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerToggleFlightEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerToggleFlightEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerToggleFlightEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerToggleFlightEventTrait<'mc> for PlayerToggleFlightEvent<'mc> {}
pub trait PlayerToggleFlightEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,is_flying: bool) 
-> Result<crate::event::player::PlayerToggleFlightEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Z)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Bool(is_flying.into());
let cls = jni.find_class("org/bukkit/event/player/PlayerToggleFlightEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerToggleFlightEvent::from_raw(&jni,res
)}
/// Returns whether the player is trying to start or stop flying.
	fn is_flying(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isFlying",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerToggleFlightEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerToggleFlightEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerToggleFlightEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for PlayerToggleFlightEvent<'mc> {}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerToggleFlightEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerToggleFlightEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerToggleFlightEvent<'mc> {}
#[repr(C)]
pub struct PlayerUnleashEntityEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerUnleashEntityEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerUnleashEntityEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerUnleashEntityEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerUnleashEntityEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerUnleashEntityEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerUnleashEntityEventTrait<'mc> for PlayerUnleashEntityEvent<'mc> {}
pub trait PlayerUnleashEntityEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,entity: impl Into<crate::entity::Entity<'mc>>,player: impl Into<crate::entity::Player<'mc>>,hand: std::option::Option<impl Into<crate::inventory::EquipmentSlot<'mc>>>) 
-> Result<crate::event::player::PlayerUnleashEntityEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/entity/Entity;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(entity.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/entity/Player;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
args.push(val_2);
if let Some(a) = hand {
sig += "Lorg/bukkit/inventory/EquipmentSlot;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_3);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/player/PlayerUnleashEntityEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerUnleashEntityEvent::from_raw(&jni,res
)}
/// Returns the player who is unleashing the entity.
	fn player(&self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Player;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Get the hand used by the player to unleash the entity.
	fn hand(&self) 
-> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/EquipmentSlot;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHand",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerUnleashEntityEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerUnleashEntityEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for PlayerUnleashEntityEvent<'mc> {}
impl<'mc> Into<crate::event::entity::EntityUnleashEvent<'mc>> for PlayerUnleashEntityEvent<'mc>{

fn into(self) -> crate::event::entity::EntityUnleashEvent<'mc> {

crate::event::entity::EntityUnleashEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerUnleashEntityEvent into crate::event::entity::EntityUnleashEvent")

   }
}
impl<'mc> crate::event::entity::EntityUnleashEventTrait<'mc> for PlayerUnleashEntityEvent<'mc> {}
#[repr(C)]
pub struct PlayerChangedMainHandEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerChangedMainHandEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerChangedMainHandEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerChangedMainHandEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerChangedMainHandEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerChangedMainHandEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerChangedMainHandEventTrait<'mc> for PlayerChangedMainHandEvent<'mc> {}
pub trait PlayerChangedMainHandEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,who: impl Into<crate::entity::Player<'mc>>,main_hand: impl Into<crate::inventory::MainHand<'mc>>) 
-> Result<crate::event::player::PlayerChangedMainHandEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Lorg/bukkit/inventory/MainHand;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(who.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(main_hand.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/player/PlayerChangedMainHandEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerChangedMainHandEvent::from_raw(&jni,res
)}
/// Gets the new main hand of the player. The old hand is still momentarily
/// available via {@link Player#getMainHand()}.
	fn main_hand(&self) 
-> Result<crate::inventory::MainHand<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/MainHand;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMainHand",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::MainHand::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerChangedMainHandEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerChangedMainHandEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerChangedMainHandEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerChangedMainHandEvent<'mc> {}
#[repr(C)]
pub struct PlayerAnimationEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerAnimationEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerAnimationEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerAnimationEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerAnimationEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerAnimationEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerAnimationEventTrait<'mc> for PlayerAnimationEvent<'mc> {}
pub trait PlayerAnimationEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Construct a new PlayerAnimation event
	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,player_animation_type: std::option::Option<impl Into<crate::event::player::PlayerAnimationType<'mc>>>) 
-> Result<crate::event::player::PlayerAnimationEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/entity/Player;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = player_animation_type {
sig += "Lorg/bukkit/event/player/PlayerAnimationType;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/player/PlayerAnimationEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerAnimationEvent::from_raw(&jni,res
)}
/// Get the type of this animation event
	fn animation_type(&self) 
-> Result<crate::event::player::PlayerAnimationType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/player/PlayerAnimationType;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAnimationType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::player::PlayerAnimationType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerAnimationEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerAnimationEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerAnimationEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for PlayerAnimationEvent<'mc> {}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerAnimationEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerAnimationEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerAnimationEvent<'mc> {}
#[repr(C)]
pub struct PlayerShearEntityEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerShearEntityEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerShearEntityEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerShearEntityEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerShearEntityEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerShearEntityEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerShearEntityEventTrait<'mc> for PlayerShearEntityEvent<'mc> {}
pub trait PlayerShearEntityEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,who: impl Into<crate::entity::Player<'mc>>,what: impl Into<crate::entity::Entity<'mc>>,item: std::option::Option<impl Into<crate::inventory::ItemStack<'mc>>>,hand: std::option::Option<impl Into<crate::inventory::EquipmentSlot<'mc>>>) 
-> Result<crate::event::player::PlayerShearEntityEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/entity/Player;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(who.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/entity/Entity;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(what.into().jni_object().clone())});
args.push(val_2);
if let Some(a) = item {
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_3);
}
if let Some(a) = hand {
sig += "Lorg/bukkit/inventory/EquipmentSlot;";
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_4);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/player/PlayerShearEntityEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerShearEntityEvent::from_raw(&jni,res
)}

	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the entity the player is shearing
	fn entity(&self) 
-> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Entity;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEntity",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Entity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the item used to shear the entity.
	fn item(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItem",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the hand used to shear the entity.
	fn hand(&self) 
-> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/EquipmentSlot;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHand",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerShearEntityEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerShearEntityEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerShearEntityEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for PlayerShearEntityEvent<'mc> {}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerShearEntityEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerShearEntityEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerShearEntityEvent<'mc> {}
pub enum PlayerSignOpenEventCause<'mc> {
	Interact {inner: PlayerSignOpenEventCauseStruct<'mc>},
	Place {inner: PlayerSignOpenEventCauseStruct<'mc>},
	Plugin {inner: PlayerSignOpenEventCauseStruct<'mc>},
	Unknown {inner: PlayerSignOpenEventCauseStruct<'mc>},
}
impl<'mc> std::fmt::Display for PlayerSignOpenEventCause<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           PlayerSignOpenEventCause::Interact { .. } => f.write_str("INTERACT"),
           PlayerSignOpenEventCause::Place { .. } => f.write_str("PLACE"),
           PlayerSignOpenEventCause::Plugin { .. } => f.write_str("PLUGIN"),
           PlayerSignOpenEventCause::Unknown { .. } => f.write_str("UNKNOWN"),
       }
   }
}

        impl<'mc> PlayerSignOpenEventCauseTrait<'mc> for PlayerSignOpenEventCause<'mc> {}
        
        pub trait PlayerSignOpenEventCauseTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<PlayerSignOpenEventCause<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/event/player/PlayerSignOpenEvent/Cause");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/event/player/PlayerSignOpenEvent/Cause;",
                    vec![jni::objects::JValueGen::from(val_1)],
                );
                let res = env.translate_error(res)?;
                let obj = res.l()?;
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    
"INTERACT" => Ok(PlayerSignOpenEventCause::Interact { inner: PlayerSignOpenEventCauseStruct::from_raw(env,obj)?}),
"PLACE" => Ok(PlayerSignOpenEventCause::Place { inner: PlayerSignOpenEventCauseStruct::from_raw(env,obj)?}),
"PLUGIN" => Ok(PlayerSignOpenEventCause::Plugin { inner: PlayerSignOpenEventCauseStruct::from_raw(env,obj)?}),
"UNKNOWN" => Ok(PlayerSignOpenEventCause::Unknown { inner: PlayerSignOpenEventCauseStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct PlayerSignOpenEventCauseStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerSignOpenEventCause<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::Interact { inner } => inner.0.clone(),
Self::Place { inner } => inner.0.clone(),
Self::Plugin { inner } => inner.0.clone(),
Self::Unknown { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::Interact { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Place { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Plugin { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Unknown { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerSignOpenEventCause<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerSignOpenEventCause from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerSignOpenEvent/Cause")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerSignOpenEventCause object, got {}",
                    name
                )
                .into())
            } else {
    
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    "INTERACT" => Ok(PlayerSignOpenEventCause::Interact { inner: PlayerSignOpenEventCauseStruct::from_raw(env,obj)?}),"PLACE" => Ok(PlayerSignOpenEventCause::Place { inner: PlayerSignOpenEventCauseStruct::from_raw(env,obj)?}),"PLUGIN" => Ok(PlayerSignOpenEventCause::Plugin { inner: PlayerSignOpenEventCauseStruct::from_raw(env,obj)?}),"UNKNOWN" => Ok(PlayerSignOpenEventCause::Unknown { inner: PlayerSignOpenEventCauseStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for PlayerSignOpenEventCauseStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerSignOpenEventCauseStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerSignOpenEventCauseStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerSignOpenEvent/Cause")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerSignOpenEventCauseStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerSignOpenEventCauseStruct<'mc> {

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::player::PlayerSignOpenEventCause<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/player/PlayerSignOpenEvent/Cause;");
let cls = jni.find_class("org/bukkit/event/player/PlayerSignOpenEvent/Cause"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::player::PlayerSignOpenEventCause::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct PlayerPortalEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerPortalEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerPortalEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerPortalEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerPortalEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerPortalEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerPortalEventTrait<'mc> for PlayerPortalEvent<'mc> {}
pub trait PlayerPortalEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,from: impl Into<crate::Location<'mc>>,to: impl Into<crate::Location<'mc>>,cause: std::option::Option<impl Into<crate::event::player::PlayerTeleportEventTeleportCause<'mc>>>,get_search_radius: std::option::Option<i32>,can_create_portal: std::option::Option<bool>,creation_radius: std::option::Option<i32>) 
-> Result<crate::event::player::PlayerPortalEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/entity/Player;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/Location;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(from.into().jni_object().clone())});
args.push(val_2);
sig += "Lorg/bukkit/Location;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(to.into().jni_object().clone())});
args.push(val_3);
if let Some(a) = cause {
sig += "Lorg/bukkit/event/player/PlayerTeleportEvent/TeleportCause;";
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_4);
}
if let Some(a) = get_search_radius {
sig += "I";
let val_5 = jni::objects::JValueGen::Int(a);
args.push(val_5);
}
if let Some(a) = can_create_portal {
sig += "Z";
let val_6 = jni::objects::JValueGen::Bool(a.into());
args.push(val_6);
}
if let Some(a) = creation_radius {
sig += "I";
let val_7 = jni::objects::JValueGen::Int(a);
args.push(val_7);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/player/PlayerPortalEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerPortalEvent::from_raw(&jni,res
)}
/// Set the Block radius to search in for available portals.
	fn set_search_radius(&self,search_radius: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(search_radius);
let res = self.jni_ref().call_method(&self.jni_object(),"setSearchRadius",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the search radius value for finding an available portal.
	fn search_radius(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getSearchRadius",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Returns whether the server will attempt to create a destination portal or
/// not.
	fn can_create_portal(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"getCanCreatePortal",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Sets whether the server should attempt to create a destination portal or
/// not.
	fn set_can_create_portal(&self,can_create_portal: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(can_create_portal.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCanCreatePortal",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Sets the maximum radius the world is searched for a free space from the
/// given location.
/// If enough free space is found then the portal will be created there, if
/// not it will force create with air-space at the target location.
/// Does not apply to end portal target platforms which will always appear at
/// the target location.
	fn set_creation_radius(&self,creation_radius: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(creation_radius);
let res = self.jni_ref().call_method(&self.jni_object(),"setCreationRadius",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the maximum radius the world is searched for a free space from the
/// given location.
/// If enough free space is found then the portal will be created there, if
/// not it will force create with air-space at the target location.
/// Does not apply to end portal target platforms which will always appear at
/// the target location.
	fn creation_radius(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getCreationRadius",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerPortalEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::player::PlayerTeleportEvent<'mc>> for PlayerPortalEvent<'mc>{

fn into(self) -> crate::event::player::PlayerTeleportEvent<'mc> {

crate::event::player::PlayerTeleportEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerPortalEvent into crate::event::player::PlayerTeleportEvent")

   }
}
impl<'mc> crate::event::player::PlayerTeleportEventTrait<'mc> for PlayerPortalEvent<'mc> {}
#[repr(C)]
pub struct PlayerFishEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerFishEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerFishEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerFishEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerFishEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerFishEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerFishEventTrait<'mc> for PlayerFishEvent<'mc> {}
pub trait PlayerFishEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,entity: impl Into<crate::entity::Entity<'mc>>,hook_entity: impl Into<crate::entity::FishHook<'mc>>,hand: impl Into<crate::inventory::EquipmentSlot<'mc>>,state: std::option::Option<impl Into<crate::event::player::PlayerFishEventState<'mc>>>) 
-> Result<crate::event::player::PlayerFishEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/entity/Player;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/entity/Entity;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(entity.into().jni_object().clone())});
args.push(val_2);
sig += "Lorg/bukkit/entity/FishHook;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(hook_entity.into().jni_object().clone())});
args.push(val_3);
sig += "Lorg/bukkit/inventory/EquipmentSlot;";
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(hand.into().jni_object().clone())});
args.push(val_4);
if let Some(a) = state {
sig += "Lorg/bukkit/event/player/PlayerFishEvent/State;";
let val_5 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_5);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/player/PlayerFishEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerFishEvent::from_raw(&jni,res
)}
/// Gets the entity caught by the player.
/// 
/// If player has fished successfully, the result may be cast to {@link
/// org.bukkit.entity.Item}.
	fn caught(&self) 
-> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Entity;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCaught",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::entity::Entity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Gets the fishing hook.
	fn hook(&self) 
-> Result<crate::entity::FishHook<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/FishHook;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHook",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::FishHook::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the amount of experience received when fishing.
/// 
/// Note: This value has no default effect unless the event state is {@link
/// State#CAUGHT_FISH}.
	fn exp_to_drop(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getExpToDrop",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Sets the amount of experience received when fishing.
/// 
/// Note: This value has no default effect unless the event state is {@link
/// State#CAUGHT_FISH}.
	fn set_exp_to_drop(&self,amount: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(amount);
let res = self.jni_ref().call_method(&self.jni_object(),"setExpToDrop",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the hand that was used in this event.
/// 
/// The hand used is only present when the event state is {@link State#FISHING}.
/// In all other states, the hand is null.
	fn hand(&self) 
-> Result<Option<crate::inventory::EquipmentSlot<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/EquipmentSlot;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHand",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Gets the state of the fishing
	fn state(&self) 
-> Result<crate::event::player::PlayerFishEventState<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/player/PlayerFishEvent/State;");
let res = self.jni_ref().call_method(&self.jni_object(),"getState",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::player::PlayerFishEventState::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerFishEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerFishEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerFishEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for PlayerFishEvent<'mc> {}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerFishEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerFishEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerFishEvent<'mc> {}
#[repr(C)]
pub struct PlayerEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerEventTrait<'mc> for PlayerEvent<'mc> {}
pub trait PlayerEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,who: impl Into<crate::entity::Player<'mc>>) 
-> Result<crate::event::player::PlayerEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(who.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/player/PlayerEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerEvent::from_raw(&jni,res
)}
/// Returns the player involved in this event
	fn player(&self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Player;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Event<'mc>> for PlayerEvent<'mc>{

fn into(self) -> crate::event::Event<'mc> {

crate::event::Event::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerEvent into crate::event::Event")

   }
}
impl<'mc> crate::event::EventTrait<'mc> for PlayerEvent<'mc> {}
#[repr(C)]
pub struct PlayerPreLoginEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerPreLoginEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerPreLoginEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerPreLoginEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerPreLoginEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerPreLoginEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerPreLoginEventTrait<'mc> for PlayerPreLoginEvent<'mc> {}
pub trait PlayerPreLoginEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,name: impl Into<String>,ip_address: jni::objects::JObject<'mc>,unique_id: std::option::Option<impl Into<blackboxmc_java::util::JavaUUID<'mc>>>) 
-> Result<crate::event::player::PlayerPreLoginEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(name.into())?));
args.push(val_1);
sig += "Ljava/net/InetAddress;";
let val_2 = jni::objects::JValueGen::Object(ip_address);
args.push(val_2);
if let Some(a) = unique_id {
sig += "Ljava/util/UUID;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_3);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/player/PlayerPreLoginEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerPreLoginEvent::from_raw(&jni,res
)}
/// Gets the current result of the login, as an enum
	fn result(&self) 
-> Result<crate::event::player::PlayerPreLoginEventResult<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/player/PlayerPreLoginEvent/Result;");
let res = self.jni_ref().call_method(&self.jni_object(),"getResult",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::player::PlayerPreLoginEventResult::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the new result of the login, as an enum
	fn set_result(&self,result: impl Into<crate::event::player::PlayerPreLoginEventResult<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/event/player/PlayerPreLoginEvent/Result;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(result.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setResult",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the current kick message that will be used if getResult() !=
/// Result.ALLOWED
	fn kick_message(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getKickMessage",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Sets the kick message to display if getResult() != Result.ALLOWED
	fn set_kick_message(&self,message: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(message.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setKickMessage",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Allows the player to log in
	fn allow(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"allow",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Disallows the player from logging in, with the given reason
	fn disallow(&self,result: impl Into<crate::event::player::PlayerPreLoginEventResult<'mc>>,message: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/event/player/PlayerPreLoginEvent/Result;Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(result.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(message.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"disallow",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the player's name.
	fn name(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Gets the player IP address.
	fn address(&self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/net/InetAddress;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAddress",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.l()?
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the player's unique ID.
	fn unique_id(&self) 
-> Result<blackboxmc_java::util::JavaUUID<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/UUID;");
let res = self.jni_ref().call_method(&self.jni_object(),"getUniqueId",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaUUID::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerPreLoginEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Event<'mc>> for PlayerPreLoginEvent<'mc>{

fn into(self) -> crate::event::Event<'mc> {

crate::event::Event::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerPreLoginEvent into crate::event::Event")

   }
}
impl<'mc> crate::event::EventTrait<'mc> for PlayerPreLoginEvent<'mc> {}
pub enum PlayerExpCooldownChangeEventChangeReason<'mc> {
	PickupOrb {inner: PlayerExpCooldownChangeEventChangeReasonStruct<'mc>},
	Plugin {inner: PlayerExpCooldownChangeEventChangeReasonStruct<'mc>},
}
impl<'mc> std::fmt::Display for PlayerExpCooldownChangeEventChangeReason<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           PlayerExpCooldownChangeEventChangeReason::PickupOrb { .. } => f.write_str("PICKUP_ORB"),
           PlayerExpCooldownChangeEventChangeReason::Plugin { .. } => f.write_str("PLUGIN"),
       }
   }
}

        impl<'mc> PlayerExpCooldownChangeEventChangeReasonTrait<'mc> for PlayerExpCooldownChangeEventChangeReason<'mc> {}
        
        pub trait PlayerExpCooldownChangeEventChangeReasonTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<PlayerExpCooldownChangeEventChangeReason<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/event/player/PlayerExpCooldownChangeEvent/ChangeReason");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/event/player/PlayerExpCooldownChangeEvent/ChangeReason;",
                    vec![jni::objects::JValueGen::from(val_1)],
                );
                let res = env.translate_error(res)?;
                let obj = res.l()?;
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    
"PICKUP_ORB" => Ok(PlayerExpCooldownChangeEventChangeReason::PickupOrb { inner: PlayerExpCooldownChangeEventChangeReasonStruct::from_raw(env,obj)?}),
"PLUGIN" => Ok(PlayerExpCooldownChangeEventChangeReason::Plugin { inner: PlayerExpCooldownChangeEventChangeReasonStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct PlayerExpCooldownChangeEventChangeReasonStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerExpCooldownChangeEventChangeReason<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::PickupOrb { inner } => inner.0.clone(),
Self::Plugin { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::PickupOrb { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Plugin { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerExpCooldownChangeEventChangeReason<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerExpCooldownChangeEventChangeReason from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerExpCooldownChangeEvent/ChangeReason")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerExpCooldownChangeEventChangeReason object, got {}",
                    name
                )
                .into())
            } else {
    
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    "PICKUP_ORB" => Ok(PlayerExpCooldownChangeEventChangeReason::PickupOrb { inner: PlayerExpCooldownChangeEventChangeReasonStruct::from_raw(env,obj)?}),"PLUGIN" => Ok(PlayerExpCooldownChangeEventChangeReason::Plugin { inner: PlayerExpCooldownChangeEventChangeReasonStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for PlayerExpCooldownChangeEventChangeReasonStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerExpCooldownChangeEventChangeReasonStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerExpCooldownChangeEventChangeReasonStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerExpCooldownChangeEvent/ChangeReason")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerExpCooldownChangeEventChangeReasonStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerExpCooldownChangeEventChangeReasonStruct<'mc> {

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::player::PlayerExpCooldownChangeEventChangeReason<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/player/PlayerExpCooldownChangeEvent/ChangeReason;");
let cls = jni.find_class("org/bukkit/event/player/PlayerExpCooldownChangeEvent/ChangeReason"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::player::PlayerExpCooldownChangeEventChangeReason::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct PlayerPickupItemEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerPickupItemEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerPickupItemEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerPickupItemEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerPickupItemEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerPickupItemEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerPickupItemEventTrait<'mc> for PlayerPickupItemEvent<'mc> {}
pub trait PlayerPickupItemEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,item: impl Into<crate::entity::Item<'mc>>,remaining: i32) 
-> Result<crate::event::player::PlayerPickupItemEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Lorg/bukkit/entity/Item;I)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Int(remaining);
let cls = jni.find_class("org/bukkit/event/player/PlayerPickupItemEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerPickupItemEvent::from_raw(&jni,res
)}
/// Gets the Item picked up by the player.
	fn item(&self) 
-> Result<crate::entity::Item<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Item;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItem",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Item::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the amount remaining on the ground, if any
	fn remaining(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getRemaining",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}

	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerPickupItemEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerPickupItemEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerPickupItemEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for PlayerPickupItemEvent<'mc> {}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerPickupItemEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerPickupItemEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerPickupItemEvent<'mc> {}
#[repr(C)]
pub struct PlayerRecipeBookSettingsChangeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerRecipeBookSettingsChangeEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerRecipeBookSettingsChangeEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerRecipeBookSettingsChangeEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerRecipeBookSettingsChangeEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerRecipeBookSettingsChangeEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerRecipeBookSettingsChangeEventTrait<'mc> for PlayerRecipeBookSettingsChangeEvent<'mc> {}
pub trait PlayerRecipeBookSettingsChangeEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,recipe_book_type: impl Into<crate::event::player::PlayerRecipeBookSettingsChangeEventRecipeBookType<'mc>>,open: bool,filtering: bool) 
-> Result<crate::event::player::PlayerRecipeBookSettingsChangeEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Lorg/bukkit/event/player/PlayerRecipeBookSettingsChangeEvent/RecipeBookType;ZZ)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(recipe_book_type.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Bool(open.into());
let val_4 = jni::objects::JValueGen::Bool(filtering.into());
let cls = jni.find_class("org/bukkit/event/player/PlayerRecipeBookSettingsChangeEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerRecipeBookSettingsChangeEvent::from_raw(&jni,res
)}
/// Gets the type of recipe book the player is changing the settings for.
	fn recipe_book_type(&self) 
-> Result<crate::event::player::PlayerRecipeBookSettingsChangeEventRecipeBookType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/player/PlayerRecipeBookSettingsChangeEvent/RecipeBookType;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRecipeBookType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::player::PlayerRecipeBookSettingsChangeEventRecipeBookType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Checks if the recipe book is being opened or closed.
	fn is_open(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isOpen",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Checks if the recipe book filter is being enabled or disabled.
	fn is_filtering(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isFiltering",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerRecipeBookSettingsChangeEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerRecipeBookSettingsChangeEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerRecipeBookSettingsChangeEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerRecipeBookSettingsChangeEvent<'mc> {}
pub enum PlayerTeleportEventTeleportCause<'mc> {
	EnderPearl {inner: PlayerTeleportEventTeleportCauseStruct<'mc>},
	Command {inner: PlayerTeleportEventTeleportCauseStruct<'mc>},
	Plugin {inner: PlayerTeleportEventTeleportCauseStruct<'mc>},
	NetherPortal {inner: PlayerTeleportEventTeleportCauseStruct<'mc>},
	EndPortal {inner: PlayerTeleportEventTeleportCauseStruct<'mc>},
	Spectate {inner: PlayerTeleportEventTeleportCauseStruct<'mc>},
	EndGateway {inner: PlayerTeleportEventTeleportCauseStruct<'mc>},
	ChorusFruit {inner: PlayerTeleportEventTeleportCauseStruct<'mc>},
	Dismount {inner: PlayerTeleportEventTeleportCauseStruct<'mc>},
	ExitBed {inner: PlayerTeleportEventTeleportCauseStruct<'mc>},
	Unknown {inner: PlayerTeleportEventTeleportCauseStruct<'mc>},
}
impl<'mc> std::fmt::Display for PlayerTeleportEventTeleportCause<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           PlayerTeleportEventTeleportCause::EnderPearl { .. } => f.write_str("ENDER_PEARL"),
           PlayerTeleportEventTeleportCause::Command { .. } => f.write_str("COMMAND"),
           PlayerTeleportEventTeleportCause::Plugin { .. } => f.write_str("PLUGIN"),
           PlayerTeleportEventTeleportCause::NetherPortal { .. } => f.write_str("NETHER_PORTAL"),
           PlayerTeleportEventTeleportCause::EndPortal { .. } => f.write_str("END_PORTAL"),
           PlayerTeleportEventTeleportCause::Spectate { .. } => f.write_str("SPECTATE"),
           PlayerTeleportEventTeleportCause::EndGateway { .. } => f.write_str("END_GATEWAY"),
           PlayerTeleportEventTeleportCause::ChorusFruit { .. } => f.write_str("CHORUS_FRUIT"),
           PlayerTeleportEventTeleportCause::Dismount { .. } => f.write_str("DISMOUNT"),
           PlayerTeleportEventTeleportCause::ExitBed { .. } => f.write_str("EXIT_BED"),
           PlayerTeleportEventTeleportCause::Unknown { .. } => f.write_str("UNKNOWN"),
       }
   }
}

        impl<'mc> PlayerTeleportEventTeleportCauseTrait<'mc> for PlayerTeleportEventTeleportCause<'mc> {}
        
        pub trait PlayerTeleportEventTeleportCauseTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<PlayerTeleportEventTeleportCause<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/event/player/PlayerTeleportEvent/TeleportCause");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/event/player/PlayerTeleportEvent/TeleportCause;",
                    vec![jni::objects::JValueGen::from(val_1)],
                );
                let res = env.translate_error(res)?;
                let obj = res.l()?;
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    
"ENDER_PEARL" => Ok(PlayerTeleportEventTeleportCause::EnderPearl { inner: PlayerTeleportEventTeleportCauseStruct::from_raw(env,obj)?}),
"COMMAND" => Ok(PlayerTeleportEventTeleportCause::Command { inner: PlayerTeleportEventTeleportCauseStruct::from_raw(env,obj)?}),
"PLUGIN" => Ok(PlayerTeleportEventTeleportCause::Plugin { inner: PlayerTeleportEventTeleportCauseStruct::from_raw(env,obj)?}),
"NETHER_PORTAL" => Ok(PlayerTeleportEventTeleportCause::NetherPortal { inner: PlayerTeleportEventTeleportCauseStruct::from_raw(env,obj)?}),
"END_PORTAL" => Ok(PlayerTeleportEventTeleportCause::EndPortal { inner: PlayerTeleportEventTeleportCauseStruct::from_raw(env,obj)?}),
"SPECTATE" => Ok(PlayerTeleportEventTeleportCause::Spectate { inner: PlayerTeleportEventTeleportCauseStruct::from_raw(env,obj)?}),
"END_GATEWAY" => Ok(PlayerTeleportEventTeleportCause::EndGateway { inner: PlayerTeleportEventTeleportCauseStruct::from_raw(env,obj)?}),
"CHORUS_FRUIT" => Ok(PlayerTeleportEventTeleportCause::ChorusFruit { inner: PlayerTeleportEventTeleportCauseStruct::from_raw(env,obj)?}),
"DISMOUNT" => Ok(PlayerTeleportEventTeleportCause::Dismount { inner: PlayerTeleportEventTeleportCauseStruct::from_raw(env,obj)?}),
"EXIT_BED" => Ok(PlayerTeleportEventTeleportCause::ExitBed { inner: PlayerTeleportEventTeleportCauseStruct::from_raw(env,obj)?}),
"UNKNOWN" => Ok(PlayerTeleportEventTeleportCause::Unknown { inner: PlayerTeleportEventTeleportCauseStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct PlayerTeleportEventTeleportCauseStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerTeleportEventTeleportCause<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::EnderPearl { inner } => inner.0.clone(),
Self::Command { inner } => inner.0.clone(),
Self::Plugin { inner } => inner.0.clone(),
Self::NetherPortal { inner } => inner.0.clone(),
Self::EndPortal { inner } => inner.0.clone(),
Self::Spectate { inner } => inner.0.clone(),
Self::EndGateway { inner } => inner.0.clone(),
Self::ChorusFruit { inner } => inner.0.clone(),
Self::Dismount { inner } => inner.0.clone(),
Self::ExitBed { inner } => inner.0.clone(),
Self::Unknown { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::EnderPearl { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Command { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Plugin { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::NetherPortal { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::EndPortal { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Spectate { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::EndGateway { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::ChorusFruit { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Dismount { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::ExitBed { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Unknown { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerTeleportEventTeleportCause<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerTeleportEventTeleportCause from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerTeleportEvent/TeleportCause")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerTeleportEventTeleportCause object, got {}",
                    name
                )
                .into())
            } else {
    
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    "ENDER_PEARL" => Ok(PlayerTeleportEventTeleportCause::EnderPearl { inner: PlayerTeleportEventTeleportCauseStruct::from_raw(env,obj)?}),"COMMAND" => Ok(PlayerTeleportEventTeleportCause::Command { inner: PlayerTeleportEventTeleportCauseStruct::from_raw(env,obj)?}),"PLUGIN" => Ok(PlayerTeleportEventTeleportCause::Plugin { inner: PlayerTeleportEventTeleportCauseStruct::from_raw(env,obj)?}),"NETHER_PORTAL" => Ok(PlayerTeleportEventTeleportCause::NetherPortal { inner: PlayerTeleportEventTeleportCauseStruct::from_raw(env,obj)?}),"END_PORTAL" => Ok(PlayerTeleportEventTeleportCause::EndPortal { inner: PlayerTeleportEventTeleportCauseStruct::from_raw(env,obj)?}),"SPECTATE" => Ok(PlayerTeleportEventTeleportCause::Spectate { inner: PlayerTeleportEventTeleportCauseStruct::from_raw(env,obj)?}),"END_GATEWAY" => Ok(PlayerTeleportEventTeleportCause::EndGateway { inner: PlayerTeleportEventTeleportCauseStruct::from_raw(env,obj)?}),"CHORUS_FRUIT" => Ok(PlayerTeleportEventTeleportCause::ChorusFruit { inner: PlayerTeleportEventTeleportCauseStruct::from_raw(env,obj)?}),"DISMOUNT" => Ok(PlayerTeleportEventTeleportCause::Dismount { inner: PlayerTeleportEventTeleportCauseStruct::from_raw(env,obj)?}),"EXIT_BED" => Ok(PlayerTeleportEventTeleportCause::ExitBed { inner: PlayerTeleportEventTeleportCauseStruct::from_raw(env,obj)?}),"UNKNOWN" => Ok(PlayerTeleportEventTeleportCause::Unknown { inner: PlayerTeleportEventTeleportCauseStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for PlayerTeleportEventTeleportCauseStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerTeleportEventTeleportCauseStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerTeleportEventTeleportCauseStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerTeleportEvent/TeleportCause")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerTeleportEventTeleportCauseStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerTeleportEventTeleportCauseStruct<'mc> {

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::player::PlayerTeleportEventTeleportCause<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/player/PlayerTeleportEvent/TeleportCause;");
let cls = jni.find_class("org/bukkit/event/player/PlayerTeleportEvent/TeleportCause"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::player::PlayerTeleportEventTeleportCause::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct PlayerBucketEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerBucketEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerBucketEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerBucketEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerBucketEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerBucketEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerBucketEventTrait<'mc> for PlayerBucketEvent<'mc> {}
pub trait PlayerBucketEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,who: impl Into<crate::entity::Player<'mc>>,block: impl Into<crate::block::Block<'mc>>,block_clicked: impl Into<crate::block::Block<'mc>>,block_face: impl Into<crate::block::BlockFace<'mc>>,bucket: impl Into<crate::Material<'mc>>,item_in_hand: std::option::Option<impl Into<crate::inventory::ItemStack<'mc>>>,hand: std::option::Option<impl Into<crate::inventory::EquipmentSlot<'mc>>>) 
-> Result<crate::event::player::PlayerBucketEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/entity/Player;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(who.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/block/Block;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
args.push(val_2);
sig += "Lorg/bukkit/block/Block;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block_clicked.into().jni_object().clone())});
args.push(val_3);
sig += "Lorg/bukkit/block/BlockFace;";
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block_face.into().jni_object().clone())});
args.push(val_4);
sig += "Lorg/bukkit/Material;";
let val_5 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(bucket.into().jni_object().clone())});
args.push(val_5);
if let Some(a) = item_in_hand {
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_6 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_6);
}
if let Some(a) = hand {
sig += "Lorg/bukkit/inventory/EquipmentSlot;";
let val_7 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_7);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/player/PlayerBucketEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerBucketEvent::from_raw(&jni,res
)}
/// Returns the bucket used in this event
	fn bucket(&self) 
-> Result<crate::Material<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Material;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBucket",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Material::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Get the resulting item in hand after the bucket event
	fn item_stack(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItemStack",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Set the item in hand after the event
	fn set_item_stack(&self,item_stack: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item_stack.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setItemStack",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the block involved in this event.
	fn block(&self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/Block;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBlock",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Return the block clicked
	fn block_clicked(&self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/Block;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBlockClicked",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Get the face on the clicked block
	fn block_face(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBlockFace",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Get the hand that was used in this event.
	fn hand(&self) 
-> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/EquipmentSlot;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHand",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerBucketEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerBucketEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for PlayerBucketEvent<'mc> {}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerBucketEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerBucketEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerBucketEvent<'mc> {}
#[repr(C)]
pub struct PlayerSignOpenEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerSignOpenEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerSignOpenEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerSignOpenEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerSignOpenEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerSignOpenEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerSignOpenEventTrait<'mc> for PlayerSignOpenEvent<'mc> {}
pub trait PlayerSignOpenEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,sign: impl Into<crate::block::Sign<'mc>>,side: impl Into<crate::block::sign::Side<'mc>>,cause: impl Into<crate::event::player::PlayerSignOpenEventCause<'mc>>) 
-> Result<crate::event::player::PlayerSignOpenEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Lorg/bukkit/block/Sign;Lorg/bukkit/block/sign/Side;Lorg/bukkit/event/player/PlayerSignOpenEvent/Cause;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(sign.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(side.into().jni_object().clone())});
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(cause.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/player/PlayerSignOpenEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerSignOpenEvent::from_raw(&jni,res
)}
/// Gets the sign that was opened.
	fn sign(&self) 
-> Result<crate::block::Sign<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/Sign;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSign",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Sign::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets side of the sign opened.
	fn side(&self) 
-> Result<crate::block::sign::Side<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/sign/Side;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSide",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::sign::Side::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the cause of the sign open.
	fn cause(&self) 
-> Result<crate::event::player::PlayerSignOpenEventCause<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/player/PlayerSignOpenEvent/Cause;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCause",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::player::PlayerSignOpenEventCause::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerSignOpenEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerSignOpenEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerSignOpenEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for PlayerSignOpenEvent<'mc> {}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerSignOpenEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerSignOpenEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerSignOpenEvent<'mc> {}
#[repr(C)]
pub struct PlayerBedEnterEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerBedEnterEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerBedEnterEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerBedEnterEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerBedEnterEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerBedEnterEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerBedEnterEventTrait<'mc> for PlayerBedEnterEvent<'mc> {}
pub trait PlayerBedEnterEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,who: impl Into<crate::entity::Player<'mc>>,bed: impl Into<crate::block::Block<'mc>>,bed_enter_result: std::option::Option<impl Into<crate::event::player::PlayerBedEnterEventBedEnterResult<'mc>>>) 
-> Result<crate::event::player::PlayerBedEnterEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/entity/Player;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(who.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/block/Block;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(bed.into().jni_object().clone())});
args.push(val_2);
if let Some(a) = bed_enter_result {
sig += "Lorg/bukkit/event/player/PlayerBedEnterEvent/BedEnterResult;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_3);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/player/PlayerBedEnterEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerBedEnterEvent::from_raw(&jni,res
)}
/// This describes the default outcome of this event.
	fn bed_enter_result(&self) 
-> Result<crate::event::player::PlayerBedEnterEventBedEnterResult<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/player/PlayerBedEnterEvent/BedEnterResult;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBedEnterResult",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::player::PlayerBedEnterEventBedEnterResult::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// This controls the action to take with the bed that was clicked on.
/// 
/// In case of {@link org.bukkit.event.Event.Result#DEFAULT}, the default outcome is described by
/// {@link #getBedEnterResult()}.
	fn use_bed(&self) 
-> Result<crate::event::EventResult<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/Event/Result;");
let res = self.jni_ref().call_method(&self.jni_object(),"useBed",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::EventResult::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the action to take with the interacted bed.
/// 
/// {@link org.bukkit.event.Event.Result#ALLOW} will result in the player sleeping, regardless of
/// the default outcome described by {@link #getBedEnterResult()}.
/// 
/// {@link org.bukkit.event.Event.Result#DENY} will prevent the player from sleeping. This has the
/// same effect as canceling the event via {@link #setCancelled(boolean)}.
/// 
/// {@link org.bukkit.event.Event.Result#DEFAULT} will result in the outcome described by
/// {@link #getBedEnterResult()}.
	fn set_use_bed(&self,use_bed: impl Into<crate::event::EventResult<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/event/Event/Result;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(use_bed.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setUseBed",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the cancellation state of this event. Set to true if you want to
/// prevent the player from sleeping.
/// 
/// Canceling the event has the same effect as setting {@link #useBed()} to
/// {@link org.bukkit.event.Event.Result#DENY}.
/// 
/// For backwards compatibility reasons this also returns true if
/// {@link #useBed()} is {@link org.bukkit.event.Event.Result#DEFAULT} and the
/// {@link #getBedEnterResult() default action} is to prevent bed entering.
	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Sets the cancellation state of this event. A canceled event will not be
/// executed in the server, but will still pass to other plugins.
/// 
/// Canceling this event will prevent use of the bed.
	fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Returns the bed block involved in this event.
	fn bed(&self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/Block;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBed",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerBedEnterEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerBedEnterEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerBedEnterEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for PlayerBedEnterEvent<'mc> {}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerBedEnterEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerBedEnterEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerBedEnterEvent<'mc> {}
#[repr(C)]
pub struct PlayerQuitEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerQuitEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerQuitEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerQuitEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerQuitEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerQuitEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerQuitEventTrait<'mc> for PlayerQuitEvent<'mc> {}
pub trait PlayerQuitEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,who: impl Into<crate::entity::Player<'mc>>,quit_message: impl Into<String>) 
-> Result<crate::event::player::PlayerQuitEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(who.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(quit_message.into())?));
let cls = jni.find_class("org/bukkit/event/player/PlayerQuitEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerQuitEvent::from_raw(&jni,res
)}
/// Gets the quit message to send to all online players
	fn quit_message(&self) 
-> Result<Option<String>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getQuitMessage",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)
)}
/// Sets the quit message to send to all online players
	fn set_quit_message(&self,quit_message: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(quit_message.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setQuitMessage",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerQuitEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerQuitEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerQuitEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerQuitEvent<'mc> {}
#[repr(C)]
pub struct PlayerInteractEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerInteractEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerInteractEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerInteractEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerInteractEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerInteractEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerInteractEventTrait<'mc> for PlayerInteractEvent<'mc> {}
pub trait PlayerInteractEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,who: impl Into<crate::entity::Player<'mc>>,action: impl Into<crate::event::block::Action<'mc>>,item: impl Into<crate::inventory::ItemStack<'mc>>,clicked_block: impl Into<crate::block::Block<'mc>>,clicked_face: impl Into<crate::block::BlockFace<'mc>>,hand: std::option::Option<impl Into<crate::inventory::EquipmentSlot<'mc>>>,clicked_position: std::option::Option<impl Into<crate::util::Vector<'mc>>>) 
-> Result<crate::event::player::PlayerInteractEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/entity/Player;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(who.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/event/block/Action;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(action.into().jni_object().clone())});
args.push(val_2);
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
args.push(val_3);
sig += "Lorg/bukkit/block/Block;";
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(clicked_block.into().jni_object().clone())});
args.push(val_4);
sig += "Lorg/bukkit/block/BlockFace;";
let val_5 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(clicked_face.into().jni_object().clone())});
args.push(val_5);
if let Some(a) = hand {
sig += "Lorg/bukkit/inventory/EquipmentSlot;";
let val_6 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_6);
}
if let Some(a) = clicked_position {
sig += "Lorg/bukkit/util/Vector;";
let val_7 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_7);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/player/PlayerInteractEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerInteractEvent::from_raw(&jni,res
)}
/// Returns the action type
	fn action(&self) 
-> Result<crate::event::block::Action<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/block/Action;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAction",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::block::Action::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]
/// Gets the cancellation state of this event. Set to true if you want to prevent buckets from placing water and so forth
	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Sets the cancellation state of this event. A canceled event will not be
/// executed in the server, but will still pass to other plugins
/// 
/// Canceling this event will prevent use of food (player won't lose the
/// food item), prevent bows/snowballs/eggs from firing, etc. (player won't
/// lose the ammo)
	fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Returns the item in hand represented by this event
	fn item(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItem",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Convenience method. Returns the material of the item represented by
/// this event
	fn material(&self) 
-> Result<crate::Material<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Material;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMaterial",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Material::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Check if this event involved a block
	fn has_block(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasBlock",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Check if this event involved an item
	fn has_item(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasItem",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Convenience method to inform the user whether this was a block
/// placement event.
	fn is_block_in_hand(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isBlockInHand",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Returns the clicked block
	fn clicked_block(&self) 
-> Result<Option<crate::block::Block<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/Block;");
let res = self.jni_ref().call_method(&self.jni_object(),"getClickedBlock",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Returns the face of the block that was clicked
	fn block_face(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBlockFace",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// This controls the action to take with the block (if any) that was
/// clicked on. This event gets processed for all blocks, but most don't
/// have a default action
	fn use_interacted_block(&self) 
-> Result<crate::event::EventResult<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/Event/Result;");
let res = self.jni_ref().call_method(&self.jni_object(),"useInteractedBlock",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::EventResult::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn set_use_interacted_block(&self,use_interacted_block: impl Into<crate::event::EventResult<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/event/Event/Result;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(use_interacted_block.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setUseInteractedBlock",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// This controls the action to take with the item the player is holding.
/// This includes both blocks and items (such as flint and steel or
/// records). When this is set to default, it will be allowed if no action
/// is taken on the interacted block.
	fn use_item_in_hand(&self) 
-> Result<crate::event::EventResult<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/Event/Result;");
let res = self.jni_ref().call_method(&self.jni_object(),"useItemInHand",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::EventResult::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn set_use_item_in_hand(&self,use_item_in_hand: impl Into<crate::event::EventResult<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/event/Event/Result;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(use_item_in_hand.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setUseItemInHand",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// The hand used to perform this interaction. May be null in the case of
/// {@link Action#PHYSICAL}.
	fn hand(&self) 
-> Result<Option<crate::inventory::EquipmentSlot<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/EquipmentSlot;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHand",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Gets the exact position on the block the player interacted with, this will
/// be null outside of {@link Action#RIGHT_CLICK_BLOCK}.
/// 
/// All vector components are between 0.0 and 1.0 inclusive.
	fn clicked_position(&self) 
-> Result<Option<crate::util::Vector<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/util/Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"getClickedPosition",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerInteractEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerInteractEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerInteractEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for PlayerInteractEvent<'mc> {}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerInteractEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerInteractEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerInteractEvent<'mc> {}
#[repr(C)]
pub struct PlayerTakeLecternBookEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerTakeLecternBookEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerTakeLecternBookEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerTakeLecternBookEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerTakeLecternBookEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerTakeLecternBookEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerTakeLecternBookEventTrait<'mc> for PlayerTakeLecternBookEvent<'mc> {}
pub trait PlayerTakeLecternBookEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,who: impl Into<crate::entity::Player<'mc>>,lectern: impl Into<crate::block::Lectern<'mc>>) 
-> Result<crate::event::player::PlayerTakeLecternBookEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Lorg/bukkit/block/Lectern;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(who.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(lectern.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/player/PlayerTakeLecternBookEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerTakeLecternBookEvent::from_raw(&jni,res
)}
/// Gets the lectern involved.
	fn lectern(&self) 
-> Result<crate::block::Lectern<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/Lectern;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLectern",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Lectern::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the current ItemStack on the lectern.
	fn book(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBook",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerTakeLecternBookEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerTakeLecternBookEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerTakeLecternBookEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for PlayerTakeLecternBookEvent<'mc> {}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerTakeLecternBookEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerTakeLecternBookEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerTakeLecternBookEvent<'mc> {}
#[repr(C)]
pub struct AsyncPlayerChatEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for AsyncPlayerChatEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for AsyncPlayerChatEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate AsyncPlayerChatEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/AsyncPlayerChatEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a AsyncPlayerChatEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> AsyncPlayerChatEventTrait<'mc> for AsyncPlayerChatEvent<'mc> {}
pub trait AsyncPlayerChatEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_async: bool,who: impl Into<crate::entity::Player<'mc>>,message: impl Into<String>,players: impl Into<blackboxmc_java::util::JavaSet<'mc>>) 
-> Result<crate::event::player::AsyncPlayerChatEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(ZLorg/bukkit/entity/Player;Ljava/lang/String;Ljava/util/Set;)V");
let val_1 = jni::objects::JValueGen::Bool(val_async.into());
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(who.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(message.into())?));
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(players.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/player/AsyncPlayerChatEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::AsyncPlayerChatEvent::from_raw(&jni,res
)}
/// Gets the message that the player is attempting to send. This message
/// will be used with {@link #getFormat()}.
	fn message(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMessage",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Sets the message that the player will send. This message will be used
/// with {@link #getFormat()}.
	fn set_message(&self,message: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(message.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setMessage",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the format to use to display this chat message.
/// 
/// When this event finishes execution, the first format parameter is the
/// {@link Player#getDisplayName()} and the second parameter is {@link
/// #getMessage()}
	fn format(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFormat",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Sets the format to use to display this chat message.
/// 
/// When this event finishes execution, the first format parameter is the
/// {@link Player#getDisplayName()} and the second parameter is {@link
/// #getMessage()}
	fn set_format(&self,format: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(format.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setFormat",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets a set of recipients that this chat message will be displayed to.
/// 
/// The set returned is not guaranteed to be mutable and may auto-populate
/// on access. Any listener accessing the returned set should be aware that
/// it may reduce performance for a lazy set implementation.
/// 
/// Listeners should be aware that modifying the list may throw {@link
/// UnsupportedOperationException} if the event caller provides an
/// unmodifiable set.
	fn recipients(&self) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Set;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRecipients",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/AsyncPlayerChatEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for AsyncPlayerChatEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting AsyncPlayerChatEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for AsyncPlayerChatEvent<'mc> {}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for AsyncPlayerChatEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting AsyncPlayerChatEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for AsyncPlayerChatEvent<'mc> {}
#[repr(C)]
pub struct PlayerBucketFillEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerBucketFillEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerBucketFillEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerBucketFillEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerBucketFillEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerBucketFillEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerBucketFillEventTrait<'mc> for PlayerBucketFillEvent<'mc> {}
pub trait PlayerBucketFillEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,who: impl Into<crate::entity::Player<'mc>>,block: impl Into<crate::block::Block<'mc>>,block_clicked: impl Into<crate::block::Block<'mc>>,block_face: impl Into<crate::block::BlockFace<'mc>>,bucket: impl Into<crate::Material<'mc>>,item_in_hand: std::option::Option<impl Into<crate::inventory::ItemStack<'mc>>>,hand: std::option::Option<impl Into<crate::inventory::EquipmentSlot<'mc>>>) 
-> Result<crate::event::player::PlayerBucketFillEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/entity/Player;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(who.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/block/Block;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
args.push(val_2);
sig += "Lorg/bukkit/block/Block;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block_clicked.into().jni_object().clone())});
args.push(val_3);
sig += "Lorg/bukkit/block/BlockFace;";
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block_face.into().jni_object().clone())});
args.push(val_4);
sig += "Lorg/bukkit/Material;";
let val_5 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(bucket.into().jni_object().clone())});
args.push(val_5);
if let Some(a) = item_in_hand {
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_6 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_6);
}
if let Some(a) = hand {
sig += "Lorg/bukkit/inventory/EquipmentSlot;";
let val_7 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_7);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/player/PlayerBucketFillEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerBucketFillEvent::from_raw(&jni,res
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerBucketFillEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::player::PlayerBucketEvent<'mc>> for PlayerBucketFillEvent<'mc>{

fn into(self) -> crate::event::player::PlayerBucketEvent<'mc> {

crate::event::player::PlayerBucketEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerBucketFillEvent into crate::event::player::PlayerBucketEvent")

   }
}
impl<'mc> crate::event::player::PlayerBucketEventTrait<'mc> for PlayerBucketFillEvent<'mc> {}
#[repr(C)]
pub struct PlayerItemDamageEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerItemDamageEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerItemDamageEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerItemDamageEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerItemDamageEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerItemDamageEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerItemDamageEventTrait<'mc> for PlayerItemDamageEvent<'mc> {}
pub trait PlayerItemDamageEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,what: impl Into<crate::inventory::ItemStack<'mc>>,damage: i32) 
-> Result<crate::event::player::PlayerItemDamageEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Lorg/bukkit/inventory/ItemStack;I)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(what.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Int(damage);
let cls = jni.find_class("org/bukkit/event/player/PlayerItemDamageEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerItemDamageEvent::from_raw(&jni,res
)}
/// Gets the item being damaged.
	fn item(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItem",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the amount of durability damage this item will be taking.
	fn damage(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getDamage",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}

	fn set_damage(&self,damage: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(damage);
let res = self.jni_ref().call_method(&self.jni_object(),"setDamage",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerItemDamageEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerItemDamageEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerItemDamageEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for PlayerItemDamageEvent<'mc> {}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerItemDamageEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerItemDamageEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerItemDamageEvent<'mc> {}
#[repr(C)]
pub struct PlayerItemHeldEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerItemHeldEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerItemHeldEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerItemHeldEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerItemHeldEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerItemHeldEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerItemHeldEventTrait<'mc> for PlayerItemHeldEvent<'mc> {}
pub trait PlayerItemHeldEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,previous: i32,current: i32) 
-> Result<crate::event::player::PlayerItemHeldEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;II)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Int(previous);
let val_3 = jni::objects::JValueGen::Int(current);
let cls = jni.find_class("org/bukkit/event/player/PlayerItemHeldEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerItemHeldEvent::from_raw(&jni,res
)}
/// Gets the previous held slot index
	fn previous_slot(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getPreviousSlot",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Gets the new held slot index
	fn new_slot(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getNewSlot",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}

	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerItemHeldEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerItemHeldEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerItemHeldEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for PlayerItemHeldEvent<'mc> {}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerItemHeldEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerItemHeldEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerItemHeldEvent<'mc> {}
#[repr(C)]
pub struct PlayerHarvestBlockEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerHarvestBlockEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerHarvestBlockEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerHarvestBlockEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerHarvestBlockEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerHarvestBlockEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerHarvestBlockEventTrait<'mc> for PlayerHarvestBlockEvent<'mc> {}
pub trait PlayerHarvestBlockEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,harvested_block: impl Into<crate::block::Block<'mc>>,hand: impl Into<crate::inventory::EquipmentSlot<'mc>>,items_harvested: std::option::Option<Vec<jni::objects::JObject<'mc>>>) 
-> Result<crate::event::player::PlayerHarvestBlockEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/entity/Player;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/block/Block;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(harvested_block.into().jni_object().clone())});
args.push(val_2);
sig += "Lorg/bukkit/inventory/EquipmentSlot;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(hand.into().jni_object().clone())});
args.push(val_3);
if let Some(a) = items_harvested {
sig += "Ljava/util/List;";
let raw_val_4 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
for v in a{
sig += "Ljava/lang/java/lang/Object;";
		let map_val_0 = jni::objects::JValueGen::Object(v);
jni.call_method(&raw_val_4,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_4 = jni::objects::JValueGen::Object(raw_val_4);
args.push(val_4);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/player/PlayerHarvestBlockEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerHarvestBlockEvent::from_raw(&jni,res
)}
/// Gets the block that is being harvested.
	fn harvested_block(&self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/Block;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHarvestedBlock",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Get the hand used to harvest the block.
	fn hand(&self) 
-> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/EquipmentSlot;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHand",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets a list of items that are being harvested from this block.
	fn items_harvested(&self) 
-> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItemsHarvested",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::inventory::ItemStack::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}

	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/HandlerList;");
let cls = jni.find_class("org/bukkit/event/player/PlayerHarvestBlockEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerHarvestBlockEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerHarvestBlockEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for PlayerHarvestBlockEvent<'mc> {}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerHarvestBlockEvent<'mc>{

fn into(self) -> crate::event::player::PlayerEvent<'mc> {

crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerHarvestBlockEvent into crate::event::player::PlayerEvent")

   }
}
impl<'mc> crate::event::player::PlayerEventTrait<'mc> for PlayerHarvestBlockEvent<'mc> {}
#[repr(C)]
pub struct PlayerUnregisterChannelEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PlayerUnregisterChannelEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PlayerUnregisterChannelEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PlayerUnregisterChannelEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/player/PlayerUnregisterChannelEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PlayerUnregisterChannelEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PlayerUnregisterChannelEventTrait<'mc> for PlayerUnregisterChannelEvent<'mc> {}
pub trait PlayerUnregisterChannelEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,channel: impl Into<String>) 
-> Result<crate::event::player::PlayerUnregisterChannelEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(channel.into())?));
let cls = jni.find_class("org/bukkit/event/player/PlayerUnregisterChannelEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::player::PlayerUnregisterChannelEvent::from_raw(&jni,res
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::player::PlayerChannelEvent<'mc>> for PlayerUnregisterChannelEvent<'mc>{

fn into(self) -> crate::event::player::PlayerChannelEvent<'mc> {

crate::event::player::PlayerChannelEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PlayerUnregisterChannelEvent into crate::event::player::PlayerChannelEvent")

   }
}
impl<'mc> crate::event::player::PlayerChannelEventTrait<'mc> for PlayerUnregisterChannelEvent<'mc> {}
