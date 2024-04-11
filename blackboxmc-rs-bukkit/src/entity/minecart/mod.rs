#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;
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
    
impl<'mc> CommandMinecart<'mc> {
	pub fn command(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCommand",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
	pub fn set_command(&self,command: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)L();");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(command.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setCommand",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn set_name(&self,name: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)L();");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(name.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setName",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn set_damage(&self,damage: f64) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(D)L();");
let val_1 = jni::objects::JValueGen::Double(damage);
let res = self.jni_ref().call_method(&self.jni_object(),"setDamage",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn damage(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDamage",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn max_speed(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMaxSpeed",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn set_max_speed(&self,speed: f64) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(D)L();");
let val_1 = jni::objects::JValueGen::Double(speed);
let res = self.jni_ref().call_method(&self.jni_object(),"setMaxSpeed",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn is_slow_when_empty(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isSlowWhenEmpty",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn set_slow_when_empty(&self,slow: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)L();");
let val_1 = jni::objects::JValueGen::Bool(slow.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setSlowWhenEmpty",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn flying_velocity_mod(&self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::util::Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFlyingVelocityMod",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_flying_velocity_mod(&self,flying: impl Into<crate::util::Vector<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(flying.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFlyingVelocityMod",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn derailed_velocity_mod(&self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::util::Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDerailedVelocityMod",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_derailed_velocity_mod(&self,derailed: impl Into<crate::util::Vector<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(derailed.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setDerailedVelocityMod",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn set_display_block(&self,material: impl Into<crate::material::MaterialData<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/material/MaterialData;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(material.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setDisplayBlock",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn display_block(&self) 
-> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::material::MaterialData;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayBlock",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::MaterialData::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_display_block_data(&self,block_data: impl Into<crate::block::data::BlockData<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/data/BlockData;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block_data.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setDisplayBlockData",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn display_block_data(&self) 
-> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::block::data::BlockData;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayBlockData",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::data::BlockData::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_display_block_offset(&self,offset: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)L();");
let val_1 = jni::objects::JValueGen::Int(offset);
let res = self.jni_ref().call_method(&self.jni_object(),"setDisplayBlockOffset",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn display_block_offset(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayBlockOffset",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::entity::Minecart<'mc>> for CommandMinecart<'mc>{

fn into(self) -> crate::entity::Minecart<'mc> {

crate::entity::Minecart::from_raw(&self.jni_ref(), self.1).expect("Error converting CommandMinecart into crate::entity::Minecart")

   }
}
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
    
impl<'mc> RideableMinecart<'mc> {
	pub fn set_damage(&self,damage: f64) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(D)L();");
let val_1 = jni::objects::JValueGen::Double(damage);
let res = self.jni_ref().call_method(&self.jni_object(),"setDamage",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn damage(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDamage",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn max_speed(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMaxSpeed",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn set_max_speed(&self,speed: f64) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(D)L();");
let val_1 = jni::objects::JValueGen::Double(speed);
let res = self.jni_ref().call_method(&self.jni_object(),"setMaxSpeed",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn is_slow_when_empty(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isSlowWhenEmpty",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn set_slow_when_empty(&self,slow: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)L();");
let val_1 = jni::objects::JValueGen::Bool(slow.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setSlowWhenEmpty",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn flying_velocity_mod(&self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::util::Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFlyingVelocityMod",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_flying_velocity_mod(&self,flying: impl Into<crate::util::Vector<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(flying.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFlyingVelocityMod",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn derailed_velocity_mod(&self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::util::Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDerailedVelocityMod",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_derailed_velocity_mod(&self,derailed: impl Into<crate::util::Vector<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(derailed.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setDerailedVelocityMod",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn set_display_block(&self,material: impl Into<crate::material::MaterialData<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/material/MaterialData;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(material.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setDisplayBlock",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn display_block(&self) 
-> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::material::MaterialData;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayBlock",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::MaterialData::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_display_block_data(&self,block_data: impl Into<crate::block::data::BlockData<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/data/BlockData;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block_data.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setDisplayBlockData",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn display_block_data(&self) 
-> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::block::data::BlockData;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayBlockData",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::data::BlockData::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_display_block_offset(&self,offset: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)L();");
let val_1 = jni::objects::JValueGen::Int(offset);
let res = self.jni_ref().call_method(&self.jni_object(),"setDisplayBlockOffset",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn display_block_offset(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayBlockOffset",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::entity::Minecart<'mc>> for RideableMinecart<'mc>{

fn into(self) -> crate::entity::Minecart<'mc> {

crate::entity::Minecart::from_raw(&self.jni_ref(), self.1).expect("Error converting RideableMinecart into crate::entity::Minecart")

   }
}
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
    
impl<'mc> PoweredMinecart<'mc> {
	pub fn fuel(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFuel",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn set_fuel(&self,fuel: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)L();");
let val_1 = jni::objects::JValueGen::Int(fuel);
let res = self.jni_ref().call_method(&self.jni_object(),"setFuel",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn set_damage(&self,damage: f64) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(D)L();");
let val_1 = jni::objects::JValueGen::Double(damage);
let res = self.jni_ref().call_method(&self.jni_object(),"setDamage",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn damage(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDamage",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn max_speed(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMaxSpeed",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn set_max_speed(&self,speed: f64) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(D)L();");
let val_1 = jni::objects::JValueGen::Double(speed);
let res = self.jni_ref().call_method(&self.jni_object(),"setMaxSpeed",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn is_slow_when_empty(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isSlowWhenEmpty",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn set_slow_when_empty(&self,slow: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)L();");
let val_1 = jni::objects::JValueGen::Bool(slow.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setSlowWhenEmpty",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn flying_velocity_mod(&self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::util::Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFlyingVelocityMod",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_flying_velocity_mod(&self,flying: impl Into<crate::util::Vector<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(flying.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFlyingVelocityMod",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn derailed_velocity_mod(&self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::util::Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDerailedVelocityMod",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_derailed_velocity_mod(&self,derailed: impl Into<crate::util::Vector<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(derailed.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setDerailedVelocityMod",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn set_display_block(&self,material: impl Into<crate::material::MaterialData<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/material/MaterialData;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(material.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setDisplayBlock",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn display_block(&self) 
-> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::material::MaterialData;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayBlock",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::MaterialData::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_display_block_data(&self,block_data: impl Into<crate::block::data::BlockData<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/data/BlockData;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block_data.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setDisplayBlockData",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn display_block_data(&self) 
-> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::block::data::BlockData;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayBlockData",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::data::BlockData::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_display_block_offset(&self,offset: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)L();");
let val_1 = jni::objects::JValueGen::Int(offset);
let res = self.jni_ref().call_method(&self.jni_object(),"setDisplayBlockOffset",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn display_block_offset(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayBlockOffset",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::entity::Minecart<'mc>> for PoweredMinecart<'mc>{

fn into(self) -> crate::entity::Minecart<'mc> {

crate::entity::Minecart::from_raw(&self.jni_ref(), self.1).expect("Error converting PoweredMinecart into crate::entity::Minecart")

   }
}
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
    
impl<'mc> ExplosiveMinecart<'mc> {
	pub fn set_fuse_ticks(&self,ticks: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)L();");
let val_1 = jni::objects::JValueGen::Int(ticks);
let res = self.jni_ref().call_method(&self.jni_object(),"setFuseTicks",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn fuse_ticks(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFuseTicks",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn ignite(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()L();");
let res = self.jni_ref().call_method(&self.jni_object(),"ignite",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn is_ignited(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isIgnited",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn explode_with_power(&self,power: std::option::Option<f64>) 
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
	pub fn set_damage(&self,damage: f64) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(D)L();");
let val_1 = jni::objects::JValueGen::Double(damage);
let res = self.jni_ref().call_method(&self.jni_object(),"setDamage",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn damage(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDamage",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn max_speed(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMaxSpeed",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn set_max_speed(&self,speed: f64) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(D)L();");
let val_1 = jni::objects::JValueGen::Double(speed);
let res = self.jni_ref().call_method(&self.jni_object(),"setMaxSpeed",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn is_slow_when_empty(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isSlowWhenEmpty",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn set_slow_when_empty(&self,slow: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)L();");
let val_1 = jni::objects::JValueGen::Bool(slow.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setSlowWhenEmpty",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn flying_velocity_mod(&self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::util::Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFlyingVelocityMod",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_flying_velocity_mod(&self,flying: impl Into<crate::util::Vector<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(flying.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFlyingVelocityMod",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn derailed_velocity_mod(&self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::util::Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDerailedVelocityMod",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_derailed_velocity_mod(&self,derailed: impl Into<crate::util::Vector<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(derailed.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setDerailedVelocityMod",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn set_display_block(&self,material: impl Into<crate::material::MaterialData<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/material/MaterialData;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(material.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setDisplayBlock",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn display_block(&self) 
-> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::material::MaterialData;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayBlock",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::MaterialData::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_display_block_data(&self,block_data: impl Into<crate::block::data::BlockData<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/data/BlockData;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block_data.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setDisplayBlockData",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn display_block_data(&self) 
-> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::block::data::BlockData;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayBlockData",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::data::BlockData::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_display_block_offset(&self,offset: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)L();");
let val_1 = jni::objects::JValueGen::Int(offset);
let res = self.jni_ref().call_method(&self.jni_object(),"setDisplayBlockOffset",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn display_block_offset(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayBlockOffset",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::entity::Minecart<'mc>> for ExplosiveMinecart<'mc>{

fn into(self) -> crate::entity::Minecart<'mc> {

crate::entity::Minecart::from_raw(&self.jni_ref(), self.1).expect("Error converting ExplosiveMinecart into crate::entity::Minecart")

   }
}
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
    
impl<'mc> SpawnerMinecart<'mc> {
	pub fn set_damage(&self,damage: f64) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(D)L();");
let val_1 = jni::objects::JValueGen::Double(damage);
let res = self.jni_ref().call_method(&self.jni_object(),"setDamage",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn damage(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDamage",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn max_speed(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMaxSpeed",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn set_max_speed(&self,speed: f64) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(D)L();");
let val_1 = jni::objects::JValueGen::Double(speed);
let res = self.jni_ref().call_method(&self.jni_object(),"setMaxSpeed",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn is_slow_when_empty(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isSlowWhenEmpty",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn set_slow_when_empty(&self,slow: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)L();");
let val_1 = jni::objects::JValueGen::Bool(slow.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setSlowWhenEmpty",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn flying_velocity_mod(&self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::util::Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFlyingVelocityMod",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_flying_velocity_mod(&self,flying: impl Into<crate::util::Vector<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(flying.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFlyingVelocityMod",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn derailed_velocity_mod(&self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::util::Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDerailedVelocityMod",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_derailed_velocity_mod(&self,derailed: impl Into<crate::util::Vector<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(derailed.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setDerailedVelocityMod",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn set_display_block(&self,material: impl Into<crate::material::MaterialData<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/material/MaterialData;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(material.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setDisplayBlock",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn display_block(&self) 
-> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::material::MaterialData;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayBlock",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::MaterialData::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_display_block_data(&self,block_data: impl Into<crate::block::data::BlockData<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/data/BlockData;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block_data.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setDisplayBlockData",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn display_block_data(&self) 
-> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::block::data::BlockData;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayBlockData",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::data::BlockData::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_display_block_offset(&self,offset: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)L();");
let val_1 = jni::objects::JValueGen::Int(offset);
let res = self.jni_ref().call_method(&self.jni_object(),"setDisplayBlockOffset",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn display_block_offset(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayBlockOffset",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::entity::Minecart<'mc>> for SpawnerMinecart<'mc>{

fn into(self) -> crate::entity::Minecart<'mc> {

crate::entity::Minecart::from_raw(&self.jni_ref(), self.1).expect("Error converting SpawnerMinecart into crate::entity::Minecart")

   }
}
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
    
impl<'mc> HopperMinecart<'mc> {
	pub fn is_enabled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isEnabled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn set_enabled(&self,enabled: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)L();");
let val_1 = jni::objects::JValueGen::Bool(enabled.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setEnabled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn set_damage(&self,damage: f64) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(D)L();");
let val_1 = jni::objects::JValueGen::Double(damage);
let res = self.jni_ref().call_method(&self.jni_object(),"setDamage",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn damage(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDamage",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn max_speed(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMaxSpeed",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn set_max_speed(&self,speed: f64) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(D)L();");
let val_1 = jni::objects::JValueGen::Double(speed);
let res = self.jni_ref().call_method(&self.jni_object(),"setMaxSpeed",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn is_slow_when_empty(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isSlowWhenEmpty",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn set_slow_when_empty(&self,slow: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)L();");
let val_1 = jni::objects::JValueGen::Bool(slow.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setSlowWhenEmpty",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn flying_velocity_mod(&self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::util::Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFlyingVelocityMod",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_flying_velocity_mod(&self,flying: impl Into<crate::util::Vector<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(flying.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFlyingVelocityMod",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn derailed_velocity_mod(&self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::util::Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDerailedVelocityMod",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_derailed_velocity_mod(&self,derailed: impl Into<crate::util::Vector<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(derailed.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setDerailedVelocityMod",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn set_display_block(&self,material: impl Into<crate::material::MaterialData<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/material/MaterialData;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(material.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setDisplayBlock",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn display_block(&self) 
-> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::material::MaterialData;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayBlock",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::MaterialData::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_display_block_data(&self,block_data: impl Into<crate::block::data::BlockData<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/data/BlockData;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block_data.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setDisplayBlockData",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn display_block_data(&self) 
-> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::block::data::BlockData;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayBlockData",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::data::BlockData::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_display_block_offset(&self,offset: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)L();");
let val_1 = jni::objects::JValueGen::Int(offset);
let res = self.jni_ref().call_method(&self.jni_object(),"setDisplayBlockOffset",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn display_block_offset(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayBlockOffset",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn inventory(&self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::Inventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getInventory",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_loot_table(&self,table: impl Into<crate::loot::LootTable<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/loot/LootTable;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(table.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setLootTable",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn loot_table(&self) 
-> Result<Option<crate::loot::LootTable<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::loot::LootTable;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLootTable",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::loot::LootTable::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn set_seed(&self,seed: i64) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(J)L();");
let val_1 = jni::objects::JValueGen::Long(seed);
let res = self.jni_ref().call_method(&self.jni_object(),"setSeed",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn seed(&self) 
-> Result<i64, Box<dyn std::error::Error>>

{let sig = String::from("()Li64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSeed",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.j()?
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::entity::Minecart<'mc>> for HopperMinecart<'mc>{

fn into(self) -> crate::entity::Minecart<'mc> {

crate::entity::Minecart::from_raw(&self.jni_ref(), self.1).expect("Error converting HopperMinecart into crate::entity::Minecart")

   }
}
impl<'mc> Into<crate::inventory::InventoryHolder<'mc>> for HopperMinecart<'mc>{

fn into(self) -> crate::inventory::InventoryHolder<'mc> {

crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), self.1).expect("Error converting HopperMinecart into crate::inventory::InventoryHolder")

   }
}
impl<'mc> Into<crate::loot::Lootable<'mc>> for HopperMinecart<'mc>{

fn into(self) -> crate::loot::Lootable<'mc> {

crate::loot::Lootable::from_raw(&self.jni_ref(), self.1).expect("Error converting HopperMinecart into crate::loot::Lootable")

   }
}
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
    
impl<'mc> StorageMinecart<'mc> {
	pub fn set_damage(&self,damage: f64) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(D)L();");
let val_1 = jni::objects::JValueGen::Double(damage);
let res = self.jni_ref().call_method(&self.jni_object(),"setDamage",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn damage(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDamage",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn max_speed(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMaxSpeed",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn set_max_speed(&self,speed: f64) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(D)L();");
let val_1 = jni::objects::JValueGen::Double(speed);
let res = self.jni_ref().call_method(&self.jni_object(),"setMaxSpeed",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn is_slow_when_empty(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isSlowWhenEmpty",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn set_slow_when_empty(&self,slow: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)L();");
let val_1 = jni::objects::JValueGen::Bool(slow.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setSlowWhenEmpty",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn flying_velocity_mod(&self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::util::Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFlyingVelocityMod",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_flying_velocity_mod(&self,flying: impl Into<crate::util::Vector<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(flying.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFlyingVelocityMod",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn derailed_velocity_mod(&self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::util::Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDerailedVelocityMod",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_derailed_velocity_mod(&self,derailed: impl Into<crate::util::Vector<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(derailed.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setDerailedVelocityMod",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn set_display_block(&self,material: impl Into<crate::material::MaterialData<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/material/MaterialData;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(material.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setDisplayBlock",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn display_block(&self) 
-> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::material::MaterialData;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayBlock",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::MaterialData::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_display_block_data(&self,block_data: impl Into<crate::block::data::BlockData<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/data/BlockData;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block_data.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setDisplayBlockData",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn display_block_data(&self) 
-> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::block::data::BlockData;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayBlockData",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::data::BlockData::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_display_block_offset(&self,offset: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)L();");
let val_1 = jni::objects::JValueGen::Int(offset);
let res = self.jni_ref().call_method(&self.jni_object(),"setDisplayBlockOffset",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn display_block_offset(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayBlockOffset",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn inventory(&self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::Inventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getInventory",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_loot_table(&self,table: impl Into<crate::loot::LootTable<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/loot/LootTable;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(table.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setLootTable",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn loot_table(&self) 
-> Result<Option<crate::loot::LootTable<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::loot::LootTable;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLootTable",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::loot::LootTable::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn set_seed(&self,seed: i64) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(J)L();");
let val_1 = jni::objects::JValueGen::Long(seed);
let res = self.jni_ref().call_method(&self.jni_object(),"setSeed",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn seed(&self) 
-> Result<i64, Box<dyn std::error::Error>>

{let sig = String::from("()Li64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSeed",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.j()?
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::entity::Minecart<'mc>> for StorageMinecart<'mc>{

fn into(self) -> crate::entity::Minecart<'mc> {

crate::entity::Minecart::from_raw(&self.jni_ref(), self.1).expect("Error converting StorageMinecart into crate::entity::Minecart")

   }
}
impl<'mc> Into<crate::inventory::InventoryHolder<'mc>> for StorageMinecart<'mc>{

fn into(self) -> crate::inventory::InventoryHolder<'mc> {

crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), self.1).expect("Error converting StorageMinecart into crate::inventory::InventoryHolder")

   }
}
impl<'mc> Into<crate::loot::Lootable<'mc>> for StorageMinecart<'mc>{

fn into(self) -> crate::loot::Lootable<'mc> {

crate::loot::Lootable::from_raw(&self.jni_ref(), self.1).expect("Error converting StorageMinecart into crate::loot::Lootable")

   }
}
