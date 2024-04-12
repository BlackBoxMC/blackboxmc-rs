#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;/*org/bukkit/event/raid/mod.rs*/

#[repr(C)]
pub struct RaidEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for RaidEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for RaidEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate RaidEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/raid/RaidEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a RaidEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> RaidEventTrait<'mc> for RaidEvent<'mc> {}
pub trait RaidEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Returns the raid involved with this event.
	fn raid(&self) 
-> Result<crate::Raid<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Raid;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRaid",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Raid::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::world::WorldEvent<'mc>> for RaidEvent<'mc>{

fn into(self) -> crate::event::world::WorldEvent<'mc> {

crate::event::world::WorldEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting RaidEvent into crate::event::world::WorldEvent")

   }
}
impl<'mc> crate::event::world::WorldEventTrait<'mc> for RaidEvent<'mc> {}
#[repr(C)]
pub struct RaidFinishEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for RaidFinishEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for RaidFinishEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate RaidFinishEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/raid/RaidFinishEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a RaidFinishEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> RaidFinishEventTrait<'mc> for RaidFinishEvent<'mc> {}
pub trait RaidFinishEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,raid: impl Into<crate::Raid<'mc>>,world: impl Into<crate::World<'mc>>,winners: Vec<jni::objects::JObject<'mc>>) 
-> Result<crate::event::raid::RaidFinishEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Raid;Lorg/bukkit/World;Ljava/util/List;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(raid.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(world.into().jni_object().clone())});
let raw_val_3 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
for v in winners{
let map_val_0 = jni::objects::JValueGen::Object(v);
jni.call_method(&raw_val_3,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_3 = jni::objects::JValueGen::Object(raw_val_3);
let cls = jni.find_class("org/bukkit/event/raid/RaidFinishEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::raid::RaidFinishEvent::from_raw(&jni,res
)}
/// Returns an immutable list contains all winners.
/// 
/// <b>Note: Players who are considered as heroes but were not online at the
/// end would not be included in this list.</b>
	fn winners(&self) 
-> Result<Vec<crate::entity::Player<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getWinners",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::entity::Player::from_raw(&self.jni_ref(),obj,)?);
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
let cls = jni.find_class("org/bukkit/event/raid/RaidFinishEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::raid::RaidEvent<'mc>> for RaidFinishEvent<'mc>{

fn into(self) -> crate::event::raid::RaidEvent<'mc> {

crate::event::raid::RaidEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting RaidFinishEvent into crate::event::raid::RaidEvent")

   }
}
impl<'mc> crate::event::raid::RaidEventTrait<'mc> for RaidFinishEvent<'mc> {}
#[repr(C)]
pub struct RaidTriggerEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for RaidTriggerEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for RaidTriggerEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate RaidTriggerEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/raid/RaidTriggerEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a RaidTriggerEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> RaidTriggerEventTrait<'mc> for RaidTriggerEvent<'mc> {}
pub trait RaidTriggerEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,raid: impl Into<crate::Raid<'mc>>,world: impl Into<crate::World<'mc>>,player: impl Into<crate::entity::Player<'mc>>) 
-> Result<crate::event::raid::RaidTriggerEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Raid;Lorg/bukkit/World;Lorg/bukkit/entity/Player;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(raid.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(world.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/raid/RaidTriggerEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::raid::RaidTriggerEvent::from_raw(&jni,res
)}
/// Returns the player who triggered the raid.
	fn player(&self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Player;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
let cls = jni.find_class("org/bukkit/event/raid/RaidTriggerEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for RaidTriggerEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting RaidTriggerEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for RaidTriggerEvent<'mc> {}
impl<'mc> Into<crate::event::raid::RaidEvent<'mc>> for RaidTriggerEvent<'mc>{

fn into(self) -> crate::event::raid::RaidEvent<'mc> {

crate::event::raid::RaidEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting RaidTriggerEvent into crate::event::raid::RaidEvent")

   }
}
impl<'mc> crate::event::raid::RaidEventTrait<'mc> for RaidTriggerEvent<'mc> {}
#[repr(C)]
pub struct RaidStopEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for RaidStopEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for RaidStopEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate RaidStopEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/raid/RaidStopEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a RaidStopEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> RaidStopEventTrait<'mc> for RaidStopEvent<'mc> {}
pub trait RaidStopEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,raid: impl Into<crate::Raid<'mc>>,world: impl Into<crate::World<'mc>>,reason: impl Into<crate::event::raid::RaidStopEventReason<'mc>>) 
-> Result<crate::event::raid::RaidStopEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Raid;Lorg/bukkit/World;Lorg/bukkit/event/raid/RaidStopEvent/Reason;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(raid.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(world.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(reason.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/raid/RaidStopEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::raid::RaidStopEvent::from_raw(&jni,res
)}
/// Returns the stop reason.
	fn reason(&self) 
-> Result<crate::event::raid::RaidStopEventReason<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/raid/RaidStopEvent/Reason;");
let res = self.jni_ref().call_method(&self.jni_object(),"getReason",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::raid::RaidStopEventReason::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
let cls = jni.find_class("org/bukkit/event/raid/RaidStopEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::raid::RaidEvent<'mc>> for RaidStopEvent<'mc>{

fn into(self) -> crate::event::raid::RaidEvent<'mc> {

crate::event::raid::RaidEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting RaidStopEvent into crate::event::raid::RaidEvent")

   }
}
impl<'mc> crate::event::raid::RaidEventTrait<'mc> for RaidStopEvent<'mc> {}
#[repr(C)]
pub struct RaidSpawnWaveEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for RaidSpawnWaveEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for RaidSpawnWaveEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate RaidSpawnWaveEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/raid/RaidSpawnWaveEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a RaidSpawnWaveEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> RaidSpawnWaveEventTrait<'mc> for RaidSpawnWaveEvent<'mc> {}
pub trait RaidSpawnWaveEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,raid: impl Into<crate::Raid<'mc>>,world: impl Into<crate::World<'mc>>,leader: impl Into<crate::entity::Raider<'mc>>,raiders: Vec<jni::objects::JObject<'mc>>) 
-> Result<crate::event::raid::RaidSpawnWaveEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Raid;Lorg/bukkit/World;Lorg/bukkit/entity/Raider;Ljava/util/List;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(raid.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(world.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(leader.into().jni_object().clone())});
let raw_val_4 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
for v in raiders{
let map_val_0 = jni::objects::JValueGen::Object(v);
jni.call_method(&raw_val_4,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_4 = jni::objects::JValueGen::Object(raw_val_4);
let cls = jni.find_class("org/bukkit/event/raid/RaidSpawnWaveEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::raid::RaidSpawnWaveEvent::from_raw(&jni,res
)}
/// Returns the patrol leader.
	fn patrol_leader(&self) 
-> Result<Option<crate::entity::Raider<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Raider;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPatrolLeader",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::entity::Raider::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Returns all {@link Raider} that spawned in this wave.
	fn raiders(&self) 
-> Result<Vec<crate::entity::Raider<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRaiders",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::entity::Raider::from_raw(&self.jni_ref(),obj,)?);
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
let cls = jni.find_class("org/bukkit/event/raid/RaidSpawnWaveEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::raid::RaidEvent<'mc>> for RaidSpawnWaveEvent<'mc>{

fn into(self) -> crate::event::raid::RaidEvent<'mc> {

crate::event::raid::RaidEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting RaidSpawnWaveEvent into crate::event::raid::RaidEvent")

   }
}
impl<'mc> crate::event::raid::RaidEventTrait<'mc> for RaidSpawnWaveEvent<'mc> {}
pub enum RaidStopEventReason<'mc> {
	Peace {inner: RaidStopEventReasonStruct<'mc>},
	Timeout {inner: RaidStopEventReasonStruct<'mc>},
	Finished {inner: RaidStopEventReasonStruct<'mc>},
	Unspawnable {inner: RaidStopEventReasonStruct<'mc>},
	NotInVillage {inner: RaidStopEventReasonStruct<'mc>},
}
impl<'mc> std::fmt::Display for RaidStopEventReason<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           RaidStopEventReason::Peace { .. } => f.write_str("PEACE"),
           RaidStopEventReason::Timeout { .. } => f.write_str("TIMEOUT"),
           RaidStopEventReason::Finished { .. } => f.write_str("FINISHED"),
           RaidStopEventReason::Unspawnable { .. } => f.write_str("UNSPAWNABLE"),
           RaidStopEventReason::NotInVillage { .. } => f.write_str("NOT_IN_VILLAGE"),
       }
   }
}

        impl<'mc> RaidStopEventReasonTrait<'mc> for RaidStopEventReason<'mc> {}
        
        pub trait RaidStopEventReasonTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<RaidStopEventReason<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/event/raid/RaidStopEvent/Reason");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/event/raid/RaidStopEvent/Reason;",
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
                    
"PEACE" => Ok(RaidStopEventReason::Peace { inner: RaidStopEventReasonStruct::from_raw(env,obj)?}),
"TIMEOUT" => Ok(RaidStopEventReason::Timeout { inner: RaidStopEventReasonStruct::from_raw(env,obj)?}),
"FINISHED" => Ok(RaidStopEventReason::Finished { inner: RaidStopEventReasonStruct::from_raw(env,obj)?}),
"UNSPAWNABLE" => Ok(RaidStopEventReason::Unspawnable { inner: RaidStopEventReasonStruct::from_raw(env,obj)?}),
"NOT_IN_VILLAGE" => Ok(RaidStopEventReason::NotInVillage { inner: RaidStopEventReasonStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct RaidStopEventReasonStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for RaidStopEventReason<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::Peace { inner } => inner.0.clone(),
Self::Timeout { inner } => inner.0.clone(),
Self::Finished { inner } => inner.0.clone(),
Self::Unspawnable { inner } => inner.0.clone(),
Self::NotInVillage { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::Peace { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Timeout { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Finished { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Unspawnable { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::NotInVillage { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for RaidStopEventReason<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate RaidStopEventReason from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/raid/RaidStopEvent/Reason")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a RaidStopEventReason object, got {}",
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
                    "PEACE" => Ok(RaidStopEventReason::Peace { inner: RaidStopEventReasonStruct::from_raw(env,obj)?}),"TIMEOUT" => Ok(RaidStopEventReason::Timeout { inner: RaidStopEventReasonStruct::from_raw(env,obj)?}),"FINISHED" => Ok(RaidStopEventReason::Finished { inner: RaidStopEventReasonStruct::from_raw(env,obj)?}),"UNSPAWNABLE" => Ok(RaidStopEventReason::Unspawnable { inner: RaidStopEventReasonStruct::from_raw(env,obj)?}),"NOT_IN_VILLAGE" => Ok(RaidStopEventReason::NotInVillage { inner: RaidStopEventReasonStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for RaidStopEventReasonStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for RaidStopEventReasonStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate RaidStopEventReasonStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/raid/RaidStopEvent/Reason")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a RaidStopEventReasonStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> RaidStopEventReasonStruct<'mc> {

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::raid::RaidStopEventReason<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/raid/RaidStopEvent/Reason;");
let cls = jni.find_class("org/bukkit/event/raid/RaidStopEvent/Reason"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::raid::RaidStopEventReason::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
