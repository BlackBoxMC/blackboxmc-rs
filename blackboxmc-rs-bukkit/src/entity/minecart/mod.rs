#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;/*org/bukkit/entity/minecart/mod.rs*/

#[repr(C)]
pub struct CommandMinecart<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for CommandMinecart<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for CommandMinecart<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate CommandMinecart from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/entity/minecart/CommandMinecart")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a CommandMinecart object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> CommandMinecartTrait<'mc> for CommandMinecart<'mc> {}
pub trait CommandMinecartTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Gets the command that this CommandMinecart will run when activated.
/// This will never return null.If the CommandMinecart does not have a
/// command, an empty String will be returned instead.
	fn command(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCommand",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Sets the command that this CommandMinecart will run when activated.
/// Setting the command to null is the same as setting it to an empty
/// String.
	fn set_command(&self,command: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(command.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setCommand",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Sets the name of this CommandMinecart.The name is used with commands
/// that this CommandMinecart executes.Setting the name to null is the
/// same as setting it to "@".
	fn set_name(&self,name: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(name.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setName",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::entity::Minecart<'mc>> for CommandMinecart<'mc>{

fn into(self) -> crate::entity::Minecart<'mc> {

crate::entity::Minecart::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting CommandMinecart into crate::entity::Minecart")

   }
}
impl<'mc> crate::entity::MinecartTrait<'mc> for CommandMinecart<'mc> {}
#[repr(C)]
pub struct RideableMinecart<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for RideableMinecart<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for RideableMinecart<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate RideableMinecart from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/entity/minecart/RideableMinecart")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a RideableMinecart object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> RideableMinecartTrait<'mc> for RideableMinecart<'mc> {}
pub trait RideableMinecartTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::entity::Minecart<'mc>> for RideableMinecart<'mc>{

fn into(self) -> crate::entity::Minecart<'mc> {

crate::entity::Minecart::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting RideableMinecart into crate::entity::Minecart")

   }
}
impl<'mc> crate::entity::MinecartTrait<'mc> for RideableMinecart<'mc> {}
#[repr(C)]
pub struct PoweredMinecart<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PoweredMinecart<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PoweredMinecart<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PoweredMinecart from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/entity/minecart/PoweredMinecart")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PoweredMinecart object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PoweredMinecartTrait<'mc> for PoweredMinecart<'mc> {}
pub trait PoweredMinecartTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Get the number of ticks until the minecart runs out of fuel.
	fn fuel(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getFuel",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Set the number of ticks until the minecart runs out of fuel.
	fn set_fuel(&self,fuel: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(fuel);
let res = self.jni_ref().call_method(&self.jni_object(),"setFuel",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::entity::Minecart<'mc>> for PoweredMinecart<'mc>{

fn into(self) -> crate::entity::Minecart<'mc> {

crate::entity::Minecart::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PoweredMinecart into crate::entity::Minecart")

   }
}
impl<'mc> crate::entity::MinecartTrait<'mc> for PoweredMinecart<'mc> {}
#[repr(C)]
pub struct ExplosiveMinecart<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ExplosiveMinecart<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ExplosiveMinecart<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ExplosiveMinecart from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/entity/minecart/ExplosiveMinecart")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ExplosiveMinecart object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ExplosiveMinecartTrait<'mc> for ExplosiveMinecart<'mc> {}
pub trait ExplosiveMinecartTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Set the fuse ticks of this minecart.
/// If the fuse ticks are set to a non-zero value, this will ignite the
/// explosive.
	fn set_fuse_ticks(&self,ticks: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(ticks);
let res = self.jni_ref().call_method(&self.jni_object(),"setFuseTicks",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the fuse ticks of this minecart.
/// If the fuse ticks reach 0, the minecart will explode.
	fn fuse_ticks(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getFuseTicks",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Ignite this minecart's fuse naturally.
	fn ignite(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"ignite",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Check whether or not this minecart's fuse has been ignited.
	fn is_ignited(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isIgnited",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Immediately explode this minecart with the given power.
	fn explode(&self,power: std::option::Option<f64>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = power {
sig += "D";
let val_1 = jni::objects::JValueGen::Double(a);
args.push(val_1);
}
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"explode",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::entity::Minecart<'mc>> for ExplosiveMinecart<'mc>{

fn into(self) -> crate::entity::Minecart<'mc> {

crate::entity::Minecart::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting ExplosiveMinecart into crate::entity::Minecart")

   }
}
impl<'mc> crate::entity::MinecartTrait<'mc> for ExplosiveMinecart<'mc> {}
#[repr(C)]
pub struct SpawnerMinecart<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for SpawnerMinecart<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for SpawnerMinecart<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate SpawnerMinecart from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/entity/minecart/SpawnerMinecart")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a SpawnerMinecart object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> SpawnerMinecartTrait<'mc> for SpawnerMinecart<'mc> {}
pub trait SpawnerMinecartTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::entity::Minecart<'mc>> for SpawnerMinecart<'mc>{

fn into(self) -> crate::entity::Minecart<'mc> {

crate::entity::Minecart::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting SpawnerMinecart into crate::entity::Minecart")

   }
}
impl<'mc> crate::entity::MinecartTrait<'mc> for SpawnerMinecart<'mc> {}
#[repr(C)]
pub struct HopperMinecart<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for HopperMinecart<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for HopperMinecart<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate HopperMinecart from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/entity/minecart/HopperMinecart")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a HopperMinecart object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> HopperMinecartTrait<'mc> for HopperMinecart<'mc> {}
pub trait HopperMinecartTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Checks whether or not this Minecart will pick up
/// items into its inventory.
	fn is_enabled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isEnabled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Sets whether this Minecart will pick up items.
	fn set_enabled(&self,enabled: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(enabled.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setEnabled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::entity::Minecart<'mc>> for HopperMinecart<'mc>{

fn into(self) -> crate::entity::Minecart<'mc> {

crate::entity::Minecart::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting HopperMinecart into crate::entity::Minecart")

   }
}
impl<'mc> crate::entity::MinecartTrait<'mc> for HopperMinecart<'mc> {}
impl<'mc> Into<crate::inventory::InventoryHolder<'mc>> for HopperMinecart<'mc>{

fn into(self) -> crate::inventory::InventoryHolder<'mc> {

crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting HopperMinecart into crate::inventory::InventoryHolder")

   }
}
impl<'mc> crate::inventory::InventoryHolderTrait<'mc> for HopperMinecart<'mc> {}
impl<'mc> Into<crate::loot::Lootable<'mc>> for HopperMinecart<'mc>{

fn into(self) -> crate::loot::Lootable<'mc> {

crate::loot::Lootable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting HopperMinecart into crate::loot::Lootable")

   }
}
impl<'mc> crate::loot::LootableTrait<'mc> for HopperMinecart<'mc> {}
#[repr(C)]
pub struct StorageMinecart<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for StorageMinecart<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for StorageMinecart<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate StorageMinecart from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/entity/minecart/StorageMinecart")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a StorageMinecart object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> StorageMinecartTrait<'mc> for StorageMinecart<'mc> {}
pub trait StorageMinecartTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::entity::Minecart<'mc>> for StorageMinecart<'mc>{

fn into(self) -> crate::entity::Minecart<'mc> {

crate::entity::Minecart::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting StorageMinecart into crate::entity::Minecart")

   }
}
impl<'mc> crate::entity::MinecartTrait<'mc> for StorageMinecart<'mc> {}
impl<'mc> Into<crate::inventory::InventoryHolder<'mc>> for StorageMinecart<'mc>{

fn into(self) -> crate::inventory::InventoryHolder<'mc> {

crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting StorageMinecart into crate::inventory::InventoryHolder")

   }
}
impl<'mc> crate::inventory::InventoryHolderTrait<'mc> for StorageMinecart<'mc> {}
impl<'mc> Into<crate::loot::Lootable<'mc>> for StorageMinecart<'mc>{

fn into(self) -> crate::loot::Lootable<'mc> {

crate::loot::Lootable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting StorageMinecart into crate::loot::Lootable")

   }
}
impl<'mc> crate::loot::LootableTrait<'mc> for StorageMinecart<'mc> {}
