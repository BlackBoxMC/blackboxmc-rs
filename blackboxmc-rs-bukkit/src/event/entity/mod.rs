#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
pub struct EntityExplodeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityExplodeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityExplodeEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityExplodeEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityExplodeEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityExplodeEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Entity<'mc>>,arg1: impl Into<&'mc crate::Location<'mc>>,arg2: impl Into<&'mc blackboxmc_java::JavaList<'mc, orgBlock>>,arg3: f32) 
-> Result<crate::event::entity::EntityExplodeEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let val_3 = jni::objects::JValueGen::Float(arg3.into());
let cls = &jni.find_class("org/bukkit/event/entity/EntityExplodeEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Entity;Lorg/bukkit/Location;Ljava/util/List;F)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
crate::event::entity::EntityExplodeEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn location(&mut self) 
-> Result<crate::Location<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getLocation","()Lorg/bukkit/Location;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_yield(&mut self,arg0: f32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Float(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setYield","(F)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn get_yield(&mut self) 
-> Result<f32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getYield","()F",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.f().unwrap())}
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
	pub fn block_list(&mut self) 
-> Result<blackboxmc_java::JavaList<'mc, orgBlock>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"blockList","()Ljava/util/List;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityExplodeEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityExplodeEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityPortalEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityPortalEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityPortalEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityPortalEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityPortalEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityPortalEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_entity(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Entity<'mc>>,arg1: impl Into<&'mc crate::Location<'mc>>,arg2: std::option::Option<impl Into<&'mc crate::Location<'mc>>>,arg3: std::option::Option<i32>) 
-> Result<crate::event::entity::EntityPortalEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let val_3 = jni::objects::JValueGen::Int(arg3.unwrap().into());
let cls = &jni.find_class("org/bukkit/event/entity/EntityPortalEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Entity;Lorg/bukkit/Location;Lorg/bukkit/Location;I)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
crate::event::entity::EntityPortalEvent::from_raw(&jni,res
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
impl<'mc> Into<crate::event::entity::EntityTeleportEvent<'mc>> for EntityPortalEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityTeleportEvent<'mc> {
       crate::event::entity::EntityTeleportEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct StriderTemperatureChangeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for StriderTemperatureChangeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> StriderTemperatureChangeEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate StriderTemperatureChangeEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "StriderTemperatureChangeEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a StriderTemperatureChangeEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Strider<'mc>>,arg1: bool) 
-> Result<crate::event::entity::StriderTemperatureChangeEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
// -2
let val_1 = jni::objects::JValueGen::Bool(arg1.into());
let cls = &jni.find_class("org/bukkit/event/entity/StriderTemperatureChangeEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Strider;Z)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::entity::StriderTemperatureChangeEvent::from_raw(&jni,res
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
	pub fn is_shivering(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isShivering","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn entity(&mut self) 
-> Result<crate::entity::Strider<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/Strider;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Strider::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for StriderTemperatureChangeEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for StriderTemperatureChangeEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityDamageByBlockEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityDamageByBlockEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityDamageByBlockEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityDamageByBlockEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityDamageByBlockEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityDamageByBlockEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_block(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::block::Block<'mc>>,arg1: impl Into<&'mc crate::entity::Entity<'mc>>,arg2: impl Into<&'mc crate::event::entity::EntityDamageEventDamageCause<'mc>>,arg3: std::option::Option<impl Into<&'mc blackboxmc_java::JavaMap<'mc, orgEntityDamageEventDamageModifier, javaDouble>>>,arg4: std::option::Option<impl Into<&'mc blackboxmc_java::JavaMap<'mc, c>>>) 
-> Result<crate::event::entity::EntityDamageByBlockEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.unwrap().into().jni_object().clone())};
let val_4 = unsafe { jni::objects::JObject::from_raw(arg4.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/EntityDamageByBlockEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/block/Block;Lorg/bukkit/entity/Entity;Lorg/bukkit/event/entity/EntityDamageEvent$DamageCause;Ljava/util/Map;Ljava/util/Map;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
crate::event::entity::EntityDamageByBlockEvent::from_raw(&jni,res
)}
	pub fn damager(&mut self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDamager","()Lorg/bukkit/block/Block;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn cause(&mut self) 
-> Result<crate::event::entity::EntityDamageEventDamageCause<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCause","()Lorg/bukkit/event/entity/EntityDamageEvent$DamageCause;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::entity::EntityDamageEventDamageCause::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_damage_with_double(&mut self,arg0: std::option::Option<impl Into<&'mc crate::event::entity::EntityDamageEventDamageModifier<'mc>>>,arg1: std::option::Option<f64>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let val_1 = jni::objects::JValueGen::Double(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"setDamage","(Lorg/bukkit/event/entity/EntityDamageEvent$DamageModifier;D)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn get_damage(&mut self,arg0: impl Into<&'mc crate::event::entity::EntityDamageEventDamageModifier<'mc>>) 
-> Result<f64, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"getDamage","(Lorg/bukkit/event/entity/EntityDamageEvent$DamageModifier;)D",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
	pub fn is_applicable(&mut self,arg0: impl Into<&'mc crate::event::entity::EntityDamageEventDamageModifier<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"isApplicable","(Lorg/bukkit/event/entity/EntityDamageEvent$DamageModifier;)Z",&[jni::objects::JValueGen::from(&val_0)]);
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
	pub fn get_original_damage(&mut self,arg0: impl Into<&'mc crate::event::entity::EntityDamageEventDamageModifier<'mc>>) 
-> Result<f64, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"getOriginalDamage","(Lorg/bukkit/event/entity/EntityDamageEvent$DamageModifier;)D",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
	pub fn final_damage(&mut self) 
-> Result<f64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getFinalDamage","()D",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
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
impl<'mc> Into<crate::event::entity::EntityDamageEvent<'mc>> for EntityDamageByBlockEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityDamageEvent<'mc> {
       crate::event::entity::EntityDamageEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct SheepRegrowWoolEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for SheepRegrowWoolEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> SheepRegrowWoolEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate SheepRegrowWoolEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "SheepRegrowWoolEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a SheepRegrowWoolEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Sheep<'mc>>) 
-> Result<crate::event::entity::SheepRegrowWoolEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/SheepRegrowWoolEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Sheep;)V",&[jni::objects::JValueGen::from(&val_0)])?;
crate::event::entity::SheepRegrowWoolEvent::from_raw(&jni,res
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
-> Result<crate::entity::Sheep<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/Sheep;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Sheep::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for SheepRegrowWoolEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for SheepRegrowWoolEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityEnterBlockEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityEnterBlockEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityEnterBlockEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityEnterBlockEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityEnterBlockEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityEnterBlockEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Entity<'mc>>,arg1: impl Into<&'mc crate::block::Block<'mc>>) 
-> Result<crate::event::entity::EntityEnterBlockEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/EntityEnterBlockEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Entity;Lorg/bukkit/block/Block;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::entity::EntityEnterBlockEvent::from_raw(&jni,res
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
	pub fn block(&mut self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBlock","()Lorg/bukkit/block/Block;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityEnterBlockEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityEnterBlockEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityPickupItemEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityPickupItemEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityPickupItemEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityPickupItemEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityPickupItemEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityPickupItemEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::LivingEntity<'mc>>,arg1: impl Into<&'mc crate::entity::Item<'mc>>,arg2: i32) 
-> Result<crate::event::entity::EntityPickupItemEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = jni::objects::JValueGen::Int(arg2.into());
let cls = &jni.find_class("org/bukkit/event/entity/EntityPickupItemEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/entity/Item;I)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::entity::EntityPickupItemEvent::from_raw(&jni,res
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
	pub fn entity(&mut self) 
-> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/LivingEntity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::LivingEntity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityPickupItemEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityPickupItemEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct HorseJumpEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for HorseJumpEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> HorseJumpEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate HorseJumpEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "HorseJumpEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a HorseJumpEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::AbstractHorse<'mc>>,arg1: f32) 
-> Result<crate::event::entity::HorseJumpEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = jni::objects::JValueGen::Float(arg1.into());
let cls = &jni.find_class("org/bukkit/event/entity/HorseJumpEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/AbstractHorse;F)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::entity::HorseJumpEvent::from_raw(&jni,res
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
	pub fn power(&mut self) 
-> Result<f32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPower","()F",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.f().unwrap())}
#[deprecated]
	pub fn set_power(&mut self,arg0: f32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Float(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setPower","(F)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn entity(&mut self) 
-> Result<crate::entity::AbstractHorse<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/AbstractHorse;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::AbstractHorse::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for HorseJumpEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for HorseJumpEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct ExplosionPrimeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ExplosionPrimeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ExplosionPrimeEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate ExplosionPrimeEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "ExplosionPrimeEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a ExplosionPrimeEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_explosive(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc crate::entity::Entity<'mc>>>,arg1: std::option::Option<f32>,arg2: std::option::Option<bool>) 
-> Result<crate::event::entity::ExplosionPrimeEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let val_1 = jni::objects::JValueGen::Float(arg1.unwrap().into());
// 0
let val_2 = jni::objects::JValueGen::Bool(arg2.unwrap().into());
let cls = &jni.find_class("org/bukkit/event/entity/ExplosionPrimeEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Entity;FZ)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::entity::ExplosionPrimeEvent::from_raw(&jni,res
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
	pub fn radius(&mut self) 
-> Result<f32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getRadius","()F",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.f().unwrap())}
	pub fn set_radius(&mut self,arg0: f32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Float(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setRadius","(F)V",&[jni::objects::JValueGen::from(&val_0)]);
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
	pub fn fire(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getFire","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_fire(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setFire","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for ExplosionPrimeEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for ExplosionPrimeEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct CreatureSpawnEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
pub struct CreatureSpawnEventSpawnReason<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for CreatureSpawnEventSpawnReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> CreatureSpawnEventSpawnReason<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate CreatureSpawnEventSpawnReason from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "CreatureSpawnEventSpawnReason")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a CreatureSpawnEventSpawnReason object, got {}",
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
impl<'mc> blackboxmc_general::JNIRaw<'mc> for CreatureSpawnEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> CreatureSpawnEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate CreatureSpawnEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "CreatureSpawnEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a CreatureSpawnEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::LivingEntity<'mc>>,arg1: impl Into<&'mc crate::event::entity::CreatureSpawnEventSpawnReason<'mc>>) 
-> Result<crate::event::entity::CreatureSpawnEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/CreatureSpawnEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/event/entity/CreatureSpawnEvent$SpawnReason;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::entity::CreatureSpawnEvent::from_raw(&jni,res
)}
	pub fn entity(&mut self) 
-> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/LivingEntity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::LivingEntity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn spawn_reason(&mut self) 
-> Result<crate::event::entity::CreatureSpawnEventSpawnReason<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getSpawnReason","()Lorg/bukkit/event/entity/CreatureSpawnEvent$SpawnReason;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::entity::CreatureSpawnEventSpawnReason::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn location(&mut self) 
-> Result<crate::Location<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getLocation","()Lorg/bukkit/Location;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
impl<'mc> Into<crate::event::entity::EntitySpawnEvent<'mc>> for CreatureSpawnEvent<'mc>{
   fn into(self) -> crate::event::entity::EntitySpawnEvent<'mc> {
       crate::event::entity::EntitySpawnEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct ItemSpawnEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ItemSpawnEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ItemSpawnEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate ItemSpawnEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "ItemSpawnEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a ItemSpawnEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
#[deprecated]
	pub fn new_with_item(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc crate::entity::Item<'mc>>>,arg1: std::option::Option<impl Into<&'mc crate::Location<'mc>>>) 
-> Result<crate::event::entity::ItemSpawnEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/ItemSpawnEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Item;Lorg/bukkit/Location;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::entity::ItemSpawnEvent::from_raw(&jni,res
)}
	pub fn entity(&mut self) 
-> Result<crate::entity::Item<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/Item;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Item::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn location(&mut self) 
-> Result<crate::Location<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getLocation","()Lorg/bukkit/Location;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
impl<'mc> Into<crate::event::entity::EntitySpawnEvent<'mc>> for ItemSpawnEvent<'mc>{
   fn into(self) -> crate::event::entity::EntitySpawnEvent<'mc> {
       crate::event::entity::EntitySpawnEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityDropItemEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityDropItemEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityDropItemEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityDropItemEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityDropItemEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityDropItemEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Entity<'mc>>,arg1: impl Into<&'mc crate::entity::Item<'mc>>) 
-> Result<crate::event::entity::EntityDropItemEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/EntityDropItemEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Entity;Lorg/bukkit/entity/Item;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::entity::EntityDropItemEvent::from_raw(&jni,res
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityDropItemEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityDropItemEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct SlimeSplitEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for SlimeSplitEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> SlimeSplitEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate SlimeSplitEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "SlimeSplitEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a SlimeSplitEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Slime<'mc>>,arg1: i32) 
-> Result<crate::event::entity::SlimeSplitEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = jni::objects::JValueGen::Int(arg1.into());
let cls = &jni.find_class("org/bukkit/event/entity/SlimeSplitEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Slime;I)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::entity::SlimeSplitEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn count(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCount","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn entity(&mut self) 
-> Result<crate::entity::Slime<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/Slime;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Slime::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
	pub fn set_count(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCount","(I)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for SlimeSplitEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for SlimeSplitEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct FoodLevelChangeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for FoodLevelChangeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> FoodLevelChangeEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate FoodLevelChangeEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "FoodLevelChangeEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a FoodLevelChangeEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_human_entity(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::HumanEntity<'mc>>,arg1: std::option::Option<i32>,arg2: std::option::Option<impl Into<&'mc crate::inventory::ItemStack<'mc>>>) 
-> Result<crate::event::entity::FoodLevelChangeEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/FoodLevelChangeEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/HumanEntity;ILorg/bukkit/inventory/ItemStack;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::entity::FoodLevelChangeEvent::from_raw(&jni,res
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
-> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/HumanEntity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::HumanEntity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn food_level(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getFoodLevel","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn set_food_level(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setFoodLevel","(I)V",&[jni::objects::JValueGen::from(&val_0)]);
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for FoodLevelChangeEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for FoodLevelChangeEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityTargetLivingEntityEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityTargetLivingEntityEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityTargetLivingEntityEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityTargetLivingEntityEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityTargetLivingEntityEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityTargetLivingEntityEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Entity<'mc>>,arg1: impl Into<&'mc crate::entity::LivingEntity<'mc>>,arg2: impl Into<&'mc crate::event::entity::EntityTargetEventTargetReason<'mc>>) 
-> Result<crate::event::entity::EntityTargetLivingEntityEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/EntityTargetLivingEntityEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Entity;Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/event/entity/EntityTargetEvent$TargetReason;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::entity::EntityTargetLivingEntityEvent::from_raw(&jni,res
)}
	pub fn target(&mut self) 
-> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getTarget","()Lorg/bukkit/entity/LivingEntity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::LivingEntity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_target(&mut self,arg0: impl Into<&'mc crate::entity::Entity<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setTarget","(Lorg/bukkit/entity/Entity;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn reason(&mut self) 
-> Result<crate::event::entity::EntityTargetEventTargetReason<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getReason","()Lorg/bukkit/event/entity/EntityTargetEvent$TargetReason;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::entity::EntityTargetEventTargetReason::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
impl<'mc> Into<crate::event::entity::EntityTargetEvent<'mc>> for EntityTargetLivingEntityEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityTargetEvent<'mc> {
       crate::event::entity::EntityTargetEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityShootBowEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityShootBowEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityShootBowEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityShootBowEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityShootBowEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityShootBowEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::LivingEntity<'mc>>,arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,arg2: impl Into<&'mc crate::inventory::ItemStack<'mc>>,arg3: impl Into<&'mc crate::entity::Entity<'mc>>,arg4: impl Into<&'mc crate::inventory::EquipmentSlot<'mc>>,arg5: f32,arg6: bool) 
-> Result<crate::event::entity::EntityShootBowEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone())};
let val_4 = unsafe { jni::objects::JObject::from_raw(arg4.into().jni_object().clone())};
let val_5 = jni::objects::JValueGen::Float(arg5.into());
// -2
let val_6 = jni::objects::JValueGen::Bool(arg6.into());
let cls = &jni.find_class("org/bukkit/event/entity/EntityShootBowEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/entity/Entity;Lorg/bukkit/inventory/EquipmentSlot;FZ)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6)])?;
crate::event::entity::EntityShootBowEvent::from_raw(&jni,res
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
-> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/LivingEntity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::LivingEntity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
	pub fn bow(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBow","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn consumable(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getConsumable","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn projectile(&mut self) 
-> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getProjectile","()Lorg/bukkit/entity/Entity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Entity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_projectile(&mut self,arg0: impl Into<&'mc crate::entity::Entity<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setProjectile","(Lorg/bukkit/entity/Entity;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn force(&mut self) 
-> Result<f32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getForce","()F",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.f().unwrap())}
	pub fn set_consume_item(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setConsumeItem","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn should_consume_item(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"shouldConsumeItem","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityShootBowEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityShootBowEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityResurrectEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityResurrectEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityResurrectEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityResurrectEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityResurrectEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityResurrectEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_living_entity(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<&'mc crate::entity::LivingEntity<'mc>>>,arg1: std::option::Option<impl Into<&'mc crate::inventory::EquipmentSlot<'mc>>>) 
-> Result<crate::event::entity::EntityResurrectEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/EntityResurrectEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/inventory/EquipmentSlot;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::entity::EntityResurrectEvent::from_raw(&jni,res
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
-> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/LivingEntity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::LivingEntity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityResurrectEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityResurrectEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PiglinBarterEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PiglinBarterEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PiglinBarterEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PiglinBarterEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PiglinBarterEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PiglinBarterEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Piglin<'mc>>,arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,arg2: impl Into<&'mc blackboxmc_java::JavaList<'mc, orgItemStack>>) 
-> Result<crate::event::entity::PiglinBarterEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/PiglinBarterEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Piglin;Lorg/bukkit/inventory/ItemStack;Ljava/util/List;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::entity::PiglinBarterEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn input(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getInput","()Lorg/bukkit/inventory/ItemStack;",&[]);
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
-> Result<crate::entity::Piglin<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/Piglin;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Piglin::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
	pub fn outcome(&mut self) 
-> Result<blackboxmc_java::JavaList<'mc, orgItemStack>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getOutcome","()Ljava/util/List;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for PiglinBarterEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for PiglinBarterEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PigZombieAngerEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PigZombieAngerEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PigZombieAngerEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PigZombieAngerEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PigZombieAngerEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PigZombieAngerEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::PigZombie<'mc>>,arg1: impl Into<&'mc crate::entity::Entity<'mc>>,arg2: i32) 
-> Result<crate::event::entity::PigZombieAngerEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = jni::objects::JValueGen::Int(arg2.into());
let cls = &jni.find_class("org/bukkit/event/entity/PigZombieAngerEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/PigZombie;Lorg/bukkit/entity/Entity;I)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::entity::PigZombieAngerEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn target(&mut self) 
-> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getTarget","()Lorg/bukkit/entity/Entity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Entity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn entity(&mut self) 
-> Result<crate::entity::PigZombie<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/PigZombie;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::PigZombie::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
	pub fn new_anger(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getNewAnger","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn set_new_anger(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setNewAnger","(I)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for PigZombieAngerEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for PigZombieAngerEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityCreatePortalEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityCreatePortalEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityCreatePortalEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityCreatePortalEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityCreatePortalEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityCreatePortalEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::LivingEntity<'mc>>,arg1: impl Into<&'mc blackboxmc_java::JavaList<'mc, orgBlockState>>,arg2: impl Into<&'mc crate::PortalType<'mc>>) 
-> Result<crate::event::entity::EntityCreatePortalEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/EntityCreatePortalEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/LivingEntity;Ljava/util/List;Lorg/bukkit/PortalType;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::entity::EntityCreatePortalEvent::from_raw(&jni,res
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
-> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/LivingEntity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::LivingEntity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn blocks(&mut self) 
-> Result<blackboxmc_java::JavaList<'mc, orgBlockState>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBlocks","()Ljava/util/List;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
	pub fn portal_type(&mut self) 
-> Result<crate::PortalType<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPortalType","()Lorg/bukkit/PortalType;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::PortalType::from_raw(&self.jni_ref(),raw_obj
, crate::PortalType::from_string(variant_str).unwrap()
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityCreatePortalEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityCreatePortalEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityToggleSwimEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityToggleSwimEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityToggleSwimEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityToggleSwimEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityToggleSwimEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityToggleSwimEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::LivingEntity<'mc>>,arg1: bool) 
-> Result<crate::event::entity::EntityToggleSwimEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
// -2
let val_1 = jni::objects::JValueGen::Bool(arg1.into());
let cls = &jni.find_class("org/bukkit/event/entity/EntityToggleSwimEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/LivingEntity;Z)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::entity::EntityToggleSwimEvent::from_raw(&jni,res
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
	pub fn is_swimming(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isSwimming","()Z",&[]);
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityToggleSwimEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityToggleSwimEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityTeleportEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityTeleportEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityTeleportEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityTeleportEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityTeleportEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityTeleportEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Entity<'mc>>,arg1: impl Into<&'mc crate::Location<'mc>>,arg2: impl Into<&'mc crate::Location<'mc>>) 
-> Result<crate::event::entity::EntityTeleportEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/EntityTeleportEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Entity;Lorg/bukkit/Location;Lorg/bukkit/Location;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::entity::EntityTeleportEvent::from_raw(&jni,res
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityTeleportEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityTeleportEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityEnterLoveModeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityEnterLoveModeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityEnterLoveModeEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityEnterLoveModeEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityEnterLoveModeEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityEnterLoveModeEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Animals<'mc>>,arg1: impl Into<&'mc crate::entity::HumanEntity<'mc>>,arg2: i32) 
-> Result<crate::event::entity::EntityEnterLoveModeEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = jni::objects::JValueGen::Int(arg2.into());
let cls = &jni.find_class("org/bukkit/event/entity/EntityEnterLoveModeEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Animals;Lorg/bukkit/entity/HumanEntity;I)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::entity::EntityEnterLoveModeEvent::from_raw(&jni,res
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
-> Result<crate::entity::Animals<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/Animals;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Animals::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
	pub fn human_entity(&mut self) 
-> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHumanEntity","()Lorg/bukkit/entity/HumanEntity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::HumanEntity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn ticks_in_love(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getTicksInLove","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn set_ticks_in_love(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setTicksInLove","(I)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityEnterLoveModeEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityEnterLoveModeEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct SheepDyeWoolEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for SheepDyeWoolEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> SheepDyeWoolEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate SheepDyeWoolEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "SheepDyeWoolEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a SheepDyeWoolEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_sheep(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Sheep<'mc>>,arg1: std::option::Option<impl Into<&'mc crate::DyeColor<'mc>>>,arg2: std::option::Option<impl Into<&'mc crate::entity::Player<'mc>>>) 
-> Result<crate::event::entity::SheepDyeWoolEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/SheepDyeWoolEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Sheep;Lorg/bukkit/DyeColor;Lorg/bukkit/entity/Player;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::entity::SheepDyeWoolEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_color(&mut self,arg0: impl Into<&'mc crate::DyeColor<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setColor","(Lorg/bukkit/DyeColor;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn color(&mut self) 
-> Result<crate::DyeColor<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getColor","()Lorg/bukkit/DyeColor;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::DyeColor::from_raw(&self.jni_ref(),raw_obj
, crate::DyeColor::from_string(variant_str).unwrap()
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn entity(&mut self) 
-> Result<crate::entity::Sheep<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/Sheep;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Sheep::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for SheepDyeWoolEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for SheepDyeWoolEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityPoseChangeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityPoseChangeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityPoseChangeEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityPoseChangeEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityPoseChangeEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityPoseChangeEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Entity<'mc>>,arg1: impl Into<&'mc crate::entity::Pose<'mc>>) 
-> Result<crate::event::entity::EntityPoseChangeEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/EntityPoseChangeEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Entity;Lorg/bukkit/entity/Pose;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::entity::EntityPoseChangeEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn pose(&mut self) 
-> Result<crate::entity::Pose<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPose","()Lorg/bukkit/entity/Pose;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::entity::Pose::from_raw(&self.jni_ref(),raw_obj
, crate::entity::Pose::from_string(variant_str).unwrap()
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
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityPoseChangeEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityDamageByEntityEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityDamageByEntityEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityDamageByEntityEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityDamageByEntityEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityDamageByEntityEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityDamageByEntityEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_entity(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Entity<'mc>>,arg1: impl Into<&'mc crate::entity::Entity<'mc>>,arg2: impl Into<&'mc crate::event::entity::EntityDamageEventDamageCause<'mc>>,arg3: std::option::Option<impl Into<&'mc blackboxmc_java::JavaMap<'mc, orgEntityDamageEventDamageModifier, javaDouble>>>,arg4: std::option::Option<impl Into<&'mc blackboxmc_java::JavaMap<'mc, c>>>) 
-> Result<crate::event::entity::EntityDamageByEntityEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.unwrap().into().jni_object().clone())};
let val_4 = unsafe { jni::objects::JObject::from_raw(arg4.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/EntityDamageByEntityEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Entity;Lorg/bukkit/entity/Entity;Lorg/bukkit/event/entity/EntityDamageEvent$DamageCause;Ljava/util/Map;Ljava/util/Map;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
crate::event::entity::EntityDamageByEntityEvent::from_raw(&jni,res
)}
	pub fn damager(&mut self) 
-> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDamager","()Lorg/bukkit/entity/Entity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Entity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn cause(&mut self) 
-> Result<crate::event::entity::EntityDamageEventDamageCause<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCause","()Lorg/bukkit/event/entity/EntityDamageEvent$DamageCause;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::entity::EntityDamageEventDamageCause::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_damage_with_double(&mut self,arg0: std::option::Option<impl Into<&'mc crate::event::entity::EntityDamageEventDamageModifier<'mc>>>,arg1: std::option::Option<f64>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let val_1 = jni::objects::JValueGen::Double(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"setDamage","(Lorg/bukkit/event/entity/EntityDamageEvent$DamageModifier;D)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn get_damage(&mut self,arg0: impl Into<&'mc crate::event::entity::EntityDamageEventDamageModifier<'mc>>) 
-> Result<f64, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"getDamage","(Lorg/bukkit/event/entity/EntityDamageEvent$DamageModifier;)D",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
	pub fn is_applicable(&mut self,arg0: impl Into<&'mc crate::event::entity::EntityDamageEventDamageModifier<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"isApplicable","(Lorg/bukkit/event/entity/EntityDamageEvent$DamageModifier;)Z",&[jni::objects::JValueGen::from(&val_0)]);
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
	pub fn get_original_damage(&mut self,arg0: impl Into<&'mc crate::event::entity::EntityDamageEventDamageModifier<'mc>>) 
-> Result<f64, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"getOriginalDamage","(Lorg/bukkit/event/entity/EntityDamageEvent$DamageModifier;)D",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
	pub fn final_damage(&mut self) 
-> Result<f64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getFinalDamage","()D",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
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
impl<'mc> Into<crate::event::entity::EntityDamageEvent<'mc>> for EntityDamageByEntityEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityDamageEvent<'mc> {
       crate::event::entity::EntityDamageEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct VillagerCareerChangeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
pub struct VillagerCareerChangeEventChangeReason<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for VillagerCareerChangeEventChangeReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> VillagerCareerChangeEventChangeReason<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate VillagerCareerChangeEventChangeReason from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "VillagerCareerChangeEventChangeReason")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a VillagerCareerChangeEventChangeReason object, got {}",
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
impl<'mc> blackboxmc_general::JNIRaw<'mc> for VillagerCareerChangeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> VillagerCareerChangeEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate VillagerCareerChangeEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "VillagerCareerChangeEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a VillagerCareerChangeEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Villager<'mc>>,arg1: impl Into<&'mc crate::entity::VillagerProfession<'mc>>,arg2: impl Into<&'mc crate::event::entity::VillagerCareerChangeEventChangeReason<'mc>>) 
-> Result<crate::event::entity::VillagerCareerChangeEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/VillagerCareerChangeEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Villager;Lorg/bukkit/entity/Villager$Profession;Lorg/bukkit/event/entity/VillagerCareerChangeEvent$ChangeReason;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::entity::VillagerCareerChangeEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn reason(&mut self) 
-> Result<crate::event::entity::VillagerCareerChangeEventChangeReason<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getReason","()Lorg/bukkit/event/entity/VillagerCareerChangeEvent$ChangeReason;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::entity::VillagerCareerChangeEventChangeReason::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
	pub fn profession(&mut self) 
-> Result<crate::entity::VillagerProfession<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getProfession","()Lorg/bukkit/entity/Villager$Profession;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::VillagerProfession::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_profession(&mut self,arg0: impl Into<&'mc crate::entity::VillagerProfession<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setProfession","(Lorg/bukkit/entity/Villager$Profession;)V",&[jni::objects::JValueGen::from(&val_0)]);
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for VillagerCareerChangeEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for VillagerCareerChangeEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Entity<'mc>>) 
-> Result<crate::event::entity::EntityEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/EntityEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Entity;)V",&[jni::objects::JValueGen::from(&val_0)])?;
crate::event::entity::EntityEvent::from_raw(&jni,res
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
impl<'mc> Into<crate::event::Event<'mc>> for EntityEvent<'mc>{
   fn into(self) -> crate::event::Event<'mc> {
       crate::event::Event::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityPotionEffectEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
pub struct EntityPotionEffectEventAction<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityPotionEffectEventAction<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityPotionEffectEventAction<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityPotionEffectEventAction from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityPotionEffectEventAction")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityPotionEffectEventAction object, got {}",
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
pub struct EntityPotionEffectEventCause<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityPotionEffectEventCause<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityPotionEffectEventCause<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityPotionEffectEventCause from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityPotionEffectEventCause")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityPotionEffectEventCause object, got {}",
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
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityPotionEffectEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityPotionEffectEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityPotionEffectEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityPotionEffectEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityPotionEffectEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::LivingEntity<'mc>>,arg1: impl Into<&'mc crate::potion::PotionEffect<'mc>>,arg2: impl Into<&'mc crate::potion::PotionEffect<'mc>>,arg3: impl Into<&'mc crate::event::entity::EntityPotionEffectEventCause<'mc>>,arg4: impl Into<&'mc crate::event::entity::EntityPotionEffectEventAction<'mc>>,arg5: bool) 
-> Result<crate::event::entity::EntityPotionEffectEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone())};
let val_4 = unsafe { jni::objects::JObject::from_raw(arg4.into().jni_object().clone())};
// -2
let val_5 = jni::objects::JValueGen::Bool(arg5.into());
let cls = &jni.find_class("org/bukkit/event/entity/EntityPotionEffectEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/potion/PotionEffect;Lorg/bukkit/potion/PotionEffect;Lorg/bukkit/event/entity/EntityPotionEffectEvent$Cause;Lorg/bukkit/event/entity/EntityPotionEffectEvent$Action;Z)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5)])?;
crate::event::entity::EntityPotionEffectEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn cause(&mut self) 
-> Result<crate::event::entity::EntityPotionEffectEventCause<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCause","()Lorg/bukkit/event/entity/EntityPotionEffectEvent$Cause;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::entity::EntityPotionEffectEventCause::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
	pub fn action(&mut self) 
-> Result<crate::event::entity::EntityPotionEffectEventAction<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getAction","()Lorg/bukkit/event/entity/EntityPotionEffectEvent$Action;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::entity::EntityPotionEffectEventAction::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn old_effect(&mut self) 
-> Result<crate::potion::PotionEffect<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getOldEffect","()Lorg/bukkit/potion/PotionEffect;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::potion::PotionEffect::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn new_effect(&mut self) 
-> Result<crate::potion::PotionEffect<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getNewEffect","()Lorg/bukkit/potion/PotionEffect;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::potion::PotionEffect::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn modified_type(&mut self) 
-> Result<crate::potion::PotionEffectType<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getModifiedType","()Lorg/bukkit/potion/PotionEffectType;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::potion::PotionEffectType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_override(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isOverride","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_override(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setOverride","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityPotionEffectEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityPotionEffectEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct ItemMergeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ItemMergeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ItemMergeEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate ItemMergeEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "ItemMergeEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a ItemMergeEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Item<'mc>>,arg1: impl Into<&'mc crate::entity::Item<'mc>>) 
-> Result<crate::event::entity::ItemMergeEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/ItemMergeEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Item;Lorg/bukkit/entity/Item;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::entity::ItemMergeEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn target(&mut self) 
-> Result<crate::entity::Item<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getTarget","()Lorg/bukkit/entity/Item;",&[]);
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
	pub fn entity(&mut self) 
-> Result<crate::entity::Item<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/Item;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Item::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for ItemMergeEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for ItemMergeEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct VillagerAcquireTradeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for VillagerAcquireTradeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> VillagerAcquireTradeEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate VillagerAcquireTradeEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "VillagerAcquireTradeEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a VillagerAcquireTradeEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::AbstractVillager<'mc>>,arg1: impl Into<&'mc crate::inventory::MerchantRecipe<'mc>>) 
-> Result<crate::event::entity::VillagerAcquireTradeEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/VillagerAcquireTradeEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/AbstractVillager;Lorg/bukkit/inventory/MerchantRecipe;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::entity::VillagerAcquireTradeEvent::from_raw(&jni,res
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
-> Result<crate::inventory::MerchantRecipe<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getRecipe","()Lorg/bukkit/inventory/MerchantRecipe;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::MerchantRecipe::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_recipe(&mut self,arg0: impl Into<&'mc crate::inventory::MerchantRecipe<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setRecipe","(Lorg/bukkit/inventory/MerchantRecipe;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for VillagerAcquireTradeEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for VillagerAcquireTradeEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct LingeringPotionSplashEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for LingeringPotionSplashEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> LingeringPotionSplashEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate LingeringPotionSplashEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "LingeringPotionSplashEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a LingeringPotionSplashEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::ThrownPotion<'mc>>,arg1: impl Into<&'mc crate::entity::AreaEffectCloud<'mc>>) 
-> Result<crate::event::entity::LingeringPotionSplashEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/LingeringPotionSplashEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/ThrownPotion;Lorg/bukkit/entity/AreaEffectCloud;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::entity::LingeringPotionSplashEvent::from_raw(&jni,res
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
	pub fn area_effect_cloud(&mut self) 
-> Result<crate::entity::AreaEffectCloud<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getAreaEffectCloud","()Lorg/bukkit/entity/AreaEffectCloud;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::AreaEffectCloud::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
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
	pub fn hit_block(&mut self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHitBlock","()Lorg/bukkit/block/Block;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn hit_block_face(&mut self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHitBlockFace","()Lorg/bukkit/block/BlockFace;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::block::BlockFace::from_raw(&self.jni_ref(),raw_obj
, crate::block::BlockFace::from_string(variant_str).unwrap()
)}
	pub fn hit_entity(&mut self) 
-> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHitEntity","()Lorg/bukkit/entity/Entity;",&[]);
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for LingeringPotionSplashEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::ProjectileHitEvent<'mc>> for LingeringPotionSplashEvent<'mc>{
   fn into(self) -> crate::event::entity::ProjectileHitEvent<'mc> {
       crate::event::entity::ProjectileHitEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityTameEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityTameEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityTameEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityTameEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityTameEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityTameEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::LivingEntity<'mc>>,arg1: impl Into<&'mc crate::entity::AnimalTamer<'mc>>) 
-> Result<crate::event::entity::EntityTameEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/EntityTameEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/entity/AnimalTamer;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::entity::EntityTameEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn owner(&mut self) 
-> Result<crate::entity::AnimalTamer<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getOwner","()Lorg/bukkit/entity/AnimalTamer;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::AnimalTamer::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn entity(&mut self) 
-> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/LivingEntity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::LivingEntity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityTameEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityTameEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityBreakDoorEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityBreakDoorEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityBreakDoorEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityBreakDoorEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityBreakDoorEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityBreakDoorEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::LivingEntity<'mc>>,arg1: impl Into<&'mc crate::block::Block<'mc>>) 
-> Result<crate::event::entity::EntityBreakDoorEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/EntityBreakDoorEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/block/Block;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::entity::EntityBreakDoorEvent::from_raw(&jni,res
)}
	pub fn entity(&mut self) 
-> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/Entity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Entity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
	pub fn block(&mut self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBlock","()Lorg/bukkit/block/Block;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn block_data(&mut self) 
-> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBlockData","()Lorg/bukkit/block/data/BlockData;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::data::BlockData::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
	pub fn to(&mut self) 
-> Result<crate::Material<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getTo","()Lorg/bukkit/Material;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::Material::from_raw(&self.jni_ref(),raw_obj
, crate::Material::from_string(variant_str).unwrap()
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
impl<'mc> Into<crate::event::entity::EntityChangeBlockEvent<'mc>> for EntityBreakDoorEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityChangeBlockEvent<'mc> {
       crate::event::entity::EntityChangeBlockEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityBreedEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityBreedEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityBreedEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityBreedEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityBreedEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityBreedEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::LivingEntity<'mc>>,arg1: impl Into<&'mc crate::entity::LivingEntity<'mc>>,arg2: impl Into<&'mc crate::entity::LivingEntity<'mc>>,arg3: impl Into<&'mc crate::entity::LivingEntity<'mc>>,arg4: impl Into<&'mc crate::inventory::ItemStack<'mc>>,arg5: i32) 
-> Result<crate::event::entity::EntityBreedEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone())};
let val_4 = unsafe { jni::objects::JObject::from_raw(arg4.into().jni_object().clone())};
let val_5 = jni::objects::JValueGen::Int(arg5.into());
let cls = &jni.find_class("org/bukkit/event/entity/EntityBreedEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/inventory/ItemStack;I)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5)])?;
crate::event::entity::EntityBreedEvent::from_raw(&jni,res
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
-> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/LivingEntity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::LivingEntity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn experience(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getExperience","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn set_experience(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setExperience","(I)V",&[jni::objects::JValueGen::from(&val_0)]);
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
	pub fn mother(&mut self) 
-> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getMother","()Lorg/bukkit/entity/LivingEntity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::LivingEntity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn father(&mut self) 
-> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getFather","()Lorg/bukkit/entity/LivingEntity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::LivingEntity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn breeder(&mut self) 
-> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBreeder","()Lorg/bukkit/entity/LivingEntity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::LivingEntity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn bred_with(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBredWith","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityBreedEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityBreedEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityPlaceEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityPlaceEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityPlaceEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityPlaceEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityPlaceEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityPlaceEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_entity(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Entity<'mc>>,arg1: impl Into<&'mc crate::entity::Player<'mc>>,arg2: impl Into<&'mc crate::block::Block<'mc>>,arg3: std::option::Option<impl Into<&'mc crate::block::BlockFace<'mc>>>,arg4: std::option::Option<impl Into<&'mc crate::inventory::EquipmentSlot<'mc>>>) 
-> Result<crate::event::entity::EntityPlaceEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.unwrap().into().jni_object().clone())};
let val_4 = unsafe { jni::objects::JObject::from_raw(arg4.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/EntityPlaceEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Entity;Lorg/bukkit/entity/Player;Lorg/bukkit/block/Block;Lorg/bukkit/block/BlockFace;Lorg/bukkit/inventory/EquipmentSlot;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
crate::event::entity::EntityPlaceEvent::from_raw(&jni,res
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
	pub fn block(&mut self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBlock","()Lorg/bukkit/block/Block;",&[]);
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityPlaceEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityPlaceEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityCombustByBlockEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityCombustByBlockEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityCombustByBlockEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityCombustByBlockEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityCombustByBlockEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityCombustByBlockEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::block::Block<'mc>>,arg1: impl Into<&'mc crate::entity::Entity<'mc>>,arg2: i32) 
-> Result<crate::event::entity::EntityCombustByBlockEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = jni::objects::JValueGen::Int(arg2.into());
let cls = &jni.find_class("org/bukkit/event/entity/EntityCombustByBlockEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/block/Block;Lorg/bukkit/entity/Entity;I)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::entity::EntityCombustByBlockEvent::from_raw(&jni,res
)}
	pub fn combuster(&mut self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCombuster","()Lorg/bukkit/block/Block;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
	pub fn duration(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDuration","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn set_duration(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setDuration","(I)V",&[jni::objects::JValueGen::from(&val_0)]);
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
impl<'mc> Into<crate::event::entity::EntityCombustEvent<'mc>> for EntityCombustByBlockEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityCombustEvent<'mc> {
       crate::event::entity::EntityCombustEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct ItemDespawnEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ItemDespawnEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ItemDespawnEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate ItemDespawnEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "ItemDespawnEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a ItemDespawnEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Item<'mc>>,arg1: impl Into<&'mc crate::Location<'mc>>) 
-> Result<crate::event::entity::ItemDespawnEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/ItemDespawnEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Item;Lorg/bukkit/Location;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::entity::ItemDespawnEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn location(&mut self) 
-> Result<crate::Location<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getLocation","()Lorg/bukkit/Location;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn entity(&mut self) 
-> Result<crate::entity::Item<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/Item;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Item::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for ItemDespawnEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for ItemDespawnEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityRegainHealthEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
pub struct EntityRegainHealthEventRegainReason<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityRegainHealthEventRegainReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityRegainHealthEventRegainReason<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityRegainHealthEventRegainReason from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityRegainHealthEventRegainReason")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityRegainHealthEventRegainReason object, got {}",
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
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityRegainHealthEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityRegainHealthEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityRegainHealthEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityRegainHealthEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityRegainHealthEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Entity<'mc>>,arg1: f64,arg2: impl Into<&'mc crate::event::entity::EntityRegainHealthEventRegainReason<'mc>>) 
-> Result<crate::event::entity::EntityRegainHealthEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = jni::objects::JValueGen::Double(arg1.into());
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/EntityRegainHealthEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Entity;DLorg/bukkit/event/entity/EntityRegainHealthEvent$RegainReason;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::entity::EntityRegainHealthEvent::from_raw(&jni,res
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
	pub fn amount(&mut self) 
-> Result<f64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getAmount","()D",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
	pub fn set_amount(&mut self,arg0: f64) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Double(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setAmount","(D)V",&[jni::objects::JValueGen::from(&val_0)]);
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
	pub fn regain_reason(&mut self) 
-> Result<crate::event::entity::EntityRegainHealthEventRegainReason<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getRegainReason","()Lorg/bukkit/event/entity/EntityRegainHealthEvent$RegainReason;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::entity::EntityRegainHealthEventRegainReason::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityRegainHealthEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityRegainHealthEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct BatToggleSleepEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for BatToggleSleepEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BatToggleSleepEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate BatToggleSleepEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "BatToggleSleepEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a BatToggleSleepEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Bat<'mc>>,arg1: bool) 
-> Result<crate::event::entity::BatToggleSleepEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
// -2
let val_1 = jni::objects::JValueGen::Bool(arg1.into());
let cls = &jni.find_class("org/bukkit/event/entity/BatToggleSleepEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Bat;Z)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::entity::BatToggleSleepEvent::from_raw(&jni,res
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
	pub fn is_awake(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAwake","()Z",&[]);
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for BatToggleSleepEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for BatToggleSleepEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct ProjectileLaunchEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ProjectileLaunchEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ProjectileLaunchEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate ProjectileLaunchEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "ProjectileLaunchEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a ProjectileLaunchEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Entity<'mc>>) 
-> Result<crate::event::entity::ProjectileLaunchEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/ProjectileLaunchEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Entity;)V",&[jni::objects::JValueGen::from(&val_0)])?;
crate::event::entity::ProjectileLaunchEvent::from_raw(&jni,res
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
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn location(&mut self) 
-> Result<crate::Location<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getLocation","()Lorg/bukkit/Location;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for ProjectileLaunchEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntitySpawnEvent<'mc>> for ProjectileLaunchEvent<'mc>{
   fn into(self) -> crate::event::entity::EntitySpawnEvent<'mc> {
       crate::event::entity::EntitySpawnEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerDeathEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerDeathEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerDeathEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerDeathEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerDeathEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerDeathEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_player(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc blackboxmc_java::JavaList<'mc, orgItemStack>>,arg2: i32,arg3: i32,arg4: i32,arg5: i32,arg6: std::option::Option<impl Into<&'mc String>>) 
-> Result<crate::event::entity::PlayerDeathEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = jni::objects::JValueGen::Int(arg2.into());
let val_3 = jni::objects::JValueGen::Int(arg3.into());
let val_4 = jni::objects::JValueGen::Int(arg4.into());
let val_5 = jni::objects::JValueGen::Int(arg5.into());
let val_6 = jni::objects::JObject::from(jni.new_string(arg6.unwrap().into()).unwrap());
let cls = &jni.find_class("org/bukkit/event/entity/PlayerDeathEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Ljava/util/List;IIIILjava/lang/String;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6)])?;
crate::event::entity::PlayerDeathEvent::from_raw(&jni,res
)}
	pub fn entity(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn new_level(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getNewLevel","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn set_new_level(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setNewLevel","(I)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn set_death_message(&mut self,arg0: impl Into<&'mc String>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"setDeathMessage","(Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn death_message(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDeathMessage","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn new_exp(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getNewExp","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn set_new_exp(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setNewExp","(I)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn new_total_exp(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getNewTotalExp","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn set_new_total_exp(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setNewTotalExp","(I)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn keep_level(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getKeepLevel","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_keep_level(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setKeepLevel","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn set_keep_inventory(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setKeepInventory","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn keep_inventory(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getKeepInventory","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn drops(&mut self) 
-> Result<blackboxmc_java::JavaList<'mc, orgItemStack>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDrops","()Ljava/util/List;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn dropped_exp(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDroppedExp","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn set_dropped_exp(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setDroppedExp","(I)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
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
impl<'mc> Into<crate::event::entity::EntityDeathEvent<'mc>> for PlayerDeathEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityDeathEvent<'mc> {
       crate::event::entity::EntityDeathEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct AreaEffectCloudApplyEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for AreaEffectCloudApplyEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> AreaEffectCloudApplyEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate AreaEffectCloudApplyEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "AreaEffectCloudApplyEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a AreaEffectCloudApplyEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::AreaEffectCloud<'mc>>,arg1: impl Into<&'mc blackboxmc_java::JavaList<'mc, orgLivingEntity>>) 
-> Result<crate::event::entity::AreaEffectCloudApplyEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/AreaEffectCloudApplyEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/AreaEffectCloud;Ljava/util/List;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::entity::AreaEffectCloudApplyEvent::from_raw(&jni,res
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
-> Result<crate::entity::AreaEffectCloud<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/AreaEffectCloud;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::AreaEffectCloud::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
	pub fn affected_entities(&mut self) 
-> Result<blackboxmc_java::JavaList<'mc, orgLivingEntity>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getAffectedEntities","()Ljava/util/List;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for AreaEffectCloudApplyEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for AreaEffectCloudApplyEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityChangeBlockEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityChangeBlockEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityChangeBlockEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityChangeBlockEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityChangeBlockEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityChangeBlockEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Entity<'mc>>,arg1: impl Into<&'mc crate::block::Block<'mc>>,arg2: impl Into<&'mc crate::block::data::BlockData<'mc>>) 
-> Result<crate::event::entity::EntityChangeBlockEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/EntityChangeBlockEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Entity;Lorg/bukkit/block/Block;Lorg/bukkit/block/data/BlockData;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::entity::EntityChangeBlockEvent::from_raw(&jni,res
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
	pub fn block(&mut self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBlock","()Lorg/bukkit/block/Block;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn block_data(&mut self) 
-> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBlockData","()Lorg/bukkit/block/data/BlockData;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::data::BlockData::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
	pub fn to(&mut self) 
-> Result<crate::Material<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getTo","()Lorg/bukkit/Material;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::Material::from_raw(&self.jni_ref(),raw_obj
, crate::Material::from_string(variant_str).unwrap()
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityChangeBlockEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityChangeBlockEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityCombustByEntityEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityCombustByEntityEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityCombustByEntityEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityCombustByEntityEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityCombustByEntityEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityCombustByEntityEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Entity<'mc>>,arg1: impl Into<&'mc crate::entity::Entity<'mc>>,arg2: i32) 
-> Result<crate::event::entity::EntityCombustByEntityEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = jni::objects::JValueGen::Int(arg2.into());
let cls = &jni.find_class("org/bukkit/event/entity/EntityCombustByEntityEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Entity;Lorg/bukkit/entity/Entity;I)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::entity::EntityCombustByEntityEvent::from_raw(&jni,res
)}
	pub fn combuster(&mut self) 
-> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCombuster","()Lorg/bukkit/entity/Entity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Entity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
	pub fn duration(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDuration","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn set_duration(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setDuration","(I)V",&[jni::objects::JValueGen::from(&val_0)]);
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
impl<'mc> Into<crate::event::entity::EntityCombustEvent<'mc>> for EntityCombustByEntityEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityCombustEvent<'mc> {
       crate::event::entity::EntityCombustEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityExhaustionEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
pub struct EntityExhaustionEventExhaustionReason<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityExhaustionEventExhaustionReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityExhaustionEventExhaustionReason<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityExhaustionEventExhaustionReason from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityExhaustionEventExhaustionReason")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityExhaustionEventExhaustionReason object, got {}",
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
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityExhaustionEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityExhaustionEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityExhaustionEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityExhaustionEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityExhaustionEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::HumanEntity<'mc>>,arg1: impl Into<&'mc crate::event::entity::EntityExhaustionEventExhaustionReason<'mc>>,arg2: f32) 
-> Result<crate::event::entity::EntityExhaustionEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = jni::objects::JValueGen::Float(arg2.into());
let cls = &jni.find_class("org/bukkit/event/entity/EntityExhaustionEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/HumanEntity;Lorg/bukkit/event/entity/EntityExhaustionEvent$ExhaustionReason;F)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::entity::EntityExhaustionEvent::from_raw(&jni,res
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
-> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/HumanEntity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::HumanEntity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn exhaustion(&mut self) 
-> Result<f32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getExhaustion","()F",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.f().unwrap())}
	pub fn set_exhaustion(&mut self,arg0: f32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Float(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setExhaustion","(F)V",&[jni::objects::JValueGen::from(&val_0)]);
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
	pub fn exhaustion_reason(&mut self) 
-> Result<crate::event::entity::EntityExhaustionEventExhaustionReason<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getExhaustionReason","()Lorg/bukkit/event/entity/EntityExhaustionEvent$ExhaustionReason;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::entity::EntityExhaustionEventExhaustionReason::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityExhaustionEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityExhaustionEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PlayerLeashEntityEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerLeashEntityEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerLeashEntityEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PlayerLeashEntityEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PlayerLeashEntityEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PlayerLeashEntityEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_entity(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Entity<'mc>>,arg1: impl Into<&'mc crate::entity::Entity<'mc>>,arg2: std::option::Option<impl Into<&'mc crate::entity::Player<'mc>>>,arg3: std::option::Option<impl Into<&'mc crate::inventory::EquipmentSlot<'mc>>>) 
-> Result<crate::event::entity::PlayerLeashEntityEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/PlayerLeashEntityEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Entity;Lorg/bukkit/entity/Entity;Lorg/bukkit/entity/Player;Lorg/bukkit/inventory/EquipmentSlot;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
crate::event::entity::PlayerLeashEntityEvent::from_raw(&jni,res
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
	pub fn leash_holder(&mut self) 
-> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getLeashHolder","()Lorg/bukkit/entity/Entity;",&[]);
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerLeashEntityEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::Event<'mc>> for PlayerLeashEntityEvent<'mc>{
   fn into(self) -> crate::event::Event<'mc> {
       crate::event::Event::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PigZapEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PigZapEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PigZapEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PigZapEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PigZapEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PigZapEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Pig<'mc>>,arg1: impl Into<&'mc crate::entity::LightningStrike<'mc>>,arg2: impl Into<&'mc crate::entity::PigZombie<'mc>>) 
-> Result<crate::event::entity::PigZapEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/PigZapEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Pig;Lorg/bukkit/entity/LightningStrike;Lorg/bukkit/entity/PigZombie;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::entity::PigZapEvent::from_raw(&jni,res
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
-> Result<crate::entity::Pig<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/Pig;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Pig::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
	pub fn lightning(&mut self) 
-> Result<crate::entity::LightningStrike<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getLightning","()Lorg/bukkit/entity/LightningStrike;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::LightningStrike::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]
	pub fn pig_zombie(&mut self) 
-> Result<crate::entity::PigZombie<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPigZombie","()Lorg/bukkit/entity/PigZombie;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::PigZombie::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn transformed_entity(&mut self) 
-> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getTransformedEntity","()Lorg/bukkit/entity/Entity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Entity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn transformed_entities(&mut self) 
-> Result<blackboxmc_java::JavaList<'mc, orgEntity>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getTransformedEntities","()Ljava/util/List;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn transform_reason(&mut self) 
-> Result<crate::event::entity::EntityTransformEventTransformReason<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getTransformReason","()Lorg/bukkit/event/entity/EntityTransformEvent$TransformReason;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::entity::EntityTransformEventTransformReason::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for PigZapEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityTransformEvent<'mc>> for PigZapEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityTransformEvent<'mc> {
       crate::event::entity::EntityTransformEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct FireworkExplodeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for FireworkExplodeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> FireworkExplodeEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate FireworkExplodeEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "FireworkExplodeEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a FireworkExplodeEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Firework<'mc>>) 
-> Result<crate::event::entity::FireworkExplodeEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/FireworkExplodeEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Firework;)V",&[jni::objects::JValueGen::from(&val_0)])?;
crate::event::entity::FireworkExplodeEvent::from_raw(&jni,res
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
-> Result<crate::entity::Firework<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/Firework;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Firework::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for FireworkExplodeEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for FireworkExplodeEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct ArrowBodyCountChangeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ArrowBodyCountChangeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ArrowBodyCountChangeEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate ArrowBodyCountChangeEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "ArrowBodyCountChangeEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a ArrowBodyCountChangeEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::LivingEntity<'mc>>,arg1: i32,arg2: i32,arg3: bool) 
-> Result<crate::event::entity::ArrowBodyCountChangeEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = jni::objects::JValueGen::Int(arg1.into());
let val_2 = jni::objects::JValueGen::Int(arg2.into());
// -2
let val_3 = jni::objects::JValueGen::Bool(arg3.into());
let cls = &jni.find_class("org/bukkit/event/entity/ArrowBodyCountChangeEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/LivingEntity;IIZ)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
crate::event::entity::ArrowBodyCountChangeEvent::from_raw(&jni,res
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
-> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/LivingEntity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::LivingEntity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
	pub fn is_reset(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isReset","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn old_amount(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getOldAmount","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn new_amount(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getNewAmount","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn set_new_amount(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setNewAmount","(I)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for ArrowBodyCountChangeEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for ArrowBodyCountChangeEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityPortalExitEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityPortalExitEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityPortalExitEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityPortalExitEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityPortalExitEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityPortalExitEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Entity<'mc>>,arg1: impl Into<&'mc crate::Location<'mc>>,arg2: impl Into<&'mc crate::Location<'mc>>,arg3: impl Into<&'mc crate::util::Vector<'mc>>,arg4: impl Into<&'mc crate::util::Vector<'mc>>) 
-> Result<crate::event::entity::EntityPortalExitEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone())};
let val_4 = unsafe { jni::objects::JObject::from_raw(arg4.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/EntityPortalExitEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Entity;Lorg/bukkit/Location;Lorg/bukkit/Location;Lorg/bukkit/util/Vector;Lorg/bukkit/util/Vector;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
crate::event::entity::EntityPortalExitEvent::from_raw(&jni,res
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
	pub fn before(&mut self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBefore","()Lorg/bukkit/util/Vector;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn after(&mut self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getAfter","()Lorg/bukkit/util/Vector;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_after(&mut self,arg0: impl Into<&'mc crate::util::Vector<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setAfter","(Lorg/bukkit/util/Vector;)V",&[jni::objects::JValueGen::from(&val_0)]);
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
impl<'mc> Into<crate::event::entity::EntityTeleportEvent<'mc>> for EntityPortalExitEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityTeleportEvent<'mc> {
       crate::event::entity::EntityTeleportEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityDamageEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
pub struct EntityDamageEventDamageCause<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityDamageEventDamageCause<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityDamageEventDamageCause<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityDamageEventDamageCause from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityDamageEventDamageCause")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityDamageEventDamageCause object, got {}",
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
pub struct EntityDamageEventDamageModifier<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityDamageEventDamageModifier<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityDamageEventDamageModifier<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityDamageEventDamageModifier from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityDamageEventDamageModifier")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityDamageEventDamageModifier object, got {}",
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
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityDamageEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityDamageEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityDamageEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityDamageEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityDamageEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_entity(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Entity<'mc>>,arg1: impl Into<&'mc crate::event::entity::EntityDamageEventDamageCause<'mc>>,arg2: std::option::Option<impl Into<&'mc blackboxmc_java::JavaMap<'mc, orgEntityDamageEventDamageModifier, javaDouble>>>,arg3: std::option::Option<impl Into<&'mc blackboxmc_java::JavaMap<'mc, c>>>) 
-> Result<crate::event::entity::EntityDamageEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/EntityDamageEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Entity;Lorg/bukkit/event/entity/EntityDamageEvent$DamageCause;Ljava/util/Map;Ljava/util/Map;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
crate::event::entity::EntityDamageEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn cause(&mut self) 
-> Result<crate::event::entity::EntityDamageEventDamageCause<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCause","()Lorg/bukkit/event/entity/EntityDamageEvent$DamageCause;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::entity::EntityDamageEventDamageCause::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_damage_with_double(&mut self,arg0: std::option::Option<impl Into<&'mc crate::event::entity::EntityDamageEventDamageModifier<'mc>>>,arg1: std::option::Option<f64>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let val_1 = jni::objects::JValueGen::Double(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"setDamage","(Lorg/bukkit/event/entity/EntityDamageEvent$DamageModifier;D)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn get_damage(&mut self,arg0: impl Into<&'mc crate::event::entity::EntityDamageEventDamageModifier<'mc>>) 
-> Result<f64, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"getDamage","(Lorg/bukkit/event/entity/EntityDamageEvent$DamageModifier;)D",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
	pub fn is_applicable(&mut self,arg0: impl Into<&'mc crate::event::entity::EntityDamageEventDamageModifier<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"isApplicable","(Lorg/bukkit/event/entity/EntityDamageEvent$DamageModifier;)Z",&[jni::objects::JValueGen::from(&val_0)]);
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
	pub fn get_original_damage(&mut self,arg0: impl Into<&'mc crate::event::entity::EntityDamageEventDamageModifier<'mc>>) 
-> Result<f64, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"getOriginalDamage","(Lorg/bukkit/event/entity/EntityDamageEvent$DamageModifier;)D",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
	pub fn final_damage(&mut self) 
-> Result<f64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getFinalDamage","()D",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityDamageEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityDamageEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityDeathEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityDeathEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityDeathEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityDeathEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityDeathEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityDeathEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_living_entity(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::LivingEntity<'mc>>,arg1: std::option::Option<impl Into<&'mc blackboxmc_java::JavaList<'mc, orgItemStack>>>,arg2: std::option::Option<i32>) 
-> Result<crate::event::entity::EntityDeathEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let val_2 = jni::objects::JValueGen::Int(arg2.unwrap().into());
let cls = &jni.find_class("org/bukkit/event/entity/EntityDeathEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/LivingEntity;Ljava/util/List;I)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::entity::EntityDeathEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn entity(&mut self) 
-> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/LivingEntity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::LivingEntity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn drops(&mut self) 
-> Result<blackboxmc_java::JavaList<'mc, orgItemStack>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDrops","()Ljava/util/List;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn dropped_exp(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDroppedExp","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn set_dropped_exp(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setDroppedExp","(I)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
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
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityDeathEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityToggleGlideEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityToggleGlideEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityToggleGlideEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityToggleGlideEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityToggleGlideEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityToggleGlideEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::LivingEntity<'mc>>,arg1: bool) 
-> Result<crate::event::entity::EntityToggleGlideEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
// -2
let val_1 = jni::objects::JValueGen::Bool(arg1.into());
let cls = &jni.find_class("org/bukkit/event/entity/EntityToggleGlideEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/LivingEntity;Z)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::entity::EntityToggleGlideEvent::from_raw(&jni,res
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
	pub fn is_gliding(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isGliding","()Z",&[]);
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityToggleGlideEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityToggleGlideEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityTargetEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
pub struct EntityTargetEventTargetReason<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityTargetEventTargetReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityTargetEventTargetReason<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityTargetEventTargetReason from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityTargetEventTargetReason")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityTargetEventTargetReason object, got {}",
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
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityTargetEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityTargetEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityTargetEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityTargetEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityTargetEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Entity<'mc>>,arg1: impl Into<&'mc crate::entity::Entity<'mc>>,arg2: impl Into<&'mc crate::event::entity::EntityTargetEventTargetReason<'mc>>) 
-> Result<crate::event::entity::EntityTargetEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/EntityTargetEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Entity;Lorg/bukkit/entity/Entity;Lorg/bukkit/event/entity/EntityTargetEvent$TargetReason;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::entity::EntityTargetEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn reason(&mut self) 
-> Result<crate::event::entity::EntityTargetEventTargetReason<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getReason","()Lorg/bukkit/event/entity/EntityTargetEvent$TargetReason;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::entity::EntityTargetEventTargetReason::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn target(&mut self) 
-> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getTarget","()Lorg/bukkit/entity/Entity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Entity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_target(&mut self,arg0: impl Into<&'mc crate::entity::Entity<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setTarget","(Lorg/bukkit/entity/Entity;)V",&[jni::objects::JValueGen::from(&val_0)]);
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityTargetEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityTargetEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct CreeperPowerEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
pub struct CreeperPowerEventPowerCause<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for CreeperPowerEventPowerCause<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> CreeperPowerEventPowerCause<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate CreeperPowerEventPowerCause from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "CreeperPowerEventPowerCause")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a CreeperPowerEventPowerCause object, got {}",
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
impl<'mc> blackboxmc_general::JNIRaw<'mc> for CreeperPowerEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> CreeperPowerEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate CreeperPowerEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "CreeperPowerEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a CreeperPowerEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_creeper(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Creeper<'mc>>,arg1: std::option::Option<impl Into<&'mc crate::entity::LightningStrike<'mc>>>,arg2: std::option::Option<impl Into<&'mc crate::event::entity::CreeperPowerEventPowerCause<'mc>>>) 
-> Result<crate::event::entity::CreeperPowerEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/CreeperPowerEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Creeper;Lorg/bukkit/entity/LightningStrike;Lorg/bukkit/event/entity/CreeperPowerEvent$PowerCause;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::entity::CreeperPowerEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn cause(&mut self) 
-> Result<crate::event::entity::CreeperPowerEventPowerCause<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCause","()Lorg/bukkit/event/entity/CreeperPowerEvent$PowerCause;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::entity::CreeperPowerEventPowerCause::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn entity(&mut self) 
-> Result<crate::entity::Creeper<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/Creeper;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Creeper::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
	pub fn lightning(&mut self) 
-> Result<crate::entity::LightningStrike<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getLightning","()Lorg/bukkit/entity/LightningStrike;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::LightningStrike::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for CreeperPowerEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for CreeperPowerEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct SpawnerSpawnEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for SpawnerSpawnEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> SpawnerSpawnEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate SpawnerSpawnEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "SpawnerSpawnEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a SpawnerSpawnEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Entity<'mc>>,arg1: impl Into<&'mc crate::block::CreatureSpawner<'mc>>) 
-> Result<crate::event::entity::SpawnerSpawnEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/SpawnerSpawnEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Entity;Lorg/bukkit/block/CreatureSpawner;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::entity::SpawnerSpawnEvent::from_raw(&jni,res
)}
	pub fn spawner(&mut self) 
-> Result<crate::block::CreatureSpawner<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getSpawner","()Lorg/bukkit/block/CreatureSpawner;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::CreatureSpawner::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn location(&mut self) 
-> Result<crate::Location<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getLocation","()Lorg/bukkit/Location;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
impl<'mc> Into<crate::event::entity::EntitySpawnEvent<'mc>> for SpawnerSpawnEvent<'mc>{
   fn into(self) -> crate::event::entity::EntitySpawnEvent<'mc> {
       crate::event::entity::EntitySpawnEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EnderDragonChangePhaseEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EnderDragonChangePhaseEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EnderDragonChangePhaseEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EnderDragonChangePhaseEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EnderDragonChangePhaseEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EnderDragonChangePhaseEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::EnderDragon<'mc>>,arg1: impl Into<&'mc crate::entity::EnderDragonPhase<'mc>>,arg2: impl Into<&'mc crate::entity::EnderDragonPhase<'mc>>) 
-> Result<crate::event::entity::EnderDragonChangePhaseEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/EnderDragonChangePhaseEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/EnderDragon;Lorg/bukkit/entity/EnderDragon$Phase;Lorg/bukkit/entity/EnderDragon$Phase;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::entity::EnderDragonChangePhaseEvent::from_raw(&jni,res
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
-> Result<crate::entity::EnderDragon<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/EnderDragon;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::EnderDragon::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
	pub fn set_new_phase(&mut self,arg0: impl Into<&'mc crate::entity::EnderDragonPhase<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setNewPhase","(Lorg/bukkit/entity/EnderDragon$Phase;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn current_phase(&mut self) 
-> Result<crate::entity::EnderDragonPhase<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCurrentPhase","()Lorg/bukkit/entity/EnderDragon$Phase;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::EnderDragonPhase::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn new_phase(&mut self) 
-> Result<crate::entity::EnderDragonPhase<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getNewPhase","()Lorg/bukkit/entity/EnderDragon$Phase;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::EnderDragonPhase::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for EnderDragonChangePhaseEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EnderDragonChangePhaseEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct ProjectileHitEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ProjectileHitEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ProjectileHitEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate ProjectileHitEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "ProjectileHitEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a ProjectileHitEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_projectile(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Projectile<'mc>>,arg1: impl Into<&'mc crate::entity::Entity<'mc>>,arg2: std::option::Option<impl Into<&'mc crate::block::Block<'mc>>>,arg3: std::option::Option<impl Into<&'mc crate::block::BlockFace<'mc>>>) 
-> Result<crate::event::entity::ProjectileHitEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/ProjectileHitEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Projectile;Lorg/bukkit/entity/Entity;Lorg/bukkit/block/Block;Lorg/bukkit/block/BlockFace;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
crate::event::entity::ProjectileHitEvent::from_raw(&jni,res
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
	pub fn hit_block(&mut self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHitBlock","()Lorg/bukkit/block/Block;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn hit_block_face(&mut self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHitBlockFace","()Lorg/bukkit/block/BlockFace;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::block::BlockFace::from_raw(&self.jni_ref(),raw_obj
, crate::block::BlockFace::from_string(variant_str).unwrap()
)}
	pub fn hit_entity(&mut self) 
-> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHitEntity","()Lorg/bukkit/entity/Entity;",&[]);
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for ProjectileHitEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for ProjectileHitEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityPortalEnterEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityPortalEnterEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityPortalEnterEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityPortalEnterEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityPortalEnterEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityPortalEnterEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Entity<'mc>>,arg1: impl Into<&'mc crate::Location<'mc>>) 
-> Result<crate::event::entity::EntityPortalEnterEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/EntityPortalEnterEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Entity;Lorg/bukkit/Location;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::entity::EntityPortalEnterEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn location(&mut self) 
-> Result<crate::Location<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getLocation","()Lorg/bukkit/Location;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityPortalEnterEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityAirChangeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityAirChangeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityAirChangeEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityAirChangeEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityAirChangeEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityAirChangeEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Entity<'mc>>,arg1: i32) 
-> Result<crate::event::entity::EntityAirChangeEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = jni::objects::JValueGen::Int(arg1.into());
let cls = &jni.find_class("org/bukkit/event/entity/EntityAirChangeEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Entity;I)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::entity::EntityAirChangeEvent::from_raw(&jni,res
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityAirChangeEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityAirChangeEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityUnleashEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
pub struct EntityUnleashEventUnleashReason<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityUnleashEventUnleashReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityUnleashEventUnleashReason<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityUnleashEventUnleashReason from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityUnleashEventUnleashReason")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityUnleashEventUnleashReason object, got {}",
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
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityUnleashEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityUnleashEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityUnleashEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityUnleashEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityUnleashEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Entity<'mc>>,arg1: impl Into<&'mc crate::event::entity::EntityUnleashEventUnleashReason<'mc>>) 
-> Result<crate::event::entity::EntityUnleashEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/EntityUnleashEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Entity;Lorg/bukkit/event/entity/EntityUnleashEvent$UnleashReason;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::entity::EntityUnleashEvent::from_raw(&jni,res
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
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityUnleashEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct ExpBottleEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ExpBottleEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ExpBottleEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate ExpBottleEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "ExpBottleEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a ExpBottleEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::ThrownExpBottle<'mc>>,arg1: i32) 
-> Result<crate::event::entity::ExpBottleEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = jni::objects::JValueGen::Int(arg1.into());
let cls = &jni.find_class("org/bukkit/event/entity/ExpBottleEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/ThrownExpBottle;I)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::entity::ExpBottleEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn entity(&mut self) 
-> Result<crate::entity::ThrownExpBottle<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/ThrownExpBottle;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::ThrownExpBottle::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn experience(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getExperience","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn set_experience(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setExperience","(I)V",&[jni::objects::JValueGen::from(&val_0)]);
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
	pub fn show_effect(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getShowEffect","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_show_effect(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setShowEffect","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
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
	pub fn hit_block(&mut self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHitBlock","()Lorg/bukkit/block/Block;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn hit_block_face(&mut self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHitBlockFace","()Lorg/bukkit/block/BlockFace;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::block::BlockFace::from_raw(&self.jni_ref(),raw_obj
, crate::block::BlockFace::from_string(variant_str).unwrap()
)}
	pub fn hit_entity(&mut self) 
-> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHitEntity","()Lorg/bukkit/entity/Entity;",&[]);
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
impl<'mc> Into<crate::event::entity::ProjectileHitEvent<'mc>> for ExpBottleEvent<'mc>{
   fn into(self) -> crate::event::entity::ProjectileHitEvent<'mc> {
       crate::event::entity::ProjectileHitEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityCombustEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityCombustEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityCombustEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityCombustEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityCombustEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityCombustEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Entity<'mc>>,arg1: i32) 
-> Result<crate::event::entity::EntityCombustEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = jni::objects::JValueGen::Int(arg1.into());
let cls = &jni.find_class("org/bukkit/event/entity/EntityCombustEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Entity;I)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::entity::EntityCombustEvent::from_raw(&jni,res
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
	pub fn duration(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDuration","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn set_duration(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setDuration","(I)V",&[jni::objects::JValueGen::from(&val_0)]);
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityCombustEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityCombustEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntitySpellCastEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntitySpellCastEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntitySpellCastEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntitySpellCastEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntitySpellCastEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntitySpellCastEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Spellcaster<'mc>>,arg1: impl Into<&'mc crate::entity::SpellcasterSpell<'mc>>) 
-> Result<crate::event::entity::EntitySpellCastEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/EntitySpellCastEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Spellcaster;Lorg/bukkit/entity/Spellcaster$Spell;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::entity::EntitySpellCastEvent::from_raw(&jni,res
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
	pub fn spell(&mut self) 
-> Result<crate::entity::SpellcasterSpell<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getSpell","()Lorg/bukkit/entity/Spellcaster$Spell;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::SpellcasterSpell::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn entity(&mut self) 
-> Result<crate::entity::Spellcaster<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/Spellcaster;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Spellcaster::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntitySpellCastEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntitySpellCastEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityTransformEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
pub struct EntityTransformEventTransformReason<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityTransformEventTransformReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityTransformEventTransformReason<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityTransformEventTransformReason from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityTransformEventTransformReason")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityTransformEventTransformReason object, got {}",
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
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityTransformEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityTransformEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityTransformEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityTransformEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityTransformEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Entity<'mc>>,arg1: impl Into<&'mc blackboxmc_java::JavaList<'mc, orgEntity>>,arg2: impl Into<&'mc crate::event::entity::EntityTransformEventTransformReason<'mc>>) 
-> Result<crate::event::entity::EntityTransformEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/EntityTransformEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Entity;Ljava/util/List;Lorg/bukkit/event/entity/EntityTransformEvent$TransformReason;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::entity::EntityTransformEvent::from_raw(&jni,res
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
	pub fn transformed_entity(&mut self) 
-> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getTransformedEntity","()Lorg/bukkit/entity/Entity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Entity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn transformed_entities(&mut self) 
-> Result<blackboxmc_java::JavaList<'mc, orgEntity>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getTransformedEntities","()Ljava/util/List;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn transform_reason(&mut self) 
-> Result<crate::event::entity::EntityTransformEventTransformReason<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getTransformReason","()Lorg/bukkit/event/entity/EntityTransformEvent$TransformReason;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::entity::EntityTransformEventTransformReason::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityTransformEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityTransformEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PotionSplashEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PotionSplashEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PotionSplashEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PotionSplashEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PotionSplashEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PotionSplashEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::ThrownPotion<'mc>>,arg1: impl Into<&'mc blackboxmc_java::JavaMap<'mc, orgLivingEntity, javaDouble>>) 
-> Result<crate::event::entity::PotionSplashEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/PotionSplashEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/ThrownPotion;Ljava/util/Map;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::entity::PotionSplashEvent::from_raw(&jni,res
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
-> Result<crate::entity::ThrownPotion<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/ThrownPotion;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::ThrownPotion::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
	pub fn affected_entities(&mut self) 
-> Result<blackboxmc_java::JavaCollection<'mc, orgLivingEntity>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getAffectedEntities","()Ljava/util/Collection;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaCollection::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn potion(&mut self) 
-> Result<crate::entity::ThrownPotion<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPotion","()Lorg/bukkit/entity/ThrownPotion;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::ThrownPotion::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_intensity(&mut self,arg0: impl Into<&'mc crate::entity::LivingEntity<'mc>>) 
-> Result<f64, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"getIntensity","(Lorg/bukkit/entity/LivingEntity;)D",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
	pub fn set_intensity(&mut self,arg0: impl Into<&'mc crate::entity::LivingEntity<'mc>>,arg1: f64) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = jni::objects::JValueGen::Double(arg1.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setIntensity","(Lorg/bukkit/entity/LivingEntity;D)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn hit_block(&mut self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHitBlock","()Lorg/bukkit/block/Block;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn hit_block_face(&mut self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHitBlockFace","()Lorg/bukkit/block/BlockFace;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::block::BlockFace::from_raw(&self.jni_ref(),raw_obj
, crate::block::BlockFace::from_string(variant_str).unwrap()
)}
	pub fn hit_entity(&mut self) 
-> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHitEntity","()Lorg/bukkit/entity/Entity;",&[]);
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for PotionSplashEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::ProjectileHitEvent<'mc>> for PotionSplashEvent<'mc>{
   fn into(self) -> crate::event::entity::ProjectileHitEvent<'mc> {
       crate::event::entity::ProjectileHitEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntityInteractEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntityInteractEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityInteractEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntityInteractEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntityInteractEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntityInteractEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Entity<'mc>>,arg1: impl Into<&'mc crate::block::Block<'mc>>) 
-> Result<crate::event::entity::EntityInteractEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/EntityInteractEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Entity;Lorg/bukkit/block/Block;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::entity::EntityInteractEvent::from_raw(&jni,res
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
	pub fn block(&mut self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBlock","()Lorg/bukkit/block/Block;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityInteractEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityInteractEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntitySpawnEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntitySpawnEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntitySpawnEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntitySpawnEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "EntitySpawnEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntitySpawnEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Entity<'mc>>) 
-> Result<crate::event::entity::EntitySpawnEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/EntitySpawnEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Entity;)V",&[jni::objects::JValueGen::from(&val_0)])?;
crate::event::entity::EntitySpawnEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn location(&mut self) 
-> Result<crate::Location<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getLocation","()Lorg/bukkit/Location;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntitySpawnEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntitySpawnEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct VillagerReplenishTradeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for VillagerReplenishTradeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> VillagerReplenishTradeEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate VillagerReplenishTradeEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "VillagerReplenishTradeEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a VillagerReplenishTradeEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::AbstractVillager<'mc>>,arg1: impl Into<&'mc crate::inventory::MerchantRecipe<'mc>>) 
-> Result<crate::event::entity::VillagerReplenishTradeEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/entity/VillagerReplenishTradeEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/AbstractVillager;Lorg/bukkit/inventory/MerchantRecipe;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::entity::VillagerReplenishTradeEvent::from_raw(&jni,res
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
-> Result<crate::inventory::MerchantRecipe<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getRecipe","()Lorg/bukkit/inventory/MerchantRecipe;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::MerchantRecipe::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_recipe(&mut self,arg0: impl Into<&'mc crate::inventory::MerchantRecipe<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setRecipe","(Lorg/bukkit/inventory/MerchantRecipe;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn entity(&mut self) 
-> Result<crate::entity::AbstractVillager<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEntity","()Lorg/bukkit/entity/AbstractVillager;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::AbstractVillager::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
#[deprecated]
	pub fn bonus(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBonus","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
#[deprecated]
	pub fn set_bonus(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setBonus","(I)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for VillagerReplenishTradeEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for VillagerReplenishTradeEvent<'mc>{
   fn into(self) -> crate::event::entity::EntityEvent<'mc> {
       crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
