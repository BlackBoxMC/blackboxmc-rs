#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
pub struct PlayerToggleFlightEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerToggleFlightEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerToggleFlightEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerToggleFlightEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerToggleFlightEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerToggleFlightEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: bool) 
-> Result<crate::event::player::PlayerToggleFlightEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
// -2
let val_1 = jni::objects::JValueGen::Bool(arg1.into());
let cls = &jni.find_class("org/bukkit/event/player/PlayerToggleFlightEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Z)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::player::PlayerToggleFlightEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn is_flying(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isFlying","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerToggleFlightEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerToggleFlightEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerFishEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
pub struct PlayerFishEventState<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerFishEventState<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerFishEventState<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerFishEventState from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerFishEventState")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerFishEventState object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn value_of_with_string(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<jni::objects::JClass<'mc>>,arg1: std::option::Option<impl Into<&'mc String>>) 
-> Result<blackboxmc_java::JavaEnum<'mc>, Box<dyn std::error::Error>>

{let val_0 = arg0.unwrap();
let val_1 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
let cls = &jni.find_class("java/lang/Enum")?;
let res = jni.call_static_method(cls,"valueOf",
"(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
blackboxmc_java::JavaEnum::from_raw(&jni,obj
)}
	pub fn name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"name","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn describe_constable(&mut self) 
-> Result<blackboxmc_java::JavaOptional<'mc, javaEnumEnumDesc<E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"describeConstable","()Ljava/util/Optional;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaOptional::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn declaring_class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDeclaringClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn ordinal(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"ordinal","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerFishEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerFishEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerFishEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerFishEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerFishEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_player(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::entity::Entity<'mc>>,arg2: impl Into<&'mc crate::entity::FishHook<'mc>>,arg3: std::option::Option<impl Into<&'mc crate::inventory::EquipmentSlot<'mc>>>,arg4: std::option::Option<impl Into<&'mc crate::event::player::PlayerFishEventState<'mc>>>) 
-> Result<crate::event::player::PlayerFishEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.unwrap().into().jni_object().clone())};
let val_4 = unsafe { jni::objects::JObject::from_raw(arg4.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerFishEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/entity/Entity;Lorg/bukkit/entity/FishHook;Lorg/bukkit/inventory/EquipmentSlot;Lorg/bukkit/event/player/PlayerFishEvent$State;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
crate::event::player::PlayerFishEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn state(&mut self) 
-> Result<crate::event::player::PlayerFishEventState<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getState","()Lorg/bukkit/event/player/PlayerFishEvent$State;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::player::PlayerFishEventState::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn hand(&mut self) 
-> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHand","()Lorg/bukkit/inventory/EquipmentSlot;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(),raw_obj
, crate::inventory::EquipmentSlot::from_string(variant_str).unwrap()
)}
	pub fn caught(&mut self) 
-> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCaught","()Lorg/bukkit/entity/Entity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Entity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn hook(&mut self) 
-> Result<crate::entity::FishHook<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHook","()Lorg/bukkit/entity/FishHook;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::FishHook::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn exp_to_drop(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getExpToDrop","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn set_exp_to_drop(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setExpToDrop","(I)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerFishEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerFishEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerShearEntityEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerShearEntityEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerShearEntityEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerShearEntityEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerShearEntityEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerShearEntityEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_player(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: std::option::Option<impl Into<&'mc crate::entity::Entity<'mc>>>,arg2: std::option::Option<impl Into<&'mc crate::inventory::ItemStack<'mc>>>,arg3: std::option::Option<impl Into<&'mc crate::inventory::EquipmentSlot<'mc>>>) 
-> Result<crate::event::player::PlayerShearEntityEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerShearEntityEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/entity/Entity;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/EquipmentSlot;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
crate::event::player::PlayerShearEntityEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn item(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getItem","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn entity(&mut self) 
-> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/Entity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Entity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn hand(&mut self) 
-> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHand","()Lorg/bukkit/inventory/EquipmentSlot;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(),raw_obj
, crate::inventory::EquipmentSlot::from_string(variant_str).unwrap()
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerShearEntityEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerShearEntityEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerInteractEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerInteractEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerInteractEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerInteractEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerInteractEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerInteractEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_player(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::event::block::Action<'mc>>,arg2: impl Into<&'mc crate::inventory::ItemStack<'mc>>,arg3: impl Into<&'mc crate::block::Block<'mc>>,arg4: std::option::Option<impl Into<&'mc crate::block::BlockFace<'mc>>>,arg5: std::option::Option<impl Into<&'mc crate::inventory::EquipmentSlot<'mc>>>,arg6: std::option::Option<impl Into<&'mc crate::util::Vector<'mc>>>) 
-> Result<crate::event::player::PlayerInteractEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone())};
let val_4 = unsafe { jni::objects::JObject::from_raw(arg4.unwrap().into().jni_object().clone())};
let val_5 = unsafe { jni::objects::JObject::from_raw(arg5.unwrap().into().jni_object().clone())};
let val_6 = unsafe { jni::objects::JObject::from_raw(arg6.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerInteractEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/event/block/Action;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/block/Block;Lorg/bukkit/block/BlockFace;Lorg/bukkit/inventory/EquipmentSlot;Lorg/bukkit/util/Vector;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6)])?;
crate::event::player::PlayerInteractEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn item(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getItem","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn material(&mut self) 
-> Result<crate::Material<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getMaterial","()Lorg/bukkit/Material;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::Material::from_raw(&self.jni_ref(),raw_obj
, crate::Material::from_string(variant_str).unwrap()
)}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn hand(&mut self) 
-> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHand","()Lorg/bukkit/inventory/EquipmentSlot;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(),raw_obj
, crate::inventory::EquipmentSlot::from_string(variant_str).unwrap()
)}
	pub fn block_face(&mut self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBlockFace","()Lorg/bukkit/block/BlockFace;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::block::BlockFace::from_raw(&self.jni_ref(),raw_obj
, crate::block::BlockFace::from_string(variant_str).unwrap()
)}
	pub fn action(&mut self) 
-> Result<crate::event::block::Action<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getAction","()Lorg/bukkit/event/block/Action;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::event::block::Action::from_raw(&self.jni_ref(),raw_obj
, crate::event::block::Action::from_string(variant_str).unwrap()
)}
	pub fn use_interacted_block(&mut self) 
-> Result<crate::event::EventResult<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"useInteractedBlock","()Lorg/bukkit/event/Event$Result;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::EventResult::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_use_interacted_block(&mut self,arg0: impl Into<&'mc crate::event::EventResult<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setUseInteractedBlock","(Lorg/bukkit/event/Event$Result;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn set_use_item_in_hand(&mut self,arg0: impl Into<&'mc crate::event::EventResult<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setUseItemInHand","(Lorg/bukkit/event/Event$Result;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn has_item(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hasItem","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn has_block(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hasBlock","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn is_block_in_hand(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isBlockInHand","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn clicked_block(&mut self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClickedBlock","()Lorg/bukkit/block/Block;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn clicked_position(&mut self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClickedPosition","()Lorg/bukkit/util/Vector;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn use_item_in_hand(&mut self) 
-> Result<crate::event::EventResult<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"useItemInHand","()Lorg/bukkit/event/Event$Result;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::EventResult::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerInteractEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerInteractEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct AsyncPlayerChatPreviewEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for AsyncPlayerChatPreviewEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> AsyncPlayerChatPreviewEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate AsyncPlayerChatPreviewEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "AsyncPlayerChatPreviewEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a AsyncPlayerChatPreviewEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: bool,arg1: impl Into<&'mc crate::entity::Player<'mc>>,arg2: impl Into<&'mc String>,arg3: impl Into<&'mc blackboxmc_java::JavaSet<'mc, orgPlayer>>) 
-> Result<crate::event::player::AsyncPlayerChatPreviewEvent<'mc>, Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = jni::objects::JObject::from(jni.new_string(arg2.into()).unwrap());
let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/AsyncPlayerChatPreviewEvent")?;
let res = jni.new_object(cls,
"(ZLorg/bukkit/entity/Player;Ljava/lang/String;Ljava/util/Set;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
crate::event::player::AsyncPlayerChatPreviewEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn message(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getMessage","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn set_message(&mut self,arg0: impl Into<&'mc String>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"setMessage","(Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn format(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getFormat","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn set_format(&mut self,arg0: impl Into<&'mc String>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"setFormat","(Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn recipients(&mut self) 
-> Result<blackboxmc_java::JavaSet<'mc, orgPlayer>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getRecipients","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::player::AsyncPlayerChatEvent<'mc>> for AsyncPlayerChatPreviewEvent<'mc>{
   fn into(self) -> crate::event::player::AsyncPlayerChatEvent<'mc> {
       crate::event::player::AsyncPlayerChatEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerAnimationEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerAnimationEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerAnimationEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerAnimationEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerAnimationEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerAnimationEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_player(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc crate::entity::Player<'mc>>>,arg1: std::option::Option<impl Into<&'mc crate::event::player::PlayerAnimationType<'mc>>>) 
-> Result<crate::event::player::PlayerAnimationEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerAnimationEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/event/player/PlayerAnimationType;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::player::PlayerAnimationEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn animation_type(&mut self) 
-> Result<crate::event::player::PlayerAnimationType<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getAnimationType","()Lorg/bukkit/event/player/PlayerAnimationType;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::event::player::PlayerAnimationType::from_raw(&self.jni_ref(),raw_obj
, crate::event::player::PlayerAnimationType::from_string(variant_str).unwrap()
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerAnimationEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerAnimationEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerBucketEntityEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerBucketEntityEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerBucketEntityEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerBucketEntityEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerBucketEntityEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerBucketEntityEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::entity::Entity<'mc>>,arg2: impl Into<&'mc crate::inventory::ItemStack<'mc>>,arg3: impl Into<&'mc crate::inventory::ItemStack<'mc>>,arg4: impl Into<&'mc crate::inventory::EquipmentSlot<'mc>>) 
-> Result<crate::event::player::PlayerBucketEntityEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone())};
let val_4 = unsafe { jni::objects::JObject::from_raw(arg4.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerBucketEntityEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/entity/Entity;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/EquipmentSlot;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
crate::event::player::PlayerBucketEntityEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn entity(&mut self) 
-> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/Entity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Entity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn hand(&mut self) 
-> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHand","()Lorg/bukkit/inventory/EquipmentSlot;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(),raw_obj
, crate::inventory::EquipmentSlot::from_string(variant_str).unwrap()
)}
	pub fn original_bucket(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getOriginalBucket","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn entity_bucket(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntityBucket","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerBucketEntityEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerBucketEntityEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerPortalEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerPortalEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerPortalEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerPortalEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerPortalEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerPortalEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_player(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::Location<'mc>>,arg2: std::option::Option<impl Into<&'mc crate::Location<'mc>>>,arg3: std::option::Option<impl Into<&'mc crate::event::player::PlayerTeleportEventTeleportCause<'mc>>>,arg4: std::option::Option<i32>,arg5: std::option::Option<bool>,arg6: std::option::Option<i32>) 
-> Result<crate::event::player::PlayerPortalEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.unwrap().into().jni_object().clone())};
let val_4 = jni::objects::JValueGen::Int(arg4.unwrap().into());
// 2
let val_5 = jni::objects::JValueGen::Bool(arg5.unwrap().into());
let val_6 = jni::objects::JValueGen::Int(arg6.unwrap().into());
let cls = &jni.find_class("org/bukkit/event/player/PlayerPortalEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/Location;Lorg/bukkit/Location;Lorg/bukkit/event/player/PlayerTeleportEvent$TeleportCause;IZI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6)])?;
crate::event::player::PlayerPortalEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn set_search_radius(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setSearchRadius","(I)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn search_radius(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getSearchRadius","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn can_create_portal(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCanCreatePortal","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_can_create_portal(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCanCreatePortal","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn set_creation_radius(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCreationRadius","(I)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn creation_radius(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCreationRadius","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn cause(&mut self) 
-> Result<crate::event::player::PlayerTeleportEventTeleportCause<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCause","()Lorg/bukkit/event/player/PlayerTeleportEvent$TeleportCause;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::player::PlayerTeleportEventTeleportCause::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn from(&mut self) 
-> Result<crate::Location<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getFrom","()Lorg/bukkit/Location;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_from(&mut self,arg0: impl Into<&'mc crate::Location<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setFrom","(Lorg/bukkit/Location;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn to(&mut self) 
-> Result<crate::Location<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getTo","()Lorg/bukkit/Location;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_to(&mut self,arg0: impl Into<&'mc crate::Location<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setTo","(Lorg/bukkit/Location;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::player::PlayerTeleportEvent<'mc>> for PlayerPortalEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerTeleportEvent<'mc> {
       crate::event::player::PlayerTeleportEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerDropItemEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerDropItemEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerDropItemEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerDropItemEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerDropItemEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerDropItemEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::entity::Item<'mc>>) 
-> Result<crate::event::player::PlayerDropItemEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerDropItemEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/entity/Item;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::player::PlayerDropItemEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn item_drop(&mut self) 
-> Result<crate::entity::Item<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getItemDrop","()Lorg/bukkit/entity/Item;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Item::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerDropItemEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerDropItemEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerResourcePackStatusEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
pub struct PlayerResourcePackStatusEventStatus<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerResourcePackStatusEventStatus<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerResourcePackStatusEventStatus<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerResourcePackStatusEventStatus from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerResourcePackStatusEventStatus")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerResourcePackStatusEventStatus object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn value_of_with_string(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<jni::objects::JClass<'mc>>,arg1: std::option::Option<impl Into<&'mc String>>) 
-> Result<blackboxmc_java::JavaEnum<'mc>, Box<dyn std::error::Error>>

{let val_0 = arg0.unwrap();
let val_1 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
let cls = &jni.find_class("java/lang/Enum")?;
let res = jni.call_static_method(cls,"valueOf",
"(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
blackboxmc_java::JavaEnum::from_raw(&jni,obj
)}
	pub fn name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"name","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn describe_constable(&mut self) 
-> Result<blackboxmc_java::JavaOptional<'mc, javaEnumEnumDesc<E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"describeConstable","()Ljava/util/Optional;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaOptional::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn declaring_class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDeclaringClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn ordinal(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"ordinal","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerResourcePackStatusEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerResourcePackStatusEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerResourcePackStatusEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerResourcePackStatusEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerResourcePackStatusEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::event::player::PlayerResourcePackStatusEventStatus<'mc>>) 
-> Result<crate::event::player::PlayerResourcePackStatusEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerResourcePackStatusEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/event/player/PlayerResourcePackStatusEvent$Status;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::player::PlayerResourcePackStatusEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn status(&mut self) 
-> Result<crate::event::player::PlayerResourcePackStatusEventStatus<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getStatus","()Lorg/bukkit/event/player/PlayerResourcePackStatusEvent$Status;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::player::PlayerResourcePackStatusEventStatus::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerResourcePackStatusEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerExpCooldownChangeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
pub struct PlayerExpCooldownChangeEventChangeReason<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerExpCooldownChangeEventChangeReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerExpCooldownChangeEventChangeReason<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerExpCooldownChangeEventChangeReason from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerExpCooldownChangeEventChangeReason")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerExpCooldownChangeEventChangeReason object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn value_of_with_string(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<jni::objects::JClass<'mc>>,arg1: std::option::Option<impl Into<&'mc String>>) 
-> Result<blackboxmc_java::JavaEnum<'mc>, Box<dyn std::error::Error>>

{let val_0 = arg0.unwrap();
let val_1 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
let cls = &jni.find_class("java/lang/Enum")?;
let res = jni.call_static_method(cls,"valueOf",
"(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
blackboxmc_java::JavaEnum::from_raw(&jni,obj
)}
	pub fn name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"name","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn describe_constable(&mut self) 
-> Result<blackboxmc_java::JavaOptional<'mc, javaEnumEnumDesc<E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"describeConstable","()Ljava/util/Optional;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaOptional::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn declaring_class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDeclaringClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn ordinal(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"ordinal","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerExpCooldownChangeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerExpCooldownChangeEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerExpCooldownChangeEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerExpCooldownChangeEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerExpCooldownChangeEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: i32,arg2: impl Into<&'mc crate::event::player::PlayerExpCooldownChangeEventChangeReason<'mc>>) 
-> Result<crate::event::player::PlayerExpCooldownChangeEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = jni::objects::JValueGen::Int(arg1.into());
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerExpCooldownChangeEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;ILorg/bukkit/event/player/PlayerExpCooldownChangeEvent$ChangeReason;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::player::PlayerExpCooldownChangeEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn reason(&mut self) 
-> Result<crate::event::player::PlayerExpCooldownChangeEventChangeReason<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getReason","()Lorg/bukkit/event/player/PlayerExpCooldownChangeEvent$ChangeReason;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::player::PlayerExpCooldownChangeEventChangeReason::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn new_cooldown(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getNewCooldown","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn set_new_cooldown(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setNewCooldown","(I)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerExpCooldownChangeEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerUnregisterChannelEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerUnregisterChannelEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerUnregisterChannelEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerUnregisterChannelEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerUnregisterChannelEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerUnregisterChannelEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc String>) 
-> Result<crate::event::player::PlayerUnregisterChannelEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = jni::objects::JObject::from(jni.new_string(arg1.into()).unwrap());
let cls = &jni.find_class("org/bukkit/event/player/PlayerUnregisterChannelEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::player::PlayerUnregisterChannelEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn channel(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getChannel","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::player::PlayerChannelEvent<'mc>> for PlayerUnregisterChannelEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerChannelEvent<'mc> {
       crate::event::player::PlayerChannelEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerToggleSneakEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerToggleSneakEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerToggleSneakEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerToggleSneakEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerToggleSneakEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerToggleSneakEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: bool) 
-> Result<crate::event::player::PlayerToggleSneakEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
// -2
let val_1 = jni::objects::JValueGen::Bool(arg1.into());
let cls = &jni.find_class("org/bukkit/event/player/PlayerToggleSneakEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Z)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::player::PlayerToggleSneakEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn is_sneaking(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isSneaking","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerToggleSneakEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerToggleSneakEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerArmorStandManipulateEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerArmorStandManipulateEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerArmorStandManipulateEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerArmorStandManipulateEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerArmorStandManipulateEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerArmorStandManipulateEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_player(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::entity::ArmorStand<'mc>>,arg2: impl Into<&'mc crate::inventory::ItemStack<'mc>>,arg3: impl Into<&'mc crate::inventory::ItemStack<'mc>>,arg4: std::option::Option<impl Into<&'mc crate::inventory::EquipmentSlot<'mc>>>,arg5: std::option::Option<impl Into<&'mc crate::inventory::EquipmentSlot<'mc>>>) 
-> Result<crate::event::player::PlayerArmorStandManipulateEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone())};
let val_4 = unsafe { jni::objects::JObject::from_raw(arg4.unwrap().into().jni_object().clone())};
let val_5 = unsafe { jni::objects::JObject::from_raw(arg5.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerArmorStandManipulateEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/entity/ArmorStand;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/EquipmentSlot;Lorg/bukkit/inventory/EquipmentSlot;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5)])?;
crate::event::player::PlayerArmorStandManipulateEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn slot(&mut self) 
-> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getSlot","()Lorg/bukkit/inventory/EquipmentSlot;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(),raw_obj
, crate::inventory::EquipmentSlot::from_string(variant_str).unwrap()
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn hand(&mut self) 
-> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHand","()Lorg/bukkit/inventory/EquipmentSlot;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(),raw_obj
, crate::inventory::EquipmentSlot::from_string(variant_str).unwrap()
)}
	pub fn right_clicked(&mut self) 
-> Result<crate::entity::ArmorStand<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getRightClicked","()Lorg/bukkit/entity/ArmorStand;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::ArmorStand::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn player_item(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayerItem","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn armor_stand_item(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getArmorStandItem","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::player::PlayerInteractEntityEvent<'mc>> for PlayerArmorStandManipulateEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerInteractEntityEvent<'mc> {
       crate::event::player::PlayerInteractEntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerInteractEntityEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerInteractEntityEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerInteractEntityEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerInteractEntityEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerInteractEntityEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerInteractEntityEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_player(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: std::option::Option<impl Into<&'mc crate::entity::Entity<'mc>>>,arg2: std::option::Option<impl Into<&'mc crate::inventory::EquipmentSlot<'mc>>>) 
-> Result<crate::event::player::PlayerInteractEntityEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerInteractEntityEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/entity/Entity;Lorg/bukkit/inventory/EquipmentSlot;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::player::PlayerInteractEntityEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn hand(&mut self) 
-> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHand","()Lorg/bukkit/inventory/EquipmentSlot;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(),raw_obj
, crate::inventory::EquipmentSlot::from_string(variant_str).unwrap()
)}
	pub fn right_clicked(&mut self) 
-> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getRightClicked","()Lorg/bukkit/entity/Entity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Entity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerInteractEntityEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerInteractEntityEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerBucketFillEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerBucketFillEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerBucketFillEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerBucketFillEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerBucketFillEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerBucketFillEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_player(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::block::Block<'mc>>,arg2: impl Into<&'mc crate::block::Block<'mc>>,arg3: impl Into<&'mc crate::block::BlockFace<'mc>>,arg4: impl Into<&'mc crate::Material<'mc>>,arg5: impl Into<&'mc crate::inventory::ItemStack<'mc>>,arg6: std::option::Option<impl Into<&'mc crate::inventory::EquipmentSlot<'mc>>>) 
-> Result<crate::event::player::PlayerBucketFillEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone())};
let val_4 = unsafe { jni::objects::JObject::from_raw(arg4.into().jni_object().clone())};
let val_5 = unsafe { jni::objects::JObject::from_raw(arg5.into().jni_object().clone())};
let val_6 = unsafe { jni::objects::JObject::from_raw(arg6.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerBucketFillEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/block/Block;Lorg/bukkit/block/Block;Lorg/bukkit/block/BlockFace;Lorg/bukkit/Material;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/EquipmentSlot;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6)])?;
crate::event::player::PlayerBucketFillEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn block(&mut self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBlock","()Lorg/bukkit/block/Block;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn item_stack(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getItemStack","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_item_stack(&mut self,arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setItemStack","(Lorg/bukkit/inventory/ItemStack;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn hand(&mut self) 
-> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHand","()Lorg/bukkit/inventory/EquipmentSlot;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(),raw_obj
, crate::inventory::EquipmentSlot::from_string(variant_str).unwrap()
)}
	pub fn block_face(&mut self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBlockFace","()Lorg/bukkit/block/BlockFace;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::block::BlockFace::from_raw(&self.jni_ref(),raw_obj
, crate::block::BlockFace::from_string(variant_str).unwrap()
)}
	pub fn bucket(&mut self) 
-> Result<crate::Material<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBucket","()Lorg/bukkit/Material;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::Material::from_raw(&self.jni_ref(),raw_obj
, crate::Material::from_string(variant_str).unwrap()
)}
	pub fn block_clicked(&mut self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBlockClicked","()Lorg/bukkit/block/Block;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::player::PlayerBucketEvent<'mc>> for PlayerBucketFillEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerBucketEvent<'mc> {
       crate::event::player::PlayerBucketEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerItemConsumeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerItemConsumeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerItemConsumeEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerItemConsumeEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerItemConsumeEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerItemConsumeEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_player(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: std::option::Option<impl Into<&'mc crate::inventory::ItemStack<'mc>>>,arg2: std::option::Option<impl Into<&'mc crate::inventory::EquipmentSlot<'mc>>>) 
-> Result<crate::event::player::PlayerItemConsumeEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerItemConsumeEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/EquipmentSlot;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::player::PlayerItemConsumeEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn item(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getItem","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_item(&mut self,arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setItem","(Lorg/bukkit/inventory/ItemStack;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn hand(&mut self) 
-> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHand","()Lorg/bukkit/inventory/EquipmentSlot;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(),raw_obj
, crate::inventory::EquipmentSlot::from_string(variant_str).unwrap()
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerItemConsumeEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerItemConsumeEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>) 
-> Result<crate::event::player::PlayerEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;)V",&[jni::objects::JValueGen::from(&val_0)])?;
crate::event::player::PlayerEvent::from_raw(&jni,res
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Event<'mc>> for PlayerEvent<'mc>{
   fn into(self) -> crate::event::Event<'mc> {
       crate::event::Event::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerTeleportEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
pub struct PlayerTeleportEventTeleportCause<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerTeleportEventTeleportCause<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerTeleportEventTeleportCause<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerTeleportEventTeleportCause from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerTeleportEventTeleportCause")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerTeleportEventTeleportCause object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn value_of_with_string(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<jni::objects::JClass<'mc>>,arg1: std::option::Option<impl Into<&'mc String>>) 
-> Result<blackboxmc_java::JavaEnum<'mc>, Box<dyn std::error::Error>>

{let val_0 = arg0.unwrap();
let val_1 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
let cls = &jni.find_class("java/lang/Enum")?;
let res = jni.call_static_method(cls,"valueOf",
"(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
blackboxmc_java::JavaEnum::from_raw(&jni,obj
)}
	pub fn name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"name","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn describe_constable(&mut self) 
-> Result<blackboxmc_java::JavaOptional<'mc, javaEnumEnumDesc<E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"describeConstable","()Ljava/util/Optional;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaOptional::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn declaring_class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDeclaringClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn ordinal(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"ordinal","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerTeleportEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerTeleportEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerTeleportEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerTeleportEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerTeleportEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_player(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::Location<'mc>>,arg2: std::option::Option<impl Into<&'mc crate::Location<'mc>>>,arg3: std::option::Option<impl Into<&'mc crate::event::player::PlayerTeleportEventTeleportCause<'mc>>>) 
-> Result<crate::event::player::PlayerTeleportEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerTeleportEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/Location;Lorg/bukkit/Location;Lorg/bukkit/event/player/PlayerTeleportEvent$TeleportCause;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
crate::event::player::PlayerTeleportEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn cause(&mut self) 
-> Result<crate::event::player::PlayerTeleportEventTeleportCause<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCause","()Lorg/bukkit/event/player/PlayerTeleportEvent$TeleportCause;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::player::PlayerTeleportEventTeleportCause::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn from(&mut self) 
-> Result<crate::Location<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getFrom","()Lorg/bukkit/Location;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_from(&mut self,arg0: impl Into<&'mc crate::Location<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setFrom","(Lorg/bukkit/Location;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn to(&mut self) 
-> Result<crate::Location<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getTo","()Lorg/bukkit/Location;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_to(&mut self,arg0: impl Into<&'mc crate::Location<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setTo","(Lorg/bukkit/Location;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::player::PlayerMoveEvent<'mc>> for PlayerTeleportEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerMoveEvent<'mc> {
       crate::event::player::PlayerMoveEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerQuitEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerQuitEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerQuitEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerQuitEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerQuitEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerQuitEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc String>) 
-> Result<crate::event::player::PlayerQuitEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = jni::objects::JObject::from(jni.new_string(arg1.into()).unwrap());
let cls = &jni.find_class("org/bukkit/event/player/PlayerQuitEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::player::PlayerQuitEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn quit_message(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getQuitMessage","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn set_quit_message(&mut self,arg0: impl Into<&'mc String>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"setQuitMessage","(Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerQuitEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerMoveEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerMoveEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerMoveEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerMoveEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerMoveEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerMoveEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::Location<'mc>>,arg2: impl Into<&'mc crate::Location<'mc>>) 
-> Result<crate::event::player::PlayerMoveEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerMoveEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/Location;Lorg/bukkit/Location;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::player::PlayerMoveEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn from(&mut self) 
-> Result<crate::Location<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getFrom","()Lorg/bukkit/Location;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_from(&mut self,arg0: impl Into<&'mc crate::Location<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setFrom","(Lorg/bukkit/Location;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn to(&mut self) 
-> Result<crate::Location<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getTo","()Lorg/bukkit/Location;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_to(&mut self,arg0: impl Into<&'mc crate::Location<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setTo","(Lorg/bukkit/Location;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerMoveEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerMoveEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerEditBookEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerEditBookEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerEditBookEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerEditBookEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerEditBookEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerEditBookEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: i32,arg2: impl Into<&'mc crate::inventory::meta::BookMeta<'mc>>,arg3: impl Into<&'mc crate::inventory::meta::BookMeta<'mc>>,arg4: bool) 
-> Result<crate::event::player::PlayerEditBookEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = jni::objects::JValueGen::Int(arg1.into());
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone())};
// -2
let val_4 = jni::objects::JValueGen::Bool(arg4.into());
let cls = &jni.find_class("org/bukkit/event/player/PlayerEditBookEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;ILorg/bukkit/inventory/meta/BookMeta;Lorg/bukkit/inventory/meta/BookMeta;Z)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
crate::event::player::PlayerEditBookEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]
	pub fn slot(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getSlot","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn is_signing(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isSigning","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn previous_book_meta(&mut self) 
-> Result<crate::inventory::meta::BookMeta<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPreviousBookMeta","()Lorg/bukkit/inventory/meta/BookMeta;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::meta::BookMeta::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn new_book_meta(&mut self) 
-> Result<crate::inventory::meta::BookMeta<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getNewBookMeta","()Lorg/bukkit/inventory/meta/BookMeta;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::meta::BookMeta::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_new_book_meta(&mut self,arg0: impl Into<&'mc crate::inventory::meta::BookMeta<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setNewBookMeta","(Lorg/bukkit/inventory/meta/BookMeta;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn set_signing(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setSigning","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerEditBookEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerEditBookEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerJoinEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerJoinEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerJoinEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerJoinEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerJoinEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerJoinEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc String>) 
-> Result<crate::event::player::PlayerJoinEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = jni::objects::JObject::from(jni.new_string(arg1.into()).unwrap());
let cls = &jni.find_class("org/bukkit/event/player/PlayerJoinEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::player::PlayerJoinEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn join_message(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getJoinMessage","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn set_join_message(&mut self,arg0: impl Into<&'mc String>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"setJoinMessage","(Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerJoinEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerItemHeldEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerItemHeldEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerItemHeldEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerItemHeldEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerItemHeldEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerItemHeldEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: i32,arg2: i32) 
-> Result<crate::event::player::PlayerItemHeldEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = jni::objects::JValueGen::Int(arg1.into());
let val_2 = jni::objects::JValueGen::Int(arg2.into());
let cls = &jni.find_class("org/bukkit/event/player/PlayerItemHeldEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;II)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::player::PlayerItemHeldEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn previous_slot(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPreviousSlot","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn new_slot(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getNewSlot","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerItemHeldEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerItemHeldEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerLevelChangeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerLevelChangeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerLevelChangeEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerLevelChangeEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerLevelChangeEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerLevelChangeEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: i32,arg2: i32) 
-> Result<crate::event::player::PlayerLevelChangeEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = jni::objects::JValueGen::Int(arg1.into());
let val_2 = jni::objects::JValueGen::Int(arg2.into());
let cls = &jni.find_class("org/bukkit/event/player/PlayerLevelChangeEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;II)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::player::PlayerLevelChangeEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn new_level(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getNewLevel","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn old_level(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getOldLevel","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerLevelChangeEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerBedEnterEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
pub struct PlayerBedEnterEventBedEnterResult<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerBedEnterEventBedEnterResult<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerBedEnterEventBedEnterResult<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerBedEnterEventBedEnterResult from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerBedEnterEventBedEnterResult")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerBedEnterEventBedEnterResult object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn value_of_with_string(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<jni::objects::JClass<'mc>>,arg1: std::option::Option<impl Into<&'mc String>>) 
-> Result<blackboxmc_java::JavaEnum<'mc>, Box<dyn std::error::Error>>

{let val_0 = arg0.unwrap();
let val_1 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
let cls = &jni.find_class("java/lang/Enum")?;
let res = jni.call_static_method(cls,"valueOf",
"(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
blackboxmc_java::JavaEnum::from_raw(&jni,obj
)}
	pub fn name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"name","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn describe_constable(&mut self) 
-> Result<blackboxmc_java::JavaOptional<'mc, javaEnumEnumDesc<E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"describeConstable","()Ljava/util/Optional;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaOptional::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn declaring_class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDeclaringClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn ordinal(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"ordinal","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerBedEnterEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerBedEnterEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerBedEnterEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerBedEnterEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerBedEnterEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_player(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: std::option::Option<impl Into<&'mc crate::block::Block<'mc>>>,arg2: std::option::Option<impl Into<&'mc crate::event::player::PlayerBedEnterEventBedEnterResult<'mc>>>) 
-> Result<crate::event::player::PlayerBedEnterEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerBedEnterEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/block/Block;Lorg/bukkit/event/player/PlayerBedEnterEvent$BedEnterResult;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::player::PlayerBedEnterEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn bed(&mut self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBed","()Lorg/bukkit/block/Block;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn use_bed(&mut self) 
-> Result<crate::event::EventResult<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"useBed","()Lorg/bukkit/event/Event$Result;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::EventResult::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn bed_enter_result(&mut self) 
-> Result<crate::event::player::PlayerBedEnterEventBedEnterResult<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBedEnterResult","()Lorg/bukkit/event/player/PlayerBedEnterEvent$BedEnterResult;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::player::PlayerBedEnterEventBedEnterResult::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_use_bed(&mut self,arg0: impl Into<&'mc crate::event::EventResult<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setUseBed","(Lorg/bukkit/event/Event$Result;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerBedEnterEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerBedEnterEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerBucketEmptyEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerBucketEmptyEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerBucketEmptyEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerBucketEmptyEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerBucketEmptyEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerBucketEmptyEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_player(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::block::Block<'mc>>,arg2: impl Into<&'mc crate::block::Block<'mc>>,arg3: impl Into<&'mc crate::block::BlockFace<'mc>>,arg4: impl Into<&'mc crate::Material<'mc>>,arg5: impl Into<&'mc crate::inventory::ItemStack<'mc>>,arg6: std::option::Option<impl Into<&'mc crate::inventory::EquipmentSlot<'mc>>>) 
-> Result<crate::event::player::PlayerBucketEmptyEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone())};
let val_4 = unsafe { jni::objects::JObject::from_raw(arg4.into().jni_object().clone())};
let val_5 = unsafe { jni::objects::JObject::from_raw(arg5.into().jni_object().clone())};
let val_6 = unsafe { jni::objects::JObject::from_raw(arg6.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerBucketEmptyEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/block/Block;Lorg/bukkit/block/Block;Lorg/bukkit/block/BlockFace;Lorg/bukkit/Material;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/EquipmentSlot;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6)])?;
crate::event::player::PlayerBucketEmptyEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn block(&mut self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBlock","()Lorg/bukkit/block/Block;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn item_stack(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getItemStack","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_item_stack(&mut self,arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setItemStack","(Lorg/bukkit/inventory/ItemStack;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn hand(&mut self) 
-> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHand","()Lorg/bukkit/inventory/EquipmentSlot;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(),raw_obj
, crate::inventory::EquipmentSlot::from_string(variant_str).unwrap()
)}
	pub fn block_face(&mut self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBlockFace","()Lorg/bukkit/block/BlockFace;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::block::BlockFace::from_raw(&self.jni_ref(),raw_obj
, crate::block::BlockFace::from_string(variant_str).unwrap()
)}
	pub fn bucket(&mut self) 
-> Result<crate::Material<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBucket","()Lorg/bukkit/Material;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::Material::from_raw(&self.jni_ref(),raw_obj
, crate::Material::from_string(variant_str).unwrap()
)}
	pub fn block_clicked(&mut self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBlockClicked","()Lorg/bukkit/block/Block;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::player::PlayerBucketEvent<'mc>> for PlayerBucketEmptyEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerBucketEvent<'mc> {
       crate::event::player::PlayerBucketEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerBedLeaveEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerBedLeaveEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerBedLeaveEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerBedLeaveEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerBedLeaveEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerBedLeaveEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::block::Block<'mc>>,arg2: bool) 
-> Result<crate::event::player::PlayerBedLeaveEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
// -2
let val_2 = jni::objects::JValueGen::Bool(arg2.into());
let cls = &jni.find_class("org/bukkit/event/player/PlayerBedLeaveEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/block/Block;Z)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::player::PlayerBedLeaveEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_spawn_location(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setSpawnLocation","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn bed(&mut self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBed","()Lorg/bukkit/block/Block;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn should_set_spawn_location(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"shouldSetSpawnLocation","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerBedLeaveEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerBedLeaveEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerSwapHandItemsEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerSwapHandItemsEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerSwapHandItemsEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerSwapHandItemsEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerSwapHandItemsEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerSwapHandItemsEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,arg2: impl Into<&'mc crate::inventory::ItemStack<'mc>>) 
-> Result<crate::event::player::PlayerSwapHandItemsEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerSwapHandItemsEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/ItemStack;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::player::PlayerSwapHandItemsEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn main_hand_item(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getMainHandItem","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_main_hand_item(&mut self,arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setMainHandItem","(Lorg/bukkit/inventory/ItemStack;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn off_hand_item(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getOffHandItem","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_off_hand_item(&mut self,arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setOffHandItem","(Lorg/bukkit/inventory/ItemStack;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerSwapHandItemsEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerSwapHandItemsEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerSpawnChangeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
pub struct PlayerSpawnChangeEventCause<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerSpawnChangeEventCause<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerSpawnChangeEventCause<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerSpawnChangeEventCause from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerSpawnChangeEventCause")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerSpawnChangeEventCause object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn value_of_with_string(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<jni::objects::JClass<'mc>>,arg1: std::option::Option<impl Into<&'mc String>>) 
-> Result<blackboxmc_java::JavaEnum<'mc>, Box<dyn std::error::Error>>

{let val_0 = arg0.unwrap();
let val_1 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
let cls = &jni.find_class("java/lang/Enum")?;
let res = jni.call_static_method(cls,"valueOf",
"(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
blackboxmc_java::JavaEnum::from_raw(&jni,obj
)}
	pub fn name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"name","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn describe_constable(&mut self) 
-> Result<blackboxmc_java::JavaOptional<'mc, javaEnumEnumDesc<E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"describeConstable","()Ljava/util/Optional;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaOptional::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn declaring_class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDeclaringClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn ordinal(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"ordinal","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerSpawnChangeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerSpawnChangeEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerSpawnChangeEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerSpawnChangeEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerSpawnChangeEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::Location<'mc>>,arg2: bool,arg3: impl Into<&'mc crate::event::player::PlayerSpawnChangeEventCause<'mc>>) 
-> Result<crate::event::player::PlayerSpawnChangeEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
// -2
let val_2 = jni::objects::JValueGen::Bool(arg2.into());
let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerSpawnChangeEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/Location;ZLorg/bukkit/event/player/PlayerSpawnChangeEvent$Cause;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
crate::event::player::PlayerSpawnChangeEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn cause(&mut self) 
-> Result<crate::event::player::PlayerSpawnChangeEventCause<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCause","()Lorg/bukkit/event/player/PlayerSpawnChangeEvent$Cause;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::player::PlayerSpawnChangeEventCause::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn is_forced(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isForced","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_forced(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setForced","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn new_spawn(&mut self) 
-> Result<crate::Location<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getNewSpawn","()Lorg/bukkit/Location;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_new_spawn(&mut self,arg0: impl Into<&'mc crate::Location<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setNewSpawn","(Lorg/bukkit/Location;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerSpawnChangeEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerSpawnChangeEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerRiptideEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerRiptideEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerRiptideEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerRiptideEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerRiptideEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerRiptideEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>) 
-> Result<crate::event::player::PlayerRiptideEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerRiptideEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/inventory/ItemStack;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::player::PlayerRiptideEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn item(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getItem","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerRiptideEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerCommandSendEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerCommandSendEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerCommandSendEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerCommandSendEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerCommandSendEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerCommandSendEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc blackboxmc_java::JavaCollection<'mc, String>>) 
-> Result<crate::event::player::PlayerCommandSendEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerCommandSendEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Ljava/util/Collection;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::player::PlayerCommandSendEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn commands(&mut self) 
-> Result<blackboxmc_java::JavaCollection<'mc, String>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCommands","()Ljava/util/Collection;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaCollection::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerCommandSendEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerChatTabCompleteEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerChatTabCompleteEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerChatTabCompleteEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerChatTabCompleteEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerChatTabCompleteEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerChatTabCompleteEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc String>,arg2: impl Into<&'mc blackboxmc_java::JavaCollection<'mc, String>>) 
-> Result<crate::event::player::PlayerChatTabCompleteEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = jni::objects::JObject::from(jni.new_string(arg1.into()).unwrap());
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerChatTabCompleteEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Ljava/lang/String;Ljava/util/Collection;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::player::PlayerChatTabCompleteEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn chat_message(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getChatMessage","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn last_token(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getLastToken","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn tab_completions(&mut self) 
-> Result<blackboxmc_java::JavaCollection<'mc, String>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getTabCompletions","()Ljava/util/Collection;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaCollection::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerChatTabCompleteEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerVelocityEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerVelocityEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerVelocityEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerVelocityEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerVelocityEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerVelocityEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::util::Vector<'mc>>) 
-> Result<crate::event::player::PlayerVelocityEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerVelocityEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/util/Vector;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::player::PlayerVelocityEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_velocity(&mut self,arg0: impl Into<&'mc crate::util::Vector<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setVelocity","(Lorg/bukkit/util/Vector;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn velocity(&mut self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getVelocity","()Lorg/bukkit/util/Vector;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerVelocityEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerVelocityEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerToggleSprintEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerToggleSprintEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerToggleSprintEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerToggleSprintEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerToggleSprintEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerToggleSprintEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: bool) 
-> Result<crate::event::player::PlayerToggleSprintEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
// -2
let val_1 = jni::objects::JValueGen::Bool(arg1.into());
let cls = &jni.find_class("org/bukkit/event/player/PlayerToggleSprintEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Z)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::player::PlayerToggleSprintEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn is_sprinting(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isSprinting","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerToggleSprintEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerToggleSprintEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub enum PlayerAnimationTypeEnum {
	ArmSwing,
	OffArmSwing,
}
impl std::fmt::Display for PlayerAnimationTypeEnum {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match &self {
           PlayerAnimationTypeEnum::ArmSwing => f.write_str("ARM_SWING"),
           PlayerAnimationTypeEnum::OffArmSwing => f.write_str("OFF_ARM_SWING"),
       }
   }
}
pub struct PlayerAnimationType<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>, pub PlayerAnimationTypeEnum);
impl<'mc> std::ops::Deref for PlayerAnimationType<'mc> {
   type Target = PlayerAnimationTypeEnum;
   fn deref(&self) -> &Self::Target {
       return &self.2;
   }
}
impl<'mc> JNIRaw<'mc> for PlayerAnimationType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerAnimationType<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
, e: PlayerAnimationTypeEnum
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerAnimationType from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerAnimationType")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerAnimationType object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
, e
))
}
}
pub const ARM_SWING: PlayerAnimationTypeEnum = PlayerAnimationTypeEnum::ArmSwing;
pub const OFF_ARM_SWING: PlayerAnimationTypeEnum = PlayerAnimationTypeEnum::OffArmSwing;
pub fn from_string(str: String) -> std::option::Option<PlayerAnimationTypeEnum> {
match str.as_str() {
"ARM_SWING" => Some(PlayerAnimationTypeEnum::ArmSwing),
"OFF_ARM_SWING" => Some(PlayerAnimationTypeEnum::OffArmSwing),
_ => None}}
	pub fn value_of(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc String>) 
-> Result<crate::event::player::PlayerAnimationType<'mc>, Box<dyn std::error::Error>>

{let val_0 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
let cls = &jni.find_class("org/bukkit/event/player/PlayerAnimationType")?;
let res = jni.call_static_method(cls,"valueOf",
"(Ljava/lang/String;)Lorg/bukkit/event/player/PlayerAnimationType;",&[jni::objects::JValueGen::from(&val_0)])?;
let mut obj = res.l()?;
let raw_obj = obj;let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = jni    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::event::player::PlayerAnimationType::from_raw(&jni,raw_obj
, crate::event::player::PlayerAnimationType::from_string(variant_str).unwrap()
)}
}
pub struct PlayerBucketEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerBucketEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerBucketEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerBucketEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerBucketEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerBucketEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_player(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::block::Block<'mc>>,arg2: impl Into<&'mc crate::block::Block<'mc>>,arg3: impl Into<&'mc crate::block::BlockFace<'mc>>,arg4: impl Into<&'mc crate::Material<'mc>>,arg5: impl Into<&'mc crate::inventory::ItemStack<'mc>>,arg6: std::option::Option<impl Into<&'mc crate::inventory::EquipmentSlot<'mc>>>) 
-> Result<crate::event::player::PlayerBucketEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone())};
let val_4 = unsafe { jni::objects::JObject::from_raw(arg4.into().jni_object().clone())};
let val_5 = unsafe { jni::objects::JObject::from_raw(arg5.into().jni_object().clone())};
let val_6 = unsafe { jni::objects::JObject::from_raw(arg6.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerBucketEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/block/Block;Lorg/bukkit/block/Block;Lorg/bukkit/block/BlockFace;Lorg/bukkit/Material;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/EquipmentSlot;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6)])?;
crate::event::player::PlayerBucketEvent::from_raw(&jni,res
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn block(&mut self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBlock","()Lorg/bukkit/block/Block;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn item_stack(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getItemStack","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_item_stack(&mut self,arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setItemStack","(Lorg/bukkit/inventory/ItemStack;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn hand(&mut self) 
-> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHand","()Lorg/bukkit/inventory/EquipmentSlot;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(),raw_obj
, crate::inventory::EquipmentSlot::from_string(variant_str).unwrap()
)}
	pub fn block_face(&mut self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBlockFace","()Lorg/bukkit/block/BlockFace;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::block::BlockFace::from_raw(&self.jni_ref(),raw_obj
, crate::block::BlockFace::from_string(variant_str).unwrap()
)}
	pub fn bucket(&mut self) 
-> Result<crate::Material<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBucket","()Lorg/bukkit/Material;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::Material::from_raw(&self.jni_ref(),raw_obj
, crate::Material::from_string(variant_str).unwrap()
)}
	pub fn block_clicked(&mut self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBlockClicked","()Lorg/bukkit/block/Block;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerBucketEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerBucketEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerRespawnEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
pub struct PlayerRespawnEventRespawnReason<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerRespawnEventRespawnReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerRespawnEventRespawnReason<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerRespawnEventRespawnReason from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerRespawnEventRespawnReason")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerRespawnEventRespawnReason object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn value_of_with_string(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<jni::objects::JClass<'mc>>,arg1: std::option::Option<impl Into<&'mc String>>) 
-> Result<blackboxmc_java::JavaEnum<'mc>, Box<dyn std::error::Error>>

{let val_0 = arg0.unwrap();
let val_1 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
let cls = &jni.find_class("java/lang/Enum")?;
let res = jni.call_static_method(cls,"valueOf",
"(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
blackboxmc_java::JavaEnum::from_raw(&jni,obj
)}
	pub fn name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"name","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn describe_constable(&mut self) 
-> Result<blackboxmc_java::JavaOptional<'mc, javaEnumEnumDesc<E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"describeConstable","()Ljava/util/Optional;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaOptional::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn declaring_class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDeclaringClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn ordinal(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"ordinal","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerRespawnEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerRespawnEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerRespawnEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerRespawnEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerRespawnEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_player(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::Location<'mc>>,arg2: std::option::Option<bool>,arg3: std::option::Option<bool>,arg4: std::option::Option<impl Into<&'mc crate::event::player::PlayerRespawnEventRespawnReason<'mc>>>) 
-> Result<crate::event::player::PlayerRespawnEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
// 2
let val_2 = jni::objects::JValueGen::Bool(arg2.unwrap().into());
// 2
let val_3 = jni::objects::JValueGen::Bool(arg3.unwrap().into());
let val_4 = unsafe { jni::objects::JObject::from_raw(arg4.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerRespawnEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/Location;ZZLorg/bukkit/event/player/PlayerRespawnEvent$RespawnReason;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
crate::event::player::PlayerRespawnEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn is_bed_spawn(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isBedSpawn","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn is_anchor_spawn(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAnchorSpawn","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn respawn_location(&mut self) 
-> Result<crate::Location<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getRespawnLocation","()Lorg/bukkit/Location;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_respawn_location(&mut self,arg0: impl Into<&'mc crate::Location<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setRespawnLocation","(Lorg/bukkit/Location;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn respawn_reason(&mut self) 
-> Result<crate::event::player::PlayerRespawnEventRespawnReason<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getRespawnReason","()Lorg/bukkit/event/player/PlayerRespawnEvent$RespawnReason;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::player::PlayerRespawnEventRespawnReason::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerRespawnEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerUnleashEntityEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerUnleashEntityEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerUnleashEntityEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerUnleashEntityEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerUnleashEntityEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerUnleashEntityEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_entity(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Entity<'mc>>,arg1: std::option::Option<impl Into<&'mc crate::entity::Player<'mc>>>,arg2: std::option::Option<impl Into<&'mc crate::inventory::EquipmentSlot<'mc>>>) 
-> Result<crate::event::player::PlayerUnleashEntityEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerUnleashEntityEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Entity;Lorg/bukkit/entity/Player;Lorg/bukkit/inventory/EquipmentSlot;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::player::PlayerUnleashEntityEvent::from_raw(&jni,res
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn hand(&mut self) 
-> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHand","()Lorg/bukkit/inventory/EquipmentSlot;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(),raw_obj
, crate::inventory::EquipmentSlot::from_string(variant_str).unwrap()
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn reason(&mut self) 
-> Result<crate::event::entity::EntityUnleashEventUnleashReason<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getReason","()Lorg/bukkit/event/entity/EntityUnleashEvent$UnleashReason;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::entity::EntityUnleashEventUnleashReason::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn entity(&mut self) 
-> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/Entity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Entity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn entity_type(&mut self) 
-> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntityType","()Lorg/bukkit/entity/EntityType;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::entity::EntityType::from_raw(&self.jni_ref(),raw_obj
, crate::entity::EntityType::from_string(variant_str).unwrap()
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerUnleashEntityEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityUnleashEvent<'mc>> for PlayerUnleashEntityEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityUnleashEvent<'mc> {
       crate::event::entity::EntityUnleashEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerPickupArrowEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerPickupArrowEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerPickupArrowEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerPickupArrowEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerPickupArrowEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerPickupArrowEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::entity::Item<'mc>>,arg2: impl Into<&'mc crate::entity::AbstractArrow<'mc>>) 
-> Result<crate::event::player::PlayerPickupArrowEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerPickupArrowEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/entity/Item;Lorg/bukkit/entity/AbstractArrow;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::player::PlayerPickupArrowEvent::from_raw(&jni,res
)}
	pub fn arrow(&mut self) 
-> Result<crate::entity::AbstractArrow<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getArrow","()Lorg/bukkit/entity/AbstractArrow;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::AbstractArrow::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn remaining(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getRemaining","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn item(&mut self) 
-> Result<crate::entity::Item<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getItem","()Lorg/bukkit/entity/Item;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Item::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::player::PlayerPickupItemEvent<'mc>> for PlayerPickupArrowEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerPickupItemEvent<'mc> {
       crate::event::player::PlayerPickupItemEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerKickEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerKickEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerKickEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerKickEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerKickEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerKickEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc String>,arg2: impl Into<&'mc String>) 
-> Result<crate::event::player::PlayerKickEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = jni::objects::JObject::from(jni.new_string(arg1.into()).unwrap());
let val_2 = jni::objects::JObject::from(jni.new_string(arg2.into()).unwrap());
let cls = &jni.find_class("org/bukkit/event/player/PlayerKickEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Ljava/lang/String;Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::player::PlayerKickEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn reason(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getReason","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_reason(&mut self,arg0: impl Into<&'mc String>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"setReason","(Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn leave_message(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getLeaveMessage","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn set_leave_message(&mut self,arg0: impl Into<&'mc String>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"setLeaveMessage","(Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerKickEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerKickEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerGameModeChangeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerGameModeChangeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerGameModeChangeEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerGameModeChangeEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerGameModeChangeEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerGameModeChangeEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::GameMode<'mc>>) 
-> Result<crate::event::player::PlayerGameModeChangeEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerGameModeChangeEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/GameMode;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::player::PlayerGameModeChangeEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn new_game_mode(&mut self) 
-> Result<crate::GameMode<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getNewGameMode","()Lorg/bukkit/GameMode;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::GameMode::from_raw(&self.jni_ref(),raw_obj
, crate::GameMode::from_string(variant_str).unwrap()
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerGameModeChangeEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerGameModeChangeEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerTakeLecternBookEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerTakeLecternBookEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerTakeLecternBookEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerTakeLecternBookEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerTakeLecternBookEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerTakeLecternBookEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::block::Lectern<'mc>>) 
-> Result<crate::event::player::PlayerTakeLecternBookEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerTakeLecternBookEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/block/Lectern;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::player::PlayerTakeLecternBookEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn lectern(&mut self) 
-> Result<crate::block::Lectern<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getLectern","()Lorg/bukkit/block/Lectern;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Lectern::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn book(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBook","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerTakeLecternBookEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerTakeLecternBookEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerRegisterChannelEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerRegisterChannelEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerRegisterChannelEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerRegisterChannelEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerRegisterChannelEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerRegisterChannelEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc String>) 
-> Result<crate::event::player::PlayerRegisterChannelEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = jni::objects::JObject::from(jni.new_string(arg1.into()).unwrap());
let cls = &jni.find_class("org/bukkit/event/player/PlayerRegisterChannelEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::player::PlayerRegisterChannelEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn channel(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getChannel","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::player::PlayerChannelEvent<'mc>> for PlayerRegisterChannelEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerChannelEvent<'mc> {
       crate::event::player::PlayerChannelEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerStatisticIncrementEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerStatisticIncrementEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerStatisticIncrementEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerStatisticIncrementEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerStatisticIncrementEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerStatisticIncrementEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_player(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::Statistic<'mc>>,arg2: i32,arg3: std::option::Option<i32>,arg4: std::option::Option<impl Into<&'mc crate::entity::EntityType<'mc>>>) 
-> Result<crate::event::player::PlayerStatisticIncrementEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = jni::objects::JValueGen::Int(arg2.into());
let val_3 = jni::objects::JValueGen::Int(arg3.unwrap().into());
let val_4 = unsafe { jni::objects::JObject::from_raw(arg4.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerStatisticIncrementEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/Statistic;IILorg/bukkit/entity/EntityType;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
crate::event::player::PlayerStatisticIncrementEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn material(&mut self) 
-> Result<crate::Material<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getMaterial","()Lorg/bukkit/Material;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::Material::from_raw(&self.jni_ref(),raw_obj
, crate::Material::from_string(variant_str).unwrap()
)}
	pub fn statistic(&mut self) 
-> Result<crate::Statistic<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getStatistic","()Lorg/bukkit/Statistic;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::Statistic::from_raw(&self.jni_ref(),raw_obj
, crate::Statistic::from_string(variant_str).unwrap()
)}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn entity_type(&mut self) 
-> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntityType","()Lorg/bukkit/entity/EntityType;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::entity::EntityType::from_raw(&self.jni_ref(),raw_obj
, crate::entity::EntityType::from_string(variant_str).unwrap()
)}
	pub fn previous_value(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPreviousValue","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn new_value(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getNewValue","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerStatisticIncrementEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerStatisticIncrementEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerInteractAtEntityEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerInteractAtEntityEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerInteractAtEntityEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerInteractAtEntityEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerInteractAtEntityEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerInteractAtEntityEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_player(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::entity::Entity<'mc>>,arg2: std::option::Option<impl Into<&'mc crate::util::Vector<'mc>>>,arg3: std::option::Option<impl Into<&'mc crate::inventory::EquipmentSlot<'mc>>>) 
-> Result<crate::event::player::PlayerInteractAtEntityEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerInteractAtEntityEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/entity/Entity;Lorg/bukkit/util/Vector;Lorg/bukkit/inventory/EquipmentSlot;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
crate::event::player::PlayerInteractAtEntityEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn clicked_position(&mut self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClickedPosition","()Lorg/bukkit/util/Vector;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn hand(&mut self) 
-> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHand","()Lorg/bukkit/inventory/EquipmentSlot;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(),raw_obj
, crate::inventory::EquipmentSlot::from_string(variant_str).unwrap()
)}
	pub fn right_clicked(&mut self) 
-> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getRightClicked","()Lorg/bukkit/entity/Entity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Entity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::player::PlayerInteractEntityEvent<'mc>> for PlayerInteractAtEntityEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerInteractEntityEvent<'mc> {
       crate::event::player::PlayerInteractEntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerItemMendEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerItemMendEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerItemMendEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerItemMendEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerItemMendEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerItemMendEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_player(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,arg2: impl Into<&'mc crate::inventory::EquipmentSlot<'mc>>,arg3: std::option::Option<impl Into<&'mc crate::entity::ExperienceOrb<'mc>>>,arg4: std::option::Option<i32>) 
-> Result<crate::event::player::PlayerItemMendEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.unwrap().into().jni_object().clone())};
let val_4 = jni::objects::JValueGen::Int(arg4.unwrap().into());
let cls = &jni.find_class("org/bukkit/event/player/PlayerItemMendEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/EquipmentSlot;Lorg/bukkit/entity/ExperienceOrb;I)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
crate::event::player::PlayerItemMendEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn slot(&mut self) 
-> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getSlot","()Lorg/bukkit/inventory/EquipmentSlot;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(),raw_obj
, crate::inventory::EquipmentSlot::from_string(variant_str).unwrap()
)}
	pub fn item(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getItem","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn experience_orb(&mut self) 
-> Result<crate::entity::ExperienceOrb<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getExperienceOrb","()Lorg/bukkit/entity/ExperienceOrb;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::ExperienceOrb::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn repair_amount(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getRepairAmount","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn set_repair_amount(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setRepairAmount","(I)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerItemMendEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerItemMendEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerLocaleChangeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerLocaleChangeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerLocaleChangeEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerLocaleChangeEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerLocaleChangeEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerLocaleChangeEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc String>) 
-> Result<crate::event::player::PlayerLocaleChangeEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = jni::objects::JObject::from(jni.new_string(arg1.into()).unwrap());
let cls = &jni.find_class("org/bukkit/event/player/PlayerLocaleChangeEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::player::PlayerLocaleChangeEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn locale(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getLocale","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerLocaleChangeEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerChangedMainHandEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerChangedMainHandEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerChangedMainHandEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerChangedMainHandEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerChangedMainHandEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerChangedMainHandEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::inventory::MainHand<'mc>>) 
-> Result<crate::event::player::PlayerChangedMainHandEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerChangedMainHandEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/inventory/MainHand;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::player::PlayerChangedMainHandEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn main_hand(&mut self) 
-> Result<crate::inventory::MainHand<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getMainHand","()Lorg/bukkit/inventory/MainHand;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::inventory::MainHand::from_raw(&self.jni_ref(),raw_obj
, crate::inventory::MainHand::from_string(variant_str).unwrap()
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerChangedMainHandEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerLoginEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
pub struct PlayerLoginEventResult<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerLoginEventResult<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerLoginEventResult<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerLoginEventResult from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerLoginEventResult")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerLoginEventResult object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn value_of_with_string(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<jni::objects::JClass<'mc>>,arg1: std::option::Option<impl Into<&'mc String>>) 
-> Result<blackboxmc_java::JavaEnum<'mc>, Box<dyn std::error::Error>>

{let val_0 = arg0.unwrap();
let val_1 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
let cls = &jni.find_class("java/lang/Enum")?;
let res = jni.call_static_method(cls,"valueOf",
"(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
blackboxmc_java::JavaEnum::from_raw(&jni,obj
)}
	pub fn name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"name","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn describe_constable(&mut self) 
-> Result<blackboxmc_java::JavaOptional<'mc, javaEnumEnumDesc<E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"describeConstable","()Ljava/util/Optional;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaOptional::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn declaring_class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDeclaringClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn ordinal(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"ordinal","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerLoginEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerLoginEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerLoginEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerLoginEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerLoginEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_result(&mut self,arg0: impl Into<&'mc crate::event::player::PlayerLoginEventResult<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setResult","(Lorg/bukkit/event/player/PlayerLoginEvent$Result;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn result(&mut self) 
-> Result<crate::event::player::PlayerLoginEventResult<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getResult","()Lorg/bukkit/event/player/PlayerLoginEvent$Result;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::player::PlayerLoginEventResult::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn disallow(&mut self,arg0: impl Into<&'mc crate::event::player::PlayerLoginEventResult<'mc>>,arg1: impl Into<&'mc String>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"disallow","(Lorg/bukkit/event/player/PlayerLoginEvent$Result;Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn allow(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"allow","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn kick_message(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getKickMessage","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn set_kick_message(&mut self,arg0: impl Into<&'mc String>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"setKickMessage","(Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn hostname(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHostname","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerLoginEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerAdvancementDoneEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerAdvancementDoneEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerAdvancementDoneEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerAdvancementDoneEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerAdvancementDoneEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerAdvancementDoneEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::advancement::Advancement<'mc>>) 
-> Result<crate::event::player::PlayerAdvancementDoneEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerAdvancementDoneEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/advancement/Advancement;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::player::PlayerAdvancementDoneEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn advancement(&mut self) 
-> Result<crate::advancement::Advancement<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getAdvancement","()Lorg/bukkit/advancement/Advancement;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::advancement::Advancement::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerAdvancementDoneEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerPreLoginEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
pub struct PlayerPreLoginEventResult<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerPreLoginEventResult<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerPreLoginEventResult<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerPreLoginEventResult from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerPreLoginEventResult")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerPreLoginEventResult object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn value_of_with_string(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<jni::objects::JClass<'mc>>,arg1: std::option::Option<impl Into<&'mc String>>) 
-> Result<blackboxmc_java::JavaEnum<'mc>, Box<dyn std::error::Error>>

{let val_0 = arg0.unwrap();
let val_1 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
let cls = &jni.find_class("java/lang/Enum")?;
let res = jni.call_static_method(cls,"valueOf",
"(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
blackboxmc_java::JavaEnum::from_raw(&jni,obj
)}
	pub fn name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"name","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn describe_constable(&mut self) 
-> Result<blackboxmc_java::JavaOptional<'mc, javaEnumEnumDesc<E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"describeConstable","()Ljava/util/Optional;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaOptional::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn declaring_class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDeclaringClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn ordinal(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"ordinal","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerPreLoginEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerPreLoginEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerPreLoginEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerPreLoginEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerPreLoginEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_result(&mut self,arg0: impl Into<&'mc crate::event::player::PlayerPreLoginEventResult<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setResult","(Lorg/bukkit/event/player/PlayerPreLoginEvent$Result;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn result(&mut self) 
-> Result<crate::event::player::PlayerPreLoginEventResult<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getResult","()Lorg/bukkit/event/player/PlayerPreLoginEvent$Result;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::player::PlayerPreLoginEventResult::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn disallow(&mut self,arg0: impl Into<&'mc crate::event::player::PlayerPreLoginEventResult<'mc>>,arg1: impl Into<&'mc String>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"disallow","(Lorg/bukkit/event/player/PlayerPreLoginEvent$Result;Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn allow(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"allow","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn kick_message(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getKickMessage","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn set_kick_message(&mut self,arg0: impl Into<&'mc String>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"setKickMessage","(Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Event<'mc>> for PlayerPreLoginEvent<'mc>{
   fn into(self) -> crate::event::Event<'mc> {
       crate::event::Event::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerExpChangeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerExpChangeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerExpChangeEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerExpChangeEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerExpChangeEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerExpChangeEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: i32) 
-> Result<crate::event::player::PlayerExpChangeEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = jni::objects::JValueGen::Int(arg1.into());
let cls = &jni.find_class("org/bukkit/event/player/PlayerExpChangeEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;I)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::player::PlayerExpChangeEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn amount(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getAmount","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn set_amount(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setAmount","(I)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerExpChangeEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerItemBreakEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerItemBreakEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerItemBreakEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerItemBreakEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerItemBreakEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerItemBreakEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>) 
-> Result<crate::event::player::PlayerItemBreakEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerItemBreakEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/inventory/ItemStack;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::player::PlayerItemBreakEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn broken_item(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBrokenItem","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerItemBreakEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerChatEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerChatEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerChatEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerChatEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerChatEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerChatEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_player(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: std::option::Option<impl Into<&'mc String>>,arg2: std::option::Option<impl Into<&'mc String>>,arg3: std::option::Option<impl Into<&'mc blackboxmc_java::JavaSet<'mc, orgPlayer>>>) 
-> Result<crate::event::player::PlayerChatEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
let val_2 = jni::objects::JObject::from(jni.new_string(arg2.unwrap().into()).unwrap());
let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerChatEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Ljava/lang/String;Ljava/lang/String;Ljava/util/Set;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
crate::event::player::PlayerChatEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn message(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getMessage","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn set_message(&mut self,arg0: impl Into<&'mc String>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"setMessage","(Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn format(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getFormat","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn set_format(&mut self,arg0: impl Into<&'mc String>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"setFormat","(Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn set_player(&mut self,arg0: impl Into<&'mc crate::entity::Player<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setPlayer","(Lorg/bukkit/entity/Player;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn recipients(&mut self) 
-> Result<blackboxmc_java::JavaSet<'mc, orgPlayer>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getRecipients","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerChatEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerChatEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerChannelEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerChannelEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerChannelEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerChannelEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerChannelEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerChannelEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc String>) 
-> Result<crate::event::player::PlayerChannelEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = jni::objects::JObject::from(jni.new_string(arg1.into()).unwrap());
let cls = &jni.find_class("org/bukkit/event/player/PlayerChannelEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::player::PlayerChannelEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn channel(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getChannel","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerChannelEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerShowEntityEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerShowEntityEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerShowEntityEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerShowEntityEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerShowEntityEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerShowEntityEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::entity::Entity<'mc>>) 
-> Result<crate::event::player::PlayerShowEntityEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerShowEntityEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/entity/Entity;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::player::PlayerShowEntityEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn entity(&mut self) 
-> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/Entity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Entity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerShowEntityEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerHarvestBlockEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerHarvestBlockEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerHarvestBlockEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerHarvestBlockEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerHarvestBlockEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerHarvestBlockEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_player(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::block::Block<'mc>>,arg2: std::option::Option<impl Into<&'mc crate::inventory::EquipmentSlot<'mc>>>,arg3: std::option::Option<impl Into<&'mc blackboxmc_java::JavaList<'mc, orgItemStack>>>) 
-> Result<crate::event::player::PlayerHarvestBlockEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerHarvestBlockEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/block/Block;Lorg/bukkit/inventory/EquipmentSlot;Ljava/util/List;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
crate::event::player::PlayerHarvestBlockEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn hand(&mut self) 
-> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHand","()Lorg/bukkit/inventory/EquipmentSlot;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(),raw_obj
, crate::inventory::EquipmentSlot::from_string(variant_str).unwrap()
)}
	pub fn harvested_block(&mut self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHarvestedBlock","()Lorg/bukkit/block/Block;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn items_harvested(&mut self) 
-> Result<blackboxmc_java::JavaList<'mc, orgItemStack>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getItemsHarvested","()Ljava/util/List;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerHarvestBlockEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerHarvestBlockEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerItemDamageEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerItemDamageEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerItemDamageEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerItemDamageEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerItemDamageEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerItemDamageEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,arg2: i32) 
-> Result<crate::event::player::PlayerItemDamageEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = jni::objects::JValueGen::Int(arg2.into());
let cls = &jni.find_class("org/bukkit/event/player/PlayerItemDamageEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/inventory/ItemStack;I)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::player::PlayerItemDamageEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn item(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getItem","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_damage(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setDamage","(I)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn damage(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDamage","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerItemDamageEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerItemDamageEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerCommandPreprocessEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerCommandPreprocessEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerCommandPreprocessEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerCommandPreprocessEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerCommandPreprocessEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerCommandPreprocessEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_player(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: std::option::Option<impl Into<&'mc String>>,arg2: std::option::Option<impl Into<&'mc blackboxmc_java::JavaSet<'mc, orgPlayer>>>) 
-> Result<crate::event::player::PlayerCommandPreprocessEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerCommandPreprocessEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Ljava/lang/String;Ljava/util/Set;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::player::PlayerCommandPreprocessEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn message(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getMessage","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn set_message(&mut self,arg0: impl Into<&'mc String>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"setMessage","(Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn set_player(&mut self,arg0: impl Into<&'mc crate::entity::Player<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setPlayer","(Lorg/bukkit/entity/Player;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
#[deprecated]
	pub fn recipients(&mut self) 
-> Result<blackboxmc_java::JavaSet<'mc, orgPlayer>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getRecipients","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerCommandPreprocessEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerCommandPreprocessEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerBucketFishEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerBucketFishEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerBucketFishEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerBucketFishEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerBucketFishEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerBucketFishEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::entity::Fish<'mc>>,arg2: impl Into<&'mc crate::inventory::ItemStack<'mc>>,arg3: impl Into<&'mc crate::inventory::ItemStack<'mc>>,arg4: impl Into<&'mc crate::inventory::EquipmentSlot<'mc>>) 
-> Result<crate::event::player::PlayerBucketFishEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone())};
let val_4 = unsafe { jni::objects::JObject::from_raw(arg4.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerBucketFishEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/entity/Fish;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/EquipmentSlot;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
crate::event::player::PlayerBucketFishEvent::from_raw(&jni,res
)}
	pub fn entity(&mut self) 
-> Result<crate::entity::Fish<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/Fish;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Fish::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]
	pub fn water_bucket(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getWaterBucket","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]
	pub fn fish_bucket(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getFishBucket","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn hand(&mut self) 
-> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHand","()Lorg/bukkit/inventory/EquipmentSlot;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(),raw_obj
, crate::inventory::EquipmentSlot::from_string(variant_str).unwrap()
)}
	pub fn original_bucket(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getOriginalBucket","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn entity_bucket(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntityBucket","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::player::PlayerBucketEntityEvent<'mc>> for PlayerBucketFishEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerBucketEntityEvent<'mc> {
       crate::event::player::PlayerBucketEntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerChangedWorldEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerChangedWorldEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerChangedWorldEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerChangedWorldEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerChangedWorldEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerChangedWorldEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::World<'mc>>) 
-> Result<crate::event::player::PlayerChangedWorldEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerChangedWorldEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/World;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::player::PlayerChangedWorldEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn from(&mut self) 
-> Result<crate::World<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getFrom","()Lorg/bukkit/World;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::World::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerChangedWorldEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerRecipeDiscoverEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerRecipeDiscoverEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerRecipeDiscoverEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerRecipeDiscoverEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerRecipeDiscoverEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerRecipeDiscoverEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::NamespacedKey<'mc>>) 
-> Result<crate::event::player::PlayerRecipeDiscoverEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerRecipeDiscoverEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/NamespacedKey;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::player::PlayerRecipeDiscoverEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn recipe(&mut self) 
-> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getRecipe","()Lorg/bukkit/NamespacedKey;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::NamespacedKey::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerRecipeDiscoverEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerRecipeDiscoverEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct AsyncPlayerPreLoginEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
pub struct AsyncPlayerPreLoginEventResult<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for AsyncPlayerPreLoginEventResult<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> AsyncPlayerPreLoginEventResult<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate AsyncPlayerPreLoginEventResult from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "AsyncPlayerPreLoginEventResult")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a AsyncPlayerPreLoginEventResult object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn value_of_with_string(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<jni::objects::JClass<'mc>>,arg1: std::option::Option<impl Into<&'mc String>>) 
-> Result<blackboxmc_java::JavaEnum<'mc>, Box<dyn std::error::Error>>

{let val_0 = arg0.unwrap();
let val_1 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
let cls = &jni.find_class("java/lang/Enum")?;
let res = jni.call_static_method(cls,"valueOf",
"(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
blackboxmc_java::JavaEnum::from_raw(&jni,obj
)}
	pub fn name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"name","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn describe_constable(&mut self) 
-> Result<blackboxmc_java::JavaOptional<'mc, javaEnumEnumDesc<E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"describeConstable","()Ljava/util/Optional;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaOptional::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn declaring_class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDeclaringClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn ordinal(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"ordinal","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> blackboxmc_general::JNIRaw<'mc> for AsyncPlayerPreLoginEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> AsyncPlayerPreLoginEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate AsyncPlayerPreLoginEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "AsyncPlayerPreLoginEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a AsyncPlayerPreLoginEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]
	pub fn set_result(&mut self,arg0: impl Into<&'mc crate::event::player::PlayerPreLoginEventResult<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setResult","(Lorg/bukkit/event/player/PlayerPreLoginEvent$Result;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
#[deprecated]
	pub fn result(&mut self) 
-> Result<crate::event::player::PlayerPreLoginEventResult<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getResult","()Lorg/bukkit/event/player/PlayerPreLoginEvent$Result;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::player::PlayerPreLoginEventResult::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn disallow_with_player_pre_login_eventresult(&mut self,arg0: impl Into<&'mc crate::event::player::AsyncPlayerPreLoginEventResult<'mc>>,arg1: std::option::Option<impl Into<&'mc String>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.unwrap().into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"disallow","(Lorg/bukkit/event/player/AsyncPlayerPreLoginEvent$Result;Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn allow(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"allow","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn login_result(&mut self) 
-> Result<crate::event::player::AsyncPlayerPreLoginEventResult<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getLoginResult","()Lorg/bukkit/event/player/AsyncPlayerPreLoginEvent$Result;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::player::AsyncPlayerPreLoginEventResult::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_login_result(&mut self,arg0: impl Into<&'mc crate::event::player::AsyncPlayerPreLoginEventResult<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setLoginResult","(Lorg/bukkit/event/player/AsyncPlayerPreLoginEvent$Result;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn kick_message(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getKickMessage","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn set_kick_message(&mut self,arg0: impl Into<&'mc String>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"setKickMessage","(Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Event<'mc>> for AsyncPlayerPreLoginEvent<'mc>{
   fn into(self) -> crate::event::Event<'mc> {
       crate::event::Event::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerEggThrowEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerEggThrowEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerEggThrowEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerEggThrowEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerEggThrowEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerEggThrowEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::entity::Egg<'mc>>,arg2: bool,arg3: i8,arg4: impl Into<&'mc crate::entity::EntityType<'mc>>) 
-> Result<crate::event::player::PlayerEggThrowEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
// -2
let val_2 = jni::objects::JValueGen::Bool(arg2.into());
let val_3 = jni::objects::JValueGen::Byte(arg3.into());
let val_4 = unsafe { jni::objects::JObject::from_raw(arg4.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerEggThrowEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/entity/Egg;ZBLorg/bukkit/entity/EntityType;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
crate::event::player::PlayerEggThrowEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn is_hatching(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isHatching","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_hatching(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setHatching","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn hatching_type(&mut self) 
-> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHatchingType","()Lorg/bukkit/entity/EntityType;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::entity::EntityType::from_raw(&self.jni_ref(),raw_obj
, crate::entity::EntityType::from_string(variant_str).unwrap()
)}
	pub fn set_hatching_type(&mut self,arg0: impl Into<&'mc crate::entity::EntityType<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setHatchingType","(Lorg/bukkit/entity/EntityType;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn num_hatches(&mut self) 
-> Result<i8, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getNumHatches","()B",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.b().unwrap())}
	pub fn set_num_hatches(&mut self,arg0: i8) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Byte(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setNumHatches","(B)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn egg(&mut self) 
-> Result<crate::entity::Egg<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEgg","()Lorg/bukkit/entity/Egg;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Egg::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerEggThrowEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerHideEntityEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerHideEntityEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerHideEntityEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerHideEntityEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerHideEntityEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerHideEntityEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::entity::Entity<'mc>>) 
-> Result<crate::event::player::PlayerHideEntityEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/PlayerHideEntityEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/entity/Entity;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::player::PlayerHideEntityEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn entity(&mut self) 
-> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/Entity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Entity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerHideEntityEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerPickupItemEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerPickupItemEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerPickupItemEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerPickupItemEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerPickupItemEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerPickupItemEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::entity::Item<'mc>>,arg2: i32) 
-> Result<crate::event::player::PlayerPickupItemEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = jni::objects::JValueGen::Int(arg2.into());
let cls = &jni.find_class("org/bukkit/event/player/PlayerPickupItemEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/entity/Item;I)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::player::PlayerPickupItemEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn remaining(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getRemaining","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn item(&mut self) 
-> Result<crate::entity::Item<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getItem","()Lorg/bukkit/entity/Item;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Item::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerPickupItemEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for PlayerPickupItemEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct AsyncPlayerChatEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for AsyncPlayerChatEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> AsyncPlayerChatEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate AsyncPlayerChatEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "AsyncPlayerChatEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a AsyncPlayerChatEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: bool,arg1: impl Into<&'mc crate::entity::Player<'mc>>,arg2: impl Into<&'mc String>,arg3: impl Into<&'mc blackboxmc_java::JavaSet<'mc, orgPlayer>>) 
-> Result<crate::event::player::AsyncPlayerChatEvent<'mc>, Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = jni::objects::JObject::from(jni.new_string(arg2.into()).unwrap());
let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/player/AsyncPlayerChatEvent")?;
let res = jni.new_object(cls,
"(ZLorg/bukkit/entity/Player;Ljava/lang/String;Ljava/util/Set;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
crate::event::player::AsyncPlayerChatEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn message(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getMessage","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn set_message(&mut self,arg0: impl Into<&'mc String>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"setMessage","(Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn format(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getFormat","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn set_format(&mut self,arg0: impl Into<&'mc String>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"setFormat","(Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn recipients(&mut self) 
-> Result<blackboxmc_java::JavaSet<'mc, orgPlayer>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getRecipients","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for AsyncPlayerChatEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::player::PlayerEvent<'mc>> for AsyncPlayerChatEvent<'mc>{
   fn into(self) -> crate::event::player::PlayerEvent<'mc> {
       crate::event::player::PlayerEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
