#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct WorldSaveEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for WorldSaveEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for WorldSaveEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate WorldSaveEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/WorldSaveEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a WorldSaveEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> WorldSaveEvent<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,world: impl Into<crate::World<'mc>>) 
-> Result<crate::event::world::WorldSaveEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/World;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(world.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/world/WorldSaveEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::world::WorldSaveEvent::from_raw(&jni,res
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/world/WorldSaveEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: WorldEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::world::WorldEvent<'mc>> for WorldSaveEvent<'mc>{

fn into(self) -> crate::event::world::WorldEvent<'mc> {

crate::event::world::WorldEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting WorldSaveEvent into crate::event::world::WorldEvent")

   }
}
#[repr(C)]
pub struct ChunkUnloadEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ChunkUnloadEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ChunkUnloadEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ChunkUnloadEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/ChunkUnloadEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ChunkUnloadEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ChunkUnloadEvent<'mc> {
	pub fn new_with_chunk(jni: &blackboxmc_general::SharedJNIEnv<'mc>,chunk: impl Into<crate::Chunk<'mc>>,save: std::option::Option<bool>) 
-> Result<crate::event::world::ChunkUnloadEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/Chunk;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(chunk.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = save {
sig += "Z";
let val_2 = jni::objects::JValueGen::Bool(a.into());
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/world/ChunkUnloadEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::world::ChunkUnloadEvent::from_raw(&jni,res
)}
	pub fn is_save_chunk(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isSaveChunk",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn set_save_chunk(&self,save_chunk: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)L();");
let val_1 = jni::objects::JValueGen::Bool(save_chunk.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setSaveChunk",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/world/ChunkUnloadEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: ChunkEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::world::ChunkEvent<'mc>> for ChunkUnloadEvent<'mc>{

fn into(self) -> crate::event::world::ChunkEvent<'mc> {

crate::event::world::ChunkEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting ChunkUnloadEvent into crate::event::world::ChunkEvent")

   }
}
#[repr(C)]
pub struct ChunkEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ChunkEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ChunkEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ChunkEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/ChunkEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ChunkEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ChunkEvent<'mc> {
	pub fn chunk(&self) 
-> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::Chunk;");
let res = self.jni_ref().call_method(&self.jni_object(),"getChunk",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Chunk::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
// SUPER CLASS: WorldEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::world::WorldEvent<'mc>> for ChunkEvent<'mc>{

fn into(self) -> crate::event::world::WorldEvent<'mc> {

crate::event::world::WorldEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting ChunkEvent into crate::event::world::WorldEvent")

   }
}
#[repr(C)]
pub struct PortalCreateEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PortalCreateEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PortalCreateEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PortalCreateEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/PortalCreateEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PortalCreateEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PortalCreateEvent<'mc> {
	pub fn new_with_blocks(jni: &blackboxmc_general::SharedJNIEnv<'mc>,blocks: Vec<jni::objects::JObject<'mc>>,world: impl Into<crate::World<'mc>>,entity: impl Into<crate::entity::Entity<'mc>>,reason: std::option::Option<impl Into<crate::event::world::PortalCreateEventCreateReason<'mc>>>) 
-> Result<crate::event::world::PortalCreateEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/util/List;";
let raw_val_1 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
for v in blocks{
sig += "L/jni::objects::JObject;";
		let map_val_0 = jni::objects::JValueGen::Object(v);
jni.call_method(&raw_val_1,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_1 = jni::objects::JValueGen::Object(raw_val_1);
args.push(val_1);
sig += "Lorg/bukkit/World;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(world.into().jni_object().clone())});
args.push(val_2);
sig += "Lorg/bukkit/entity/Entity;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(entity.into().jni_object().clone())});
args.push(val_3);
if let Some(a) = reason {
sig += "Lorg/bukkit/event/world/PortalCreateEvent/CreateReason;";
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_4);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/world/PortalCreateEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::world::PortalCreateEvent::from_raw(&jni,res
)}
	pub fn blocks(&self) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()LVec;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBlocks",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn entity(&self) 
-> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::entity::Entity;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEntity",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::entity::Entity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)L();");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn reason(&self) 
-> Result<crate::event::world::PortalCreateEventCreateReason<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::world::PortalCreateEventCreateReason;");
let res = self.jni_ref().call_method(&self.jni_object(),"getReason",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::world::PortalCreateEventCreateReason::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/world/PortalCreateEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: WorldEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PortalCreateEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).expect("Error converting PortalCreateEvent into crate::event::Cancellable")

   }
}
impl<'mc> Into<crate::event::world::WorldEvent<'mc>> for PortalCreateEvent<'mc>{

fn into(self) -> crate::event::world::WorldEvent<'mc> {

crate::event::world::WorldEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting PortalCreateEvent into crate::event::world::WorldEvent")

   }
}
#[repr(C)]
pub struct ChunkLoadEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ChunkLoadEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ChunkLoadEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ChunkLoadEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/ChunkLoadEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ChunkLoadEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ChunkLoadEvent<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,chunk: impl Into<crate::Chunk<'mc>>,new_chunk: bool) 
-> Result<crate::event::world::ChunkLoadEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Chunk;Z)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(chunk.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Bool(new_chunk.into());
let cls = jni.find_class("org/bukkit/event/world/ChunkLoadEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::world::ChunkLoadEvent::from_raw(&jni,res
)}
	pub fn is_new_chunk(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isNewChunk",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/world/ChunkLoadEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: ChunkEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::world::ChunkEvent<'mc>> for ChunkLoadEvent<'mc>{

fn into(self) -> crate::event::world::ChunkEvent<'mc> {

crate::event::world::ChunkEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting ChunkLoadEvent into crate::event::world::ChunkEvent")

   }
}
#[repr(C)]
pub struct AsyncStructureGenerateEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for AsyncStructureGenerateEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for AsyncStructureGenerateEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate AsyncStructureGenerateEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/AsyncStructureGenerateEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a AsyncStructureGenerateEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> AsyncStructureGenerateEvent<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,world: impl Into<crate::World<'mc>>,val_async: bool,cause: impl Into<crate::event::world::AsyncStructureGenerateEventCause<'mc>>,structure: impl Into<crate::generator::structure::Structure<'mc>>,bounding_box: impl Into<crate::util::BoundingBox<'mc>>,chunk_x: i32,chunk_z: i32) 
-> Result<crate::event::world::AsyncStructureGenerateEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/World;ZLorg/bukkit/event/world/AsyncStructureGenerateEvent/Cause;Lorg/bukkit/generator/structure/Structure;Lorg/bukkit/util/BoundingBox;II)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(world.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Bool(val_async.into());
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(cause.into().jni_object().clone())});
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(structure.into().jni_object().clone())});
let val_5 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(bounding_box.into().jni_object().clone())});
let val_6 = jni::objects::JValueGen::Int(chunk_x);
let val_7 = jni::objects::JValueGen::Int(chunk_z);
let cls = jni.find_class("org/bukkit/event/world/AsyncStructureGenerateEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4),jni::objects::JValueGen::from(val_5),jni::objects::JValueGen::from(val_6),jni::objects::JValueGen::from(val_7)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::world::AsyncStructureGenerateEvent::from_raw(&jni,res
)}
	pub fn cause(&self) 
-> Result<crate::event::world::AsyncStructureGenerateEventCause<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::world::AsyncStructureGenerateEventCause;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCause",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::world::AsyncStructureGenerateEventCause::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_block_transformer(&self,key: impl Into<crate::NamespacedKey<'mc>>) 
-> Result<Option<crate::util::BlockTransformer<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/NamespacedKey;)Lcrate::util::BlockTransformer;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(key.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"getBlockTransformer",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::util::BlockTransformer::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn set_block_transformer(&self,key: impl Into<crate::NamespacedKey<'mc>>,transformer: impl Into<crate::util::BlockTransformer<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/NamespacedKey;Lorg/bukkit/util/BlockTransformer;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(key.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(transformer.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setBlockTransformer",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn remove_block_transformer(&self,key: impl Into<crate::NamespacedKey<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/NamespacedKey;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(key.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"removeBlockTransformer",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn clear_block_transformers(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()L();");
let res = self.jni_ref().call_method(&self.jni_object(),"clearBlockTransformers",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn block_transformers(&self) 
-> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lblackboxmc_java::util::Map;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBlockTransformers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_entity_transformer(&self,key: impl Into<crate::NamespacedKey<'mc>>) 
-> Result<Option<crate::util::EntityTransformer<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/NamespacedKey;)Lcrate::util::EntityTransformer;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(key.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"getEntityTransformer",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::util::EntityTransformer::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn set_entity_transformer(&self,key: impl Into<crate::NamespacedKey<'mc>>,transformer: impl Into<crate::util::EntityTransformer<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/NamespacedKey;Lorg/bukkit/util/EntityTransformer;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(key.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(transformer.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setEntityTransformer",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn remove_entity_transformer(&self,key: impl Into<crate::NamespacedKey<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/NamespacedKey;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(key.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"removeEntityTransformer",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn clear_entity_transformers(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()L();");
let res = self.jni_ref().call_method(&self.jni_object(),"clearEntityTransformers",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn entity_transformers(&self) 
-> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lblackboxmc_java::util::Map;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEntityTransformers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn structure(&self) 
-> Result<crate::generator::structure::Structure<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::generator::structure::Structure;");
let res = self.jni_ref().call_method(&self.jni_object(),"getStructure",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::generator::structure::Structure::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn bounding_box(&self) 
-> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::util::BoundingBox;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBoundingBox",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::BoundingBox::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn chunk_x(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getChunkX",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn chunk_z(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getChunkZ",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/world/AsyncStructureGenerateEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: WorldEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::world::WorldEvent<'mc>> for AsyncStructureGenerateEvent<'mc>{

fn into(self) -> crate::event::world::WorldEvent<'mc> {

crate::event::world::WorldEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting AsyncStructureGenerateEvent into crate::event::world::WorldEvent")

   }
}
#[repr(C)]
pub struct WorldEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for WorldEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for WorldEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate WorldEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/WorldEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a WorldEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> WorldEvent<'mc> {
	pub fn new_with_world(jni: &blackboxmc_general::SharedJNIEnv<'mc>,world: impl Into<crate::World<'mc>>,is_async: std::option::Option<bool>) 
-> Result<crate::event::world::WorldEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/World;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(world.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = is_async {
sig += "Z";
let val_2 = jni::objects::JValueGen::Bool(a.into());
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/world/WorldEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::world::WorldEvent::from_raw(&jni,res
)}
	pub fn world(&self) 
-> Result<crate::World<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::World;");
let res = self.jni_ref().call_method(&self.jni_object(),"getWorld",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::World::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
// SUPER CLASS: Event

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Event<'mc>> for WorldEvent<'mc>{

fn into(self) -> crate::event::Event<'mc> {

crate::event::Event::from_raw(&self.jni_ref(), self.1).expect("Error converting WorldEvent into crate::event::Event")

   }
}
#[repr(C)]
pub struct StructureGrowEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for StructureGrowEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for StructureGrowEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate StructureGrowEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/StructureGrowEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a StructureGrowEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> StructureGrowEvent<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,location: impl Into<crate::Location<'mc>>,species: impl Into<crate::TreeType<'mc>>,bonemeal: bool,player: impl Into<crate::entity::Player<'mc>>,blocks: Vec<jni::objects::JObject<'mc>>) 
-> Result<crate::event::world::StructureGrowEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Location;Lorg/bukkit/TreeType;ZLorg/bukkit/entity/Player;Ljava/util/List;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(location.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(species.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Bool(bonemeal.into());
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let raw_val_5 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
for v in blocks{
let map_val_0 = jni::objects::JValueGen::Object(v);
jni.call_method(&raw_val_5,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_5 = jni::objects::JValueGen::Object(raw_val_5);
let cls = jni.find_class("org/bukkit/event/world/StructureGrowEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4),jni::objects::JValueGen::from(val_5)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::world::StructureGrowEvent::from_raw(&jni,res
)}
	pub fn location(&self) 
-> Result<crate::Location<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::Location;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLocation",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn species(&self) 
-> Result<crate::TreeType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::TreeType;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSpecies",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::TreeType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_from_bonemeal(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isFromBonemeal",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn player(&self) 
-> Result<Option<crate::entity::Player<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::entity::Player;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn blocks(&self) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()LVec;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBlocks",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)L();");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/world/StructureGrowEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: WorldEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for StructureGrowEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).expect("Error converting StructureGrowEvent into crate::event::Cancellable")

   }
}
impl<'mc> Into<crate::event::world::WorldEvent<'mc>> for StructureGrowEvent<'mc>{

fn into(self) -> crate::event::world::WorldEvent<'mc> {

crate::event::world::WorldEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting StructureGrowEvent into crate::event::world::WorldEvent")

   }
}
#[repr(C)]
pub struct EntitiesUnloadEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for EntitiesUnloadEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for EntitiesUnloadEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate EntitiesUnloadEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/EntitiesUnloadEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntitiesUnloadEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> EntitiesUnloadEvent<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,chunk: impl Into<crate::Chunk<'mc>>,entities: Vec<jni::objects::JObject<'mc>>) 
-> Result<crate::event::world::EntitiesUnloadEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Chunk;Ljava/util/List;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(chunk.into().jni_object().clone())});
let raw_val_2 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
for v in entities{
let map_val_0 = jni::objects::JValueGen::Object(v);
jni.call_method(&raw_val_2,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_2 = jni::objects::JValueGen::Object(raw_val_2);
let cls = jni.find_class("org/bukkit/event/world/EntitiesUnloadEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::world::EntitiesUnloadEvent::from_raw(&jni,res
)}
	pub fn entities(&self) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()LVec;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEntities",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/world/EntitiesUnloadEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: ChunkEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::world::ChunkEvent<'mc>> for EntitiesUnloadEvent<'mc>{

fn into(self) -> crate::event::world::ChunkEvent<'mc> {

crate::event::world::ChunkEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting EntitiesUnloadEvent into crate::event::world::ChunkEvent")

   }
}
#[repr(C)]
pub struct WorldLoadEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for WorldLoadEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for WorldLoadEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate WorldLoadEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/WorldLoadEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a WorldLoadEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> WorldLoadEvent<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,world: impl Into<crate::World<'mc>>) 
-> Result<crate::event::world::WorldLoadEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/World;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(world.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/world/WorldLoadEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::world::WorldLoadEvent::from_raw(&jni,res
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/world/WorldLoadEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: WorldEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::world::WorldEvent<'mc>> for WorldLoadEvent<'mc>{

fn into(self) -> crate::event::world::WorldEvent<'mc> {

crate::event::world::WorldEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting WorldLoadEvent into crate::event::world::WorldEvent")

   }
}
#[repr(C)]
pub struct TimeSkipEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for TimeSkipEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for TimeSkipEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate TimeSkipEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/TimeSkipEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a TimeSkipEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> TimeSkipEvent<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,world: impl Into<crate::World<'mc>>,skip_reason: impl Into<crate::event::world::TimeSkipEventSkipReason<'mc>>,skip_amount: i64) 
-> Result<crate::event::world::TimeSkipEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/World;Lorg/bukkit/event/world/TimeSkipEvent/SkipReason;J)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(world.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(skip_reason.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Long(skip_amount);
let cls = jni.find_class("org/bukkit/event/world/TimeSkipEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::world::TimeSkipEvent::from_raw(&jni,res
)}
	pub fn skip_reason(&self) 
-> Result<crate::event::world::TimeSkipEventSkipReason<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::world::TimeSkipEventSkipReason;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSkipReason",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::world::TimeSkipEventSkipReason::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn skip_amount(&self) 
-> Result<i64, Box<dyn std::error::Error>>

{let sig = String::from("()Li64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSkipAmount",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.j()?
)}
	pub fn set_skip_amount(&self,skip_amount: i64) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(J)L();");
let val_1 = jni::objects::JValueGen::Long(skip_amount);
let res = self.jni_ref().call_method(&self.jni_object(),"setSkipAmount",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)L();");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/world/TimeSkipEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: WorldEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for TimeSkipEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).expect("Error converting TimeSkipEvent into crate::event::Cancellable")

   }
}
impl<'mc> Into<crate::event::world::WorldEvent<'mc>> for TimeSkipEvent<'mc>{

fn into(self) -> crate::event::world::WorldEvent<'mc> {

crate::event::world::WorldEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting TimeSkipEvent into crate::event::world::WorldEvent")

   }
}
pub enum AsyncStructureGenerateEventCause<'mc> {
	Command {inner: AsyncStructureGenerateEventCauseStruct<'mc>},
	WorldGeneration {inner: AsyncStructureGenerateEventCauseStruct<'mc>},
	Custom {inner: AsyncStructureGenerateEventCauseStruct<'mc>},
}
impl<'mc> std::fmt::Display for AsyncStructureGenerateEventCause<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           AsyncStructureGenerateEventCause::Command { .. } => f.write_str("COMMAND"),
           AsyncStructureGenerateEventCause::WorldGeneration { .. } => f.write_str("WORLD_GENERATION"),
           AsyncStructureGenerateEventCause::Custom { .. } => f.write_str("CUSTOM"),
       }
   }
}

        impl<'mc> AsyncStructureGenerateEventCause<'mc> {
            pub fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<AsyncStructureGenerateEventCause<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/event/world/AsyncStructureGenerateEvent/Cause");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/event/world/AsyncStructureGenerateEvent/Cause;",
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
                    
"COMMAND" => Ok(AsyncStructureGenerateEventCause::Command { inner: AsyncStructureGenerateEventCauseStruct::from_raw(env,obj)?}),
"WORLD_GENERATION" => Ok(AsyncStructureGenerateEventCause::WorldGeneration { inner: AsyncStructureGenerateEventCauseStruct::from_raw(env,obj)?}),
"CUSTOM" => Ok(AsyncStructureGenerateEventCause::Custom { inner: AsyncStructureGenerateEventCauseStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct AsyncStructureGenerateEventCauseStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for AsyncStructureGenerateEventCause<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::Command { inner } => inner.0.clone(),
Self::WorldGeneration { inner } => inner.0.clone(),
Self::Custom { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::Command { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::WorldGeneration { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Custom { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for AsyncStructureGenerateEventCause<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate AsyncStructureGenerateEventCause from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/AsyncStructureGenerateEvent/Cause")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a AsyncStructureGenerateEventCause object, got {}",
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
                    "COMMAND" => Ok(AsyncStructureGenerateEventCause::Command { inner: AsyncStructureGenerateEventCauseStruct::from_raw(env,obj)?}),"WORLD_GENERATION" => Ok(AsyncStructureGenerateEventCause::WorldGeneration { inner: AsyncStructureGenerateEventCauseStruct::from_raw(env,obj)?}),"CUSTOM" => Ok(AsyncStructureGenerateEventCause::Custom { inner: AsyncStructureGenerateEventCauseStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for AsyncStructureGenerateEventCauseStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for AsyncStructureGenerateEventCauseStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate AsyncStructureGenerateEventCauseStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/AsyncStructureGenerateEvent/Cause")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a AsyncStructureGenerateEventCauseStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> AsyncStructureGenerateEventCauseStruct<'mc> {
	pub fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::world::AsyncStructureGenerateEventCause<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::world::AsyncStructureGenerateEventCause;");
let cls = jni.find_class("org/bukkit/event/world/AsyncStructureGenerateEvent/Cause"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::world::AsyncStructureGenerateEventCause::from_raw(&jni,obj
)}
// SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct ChunkPopulateEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ChunkPopulateEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ChunkPopulateEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ChunkPopulateEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/ChunkPopulateEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ChunkPopulateEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ChunkPopulateEvent<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,chunk: impl Into<crate::Chunk<'mc>>) 
-> Result<crate::event::world::ChunkPopulateEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Chunk;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(chunk.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/world/ChunkPopulateEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::world::ChunkPopulateEvent::from_raw(&jni,res
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/world/ChunkPopulateEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: ChunkEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::world::ChunkEvent<'mc>> for ChunkPopulateEvent<'mc>{

fn into(self) -> crate::event::world::ChunkEvent<'mc> {

crate::event::world::ChunkEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting ChunkPopulateEvent into crate::event::world::ChunkEvent")

   }
}
#[repr(C)]
pub struct AsyncStructureSpawnEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for AsyncStructureSpawnEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for AsyncStructureSpawnEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate AsyncStructureSpawnEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/AsyncStructureSpawnEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a AsyncStructureSpawnEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> AsyncStructureSpawnEvent<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,world: impl Into<crate::World<'mc>>,structure: impl Into<crate::generator::structure::Structure<'mc>>,bounding_box: impl Into<crate::util::BoundingBox<'mc>>,chunk_x: i32,chunk_z: i32) 
-> Result<crate::event::world::AsyncStructureSpawnEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/World;Lorg/bukkit/generator/structure/Structure;Lorg/bukkit/util/BoundingBox;II)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(world.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(structure.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(bounding_box.into().jni_object().clone())});
let val_4 = jni::objects::JValueGen::Int(chunk_x);
let val_5 = jni::objects::JValueGen::Int(chunk_z);
let cls = jni.find_class("org/bukkit/event/world/AsyncStructureSpawnEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4),jni::objects::JValueGen::from(val_5)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::world::AsyncStructureSpawnEvent::from_raw(&jni,res
)}
	pub fn structure(&self) 
-> Result<crate::generator::structure::Structure<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::generator::structure::Structure;");
let res = self.jni_ref().call_method(&self.jni_object(),"getStructure",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::generator::structure::Structure::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn bounding_box(&self) 
-> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::util::BoundingBox;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBoundingBox",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::BoundingBox::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn chunk_x(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getChunkX",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn chunk_z(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getChunkZ",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)L();");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/world/AsyncStructureSpawnEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: WorldEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for AsyncStructureSpawnEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).expect("Error converting AsyncStructureSpawnEvent into crate::event::Cancellable")

   }
}
impl<'mc> Into<crate::event::world::WorldEvent<'mc>> for AsyncStructureSpawnEvent<'mc>{

fn into(self) -> crate::event::world::WorldEvent<'mc> {

crate::event::world::WorldEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting AsyncStructureSpawnEvent into crate::event::world::WorldEvent")

   }
}
pub enum PortalCreateEventCreateReason<'mc> {
	Fire {inner: PortalCreateEventCreateReasonStruct<'mc>},
	NetherPair {inner: PortalCreateEventCreateReasonStruct<'mc>},
	EndPlatform {inner: PortalCreateEventCreateReasonStruct<'mc>},
}
impl<'mc> std::fmt::Display for PortalCreateEventCreateReason<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           PortalCreateEventCreateReason::Fire { .. } => f.write_str("FIRE"),
           PortalCreateEventCreateReason::NetherPair { .. } => f.write_str("NETHER_PAIR"),
           PortalCreateEventCreateReason::EndPlatform { .. } => f.write_str("END_PLATFORM"),
       }
   }
}

        impl<'mc> PortalCreateEventCreateReason<'mc> {
            pub fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<PortalCreateEventCreateReason<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/event/world/PortalCreateEvent/CreateReason");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/event/world/PortalCreateEvent/CreateReason;",
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
                    
"FIRE" => Ok(PortalCreateEventCreateReason::Fire { inner: PortalCreateEventCreateReasonStruct::from_raw(env,obj)?}),
"NETHER_PAIR" => Ok(PortalCreateEventCreateReason::NetherPair { inner: PortalCreateEventCreateReasonStruct::from_raw(env,obj)?}),
"END_PLATFORM" => Ok(PortalCreateEventCreateReason::EndPlatform { inner: PortalCreateEventCreateReasonStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct PortalCreateEventCreateReasonStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PortalCreateEventCreateReason<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::Fire { inner } => inner.0.clone(),
Self::NetherPair { inner } => inner.0.clone(),
Self::EndPlatform { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::Fire { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::NetherPair { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::EndPlatform { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for PortalCreateEventCreateReason<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PortalCreateEventCreateReason from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/PortalCreateEvent/CreateReason")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PortalCreateEventCreateReason object, got {}",
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
                    "FIRE" => Ok(PortalCreateEventCreateReason::Fire { inner: PortalCreateEventCreateReasonStruct::from_raw(env,obj)?}),"NETHER_PAIR" => Ok(PortalCreateEventCreateReason::NetherPair { inner: PortalCreateEventCreateReasonStruct::from_raw(env,obj)?}),"END_PLATFORM" => Ok(PortalCreateEventCreateReason::EndPlatform { inner: PortalCreateEventCreateReasonStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for PortalCreateEventCreateReasonStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PortalCreateEventCreateReasonStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PortalCreateEventCreateReasonStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/PortalCreateEvent/CreateReason")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PortalCreateEventCreateReasonStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PortalCreateEventCreateReasonStruct<'mc> {
	pub fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::world::PortalCreateEventCreateReason<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::world::PortalCreateEventCreateReason;");
let cls = jni.find_class("org/bukkit/event/world/PortalCreateEvent/CreateReason"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::world::PortalCreateEventCreateReason::from_raw(&jni,obj
)}
// SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct SpawnChangeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for SpawnChangeEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for SpawnChangeEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate SpawnChangeEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/SpawnChangeEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a SpawnChangeEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> SpawnChangeEvent<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,world: impl Into<crate::World<'mc>>,previous_location: impl Into<crate::Location<'mc>>) 
-> Result<crate::event::world::SpawnChangeEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/World;Lorg/bukkit/Location;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(world.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(previous_location.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/world/SpawnChangeEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::world::SpawnChangeEvent::from_raw(&jni,res
)}
	pub fn previous_location(&self) 
-> Result<crate::Location<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::Location;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPreviousLocation",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/world/SpawnChangeEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: WorldEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::world::WorldEvent<'mc>> for SpawnChangeEvent<'mc>{

fn into(self) -> crate::event::world::WorldEvent<'mc> {

crate::event::world::WorldEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting SpawnChangeEvent into crate::event::world::WorldEvent")

   }
}
#[repr(C)]
pub struct WorldInitEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for WorldInitEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for WorldInitEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate WorldInitEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/WorldInitEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a WorldInitEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> WorldInitEvent<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,world: impl Into<crate::World<'mc>>) 
-> Result<crate::event::world::WorldInitEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/World;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(world.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/world/WorldInitEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::world::WorldInitEvent::from_raw(&jni,res
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/world/WorldInitEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: WorldEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::world::WorldEvent<'mc>> for WorldInitEvent<'mc>{

fn into(self) -> crate::event::world::WorldEvent<'mc> {

crate::event::world::WorldEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting WorldInitEvent into crate::event::world::WorldEvent")

   }
}
pub enum TimeSkipEventSkipReason<'mc> {
	Command {inner: TimeSkipEventSkipReasonStruct<'mc>},
	Custom {inner: TimeSkipEventSkipReasonStruct<'mc>},
	NightSkip {inner: TimeSkipEventSkipReasonStruct<'mc>},
}
impl<'mc> std::fmt::Display for TimeSkipEventSkipReason<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           TimeSkipEventSkipReason::Command { .. } => f.write_str("COMMAND"),
           TimeSkipEventSkipReason::Custom { .. } => f.write_str("CUSTOM"),
           TimeSkipEventSkipReason::NightSkip { .. } => f.write_str("NIGHT_SKIP"),
       }
   }
}

        impl<'mc> TimeSkipEventSkipReason<'mc> {
            pub fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<TimeSkipEventSkipReason<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/event/world/TimeSkipEvent/SkipReason");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/event/world/TimeSkipEvent/SkipReason;",
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
                    
"COMMAND" => Ok(TimeSkipEventSkipReason::Command { inner: TimeSkipEventSkipReasonStruct::from_raw(env,obj)?}),
"CUSTOM" => Ok(TimeSkipEventSkipReason::Custom { inner: TimeSkipEventSkipReasonStruct::from_raw(env,obj)?}),
"NIGHT_SKIP" => Ok(TimeSkipEventSkipReason::NightSkip { inner: TimeSkipEventSkipReasonStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct TimeSkipEventSkipReasonStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for TimeSkipEventSkipReason<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::Command { inner } => inner.0.clone(),
Self::Custom { inner } => inner.0.clone(),
Self::NightSkip { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::Command { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Custom { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::NightSkip { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for TimeSkipEventSkipReason<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate TimeSkipEventSkipReason from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/TimeSkipEvent/SkipReason")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a TimeSkipEventSkipReason object, got {}",
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
                    "COMMAND" => Ok(TimeSkipEventSkipReason::Command { inner: TimeSkipEventSkipReasonStruct::from_raw(env,obj)?}),"CUSTOM" => Ok(TimeSkipEventSkipReason::Custom { inner: TimeSkipEventSkipReasonStruct::from_raw(env,obj)?}),"NIGHT_SKIP" => Ok(TimeSkipEventSkipReason::NightSkip { inner: TimeSkipEventSkipReasonStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for TimeSkipEventSkipReasonStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for TimeSkipEventSkipReasonStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate TimeSkipEventSkipReasonStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/TimeSkipEvent/SkipReason")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a TimeSkipEventSkipReasonStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> TimeSkipEventSkipReasonStruct<'mc> {
	pub fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::world::TimeSkipEventSkipReason<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::world::TimeSkipEventSkipReason;");
let cls = jni.find_class("org/bukkit/event/world/TimeSkipEvent/SkipReason"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::world::TimeSkipEventSkipReason::from_raw(&jni,obj
)}
// SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct GenericGameEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for GenericGameEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for GenericGameEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate GenericGameEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/GenericGameEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a GenericGameEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> GenericGameEvent<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,event: impl Into<crate::GameEvent<'mc>>,location: impl Into<crate::Location<'mc>>,entity: impl Into<crate::entity::Entity<'mc>>,radius: i32,is_async: bool) 
-> Result<crate::event::world::GenericGameEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/GameEvent;Lorg/bukkit/Location;Lorg/bukkit/entity/Entity;IZ)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(event.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(location.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(entity.into().jni_object().clone())});
let val_4 = jni::objects::JValueGen::Int(radius);
let val_5 = jni::objects::JValueGen::Bool(is_async.into());
let cls = jni.find_class("org/bukkit/event/world/GenericGameEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4),jni::objects::JValueGen::from(val_5)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::world::GenericGameEvent::from_raw(&jni,res
)}
	pub fn event(&self) 
-> Result<crate::GameEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::GameEvent;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEvent",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::GameEvent::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn location(&self) 
-> Result<crate::Location<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::Location;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLocation",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn entity(&self) 
-> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::entity::Entity;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEntity",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::entity::Entity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn radius(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRadius",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn set_radius(&self,radius: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)L();");
let val_1 = jni::objects::JValueGen::Int(radius);
let res = self.jni_ref().call_method(&self.jni_object(),"setRadius",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)L();");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/world/GenericGameEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: WorldEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for GenericGameEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).expect("Error converting GenericGameEvent into crate::event::Cancellable")

   }
}
impl<'mc> Into<crate::event::world::WorldEvent<'mc>> for GenericGameEvent<'mc>{

fn into(self) -> crate::event::world::WorldEvent<'mc> {

crate::event::world::WorldEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting GenericGameEvent into crate::event::world::WorldEvent")

   }
}
#[repr(C)]
pub struct EntitiesLoadEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for EntitiesLoadEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for EntitiesLoadEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate EntitiesLoadEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/EntitiesLoadEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntitiesLoadEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> EntitiesLoadEvent<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,chunk: impl Into<crate::Chunk<'mc>>,entities: Vec<jni::objects::JObject<'mc>>) 
-> Result<crate::event::world::EntitiesLoadEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Chunk;Ljava/util/List;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(chunk.into().jni_object().clone())});
let raw_val_2 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
for v in entities{
let map_val_0 = jni::objects::JValueGen::Object(v);
jni.call_method(&raw_val_2,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_2 = jni::objects::JValueGen::Object(raw_val_2);
let cls = jni.find_class("org/bukkit/event/world/EntitiesLoadEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::world::EntitiesLoadEvent::from_raw(&jni,res
)}
	pub fn entities(&self) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()LVec;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEntities",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/world/EntitiesLoadEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: ChunkEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::world::ChunkEvent<'mc>> for EntitiesLoadEvent<'mc>{

fn into(self) -> crate::event::world::ChunkEvent<'mc> {

crate::event::world::ChunkEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting EntitiesLoadEvent into crate::event::world::ChunkEvent")

   }
}
#[repr(C)]
pub struct LootGenerateEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for LootGenerateEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for LootGenerateEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate LootGenerateEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/LootGenerateEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a LootGenerateEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> LootGenerateEvent<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,world: impl Into<crate::World<'mc>>,entity: impl Into<crate::entity::Entity<'mc>>,inventory_holder: impl Into<crate::inventory::InventoryHolder<'mc>>,loot_table: impl Into<crate::loot::LootTable<'mc>>,loot_context: impl Into<crate::loot::LootContext<'mc>>,items: Vec<jni::objects::JObject<'mc>>,plugin: bool) 
-> Result<crate::event::world::LootGenerateEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/World;Lorg/bukkit/entity/Entity;Lorg/bukkit/inventory/InventoryHolder;Lorg/bukkit/loot/LootTable;Lorg/bukkit/loot/LootContext;Ljava/util/List;Z)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(world.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(entity.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(inventory_holder.into().jni_object().clone())});
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(loot_table.into().jni_object().clone())});
let val_5 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(loot_context.into().jni_object().clone())});
let raw_val_6 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
for v in items{
let map_val_0 = jni::objects::JValueGen::Object(v);
jni.call_method(&raw_val_6,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_6 = jni::objects::JValueGen::Object(raw_val_6);
let val_7 = jni::objects::JValueGen::Bool(plugin.into());
let cls = jni.find_class("org/bukkit/event/world/LootGenerateEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4),jni::objects::JValueGen::from(val_5),jni::objects::JValueGen::from(val_6),jni::objects::JValueGen::from(val_7)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::world::LootGenerateEvent::from_raw(&jni,res
)}
	pub fn entity(&self) 
-> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::entity::Entity;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEntity",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::entity::Entity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn inventory_holder(&self) 
-> Result<Option<crate::inventory::InventoryHolder<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::InventoryHolder;");
let res = self.jni_ref().call_method(&self.jni_object(),"getInventoryHolder",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::InventoryHolder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn loot_table(&self) 
-> Result<crate::loot::LootTable<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::loot::LootTable;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLootTable",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::loot::LootTable::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn loot_context(&self) 
-> Result<crate::loot::LootContext<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::loot::LootContext;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLootContext",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::loot::LootContext::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn loot(&self) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()LVec;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLoot",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn is_plugin(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isPlugin",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)L();");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/world/LootGenerateEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: WorldEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for LootGenerateEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).expect("Error converting LootGenerateEvent into crate::event::Cancellable")

   }
}
impl<'mc> Into<crate::event::world::WorldEvent<'mc>> for LootGenerateEvent<'mc>{

fn into(self) -> crate::event::world::WorldEvent<'mc> {

crate::event::world::WorldEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting LootGenerateEvent into crate::event::world::WorldEvent")

   }
}
#[repr(C)]
pub struct WorldUnloadEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for WorldUnloadEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for WorldUnloadEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate WorldUnloadEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/WorldUnloadEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a WorldUnloadEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> WorldUnloadEvent<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,world: impl Into<crate::World<'mc>>) 
-> Result<crate::event::world::WorldUnloadEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/World;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(world.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/world/WorldUnloadEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::world::WorldUnloadEvent::from_raw(&jni,res
)}
	pub fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)L();");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/world/WorldUnloadEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: WorldEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for WorldUnloadEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).expect("Error converting WorldUnloadEvent into crate::event::Cancellable")

   }
}
impl<'mc> Into<crate::event::world::WorldEvent<'mc>> for WorldUnloadEvent<'mc>{

fn into(self) -> crate::event::world::WorldEvent<'mc> {

crate::event::world::WorldEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting WorldUnloadEvent into crate::event::world::WorldEvent")

   }
}
