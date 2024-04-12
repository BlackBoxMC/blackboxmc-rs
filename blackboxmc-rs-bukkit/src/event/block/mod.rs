#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;/*org/bukkit/event/block/mod.rs*/

#[repr(C)]
pub struct BlockIgniteEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockIgniteEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockIgniteEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockIgniteEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockIgniteEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockIgniteEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockIgniteEventTrait<'mc> for BlockIgniteEvent<'mc> {}
pub trait BlockIgniteEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,the_block: impl Into<crate::block::Block<'mc>>,cause: impl Into<crate::event::block::BlockIgniteEventIgniteCause<'mc>>,igniting_entity: impl Into<crate::entity::Entity<'mc>>,igniting_block: std::option::Option<impl Into<crate::block::Block<'mc>>>) 
-> Result<crate::event::block::BlockIgniteEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/block/Block;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(the_block.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/event/block/BlockIgniteEvent/IgniteCause;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(cause.into().jni_object().clone())});
args.push(val_2);
sig += "Lorg/bukkit/entity/Entity;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(igniting_entity.into().jni_object().clone())});
args.push(val_3);
if let Some(a) = igniting_block {
sig += "Lorg/bukkit/block/Block;";
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_4);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/block/BlockIgniteEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::BlockIgniteEvent::from_raw(&jni,res
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
/// Gets the cause of block ignite.
	fn cause(&self) 
-> Result<crate::event::block::BlockIgniteEventIgniteCause<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/block/BlockIgniteEvent/IgniteCause;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCause",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::block::BlockIgniteEventIgniteCause::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the player who ignited this block
	fn player(&self) 
-> Result<Option<crate::entity::Player<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Player;");
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
/// Gets the entity who ignited this block
	fn igniting_entity(&self) 
-> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Entity;");
let res = self.jni_ref().call_method(&self.jni_object(),"getIgnitingEntity",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::entity::Entity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Gets the block which ignited this block
	fn igniting_block(&self) 
-> Result<Option<crate::block::Block<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/Block;");
let res = self.jni_ref().call_method(&self.jni_object(),"getIgnitingBlock",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
let cls = jni.find_class("org/bukkit/event/block/BlockIgniteEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockIgniteEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockIgniteEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for BlockIgniteEvent<'mc> {}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockIgniteEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockIgniteEvent into crate::event::block::BlockEvent")

   }
}
impl<'mc> crate::event::block::BlockEventTrait<'mc> for BlockIgniteEvent<'mc> {}
#[repr(C)]
pub struct MoistureChangeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for MoistureChangeEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for MoistureChangeEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate MoistureChangeEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/MoistureChangeEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a MoistureChangeEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> MoistureChangeEventTrait<'mc> for MoistureChangeEvent<'mc> {}
pub trait MoistureChangeEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,block: impl Into<crate::block::Block<'mc>>,new_state: impl Into<crate::block::BlockState<'mc>>) 
-> Result<crate::event::block::MoistureChangeEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/block/BlockState;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(new_state.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/block/MoistureChangeEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::MoistureChangeEvent::from_raw(&jni,res
)}
/// Gets the new state of the affected block.
	fn new_state(&self) 
-> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockState;");
let res = self.jni_ref().call_method(&self.jni_object(),"getNewState",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockState::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
let cls = jni.find_class("org/bukkit/event/block/MoistureChangeEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for MoistureChangeEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting MoistureChangeEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for MoistureChangeEvent<'mc> {}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for MoistureChangeEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting MoistureChangeEvent into crate::event::block::BlockEvent")

   }
}
impl<'mc> crate::event::block::BlockEventTrait<'mc> for MoistureChangeEvent<'mc> {}
pub enum TNTPrimeEventPrimeCause<'mc> {
	Fire {inner: TNTPrimeEventPrimeCauseStruct<'mc>},
	Redstone {inner: TNTPrimeEventPrimeCauseStruct<'mc>},
	Player {inner: TNTPrimeEventPrimeCauseStruct<'mc>},
	Explosion {inner: TNTPrimeEventPrimeCauseStruct<'mc>},
	Projectile {inner: TNTPrimeEventPrimeCauseStruct<'mc>},
	BlockBreak {inner: TNTPrimeEventPrimeCauseStruct<'mc>},
	Dispenser {inner: TNTPrimeEventPrimeCauseStruct<'mc>},
}
impl<'mc> std::fmt::Display for TNTPrimeEventPrimeCause<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           TNTPrimeEventPrimeCause::Fire { .. } => f.write_str("FIRE"),
           TNTPrimeEventPrimeCause::Redstone { .. } => f.write_str("REDSTONE"),
           TNTPrimeEventPrimeCause::Player { .. } => f.write_str("PLAYER"),
           TNTPrimeEventPrimeCause::Explosion { .. } => f.write_str("EXPLOSION"),
           TNTPrimeEventPrimeCause::Projectile { .. } => f.write_str("PROJECTILE"),
           TNTPrimeEventPrimeCause::BlockBreak { .. } => f.write_str("BLOCK_BREAK"),
           TNTPrimeEventPrimeCause::Dispenser { .. } => f.write_str("DISPENSER"),
       }
   }
}

        impl<'mc> TNTPrimeEventPrimeCauseTrait<'mc> for TNTPrimeEventPrimeCause<'mc> {}
        
        pub trait TNTPrimeEventPrimeCauseTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<TNTPrimeEventPrimeCause<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/event/block/TNTPrimeEvent/PrimeCause");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/event/block/TNTPrimeEvent/PrimeCause;",
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
                    
"FIRE" => Ok(TNTPrimeEventPrimeCause::Fire { inner: TNTPrimeEventPrimeCauseStruct::from_raw(env,obj)?}),
"REDSTONE" => Ok(TNTPrimeEventPrimeCause::Redstone { inner: TNTPrimeEventPrimeCauseStruct::from_raw(env,obj)?}),
"PLAYER" => Ok(TNTPrimeEventPrimeCause::Player { inner: TNTPrimeEventPrimeCauseStruct::from_raw(env,obj)?}),
"EXPLOSION" => Ok(TNTPrimeEventPrimeCause::Explosion { inner: TNTPrimeEventPrimeCauseStruct::from_raw(env,obj)?}),
"PROJECTILE" => Ok(TNTPrimeEventPrimeCause::Projectile { inner: TNTPrimeEventPrimeCauseStruct::from_raw(env,obj)?}),
"BLOCK_BREAK" => Ok(TNTPrimeEventPrimeCause::BlockBreak { inner: TNTPrimeEventPrimeCauseStruct::from_raw(env,obj)?}),
"DISPENSER" => Ok(TNTPrimeEventPrimeCause::Dispenser { inner: TNTPrimeEventPrimeCauseStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct TNTPrimeEventPrimeCauseStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for TNTPrimeEventPrimeCause<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::Fire { inner } => inner.0.clone(),
Self::Redstone { inner } => inner.0.clone(),
Self::Player { inner } => inner.0.clone(),
Self::Explosion { inner } => inner.0.clone(),
Self::Projectile { inner } => inner.0.clone(),
Self::BlockBreak { inner } => inner.0.clone(),
Self::Dispenser { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::Fire { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Redstone { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Player { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Explosion { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Projectile { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::BlockBreak { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Dispenser { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for TNTPrimeEventPrimeCause<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate TNTPrimeEventPrimeCause from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/TNTPrimeEvent/PrimeCause")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a TNTPrimeEventPrimeCause object, got {}",
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
                    "FIRE" => Ok(TNTPrimeEventPrimeCause::Fire { inner: TNTPrimeEventPrimeCauseStruct::from_raw(env,obj)?}),"REDSTONE" => Ok(TNTPrimeEventPrimeCause::Redstone { inner: TNTPrimeEventPrimeCauseStruct::from_raw(env,obj)?}),"PLAYER" => Ok(TNTPrimeEventPrimeCause::Player { inner: TNTPrimeEventPrimeCauseStruct::from_raw(env,obj)?}),"EXPLOSION" => Ok(TNTPrimeEventPrimeCause::Explosion { inner: TNTPrimeEventPrimeCauseStruct::from_raw(env,obj)?}),"PROJECTILE" => Ok(TNTPrimeEventPrimeCause::Projectile { inner: TNTPrimeEventPrimeCauseStruct::from_raw(env,obj)?}),"BLOCK_BREAK" => Ok(TNTPrimeEventPrimeCause::BlockBreak { inner: TNTPrimeEventPrimeCauseStruct::from_raw(env,obj)?}),"DISPENSER" => Ok(TNTPrimeEventPrimeCause::Dispenser { inner: TNTPrimeEventPrimeCauseStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for TNTPrimeEventPrimeCauseStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for TNTPrimeEventPrimeCauseStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate TNTPrimeEventPrimeCauseStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/TNTPrimeEvent/PrimeCause")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a TNTPrimeEventPrimeCauseStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> TNTPrimeEventPrimeCauseStruct<'mc> {

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::block::TNTPrimeEventPrimeCause<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/block/TNTPrimeEvent/PrimeCause;");
let cls = jni.find_class("org/bukkit/event/block/TNTPrimeEvent/PrimeCause"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::block::TNTPrimeEventPrimeCause::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct BlockEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockEventTrait<'mc> for BlockEvent<'mc> {}
pub trait BlockEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,the_block: impl Into<crate::block::Block<'mc>>) 
-> Result<crate::event::block::BlockEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(the_block.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/block/BlockEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::BlockEvent::from_raw(&jni,res
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

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Event<'mc>> for BlockEvent<'mc>{

fn into(self) -> crate::event::Event<'mc> {

crate::event::Event::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockEvent into crate::event::Event")

   }
}
impl<'mc> crate::event::EventTrait<'mc> for BlockEvent<'mc> {}
#[repr(C)]
pub struct SculkBloomEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for SculkBloomEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for SculkBloomEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate SculkBloomEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/SculkBloomEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a SculkBloomEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> SculkBloomEventTrait<'mc> for SculkBloomEvent<'mc> {}
pub trait SculkBloomEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,the_block: impl Into<crate::block::Block<'mc>>,charge: i32) 
-> Result<crate::event::block::SculkBloomEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;I)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(the_block.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Int(charge);
let cls = jni.find_class("org/bukkit/event/block/SculkBloomEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::SculkBloomEvent::from_raw(&jni,res
)}
/// Returns the charge of the cursor, &lt; 1000 by default.
	fn charge(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getCharge",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Sets the charge of the cursor.
/// 
/// Increasing the charge of a cursor makes the cursor last longer, giving
/// it more time to spread sculk blocks across a larger range.
/// 
/// Typically, charges should be set to the exp reward of a mob
/// ({@link EntityDeathEvent#getDroppedExp()}), which is usually
/// 3-5 for animals, and 5-10 for the average mob (up to 50 for
/// wither skeletons). Roughly speaking, for each charge, 1 more
/// sculk block will be placed.
	fn set_charge(&self,charge: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(charge);
let res = self.jni_ref().call_method(&self.jni_object(),"setCharge",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
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
let cls = jni.find_class("org/bukkit/event/block/SculkBloomEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for SculkBloomEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting SculkBloomEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for SculkBloomEvent<'mc> {}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for SculkBloomEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting SculkBloomEvent into crate::event::block::BlockEvent")

   }
}
impl<'mc> crate::event::block::BlockEventTrait<'mc> for SculkBloomEvent<'mc> {}
pub enum Action<'mc> {
	LeftClickBlock {inner: ActionStruct<'mc>},
	RightClickBlock {inner: ActionStruct<'mc>},
	LeftClickAir {inner: ActionStruct<'mc>},
	RightClickAir {inner: ActionStruct<'mc>},
	Physical {inner: ActionStruct<'mc>},
}
impl<'mc> std::fmt::Display for Action<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           Action::LeftClickBlock { .. } => f.write_str("LEFT_CLICK_BLOCK"),
           Action::RightClickBlock { .. } => f.write_str("RIGHT_CLICK_BLOCK"),
           Action::LeftClickAir { .. } => f.write_str("LEFT_CLICK_AIR"),
           Action::RightClickAir { .. } => f.write_str("RIGHT_CLICK_AIR"),
           Action::Physical { .. } => f.write_str("PHYSICAL"),
       }
   }
}

        impl<'mc> ActionTrait<'mc> for Action<'mc> {}
        
        pub trait ActionTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<Action<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/event/block/Action");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/event/block/Action;",
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
                    
"LEFT_CLICK_BLOCK" => Ok(Action::LeftClickBlock { inner: ActionStruct::from_raw(env,obj)?}),
"RIGHT_CLICK_BLOCK" => Ok(Action::RightClickBlock { inner: ActionStruct::from_raw(env,obj)?}),
"LEFT_CLICK_AIR" => Ok(Action::LeftClickAir { inner: ActionStruct::from_raw(env,obj)?}),
"RIGHT_CLICK_AIR" => Ok(Action::RightClickAir { inner: ActionStruct::from_raw(env,obj)?}),
"PHYSICAL" => Ok(Action::Physical { inner: ActionStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct ActionStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Action<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::LeftClickBlock { inner } => inner.0.clone(),
Self::RightClickBlock { inner } => inner.0.clone(),
Self::LeftClickAir { inner } => inner.0.clone(),
Self::RightClickAir { inner } => inner.0.clone(),
Self::Physical { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::LeftClickBlock { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::RightClickBlock { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::LeftClickAir { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::RightClickAir { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Physical { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for Action<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Action from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/Action")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Action object, got {}",
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
                    "LEFT_CLICK_BLOCK" => Ok(Action::LeftClickBlock { inner: ActionStruct::from_raw(env,obj)?}),"RIGHT_CLICK_BLOCK" => Ok(Action::RightClickBlock { inner: ActionStruct::from_raw(env,obj)?}),"LEFT_CLICK_AIR" => Ok(Action::LeftClickAir { inner: ActionStruct::from_raw(env,obj)?}),"RIGHT_CLICK_AIR" => Ok(Action::RightClickAir { inner: ActionStruct::from_raw(env,obj)?}),"PHYSICAL" => Ok(Action::Physical { inner: ActionStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for ActionStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ActionStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ActionStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/Action")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ActionStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ActionStruct<'mc> {

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::block::Action<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/block/Action;");
let cls = jni.find_class("org/bukkit/event/block/Action"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::block::Action::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct BlockPistonExtendEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockPistonExtendEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockPistonExtendEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockPistonExtendEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockPistonExtendEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockPistonExtendEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockPistonExtendEventTrait<'mc> for BlockPistonExtendEvent<'mc> {}
pub trait BlockPistonExtendEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,block: impl Into<crate::block::Block<'mc>>,blocks: Vec<jni::objects::JObject<'mc>>,direction: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<crate::event::block::BlockPistonExtendEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/block/Block;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
args.push(val_1);
sig += "Ljava/util/List;";
let raw_val_2 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
for v in blocks{
sig += "Ljava/lang/java/lang/Object;";
		let map_val_0 = jni::objects::JValueGen::Object(v);
jni.call_method(&raw_val_2,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_2 = jni::objects::JValueGen::Object(raw_val_2);
args.push(val_2);
sig += "Lorg/bukkit/block/BlockFace;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(direction.into().jni_object().clone())});
args.push(val_3);
sig += ")V";
let cls = jni.find_class("org/bukkit/event/block/BlockPistonExtendEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::BlockPistonExtendEvent::from_raw(&jni,res
)}
#[deprecated]
/// Get the amount of blocks which will be moved while extending.
	fn length(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getLength",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Get an immutable list of the blocks which will be moved by the
/// extending.
	fn blocks(&self) 
-> Result<Vec<crate::block::Block<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBlocks",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::block::Block::from_raw(&self.jni_ref(),obj,)?);
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
let cls = jni.find_class("org/bukkit/event/block/BlockPistonExtendEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::block::BlockPistonEvent<'mc>> for BlockPistonExtendEvent<'mc>{

fn into(self) -> crate::event::block::BlockPistonEvent<'mc> {

crate::event::block::BlockPistonEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockPistonExtendEvent into crate::event::block::BlockPistonEvent")

   }
}
impl<'mc> crate::event::block::BlockPistonEventTrait<'mc> for BlockPistonExtendEvent<'mc> {}
#[repr(C)]
pub struct BlockBreakEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockBreakEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockBreakEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockBreakEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockBreakEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockBreakEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockBreakEventTrait<'mc> for BlockBreakEvent<'mc> {}
pub trait BlockBreakEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,the_block: impl Into<crate::block::Block<'mc>>,player: impl Into<crate::entity::Player<'mc>>) 
-> Result<crate::event::block::BlockBreakEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/entity/Player;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(the_block.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/block/BlockBreakEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::BlockBreakEvent::from_raw(&jni,res
)}
/// Gets the Player that is breaking the block involved in this event.
	fn player(&self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Player;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets whether or not the block will attempt to drop items as it normally
/// would.
/// If and only if this is false then {@link BlockDropItemEvent} will not be
/// called after this event.
	fn set_drop_items(&self,drop_items: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(drop_items.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setDropItems",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets whether or not the block will attempt to drop items.
/// If and only if this is false then {@link BlockDropItemEvent} will not be
/// called after this event.
	fn is_drop_items(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isDropItems",sig.as_str(),vec![]);
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

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockBreakEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockBreakEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for BlockBreakEvent<'mc> {}
impl<'mc> Into<crate::event::block::BlockExpEvent<'mc>> for BlockBreakEvent<'mc>{

fn into(self) -> crate::event::block::BlockExpEvent<'mc> {

crate::event::block::BlockExpEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockBreakEvent into crate::event::block::BlockExpEvent")

   }
}
impl<'mc> crate::event::block::BlockExpEventTrait<'mc> for BlockBreakEvent<'mc> {}
#[repr(C)]
pub struct BlockDamageEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockDamageEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockDamageEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockDamageEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockDamageEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockDamageEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockDamageEventTrait<'mc> for BlockDamageEvent<'mc> {}
pub trait BlockDamageEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,block: impl Into<crate::block::Block<'mc>>,item_in_hand: impl Into<crate::inventory::ItemStack<'mc>>,insta_break: bool) 
-> Result<crate::event::block::BlockDamageEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;Z)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item_in_hand.into().jni_object().clone())});
let val_4 = jni::objects::JValueGen::Bool(insta_break.into());
let cls = jni.find_class("org/bukkit/event/block/BlockDamageEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::BlockDamageEvent::from_raw(&jni,res
)}
/// Gets the player damaging the block involved in this event.
	fn player(&self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Player;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets if the block is set to instantly break when damaged by the player.
	fn insta_break(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"getInstaBreak",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Sets if the block should instantly break when damaged by the player.
	fn set_insta_break(&self,bool: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(bool.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setInstaBreak",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the ItemStack for the item currently in the player's hand.
	fn item_in_hand(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItemInHand",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
let cls = jni.find_class("org/bukkit/event/block/BlockDamageEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockDamageEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockDamageEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for BlockDamageEvent<'mc> {}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockDamageEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockDamageEvent into crate::event::block::BlockEvent")

   }
}
impl<'mc> crate::event::block::BlockEventTrait<'mc> for BlockDamageEvent<'mc> {}
#[repr(C)]
pub struct BlockBurnEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockBurnEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockBurnEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockBurnEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockBurnEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockBurnEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockBurnEventTrait<'mc> for BlockBurnEvent<'mc> {}
pub trait BlockBurnEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,block: impl Into<crate::block::Block<'mc>>,igniting_block: std::option::Option<impl Into<crate::block::Block<'mc>>>) 
-> Result<crate::event::block::BlockBurnEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/block/Block;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = igniting_block {
sig += "Lorg/bukkit/block/Block;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/block/BlockBurnEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::BlockBurnEvent::from_raw(&jni,res
)}
/// Gets the block which ignited this block.
	fn igniting_block(&self) 
-> Result<Option<crate::block::Block<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/Block;");
let res = self.jni_ref().call_method(&self.jni_object(),"getIgnitingBlock",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
let cls = jni.find_class("org/bukkit/event/block/BlockBurnEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockBurnEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockBurnEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for BlockBurnEvent<'mc> {}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockBurnEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockBurnEvent into crate::event::block::BlockEvent")

   }
}
impl<'mc> crate::event::block::BlockEventTrait<'mc> for BlockBurnEvent<'mc> {}
#[repr(C)]
pub struct BlockExplodeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockExplodeEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockExplodeEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockExplodeEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockExplodeEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockExplodeEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockExplodeEventTrait<'mc> for BlockExplodeEvent<'mc> {}
pub trait BlockExplodeEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,what: impl Into<crate::block::Block<'mc>>,blocks: Vec<jni::objects::JObject<'mc>>,val_yield: f32) 
-> Result<crate::event::block::BlockExplodeEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;Ljava/util/List;F)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(what.into().jni_object().clone())});
let raw_val_2 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
for v in blocks{
let map_val_0 = jni::objects::JValueGen::Object(v);
jni.call_method(&raw_val_2,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_2 = jni::objects::JValueGen::Object(raw_val_2);
let val_3 = jni::objects::JValueGen::Float(val_yield);
let cls = jni.find_class("org/bukkit/event/block/BlockExplodeEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::BlockExplodeEvent::from_raw(&jni,res
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
/// Returns the list of blocks that would have been removed or were removed
/// from the explosion event.
	fn block_list(&self) 
-> Result<Vec<crate::block::Block<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"blockList",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::block::Block::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}
/// Returns the percentage of blocks to drop from this explosion
	fn get_yield(&self) 
-> Result<f32, Box<dyn std::error::Error>>

{let sig = String::from("()F");
let res = self.jni_ref().call_method(&self.jni_object(),"getYield",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.f()?
)}
/// Sets the percentage of blocks to drop from this explosion
	fn set_yield(&self,val_yield: f32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(F)V");
let val_1 = jni::objects::JValueGen::Float(val_yield);
let res = self.jni_ref().call_method(&self.jni_object(),"setYield",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
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
let cls = jni.find_class("org/bukkit/event/block/BlockExplodeEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockExplodeEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockExplodeEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for BlockExplodeEvent<'mc> {}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockExplodeEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockExplodeEvent into crate::event::block::BlockEvent")

   }
}
impl<'mc> crate::event::block::BlockEventTrait<'mc> for BlockExplodeEvent<'mc> {}
#[repr(C)]
pub struct CampfireStartEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for CampfireStartEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for CampfireStartEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate CampfireStartEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/CampfireStartEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a CampfireStartEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> CampfireStartEventTrait<'mc> for CampfireStartEvent<'mc> {}
pub trait CampfireStartEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,furnace: impl Into<crate::block::Block<'mc>>,source: impl Into<crate::inventory::ItemStack<'mc>>,recipe: impl Into<crate::inventory::CampfireRecipe<'mc>>) 
-> Result<crate::event::block::CampfireStartEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/CampfireRecipe;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(furnace.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(source.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(recipe.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/block/CampfireStartEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::CampfireStartEvent::from_raw(&jni,res
)}
/// Gets the CampfireRecipe associated with this event.
	fn recipe(&self) 
-> Result<crate::inventory::CampfireRecipe<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/CampfireRecipe;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRecipe",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::CampfireRecipe::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the total cook time associated with this event.
	fn total_cook_time(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getTotalCookTime",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Sets the total cook time for this event.
	fn set_total_cook_time(&self,cook_time: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(cook_time);
let res = self.jni_ref().call_method(&self.jni_object(),"setTotalCookTime",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
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
let cls = jni.find_class("org/bukkit/event/block/CampfireStartEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::block::InventoryBlockStartEvent<'mc>> for CampfireStartEvent<'mc>{

fn into(self) -> crate::event::block::InventoryBlockStartEvent<'mc> {

crate::event::block::InventoryBlockStartEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting CampfireStartEvent into crate::event::block::InventoryBlockStartEvent")

   }
}
impl<'mc> crate::event::block::InventoryBlockStartEventTrait<'mc> for CampfireStartEvent<'mc> {}
#[repr(C)]
pub struct BrewingStartEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BrewingStartEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BrewingStartEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BrewingStartEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BrewingStartEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BrewingStartEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BrewingStartEventTrait<'mc> for BrewingStartEvent<'mc> {}
pub trait BrewingStartEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,furnace: impl Into<crate::block::Block<'mc>>,source: impl Into<crate::inventory::ItemStack<'mc>>,brewing_time: i32) 
-> Result<crate::event::block::BrewingStartEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;I)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(furnace.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(source.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Int(brewing_time);
let cls = jni.find_class("org/bukkit/event/block/BrewingStartEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::BrewingStartEvent::from_raw(&jni,res
)}
/// Gets the total brew time associated with this event.
	fn total_brew_time(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getTotalBrewTime",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Sets the total brew time for this event.
	fn set_total_brew_time(&self,brew_time: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(brew_time);
let res = self.jni_ref().call_method(&self.jni_object(),"setTotalBrewTime",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
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
let cls = jni.find_class("org/bukkit/event/block/BrewingStartEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::block::InventoryBlockStartEvent<'mc>> for BrewingStartEvent<'mc>{

fn into(self) -> crate::event::block::InventoryBlockStartEvent<'mc> {

crate::event::block::InventoryBlockStartEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BrewingStartEvent into crate::event::block::InventoryBlockStartEvent")

   }
}
impl<'mc> crate::event::block::InventoryBlockStartEventTrait<'mc> for BrewingStartEvent<'mc> {}
#[repr(C)]
pub struct BellResonateEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BellResonateEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BellResonateEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BellResonateEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BellResonateEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BellResonateEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BellResonateEventTrait<'mc> for BellResonateEvent<'mc> {}
pub trait BellResonateEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,the_block: impl Into<crate::block::Block<'mc>>,resonated_entities: Vec<jni::objects::JObject<'mc>>) 
-> Result<crate::event::block::BellResonateEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;Ljava/util/List;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(the_block.into().jni_object().clone())});
let raw_val_2 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
for v in resonated_entities{
let map_val_0 = jni::objects::JValueGen::Object(v);
jni.call_method(&raw_val_2,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_2 = jni::objects::JValueGen::Object(raw_val_2);
let cls = jni.find_class("org/bukkit/event/block/BellResonateEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::BellResonateEvent::from_raw(&jni,res
)}
/// Get a mutable list of all {@link LivingEntity LivingEntities} to be
/// highlighted by the bell's resonating. This list can be added to or
/// removed from to change which entities are highlighted, and may be empty
/// if no entities were resonated as a result of this event.
/// 
/// While the highlighted entities will change, the particles that display
/// over a resonated entity and their colors will not. This is handled by the
/// client and cannot be controlled by the server.
	fn resonated_entities(&self) 
-> Result<Vec<crate::entity::LivingEntity<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getResonatedEntities",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::entity::LivingEntity::from_raw(&self.jni_ref(),obj,)?);
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
let cls = jni.find_class("org/bukkit/event/block/BellResonateEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BellResonateEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BellResonateEvent into crate::event::block::BlockEvent")

   }
}
impl<'mc> crate::event::block::BlockEventTrait<'mc> for BellResonateEvent<'mc> {}
#[repr(C)]
pub struct BlockSpreadEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockSpreadEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockSpreadEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockSpreadEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockSpreadEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockSpreadEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockSpreadEventTrait<'mc> for BlockSpreadEvent<'mc> {}
pub trait BlockSpreadEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,block: impl Into<crate::block::Block<'mc>>,source: impl Into<crate::block::Block<'mc>>,new_state: impl Into<crate::block::BlockState<'mc>>) 
-> Result<crate::event::block::BlockSpreadEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/block/Block;Lorg/bukkit/block/BlockState;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(source.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(new_state.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/block/BlockSpreadEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::BlockSpreadEvent::from_raw(&jni,res
)}
/// Gets the source block involved in this event.
	fn source(&self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/Block;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSource",sig.as_str(),vec![]);
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
let cls = jni.find_class("org/bukkit/event/block/BlockSpreadEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::block::BlockFormEvent<'mc>> for BlockSpreadEvent<'mc>{

fn into(self) -> crate::event::block::BlockFormEvent<'mc> {

crate::event::block::BlockFormEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockSpreadEvent into crate::event::block::BlockFormEvent")

   }
}
impl<'mc> crate::event::block::BlockFormEventTrait<'mc> for BlockSpreadEvent<'mc> {}
#[repr(C)]
pub struct SpongeAbsorbEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for SpongeAbsorbEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for SpongeAbsorbEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate SpongeAbsorbEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/SpongeAbsorbEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a SpongeAbsorbEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> SpongeAbsorbEventTrait<'mc> for SpongeAbsorbEvent<'mc> {}
pub trait SpongeAbsorbEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,block: impl Into<crate::block::Block<'mc>>,waterblocks: Vec<jni::objects::JObject<'mc>>) 
-> Result<crate::event::block::SpongeAbsorbEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;Ljava/util/List;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
let raw_val_2 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
for v in waterblocks{
let map_val_0 = jni::objects::JValueGen::Object(v);
jni.call_method(&raw_val_2,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_2 = jni::objects::JValueGen::Object(raw_val_2);
let cls = jni.find_class("org/bukkit/event/block/SpongeAbsorbEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::SpongeAbsorbEvent::from_raw(&jni,res
)}
/// Get a list of all blocks to be removed by the sponge.
/// 
/// This list is mutable and contains the blocks in their removed state, i.e.
/// having a type of {@link Material#AIR}.
	fn blocks(&self) 
-> Result<Vec<crate::block::BlockState<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBlocks",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::block::BlockState::from_raw(&self.jni_ref(),obj,)?);
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
let cls = jni.find_class("org/bukkit/event/block/SpongeAbsorbEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for SpongeAbsorbEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting SpongeAbsorbEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for SpongeAbsorbEvent<'mc> {}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for SpongeAbsorbEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting SpongeAbsorbEvent into crate::event::block::BlockEvent")

   }
}
impl<'mc> crate::event::block::BlockEventTrait<'mc> for SpongeAbsorbEvent<'mc> {}
#[repr(C)]
pub struct BlockDispenseEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockDispenseEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockDispenseEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockDispenseEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockDispenseEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockDispenseEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockDispenseEventTrait<'mc> for BlockDispenseEvent<'mc> {}
pub trait BlockDispenseEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,block: impl Into<crate::block::Block<'mc>>,dispensed: impl Into<crate::inventory::ItemStack<'mc>>,velocity: impl Into<crate::util::Vector<'mc>>) 
-> Result<crate::event::block::BlockDispenseEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/util/Vector;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(dispensed.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(velocity.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/block/BlockDispenseEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::BlockDispenseEvent::from_raw(&jni,res
)}
/// Gets the item that is being dispensed. Modifying the returned item will
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
/// Sets the item being dispensed.
	fn set_item(&self,item: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setItem",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the velocity in meters per tick.
/// 
/// Note: Modifying the returned Vector will not change the velocity, you
/// must use {@link #setVelocity(org.bukkit.util.Vector)} instead.
	fn velocity(&self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/util/Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"getVelocity",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the velocity of the item being dispensed in meters per tick.
	fn set_velocity(&self,vel: impl Into<crate::util::Vector<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(vel.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setVelocity",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
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
let cls = jni.find_class("org/bukkit/event/block/BlockDispenseEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockDispenseEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockDispenseEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for BlockDispenseEvent<'mc> {}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockDispenseEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockDispenseEvent into crate::event::block::BlockEvent")

   }
}
impl<'mc> crate::event::block::BlockEventTrait<'mc> for BlockDispenseEvent<'mc> {}
#[repr(C)]
pub struct SignChangeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for SignChangeEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for SignChangeEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate SignChangeEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/SignChangeEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a SignChangeEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> SignChangeEventTrait<'mc> for SignChangeEvent<'mc> {}
pub trait SignChangeEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,the_block: impl Into<crate::block::Block<'mc>>,the_player: impl Into<crate::entity::Player<'mc>>,the_lines: impl Into<String>,side: std::option::Option<impl Into<crate::block::sign::Side<'mc>>>) 
-> Result<crate::event::block::SignChangeEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/block/Block;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(the_block.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/entity/Player;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(the_player.into().jni_object().clone())});
args.push(val_2);
sig += "Ljava/lang/String;";
let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(the_lines.into())?));
args.push(val_3);
if let Some(a) = side {
sig += "Lorg/bukkit/block/sign/Side;";
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_4);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/block/SignChangeEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::SignChangeEvent::from_raw(&jni,res
)}
/// Gets the player changing the sign involved in this event.
	fn player(&self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Player;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets all of the lines of text from the sign involved in this event.
	fn lines(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLines",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Gets a single line of text from the sign involved in this event.
	fn get_line(&self,index: i32) 
-> Result<Option<String>, Box<dyn std::error::Error>>

{let sig = String::from("(I)Ljava/lang/String;");
let val_1 = jni::objects::JValueGen::Int(index);
let res = self.jni_ref().call_method(&self.jni_object(),"getLine",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)
)}
/// Sets a single line for the sign involved in this event
	fn set_line(&self,index: i32,line: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(ILjava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Int(index);
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(line.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setLine",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Returns which side is changed.
	fn side(&self) 
-> Result<crate::block::sign::Side<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/sign/Side;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSide",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::sign::Side::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
let cls = jni.find_class("org/bukkit/event/block/SignChangeEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for SignChangeEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting SignChangeEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for SignChangeEvent<'mc> {}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for SignChangeEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting SignChangeEvent into crate::event::block::BlockEvent")

   }
}
impl<'mc> crate::event::block::BlockEventTrait<'mc> for SignChangeEvent<'mc> {}
#[repr(C)]
pub struct TNTPrimeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for TNTPrimeEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for TNTPrimeEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate TNTPrimeEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/TNTPrimeEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a TNTPrimeEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> TNTPrimeEventTrait<'mc> for TNTPrimeEvent<'mc> {}
pub trait TNTPrimeEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,block: impl Into<crate::block::Block<'mc>>,ignite_cause: impl Into<crate::event::block::TNTPrimeEventPrimeCause<'mc>>,priming_entity: impl Into<crate::entity::Entity<'mc>>,priming_block: impl Into<crate::block::Block<'mc>>) 
-> Result<crate::event::block::TNTPrimeEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/event/block/TNTPrimeEvent/PrimeCause;Lorg/bukkit/entity/Entity;Lorg/bukkit/block/Block;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(ignite_cause.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(priming_entity.into().jni_object().clone())});
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(priming_block.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/block/TNTPrimeEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::TNTPrimeEvent::from_raw(&jni,res
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
/// Get the cause of the TNT becoming primed.
	fn cause(&self) 
-> Result<crate::event::block::TNTPrimeEventPrimeCause<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/block/TNTPrimeEvent/PrimeCause;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCause",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::block::TNTPrimeEventPrimeCause::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Get the entity that caused the TNT to be primed.
	fn priming_entity(&self) 
-> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Entity;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPrimingEntity",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::entity::Entity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Get the block that caused the TNT to be primed.
	fn priming_block(&self) 
-> Result<Option<crate::block::Block<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/Block;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPrimingBlock",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
let cls = jni.find_class("org/bukkit/event/block/TNTPrimeEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for TNTPrimeEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting TNTPrimeEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for TNTPrimeEvent<'mc> {}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for TNTPrimeEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting TNTPrimeEvent into crate::event::block::BlockEvent")

   }
}
impl<'mc> crate::event::block::BlockEventTrait<'mc> for TNTPrimeEvent<'mc> {}
#[repr(C)]
pub struct BlockReceiveGameEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockReceiveGameEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockReceiveGameEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockReceiveGameEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockReceiveGameEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockReceiveGameEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockReceiveGameEventTrait<'mc> for BlockReceiveGameEvent<'mc> {}
pub trait BlockReceiveGameEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,event: impl Into<crate::GameEvent<'mc>>,block: impl Into<crate::block::Block<'mc>>,entity: impl Into<crate::entity::Entity<'mc>>) 
-> Result<crate::event::block::BlockReceiveGameEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/GameEvent;Lorg/bukkit/block/Block;Lorg/bukkit/entity/Entity;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(event.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(entity.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/block/BlockReceiveGameEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::BlockReceiveGameEvent::from_raw(&jni,res
)}
/// Get the underlying event.
	fn event(&self) 
-> Result<crate::GameEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/GameEvent;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEvent",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::GameEvent::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Get the entity which triggered this event, if present.
	fn entity(&self) 
-> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Entity;");
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

	fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
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
let cls = jni.find_class("org/bukkit/event/block/BlockReceiveGameEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockReceiveGameEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockReceiveGameEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for BlockReceiveGameEvent<'mc> {}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockReceiveGameEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockReceiveGameEvent into crate::event::block::BlockEvent")

   }
}
impl<'mc> crate::event::block::BlockEventTrait<'mc> for BlockReceiveGameEvent<'mc> {}
#[repr(C)]
pub struct BlockDamageAbortEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockDamageAbortEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockDamageAbortEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockDamageAbortEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockDamageAbortEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockDamageAbortEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockDamageAbortEventTrait<'mc> for BlockDamageAbortEvent<'mc> {}
pub trait BlockDamageAbortEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,block: impl Into<crate::block::Block<'mc>>,item_in_hand: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<crate::event::block::BlockDamageAbortEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item_in_hand.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/block/BlockDamageAbortEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::BlockDamageAbortEvent::from_raw(&jni,res
)}
/// Gets the player that stopped damaging the block involved in this event.
	fn player(&self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Player;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the ItemStack for the item currently in the player's hand.
	fn item_in_hand(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItemInHand",sig.as_str(),vec![]);
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
let cls = jni.find_class("org/bukkit/event/block/BlockDamageAbortEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockDamageAbortEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockDamageAbortEvent into crate::event::block::BlockEvent")

   }
}
impl<'mc> crate::event::block::BlockEventTrait<'mc> for BlockDamageAbortEvent<'mc> {}
#[repr(C)]
pub struct BellRingEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BellRingEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BellRingEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BellRingEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BellRingEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BellRingEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BellRingEventTrait<'mc> for BellRingEvent<'mc> {}
pub trait BellRingEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,the_block: impl Into<crate::block::Block<'mc>>,direction: impl Into<crate::block::BlockFace<'mc>>,entity: impl Into<crate::entity::Entity<'mc>>) 
-> Result<crate::event::block::BellRingEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/block/BlockFace;Lorg/bukkit/entity/Entity;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(the_block.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(direction.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(entity.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/block/BellRingEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::BellRingEvent::from_raw(&jni,res
)}
/// Get the direction in which the bell was rung.
	fn direction(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDirection",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Get the {@link Entity} that rang the bell (if there was one).
	fn entity(&self) 
-> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Entity;");
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

	fn set_cancelled(&self,cancelled: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancelled.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
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
let cls = jni.find_class("org/bukkit/event/block/BellRingEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for BellRingEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BellRingEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for BellRingEvent<'mc> {}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BellRingEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BellRingEvent into crate::event::block::BlockEvent")

   }
}
impl<'mc> crate::event::block::BlockEventTrait<'mc> for BellRingEvent<'mc> {}
#[repr(C)]
pub struct BlockDropItemEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockDropItemEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockDropItemEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockDropItemEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockDropItemEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockDropItemEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockDropItemEventTrait<'mc> for BlockDropItemEvent<'mc> {}
pub trait BlockDropItemEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,block: impl Into<crate::block::Block<'mc>>,block_state: impl Into<crate::block::BlockState<'mc>>,player: impl Into<crate::entity::Player<'mc>>,items: Vec<jni::objects::JObject<'mc>>) 
-> Result<crate::event::block::BlockDropItemEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/block/BlockState;Lorg/bukkit/entity/Player;Ljava/util/List;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block_state.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let raw_val_4 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
for v in items{
let map_val_0 = jni::objects::JValueGen::Object(v);
jni.call_method(&raw_val_4,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_4 = jni::objects::JValueGen::Object(raw_val_4);
let cls = jni.find_class("org/bukkit/event/block/BlockDropItemEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::BlockDropItemEvent::from_raw(&jni,res
)}
/// Gets the Player that is breaking the block involved in this event.
	fn player(&self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Player;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the BlockState of the block involved in this event before it was
/// broken.
	fn block_state(&self) 
-> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockState;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBlockState",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockState::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets list of the Item drops caused by the block break.
/// This list is mutable - removing an item from it will cause it to not
/// drop. It is not legal however to add new items to the list.
	fn items(&self) 
-> Result<Vec<crate::entity::Item<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItems",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::entity::Item::from_raw(&self.jni_ref(),obj,)?);
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
let cls = jni.find_class("org/bukkit/event/block/BlockDropItemEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockDropItemEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockDropItemEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for BlockDropItemEvent<'mc> {}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockDropItemEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockDropItemEvent into crate::event::block::BlockEvent")

   }
}
impl<'mc> crate::event::block::BlockEventTrait<'mc> for BlockDropItemEvent<'mc> {}
#[repr(C)]
pub struct BlockMultiPlaceEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockMultiPlaceEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockMultiPlaceEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockMultiPlaceEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockMultiPlaceEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockMultiPlaceEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockMultiPlaceEventTrait<'mc> for BlockMultiPlaceEvent<'mc> {}
pub trait BlockMultiPlaceEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,states: Vec<jni::objects::JObject<'mc>>,clicked: impl Into<crate::block::Block<'mc>>,item_in_hand: impl Into<crate::inventory::ItemStack<'mc>>,the_player: impl Into<crate::entity::Player<'mc>>,can_build: bool) 
-> Result<crate::event::block::BlockMultiPlaceEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/util/List;Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/entity/Player;Z)V");
let raw_val_1 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
for v in states{
let map_val_0 = jni::objects::JValueGen::Object(v);
jni.call_method(&raw_val_1,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_1 = jni::objects::JValueGen::Object(raw_val_1);
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(clicked.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item_in_hand.into().jni_object().clone())});
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(the_player.into().jni_object().clone())});
let val_5 = jni::objects::JValueGen::Bool(can_build.into());
let cls = jni.find_class("org/bukkit/event/block/BlockMultiPlaceEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4),jni::objects::JValueGen::from(val_5)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::BlockMultiPlaceEvent::from_raw(&jni,res
)}
/// Gets a list of blockstates for all blocks which were replaced by the
/// placement of the new blocks. Most of these blocks will just have a
/// Material type of AIR.
	fn replaced_block_states(&self) 
-> Result<Vec<crate::block::BlockState<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getReplacedBlockStates",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::block::BlockState::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::block::BlockPlaceEvent<'mc>> for BlockMultiPlaceEvent<'mc>{

fn into(self) -> crate::event::block::BlockPlaceEvent<'mc> {

crate::event::block::BlockPlaceEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockMultiPlaceEvent into crate::event::block::BlockPlaceEvent")

   }
}
impl<'mc> crate::event::block::BlockPlaceEventTrait<'mc> for BlockMultiPlaceEvent<'mc> {}
#[repr(C)]
pub struct BlockPlaceEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockPlaceEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockPlaceEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockPlaceEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockPlaceEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockPlaceEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockPlaceEventTrait<'mc> for BlockPlaceEvent<'mc> {}
pub trait BlockPlaceEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,placed_block: impl Into<crate::block::Block<'mc>>,replaced_block_state: impl Into<crate::block::BlockState<'mc>>,placed_against: impl Into<crate::block::Block<'mc>>,item_in_hand: impl Into<crate::inventory::ItemStack<'mc>>,the_player: impl Into<crate::entity::Player<'mc>>,can_build: bool,hand: std::option::Option<impl Into<crate::inventory::EquipmentSlot<'mc>>>) 
-> Result<crate::event::block::BlockPlaceEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/block/Block;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(placed_block.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/block/BlockState;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(replaced_block_state.into().jni_object().clone())});
args.push(val_2);
sig += "Lorg/bukkit/block/Block;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(placed_against.into().jni_object().clone())});
args.push(val_3);
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item_in_hand.into().jni_object().clone())});
args.push(val_4);
sig += "Lorg/bukkit/entity/Player;";
let val_5 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(the_player.into().jni_object().clone())});
args.push(val_5);
sig += "Z";
let val_6 = jni::objects::JValueGen::Bool(can_build.into());
args.push(val_6);
if let Some(a) = hand {
sig += "Lorg/bukkit/inventory/EquipmentSlot;";
let val_7 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_7);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/block/BlockPlaceEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::BlockPlaceEvent::from_raw(&jni,res
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
/// Gets the player who placed the block involved in this event.
	fn player(&self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Player;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Clarity method for getting the placed block. Not really needed except
/// for reasons of clarity.
	fn block_placed(&self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/Block;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBlockPlaced",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the BlockState for the block which was replaced. Material type air
/// mostly.
	fn block_replaced_state(&self) 
-> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockState;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBlockReplacedState",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockState::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the block that this block was placed against
	fn block_against(&self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/Block;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBlockAgainst",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the item in the player's hand when they placed the block.
	fn item_in_hand(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItemInHand",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the hand which placed the block
	fn hand(&self) 
-> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/EquipmentSlot;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHand",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the value whether the player would be allowed to build here.
/// Defaults to spawn if the server was going to stop them (such as, the
/// player is in Spawn). Note that this is an entirely different check
/// than BLOCK_CANBUILD, as this refers to a player, not universe-physics
/// rule like cactus on dirt.
	fn can_build(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"canBuild",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Sets the canBuild state of this event. Set to true if you want the
/// player to be able to build.
	fn set_build(&self,can_build: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(can_build.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setBuild",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
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
let cls = jni.find_class("org/bukkit/event/block/BlockPlaceEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockPlaceEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockPlaceEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for BlockPlaceEvent<'mc> {}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockPlaceEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockPlaceEvent into crate::event::block::BlockEvent")

   }
}
impl<'mc> crate::event::block::BlockEventTrait<'mc> for BlockPlaceEvent<'mc> {}
#[repr(C)]
pub struct LeavesDecayEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for LeavesDecayEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for LeavesDecayEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate LeavesDecayEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/LeavesDecayEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a LeavesDecayEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> LeavesDecayEventTrait<'mc> for LeavesDecayEvent<'mc> {}
pub trait LeavesDecayEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,block: impl Into<crate::block::Block<'mc>>) 
-> Result<crate::event::block::LeavesDecayEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/block/LeavesDecayEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::LeavesDecayEvent::from_raw(&jni,res
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
let cls = jni.find_class("org/bukkit/event/block/LeavesDecayEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for LeavesDecayEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting LeavesDecayEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for LeavesDecayEvent<'mc> {}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for LeavesDecayEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting LeavesDecayEvent into crate::event::block::BlockEvent")

   }
}
impl<'mc> crate::event::block::BlockEventTrait<'mc> for LeavesDecayEvent<'mc> {}
#[repr(C)]
pub struct BlockFertilizeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockFertilizeEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockFertilizeEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockFertilizeEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockFertilizeEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockFertilizeEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockFertilizeEventTrait<'mc> for BlockFertilizeEvent<'mc> {}
pub trait BlockFertilizeEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,the_block: impl Into<crate::block::Block<'mc>>,player: impl Into<crate::entity::Player<'mc>>,blocks: Vec<jni::objects::JObject<'mc>>) 
-> Result<crate::event::block::BlockFertilizeEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/entity/Player;Ljava/util/List;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(the_block.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let raw_val_3 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
for v in blocks{
let map_val_0 = jni::objects::JValueGen::Object(v);
jni.call_method(&raw_val_3,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_3 = jni::objects::JValueGen::Object(raw_val_3);
let cls = jni.find_class("org/bukkit/event/block/BlockFertilizeEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::BlockFertilizeEvent::from_raw(&jni,res
)}
/// Gets the player that triggered the fertilization.
	fn player(&self) 
-> Result<Option<crate::entity::Player<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Player;");
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
/// Gets a list of all blocks changed by the fertilization.
	fn blocks(&self) 
-> Result<Vec<crate::block::BlockState<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBlocks",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::block::BlockState::from_raw(&self.jni_ref(),obj,)?);
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
let cls = jni.find_class("org/bukkit/event/block/BlockFertilizeEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockFertilizeEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockFertilizeEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for BlockFertilizeEvent<'mc> {}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockFertilizeEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockFertilizeEvent into crate::event::block::BlockEvent")

   }
}
impl<'mc> crate::event::block::BlockEventTrait<'mc> for BlockFertilizeEvent<'mc> {}
#[repr(C)]
pub struct BlockExpEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockExpEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockExpEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockExpEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockExpEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockExpEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockExpEventTrait<'mc> for BlockExpEvent<'mc> {}
pub trait BlockExpEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,block: impl Into<crate::block::Block<'mc>>,exp: i32) 
-> Result<crate::event::block::BlockExpEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;I)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Int(exp);
let cls = jni.find_class("org/bukkit/event/block/BlockExpEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::BlockExpEvent::from_raw(&jni,res
)}
/// Get the experience dropped by the block after the event has processed
	fn exp_to_drop(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getExpToDrop",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Set the amount of experience dropped by the block after the event has
/// processed
	fn set_exp_to_drop(&self,exp: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(exp);
let res = self.jni_ref().call_method(&self.jni_object(),"setExpToDrop",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
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
let cls = jni.find_class("org/bukkit/event/block/BlockExpEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockExpEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockExpEvent into crate::event::block::BlockEvent")

   }
}
impl<'mc> crate::event::block::BlockEventTrait<'mc> for BlockExpEvent<'mc> {}
#[repr(C)]
pub struct BlockCookEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockCookEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockCookEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockCookEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockCookEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockCookEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockCookEventTrait<'mc> for BlockCookEvent<'mc> {}
pub trait BlockCookEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,block: impl Into<crate::block::Block<'mc>>,source: impl Into<crate::inventory::ItemStack<'mc>>,result: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<crate::event::block::BlockCookEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(source.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(result.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/block/BlockCookEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::BlockCookEvent::from_raw(&jni,res
)}
/// Gets the smelted ItemStack for this event
	fn source(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSource",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the resultant ItemStack for this event
	fn result(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getResult",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the resultant ItemStack for this event
	fn set_result(&self,result: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(result.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setResult",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
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
let cls = jni.find_class("org/bukkit/event/block/BlockCookEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockCookEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockCookEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for BlockCookEvent<'mc> {}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockCookEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockCookEvent into crate::event::block::BlockEvent")

   }
}
impl<'mc> crate::event::block::BlockEventTrait<'mc> for BlockCookEvent<'mc> {}
#[repr(C)]
pub struct BlockPistonRetractEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockPistonRetractEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockPistonRetractEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockPistonRetractEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockPistonRetractEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockPistonRetractEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockPistonRetractEventTrait<'mc> for BlockPistonRetractEvent<'mc> {}
pub trait BlockPistonRetractEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,block: impl Into<crate::block::Block<'mc>>,blocks: Vec<jni::objects::JObject<'mc>>,direction: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<crate::event::block::BlockPistonRetractEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;Ljava/util/List;Lorg/bukkit/block/BlockFace;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
let raw_val_2 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
for v in blocks{
let map_val_0 = jni::objects::JValueGen::Object(v);
jni.call_method(&raw_val_2,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_2 = jni::objects::JValueGen::Object(raw_val_2);
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(direction.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/block/BlockPistonRetractEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::BlockPistonRetractEvent::from_raw(&jni,res
)}
#[deprecated]
/// Gets the location where the possible moving block might be if the retracting piston is sticky.
	fn retract_location(&self) 
-> Result<crate::Location<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Location;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRetractLocation",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Get an immutable list of the blocks which will be moved by the
/// extending.
	fn blocks(&self) 
-> Result<Vec<crate::block::Block<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBlocks",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::block::Block::from_raw(&self.jni_ref(),obj,)?);
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
let cls = jni.find_class("org/bukkit/event/block/BlockPistonRetractEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::block::BlockPistonEvent<'mc>> for BlockPistonRetractEvent<'mc>{

fn into(self) -> crate::event::block::BlockPistonEvent<'mc> {

crate::event::block::BlockPistonEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockPistonRetractEvent into crate::event::block::BlockPistonEvent")

   }
}
impl<'mc> crate::event::block::BlockPistonEventTrait<'mc> for BlockPistonRetractEvent<'mc> {}
#[repr(C)]
pub struct BlockFromToEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockFromToEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockFromToEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockFromToEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockFromToEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockFromToEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockFromToEventTrait<'mc> for BlockFromToEvent<'mc> {}
pub trait BlockFromToEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,block: impl Into<crate::block::Block<'mc>>,to_block: impl Into<crate::block::Block<'mc>>) 
-> Result<crate::event::block::BlockFromToEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/block/Block;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/block/Block;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(to_block.into().jni_object().clone())});
args.push(val_2);
sig += ")V";
let cls = jni.find_class("org/bukkit/event/block/BlockFromToEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::BlockFromToEvent::from_raw(&jni,res
)}
/// Gets the BlockFace that the block is moving to.
	fn face(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFace",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Convenience method for getting the faced Block.
	fn to_block(&self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/Block;");
let res = self.jni_ref().call_method(&self.jni_object(),"getToBlock",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
let cls = jni.find_class("org/bukkit/event/block/BlockFromToEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockFromToEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockFromToEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for BlockFromToEvent<'mc> {}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockFromToEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockFromToEvent into crate::event::block::BlockEvent")

   }
}
impl<'mc> crate::event::block::BlockEventTrait<'mc> for BlockFromToEvent<'mc> {}
#[repr(C)]
pub struct FluidLevelChangeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for FluidLevelChangeEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for FluidLevelChangeEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate FluidLevelChangeEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/FluidLevelChangeEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a FluidLevelChangeEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> FluidLevelChangeEventTrait<'mc> for FluidLevelChangeEvent<'mc> {}
pub trait FluidLevelChangeEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,the_block: impl Into<crate::block::Block<'mc>>,new_data: impl Into<crate::block::data::BlockData<'mc>>) 
-> Result<crate::event::block::FluidLevelChangeEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/block/data/BlockData;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(the_block.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(new_data.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/block/FluidLevelChangeEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::FluidLevelChangeEvent::from_raw(&jni,res
)}
/// Gets the new data of the changed block.
	fn new_data(&self) 
-> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
let res = self.jni_ref().call_method(&self.jni_object(),"getNewData",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::data::BlockData::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the new data of the changed block. Must be of the same Material as
/// the old one.
	fn set_new_data(&self,new_data: impl Into<crate::block::data::BlockData<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(new_data.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setNewData",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
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
let cls = jni.find_class("org/bukkit/event/block/FluidLevelChangeEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for FluidLevelChangeEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting FluidLevelChangeEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for FluidLevelChangeEvent<'mc> {}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for FluidLevelChangeEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting FluidLevelChangeEvent into crate::event::block::BlockEvent")

   }
}
impl<'mc> crate::event::block::BlockEventTrait<'mc> for FluidLevelChangeEvent<'mc> {}
#[repr(C)]
pub struct BlockPhysicsEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockPhysicsEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockPhysicsEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockPhysicsEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockPhysicsEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockPhysicsEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockPhysicsEventTrait<'mc> for BlockPhysicsEvent<'mc> {}
pub trait BlockPhysicsEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,block: impl Into<crate::block::Block<'mc>>,changed: impl Into<crate::block::data::BlockData<'mc>>,source_block: std::option::Option<impl Into<crate::block::Block<'mc>>>) 
-> Result<crate::event::block::BlockPhysicsEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/block/Block;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/block/data/BlockData;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(changed.into().jni_object().clone())});
args.push(val_2);
if let Some(a) = source_block {
sig += "Lorg/bukkit/block/Block;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_3);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/block/BlockPhysicsEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::BlockPhysicsEvent::from_raw(&jni,res
)}
/// Gets the source block that triggered this event.
/// Note: This will default to block if not set.
	fn source_block(&self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/Block;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSourceBlock",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the type of block that changed, causing this event
	fn changed_type(&self) 
-> Result<crate::Material<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Material;");
let res = self.jni_ref().call_method(&self.jni_object(),"getChangedType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Material::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
let cls = jni.find_class("org/bukkit/event/block/BlockPhysicsEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockPhysicsEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockPhysicsEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for BlockPhysicsEvent<'mc> {}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockPhysicsEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockPhysicsEvent into crate::event::block::BlockEvent")

   }
}
impl<'mc> crate::event::block::BlockEventTrait<'mc> for BlockPhysicsEvent<'mc> {}
#[repr(C)]
pub struct BlockFadeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockFadeEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockFadeEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockFadeEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockFadeEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockFadeEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockFadeEventTrait<'mc> for BlockFadeEvent<'mc> {}
pub trait BlockFadeEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,block: impl Into<crate::block::Block<'mc>>,new_state: impl Into<crate::block::BlockState<'mc>>) 
-> Result<crate::event::block::BlockFadeEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/block/BlockState;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(new_state.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/block/BlockFadeEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::BlockFadeEvent::from_raw(&jni,res
)}
/// Gets the state of the block that will be fading, melting or
/// disappearing.
	fn new_state(&self) 
-> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockState;");
let res = self.jni_ref().call_method(&self.jni_object(),"getNewState",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockState::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
let cls = jni.find_class("org/bukkit/event/block/BlockFadeEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockFadeEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockFadeEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for BlockFadeEvent<'mc> {}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockFadeEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockFadeEvent into crate::event::block::BlockEvent")

   }
}
impl<'mc> crate::event::block::BlockEventTrait<'mc> for BlockFadeEvent<'mc> {}
#[repr(C)]
pub struct BlockDispenseArmorEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockDispenseArmorEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockDispenseArmorEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockDispenseArmorEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockDispenseArmorEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockDispenseArmorEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockDispenseArmorEventTrait<'mc> for BlockDispenseArmorEvent<'mc> {}
pub trait BlockDispenseArmorEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,block: impl Into<crate::block::Block<'mc>>,dispensed: impl Into<crate::inventory::ItemStack<'mc>>,target: impl Into<crate::entity::LivingEntity<'mc>>) 
-> Result<crate::event::block::BlockDispenseArmorEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/entity/LivingEntity;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(dispensed.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(target.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/block/BlockDispenseArmorEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::BlockDispenseArmorEvent::from_raw(&jni,res
)}
/// Get the living entity on which the armor was dispensed.
	fn tarentity(&self) 
-> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/LivingEntity;");
let res = self.jni_ref().call_method(&self.jni_object(),"getTargetEntity",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::LivingEntity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::block::BlockDispenseEvent<'mc>> for BlockDispenseArmorEvent<'mc>{

fn into(self) -> crate::event::block::BlockDispenseEvent<'mc> {

crate::event::block::BlockDispenseEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockDispenseArmorEvent into crate::event::block::BlockDispenseEvent")

   }
}
impl<'mc> crate::event::block::BlockDispenseEventTrait<'mc> for BlockDispenseArmorEvent<'mc> {}
#[repr(C)]
pub struct InventoryBlockStartEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for InventoryBlockStartEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for InventoryBlockStartEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate InventoryBlockStartEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/InventoryBlockStartEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a InventoryBlockStartEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> InventoryBlockStartEventTrait<'mc> for InventoryBlockStartEvent<'mc> {}
pub trait InventoryBlockStartEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,block: impl Into<crate::block::Block<'mc>>,source: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<crate::event::block::InventoryBlockStartEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(source.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/block/InventoryBlockStartEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::InventoryBlockStartEvent::from_raw(&jni,res
)}
/// Gets the source ItemStack for this event.
	fn source(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSource",sig.as_str(),vec![]);
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
let cls = jni.find_class("org/bukkit/event/block/InventoryBlockStartEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for InventoryBlockStartEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting InventoryBlockStartEvent into crate::event::block::BlockEvent")

   }
}
impl<'mc> crate::event::block::BlockEventTrait<'mc> for InventoryBlockStartEvent<'mc> {}
#[repr(C)]
pub struct BlockPistonEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockPistonEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockPistonEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockPistonEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockPistonEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockPistonEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockPistonEventTrait<'mc> for BlockPistonEvent<'mc> {}
pub trait BlockPistonEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,block: impl Into<crate::block::Block<'mc>>,direction: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<crate::event::block::BlockPistonEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/block/BlockFace;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(direction.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/block/BlockPistonEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::BlockPistonEvent::from_raw(&jni,res
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
/// Returns true if the Piston in the event is sticky.
	fn is_sticky(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isSticky",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Return the direction in which the piston will operate.
	fn direction(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDirection",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockPistonEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockPistonEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for BlockPistonEvent<'mc> {}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockPistonEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockPistonEvent into crate::event::block::BlockEvent")

   }
}
impl<'mc> crate::event::block::BlockEventTrait<'mc> for BlockPistonEvent<'mc> {}
#[repr(C)]
pub struct CauldronLevelChangeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for CauldronLevelChangeEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for CauldronLevelChangeEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate CauldronLevelChangeEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/CauldronLevelChangeEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a CauldronLevelChangeEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> CauldronLevelChangeEventTrait<'mc> for CauldronLevelChangeEvent<'mc> {}
pub trait CauldronLevelChangeEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,block: impl Into<crate::block::Block<'mc>>,entity: impl Into<crate::entity::Entity<'mc>>,reason: impl Into<crate::event::block::CauldronLevelChangeEventChangeReason<'mc>>,new_block: impl Into<crate::block::BlockState<'mc>>) 
-> Result<crate::event::block::CauldronLevelChangeEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/entity/Entity;Lorg/bukkit/event/block/CauldronLevelChangeEvent/ChangeReason;Lorg/bukkit/block/BlockState;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(entity.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(reason.into().jni_object().clone())});
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(new_block.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/block/CauldronLevelChangeEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::CauldronLevelChangeEvent::from_raw(&jni,res
)}
/// Get entity which did this. May be null.
	fn entity(&self) 
-> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Entity;");
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

	fn reason(&self) 
-> Result<crate::event::block::CauldronLevelChangeEventChangeReason<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/block/CauldronLevelChangeEvent/ChangeReason;");
let res = self.jni_ref().call_method(&self.jni_object(),"getReason",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::block::CauldronLevelChangeEventChangeReason::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the new state of the cauldron.
	fn new_state(&self) 
-> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockState;");
let res = self.jni_ref().call_method(&self.jni_object(),"getNewState",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockState::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]
/// Gets the old level of the cauldron.
	fn old_level(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getOldLevel",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
#[deprecated]
/// Gets the new level of the cauldron.
	fn new_level(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getNewLevel",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
#[deprecated]
/// Sets the new level of the cauldron.
	fn set_new_level(&self,new_level: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(new_level);
let res = self.jni_ref().call_method(&self.jni_object(),"setNewLevel",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
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
let cls = jni.find_class("org/bukkit/event/block/CauldronLevelChangeEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for CauldronLevelChangeEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting CauldronLevelChangeEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for CauldronLevelChangeEvent<'mc> {}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for CauldronLevelChangeEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting CauldronLevelChangeEvent into crate::event::block::BlockEvent")

   }
}
impl<'mc> crate::event::block::BlockEventTrait<'mc> for CauldronLevelChangeEvent<'mc> {}
#[repr(C)]
pub struct EntityBlockFormEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for EntityBlockFormEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for EntityBlockFormEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate EntityBlockFormEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/EntityBlockFormEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntityBlockFormEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> EntityBlockFormEventTrait<'mc> for EntityBlockFormEvent<'mc> {}
pub trait EntityBlockFormEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,entity: impl Into<crate::entity::Entity<'mc>>,block: impl Into<crate::block::Block<'mc>>,blockstate: impl Into<crate::block::BlockState<'mc>>) 
-> Result<crate::event::block::EntityBlockFormEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Entity;Lorg/bukkit/block/Block;Lorg/bukkit/block/BlockState;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(entity.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(blockstate.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/block/EntityBlockFormEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::EntityBlockFormEvent::from_raw(&jni,res
)}
/// Get the entity that formed the block.
	fn entity(&self) 
-> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Entity;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEntity",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Entity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::block::BlockFormEvent<'mc>> for EntityBlockFormEvent<'mc>{

fn into(self) -> crate::event::block::BlockFormEvent<'mc> {

crate::event::block::BlockFormEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting EntityBlockFormEvent into crate::event::block::BlockFormEvent")

   }
}
impl<'mc> crate::event::block::BlockFormEventTrait<'mc> for EntityBlockFormEvent<'mc> {}
#[repr(C)]
pub struct BlockGrowEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockGrowEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockGrowEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockGrowEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockGrowEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockGrowEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockGrowEventTrait<'mc> for BlockGrowEvent<'mc> {}
pub trait BlockGrowEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,block: impl Into<crate::block::Block<'mc>>,new_state: impl Into<crate::block::BlockState<'mc>>) 
-> Result<crate::event::block::BlockGrowEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/block/BlockState;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(new_state.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/block/BlockGrowEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::BlockGrowEvent::from_raw(&jni,res
)}
/// Gets the state of the block where it will form or spread to.
	fn new_state(&self) 
-> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockState;");
let res = self.jni_ref().call_method(&self.jni_object(),"getNewState",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockState::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
let cls = jni.find_class("org/bukkit/event/block/BlockGrowEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockGrowEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockGrowEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for BlockGrowEvent<'mc> {}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockGrowEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockGrowEvent into crate::event::block::BlockEvent")

   }
}
impl<'mc> crate::event::block::BlockEventTrait<'mc> for BlockGrowEvent<'mc> {}
#[repr(C)]
pub struct BlockFormEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockFormEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockFormEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockFormEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockFormEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockFormEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockFormEventTrait<'mc> for BlockFormEvent<'mc> {}
pub trait BlockFormEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,block: impl Into<crate::block::Block<'mc>>,new_state: impl Into<crate::block::BlockState<'mc>>) 
-> Result<crate::event::block::BlockFormEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/block/BlockState;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(new_state.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/block/BlockFormEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::BlockFormEvent::from_raw(&jni,res
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
let cls = jni.find_class("org/bukkit/event/block/BlockFormEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::block::BlockGrowEvent<'mc>> for BlockFormEvent<'mc>{

fn into(self) -> crate::event::block::BlockGrowEvent<'mc> {

crate::event::block::BlockGrowEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockFormEvent into crate::event::block::BlockGrowEvent")

   }
}
impl<'mc> crate::event::block::BlockGrowEventTrait<'mc> for BlockFormEvent<'mc> {}
#[repr(C)]
pub struct BlockRedstoneEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockRedstoneEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockRedstoneEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockRedstoneEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockRedstoneEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockRedstoneEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockRedstoneEventTrait<'mc> for BlockRedstoneEvent<'mc> {}
pub trait BlockRedstoneEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,block: impl Into<crate::block::Block<'mc>>,old_current: i32,new_current: i32) 
-> Result<crate::event::block::BlockRedstoneEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;II)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Int(old_current);
let val_3 = jni::objects::JValueGen::Int(new_current);
let cls = jni.find_class("org/bukkit/event/block/BlockRedstoneEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::BlockRedstoneEvent::from_raw(&jni,res
)}
/// Gets the old current of this block
	fn old_current(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getOldCurrent",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Gets the new current of this block
	fn new_current(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getNewCurrent",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Sets the new current of this block
	fn set_new_current(&self,new_current: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(new_current);
let res = self.jni_ref().call_method(&self.jni_object(),"setNewCurrent",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
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
let cls = jni.find_class("org/bukkit/event/block/BlockRedstoneEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockRedstoneEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockRedstoneEvent into crate::event::block::BlockEvent")

   }
}
impl<'mc> crate::event::block::BlockEventTrait<'mc> for BlockRedstoneEvent<'mc> {}
pub enum CauldronLevelChangeEventChangeReason<'mc> {
	BucketFill {inner: CauldronLevelChangeEventChangeReasonStruct<'mc>},
	BucketEmpty {inner: CauldronLevelChangeEventChangeReasonStruct<'mc>},
	BottleFill {inner: CauldronLevelChangeEventChangeReasonStruct<'mc>},
	BottleEmpty {inner: CauldronLevelChangeEventChangeReasonStruct<'mc>},
	BannerWash {inner: CauldronLevelChangeEventChangeReasonStruct<'mc>},
	ArmorWash {inner: CauldronLevelChangeEventChangeReasonStruct<'mc>},
	ShulkerWash {inner: CauldronLevelChangeEventChangeReasonStruct<'mc>},
	Extinguish {inner: CauldronLevelChangeEventChangeReasonStruct<'mc>},
	Evaporate {inner: CauldronLevelChangeEventChangeReasonStruct<'mc>},
	NaturalFill {inner: CauldronLevelChangeEventChangeReasonStruct<'mc>},
	Unknown {inner: CauldronLevelChangeEventChangeReasonStruct<'mc>},
}
impl<'mc> std::fmt::Display for CauldronLevelChangeEventChangeReason<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           CauldronLevelChangeEventChangeReason::BucketFill { .. } => f.write_str("BUCKET_FILL"),
           CauldronLevelChangeEventChangeReason::BucketEmpty { .. } => f.write_str("BUCKET_EMPTY"),
           CauldronLevelChangeEventChangeReason::BottleFill { .. } => f.write_str("BOTTLE_FILL"),
           CauldronLevelChangeEventChangeReason::BottleEmpty { .. } => f.write_str("BOTTLE_EMPTY"),
           CauldronLevelChangeEventChangeReason::BannerWash { .. } => f.write_str("BANNER_WASH"),
           CauldronLevelChangeEventChangeReason::ArmorWash { .. } => f.write_str("ARMOR_WASH"),
           CauldronLevelChangeEventChangeReason::ShulkerWash { .. } => f.write_str("SHULKER_WASH"),
           CauldronLevelChangeEventChangeReason::Extinguish { .. } => f.write_str("EXTINGUISH"),
           CauldronLevelChangeEventChangeReason::Evaporate { .. } => f.write_str("EVAPORATE"),
           CauldronLevelChangeEventChangeReason::NaturalFill { .. } => f.write_str("NATURAL_FILL"),
           CauldronLevelChangeEventChangeReason::Unknown { .. } => f.write_str("UNKNOWN"),
       }
   }
}

        impl<'mc> CauldronLevelChangeEventChangeReasonTrait<'mc> for CauldronLevelChangeEventChangeReason<'mc> {}
        
        pub trait CauldronLevelChangeEventChangeReasonTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<CauldronLevelChangeEventChangeReason<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/event/block/CauldronLevelChangeEvent/ChangeReason");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/event/block/CauldronLevelChangeEvent/ChangeReason;",
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
                    
"BUCKET_FILL" => Ok(CauldronLevelChangeEventChangeReason::BucketFill { inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env,obj)?}),
"BUCKET_EMPTY" => Ok(CauldronLevelChangeEventChangeReason::BucketEmpty { inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env,obj)?}),
"BOTTLE_FILL" => Ok(CauldronLevelChangeEventChangeReason::BottleFill { inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env,obj)?}),
"BOTTLE_EMPTY" => Ok(CauldronLevelChangeEventChangeReason::BottleEmpty { inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env,obj)?}),
"BANNER_WASH" => Ok(CauldronLevelChangeEventChangeReason::BannerWash { inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env,obj)?}),
"ARMOR_WASH" => Ok(CauldronLevelChangeEventChangeReason::ArmorWash { inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env,obj)?}),
"SHULKER_WASH" => Ok(CauldronLevelChangeEventChangeReason::ShulkerWash { inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env,obj)?}),
"EXTINGUISH" => Ok(CauldronLevelChangeEventChangeReason::Extinguish { inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env,obj)?}),
"EVAPORATE" => Ok(CauldronLevelChangeEventChangeReason::Evaporate { inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env,obj)?}),
"NATURAL_FILL" => Ok(CauldronLevelChangeEventChangeReason::NaturalFill { inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env,obj)?}),
"UNKNOWN" => Ok(CauldronLevelChangeEventChangeReason::Unknown { inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct CauldronLevelChangeEventChangeReasonStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for CauldronLevelChangeEventChangeReason<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::BucketFill { inner } => inner.0.clone(),
Self::BucketEmpty { inner } => inner.0.clone(),
Self::BottleFill { inner } => inner.0.clone(),
Self::BottleEmpty { inner } => inner.0.clone(),
Self::BannerWash { inner } => inner.0.clone(),
Self::ArmorWash { inner } => inner.0.clone(),
Self::ShulkerWash { inner } => inner.0.clone(),
Self::Extinguish { inner } => inner.0.clone(),
Self::Evaporate { inner } => inner.0.clone(),
Self::NaturalFill { inner } => inner.0.clone(),
Self::Unknown { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::BucketFill { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::BucketEmpty { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::BottleFill { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::BottleEmpty { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::BannerWash { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::ArmorWash { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::ShulkerWash { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Extinguish { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Evaporate { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::NaturalFill { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Unknown { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for CauldronLevelChangeEventChangeReason<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate CauldronLevelChangeEventChangeReason from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/CauldronLevelChangeEvent/ChangeReason")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a CauldronLevelChangeEventChangeReason object, got {}",
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
                    "BUCKET_FILL" => Ok(CauldronLevelChangeEventChangeReason::BucketFill { inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env,obj)?}),"BUCKET_EMPTY" => Ok(CauldronLevelChangeEventChangeReason::BucketEmpty { inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env,obj)?}),"BOTTLE_FILL" => Ok(CauldronLevelChangeEventChangeReason::BottleFill { inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env,obj)?}),"BOTTLE_EMPTY" => Ok(CauldronLevelChangeEventChangeReason::BottleEmpty { inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env,obj)?}),"BANNER_WASH" => Ok(CauldronLevelChangeEventChangeReason::BannerWash { inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env,obj)?}),"ARMOR_WASH" => Ok(CauldronLevelChangeEventChangeReason::ArmorWash { inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env,obj)?}),"SHULKER_WASH" => Ok(CauldronLevelChangeEventChangeReason::ShulkerWash { inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env,obj)?}),"EXTINGUISH" => Ok(CauldronLevelChangeEventChangeReason::Extinguish { inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env,obj)?}),"EVAPORATE" => Ok(CauldronLevelChangeEventChangeReason::Evaporate { inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env,obj)?}),"NATURAL_FILL" => Ok(CauldronLevelChangeEventChangeReason::NaturalFill { inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env,obj)?}),"UNKNOWN" => Ok(CauldronLevelChangeEventChangeReason::Unknown { inner: CauldronLevelChangeEventChangeReasonStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for CauldronLevelChangeEventChangeReasonStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for CauldronLevelChangeEventChangeReasonStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate CauldronLevelChangeEventChangeReasonStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/CauldronLevelChangeEvent/ChangeReason")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a CauldronLevelChangeEventChangeReasonStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> CauldronLevelChangeEventChangeReasonStruct<'mc> {

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::block::CauldronLevelChangeEventChangeReason<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/block/CauldronLevelChangeEvent/ChangeReason;");
let cls = jni.find_class("org/bukkit/event/block/CauldronLevelChangeEvent/ChangeReason"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::block::CauldronLevelChangeEventChangeReason::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct BlockCanBuildEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockCanBuildEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockCanBuildEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockCanBuildEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockCanBuildEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockCanBuildEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockCanBuildEventTrait<'mc> for BlockCanBuildEvent<'mc> {}
pub trait BlockCanBuildEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,block: impl Into<crate::block::Block<'mc>>,player: impl Into<crate::entity::Player<'mc>>,val_type: impl Into<crate::block::data::BlockData<'mc>>,can_build: std::option::Option<bool>) 
-> Result<crate::event::block::BlockCanBuildEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/block/Block;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/entity/Player;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
args.push(val_2);
sig += "Lorg/bukkit/block/data/BlockData;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
args.push(val_3);
if let Some(a) = can_build {
sig += "Z";
let val_4 = jni::objects::JValueGen::Bool(a.into());
args.push(val_4);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/block/BlockCanBuildEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::BlockCanBuildEvent::from_raw(&jni,res
)}
/// Gets whether or not the block can be built here.
/// 
/// By default, returns Minecraft's answer on whether the block can be
/// built here or not.
	fn is_buildable(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isBuildable",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Sets whether the block can be built here or not.
	fn set_buildable(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setBuildable",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the Material that we are trying to place.
	fn material(&self) 
-> Result<crate::Material<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Material;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMaterial",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Material::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the BlockData that we are trying to place.
	fn block_data(&self) 
-> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBlockData",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::data::BlockData::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the player who placed the block involved in this event.
/// 
/// May be null for legacy calls of the event.
	fn player(&self) 
-> Result<Option<crate::entity::Player<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Player;");
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
let cls = jni.find_class("org/bukkit/event/block/BlockCanBuildEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockCanBuildEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockCanBuildEvent into crate::event::block::BlockEvent")

   }
}
impl<'mc> crate::event::block::BlockEventTrait<'mc> for BlockCanBuildEvent<'mc> {}
#[repr(C)]
pub struct NotePlayEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for NotePlayEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for NotePlayEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate NotePlayEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/NotePlayEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a NotePlayEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> NotePlayEventTrait<'mc> for NotePlayEvent<'mc> {}
pub trait NotePlayEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,block: impl Into<crate::block::Block<'mc>>,instrument: impl Into<crate::Instrument<'mc>>,note: impl Into<crate::Note<'mc>>) 
-> Result<crate::event::block::NotePlayEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/Instrument;Lorg/bukkit/Note;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(instrument.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(note.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/block/NotePlayEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::NotePlayEvent::from_raw(&jni,res
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
/// Gets the {@link Instrument} to be used.
	fn instrument(&self) 
-> Result<crate::Instrument<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Instrument;");
let res = self.jni_ref().call_method(&self.jni_object(),"getInstrument",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Instrument::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the {@link Note} to be played.
	fn note(&self) 
-> Result<crate::Note<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Note;");
let res = self.jni_ref().call_method(&self.jni_object(),"getNote",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Note::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]
/// Overrides the {@link Instrument} to be used.
	fn set_instrument(&self,instrument: impl Into<crate::Instrument<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Instrument;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(instrument.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setInstrument",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[deprecated]
/// Overrides the {@link Note} to be played.
	fn set_note(&self,note: impl Into<crate::Note<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Note;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(note.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setNote",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
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
let cls = jni.find_class("org/bukkit/event/block/NotePlayEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for NotePlayEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting NotePlayEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for NotePlayEvent<'mc> {}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for NotePlayEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting NotePlayEvent into crate::event::block::BlockEvent")

   }
}
impl<'mc> crate::event::block::BlockEventTrait<'mc> for NotePlayEvent<'mc> {}
#[repr(C)]
pub struct BlockShearEntityEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockShearEntityEvent<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockShearEntityEvent<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockShearEntityEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockShearEntityEvent")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockShearEntityEvent object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockShearEntityEventTrait<'mc> for BlockShearEntityEvent<'mc> {}
pub trait BlockShearEntityEventTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,dispenser: impl Into<crate::block::Block<'mc>>,sheared: impl Into<crate::entity::Entity<'mc>>,tool: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<crate::event::block::BlockShearEntityEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/entity/Entity;Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(dispenser.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(sheared.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(tool.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/block/BlockShearEntityEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::block::BlockShearEntityEvent::from_raw(&jni,res
)}
/// Gets the entity that was sheared.
	fn entity(&self) 
-> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Entity;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEntity",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Entity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the item used to shear this sheep.
	fn tool(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getTool",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
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
let cls = jni.find_class("org/bukkit/event/block/BlockShearEntityEvent"); let cls = jni.translate_error_with_class(cls)?;
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for BlockShearEntityEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockShearEntityEvent into crate::event::Cancellable")

   }
}
impl<'mc> crate::event::CancellableTrait<'mc> for BlockShearEntityEvent<'mc> {}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BlockShearEntityEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockShearEntityEvent into crate::event::block::BlockEvent")

   }
}
impl<'mc> crate::event::block::BlockEventTrait<'mc> for BlockShearEntityEvent<'mc> {}
pub enum BlockIgniteEventIgniteCause<'mc> {
	Lava {inner: BlockIgniteEventIgniteCauseStruct<'mc>},
	FlintAndSteel {inner: BlockIgniteEventIgniteCauseStruct<'mc>},
	Spread {inner: BlockIgniteEventIgniteCauseStruct<'mc>},
	Lightning {inner: BlockIgniteEventIgniteCauseStruct<'mc>},
	Fireball {inner: BlockIgniteEventIgniteCauseStruct<'mc>},
	EnderCrystal {inner: BlockIgniteEventIgniteCauseStruct<'mc>},
	Explosion {inner: BlockIgniteEventIgniteCauseStruct<'mc>},
	Arrow {inner: BlockIgniteEventIgniteCauseStruct<'mc>},
}
impl<'mc> std::fmt::Display for BlockIgniteEventIgniteCause<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           BlockIgniteEventIgniteCause::Lava { .. } => f.write_str("LAVA"),
           BlockIgniteEventIgniteCause::FlintAndSteel { .. } => f.write_str("FLINT_AND_STEEL"),
           BlockIgniteEventIgniteCause::Spread { .. } => f.write_str("SPREAD"),
           BlockIgniteEventIgniteCause::Lightning { .. } => f.write_str("LIGHTNING"),
           BlockIgniteEventIgniteCause::Fireball { .. } => f.write_str("FIREBALL"),
           BlockIgniteEventIgniteCause::EnderCrystal { .. } => f.write_str("ENDER_CRYSTAL"),
           BlockIgniteEventIgniteCause::Explosion { .. } => f.write_str("EXPLOSION"),
           BlockIgniteEventIgniteCause::Arrow { .. } => f.write_str("ARROW"),
       }
   }
}

        impl<'mc> BlockIgniteEventIgniteCauseTrait<'mc> for BlockIgniteEventIgniteCause<'mc> {}
        
        pub trait BlockIgniteEventIgniteCauseTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<BlockIgniteEventIgniteCause<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/event/block/BlockIgniteEvent/IgniteCause");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/event/block/BlockIgniteEvent/IgniteCause;",
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
                    
"LAVA" => Ok(BlockIgniteEventIgniteCause::Lava { inner: BlockIgniteEventIgniteCauseStruct::from_raw(env,obj)?}),
"FLINT_AND_STEEL" => Ok(BlockIgniteEventIgniteCause::FlintAndSteel { inner: BlockIgniteEventIgniteCauseStruct::from_raw(env,obj)?}),
"SPREAD" => Ok(BlockIgniteEventIgniteCause::Spread { inner: BlockIgniteEventIgniteCauseStruct::from_raw(env,obj)?}),
"LIGHTNING" => Ok(BlockIgniteEventIgniteCause::Lightning { inner: BlockIgniteEventIgniteCauseStruct::from_raw(env,obj)?}),
"FIREBALL" => Ok(BlockIgniteEventIgniteCause::Fireball { inner: BlockIgniteEventIgniteCauseStruct::from_raw(env,obj)?}),
"ENDER_CRYSTAL" => Ok(BlockIgniteEventIgniteCause::EnderCrystal { inner: BlockIgniteEventIgniteCauseStruct::from_raw(env,obj)?}),
"EXPLOSION" => Ok(BlockIgniteEventIgniteCause::Explosion { inner: BlockIgniteEventIgniteCauseStruct::from_raw(env,obj)?}),
"ARROW" => Ok(BlockIgniteEventIgniteCause::Arrow { inner: BlockIgniteEventIgniteCauseStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct BlockIgniteEventIgniteCauseStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockIgniteEventIgniteCause<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::Lava { inner } => inner.0.clone(),
Self::FlintAndSteel { inner } => inner.0.clone(),
Self::Spread { inner } => inner.0.clone(),
Self::Lightning { inner } => inner.0.clone(),
Self::Fireball { inner } => inner.0.clone(),
Self::EnderCrystal { inner } => inner.0.clone(),
Self::Explosion { inner } => inner.0.clone(),
Self::Arrow { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::Lava { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::FlintAndSteel { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Spread { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Lightning { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Fireball { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::EnderCrystal { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Explosion { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Arrow { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockIgniteEventIgniteCause<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockIgniteEventIgniteCause from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockIgniteEvent/IgniteCause")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockIgniteEventIgniteCause object, got {}",
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
                    "LAVA" => Ok(BlockIgniteEventIgniteCause::Lava { inner: BlockIgniteEventIgniteCauseStruct::from_raw(env,obj)?}),"FLINT_AND_STEEL" => Ok(BlockIgniteEventIgniteCause::FlintAndSteel { inner: BlockIgniteEventIgniteCauseStruct::from_raw(env,obj)?}),"SPREAD" => Ok(BlockIgniteEventIgniteCause::Spread { inner: BlockIgniteEventIgniteCauseStruct::from_raw(env,obj)?}),"LIGHTNING" => Ok(BlockIgniteEventIgniteCause::Lightning { inner: BlockIgniteEventIgniteCauseStruct::from_raw(env,obj)?}),"FIREBALL" => Ok(BlockIgniteEventIgniteCause::Fireball { inner: BlockIgniteEventIgniteCauseStruct::from_raw(env,obj)?}),"ENDER_CRYSTAL" => Ok(BlockIgniteEventIgniteCause::EnderCrystal { inner: BlockIgniteEventIgniteCauseStruct::from_raw(env,obj)?}),"EXPLOSION" => Ok(BlockIgniteEventIgniteCause::Explosion { inner: BlockIgniteEventIgniteCauseStruct::from_raw(env,obj)?}),"ARROW" => Ok(BlockIgniteEventIgniteCause::Arrow { inner: BlockIgniteEventIgniteCauseStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for BlockIgniteEventIgniteCauseStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockIgniteEventIgniteCauseStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockIgniteEventIgniteCauseStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/block/BlockIgniteEvent/IgniteCause")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockIgniteEventIgniteCauseStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockIgniteEventIgniteCauseStruct<'mc> {

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::block::BlockIgniteEventIgniteCause<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/event/block/BlockIgniteEvent/IgniteCause;");
let cls = jni.find_class("org/bukkit/event/block/BlockIgniteEvent/IgniteCause"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::block::BlockIgniteEventIgniteCause::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
