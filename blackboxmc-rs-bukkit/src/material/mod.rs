#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;/*org/bukkit/material/mod.rs*/

#[repr(C)]
pub struct Attachable<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Attachable<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Attachable<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Attachable from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Attachable")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Attachable object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> AttachableTrait<'mc> for Attachable<'mc> {}
pub trait AttachableTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Gets the face that this block is attached on
	fn attached_face(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAttachedFace",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::material::Directional<'mc>> for Attachable<'mc>{

fn into(self) -> crate::material::Directional<'mc> {

crate::material::Directional::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Attachable into crate::material::Directional")

   }
}
impl<'mc> crate::material::DirectionalTrait<'mc> for Attachable<'mc> {}
#[repr(C)]
pub struct Dispenser<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Dispenser<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Dispenser<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Dispenser from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Dispenser")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Dispenser object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> DispenserTrait<'mc> for Dispenser<'mc> {}
pub trait DispenserTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::Dispenser<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Dispenser"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Dispenser::from_raw(&jni,res
)}

	fn set_facing_direction(&self,face: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFacingDirection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn facing(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFacing",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn clone(&self) 
-> Result<crate::material::Dispenser<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Dispenser;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Dispenser::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::material::FurnaceAndDispenser<'mc>> for Dispenser<'mc>{

fn into(self) -> crate::material::FurnaceAndDispenser<'mc> {

crate::material::FurnaceAndDispenser::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Dispenser into crate::material::FurnaceAndDispenser")

   }
}
impl<'mc> crate::material::FurnaceAndDispenserTrait<'mc> for Dispenser<'mc> {}
#[repr(C)]
pub struct LongGrass<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for LongGrass<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for LongGrass<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate LongGrass from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/LongGrass")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a LongGrass object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> LongGrassTrait<'mc> for LongGrass<'mc> {}
pub trait LongGrassTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::LongGrass<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/LongGrass"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::LongGrass::from_raw(&jni,res
)}
/// Gets the current species of this grass
	fn species(&self) 
-> Result<crate::GrassSpecies<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/GrassSpecies;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSpecies",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::GrassSpecies::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the species of this grass
	fn set_species(&self,species: impl Into<crate::GrassSpecies<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/GrassSpecies;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(species.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setSpecies",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::LongGrass<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/LongGrass;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::LongGrass::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for LongGrass<'mc> {
            fn to_string(&self) -> String {
                match LongGrassTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling LongGrass.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::MaterialData<'mc>> for LongGrass<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting LongGrass into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for LongGrass<'mc> {}
#[repr(C)]
pub struct Door<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Door<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Door<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Door from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Door")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Door object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> DoorTrait<'mc> for Door<'mc> {}
pub trait DoorTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Constructs the bottom half of a wooden door of the given species, facing the specified direction and set to open
/// or closed
	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,species: std::option::Option<impl Into<crate::TreeSpecies<'mc>>>,face: std::option::Option<impl Into<crate::block::BlockFace<'mc>>>,is_open: std::option::Option<bool>) 
-> Result<crate::material::Door<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = species {
sig += "Lorg/bukkit/TreeSpecies;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = face {
sig += "Lorg/bukkit/block/BlockFace;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
if let Some(a) = is_open {
sig += "Z";
let val_3 = jni::objects::JValueGen::Bool(a.into());
args.push(val_3);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Door"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Door::from_raw(&jni,res
)}
/// Returns the item type of a wooden door for the given tree species.
	fn get_wood_door_of_species(jni: &blackboxmc_general::SharedJNIEnv<'mc>,species: impl Into<crate::TreeSpecies<'mc>>) 
-> Result<crate::Material<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/TreeSpecies;)Lorg/bukkit/Material;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(species.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/material/Door"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getWoodDoorOfSpecies",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::Material::from_raw(&jni,obj
)}
/// Result is undefined if <code>isTopHalf()</code> is true.
	fn is_open(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isOpen",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Set whether the door is open. Undefined if <code>isTopHalf()</code> is true.
	fn set_open(&self,is_open: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(is_open.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setOpen",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn is_top_half(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isTopHalf",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Configure this part of the door to be either the top or the bottom half
	fn set_top_half(&self,is_top_half: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(is_top_half.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setTopHalf",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[deprecated]

	fn hinge_corner(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHingeCorner",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Set the direction that this door should is facing.
/// Undefined if <code>isTopHalf()</code> is true.
	fn set_facing_direction(&self,face: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFacingDirection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the direction that this door is facing.
/// Undefined if <code>isTopHalf()</code> is true.
	fn facing(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFacing",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Returns the side of the door the hinge is on.
/// Undefined if <code>isTopHalf()</code> is false.
	fn hinge(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"getHinge",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Set whether the hinge is on the left or right side. Left is false, right is true.
/// Undefined if <code>isTopHalf()</code> is false.
	fn set_hinge(&self,is_hinge_right: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(is_hinge_right.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setHinge",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn clone(&self) 
-> Result<crate::material::Door<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Door;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Door::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Door<'mc> {
            fn to_string(&self) -> String {
                match DoorTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Door.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::Directional<'mc>> for Door<'mc>{

fn into(self) -> crate::material::Directional<'mc> {

crate::material::Directional::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Door into crate::material::Directional")

   }
}
impl<'mc> crate::material::DirectionalTrait<'mc> for Door<'mc> {}
impl<'mc> Into<crate::material::Openable<'mc>> for Door<'mc>{

fn into(self) -> crate::material::Openable<'mc> {

crate::material::Openable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Door into crate::material::Openable")

   }
}
impl<'mc> crate::material::OpenableTrait<'mc> for Door<'mc> {}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Door<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Door into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for Door<'mc> {}
#[repr(C)]
pub struct TripwireHook<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for TripwireHook<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for TripwireHook<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate TripwireHook from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/TripwireHook")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a TripwireHook object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> TripwireHookTrait<'mc> for TripwireHook<'mc> {}
pub trait TripwireHookTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::TripwireHook<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/TripwireHook"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::TripwireHook::from_raw(&jni,res
)}
/// Test if tripwire is connected
	fn is_connected(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isConnected",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Set tripwire connection state
	fn set_connected(&self,connected: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(connected.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setConnected",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Test if hook is currently activated
	fn is_activated(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isActivated",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Set hook activated state
	fn set_activated(&self,act: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(act.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setActivated",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn set_facing_direction(&self,face: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFacingDirection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn attached_face(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAttachedFace",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn is_powered(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isPowered",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn clone(&self) 
-> Result<crate::material::TripwireHook<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/TripwireHook;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::TripwireHook::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for TripwireHook<'mc> {
            fn to_string(&self) -> String {
                match TripwireHookTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling TripwireHook.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::Redstone<'mc>> for TripwireHook<'mc>{

fn into(self) -> crate::material::Redstone<'mc> {

crate::material::Redstone::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting TripwireHook into crate::material::Redstone")

   }
}
impl<'mc> crate::material::RedstoneTrait<'mc> for TripwireHook<'mc> {}
impl<'mc> Into<crate::material::SimpleAttachableMaterialData<'mc>> for TripwireHook<'mc>{

fn into(self) -> crate::material::SimpleAttachableMaterialData<'mc> {

crate::material::SimpleAttachableMaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting TripwireHook into crate::material::SimpleAttachableMaterialData")

   }
}
impl<'mc> crate::material::SimpleAttachableMaterialDataTrait<'mc> for TripwireHook<'mc> {}
#[repr(C)]
pub struct RedstoneWire<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for RedstoneWire<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for RedstoneWire<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate RedstoneWire from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/RedstoneWire")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a RedstoneWire object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> RedstoneWireTrait<'mc> for RedstoneWire<'mc> {}
pub trait RedstoneWireTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::RedstoneWire<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/RedstoneWire"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::RedstoneWire::from_raw(&jni,res
)}
/// Gets the current state of this Material, indicating if it's powered or
/// unpowered
	fn is_powered(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isPowered",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::RedstoneWire<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/RedstoneWire;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::RedstoneWire::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for RedstoneWire<'mc> {
            fn to_string(&self) -> String {
                match RedstoneWireTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling RedstoneWire.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::Redstone<'mc>> for RedstoneWire<'mc>{

fn into(self) -> crate::material::Redstone<'mc> {

crate::material::Redstone::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting RedstoneWire into crate::material::Redstone")

   }
}
impl<'mc> crate::material::RedstoneTrait<'mc> for RedstoneWire<'mc> {}
impl<'mc> Into<crate::material::MaterialData<'mc>> for RedstoneWire<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting RedstoneWire into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for RedstoneWire<'mc> {}
#[repr(C)]
pub struct ExtendedRails<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ExtendedRails<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ExtendedRails<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ExtendedRails from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/ExtendedRails")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ExtendedRails object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ExtendedRailsTrait<'mc> for ExtendedRails<'mc> {}
pub trait ExtendedRailsTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: impl Into<crate::Material<'mc>>,data: std::option::Option<i8>) 
-> Result<crate::material::ExtendedRails<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/ExtendedRails"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::ExtendedRails::from_raw(&jni,res
)}

	fn is_curve(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCurve",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_direction(&self,face: impl Into<crate::block::BlockFace<'mc>>,is_on_slope: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;Z)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Bool(is_on_slope.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setDirection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn clone(&self) 
-> Result<crate::material::ExtendedRails<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/ExtendedRails;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::ExtendedRails::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::material::Rails<'mc>> for ExtendedRails<'mc>{

fn into(self) -> crate::material::Rails<'mc> {

crate::material::Rails::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting ExtendedRails into crate::material::Rails")

   }
}
impl<'mc> crate::material::RailsTrait<'mc> for ExtendedRails<'mc> {}
#[repr(C)]
pub struct Chest<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Chest<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Chest<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Chest from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Chest")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Chest object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ChestTrait<'mc> for Chest<'mc> {}
pub trait ChestTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::Chest<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Chest"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Chest::from_raw(&jni,res
)}

	fn clone(&self) 
-> Result<crate::material::Chest<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Chest;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Chest::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::material::DirectionalContainer<'mc>> for Chest<'mc>{

fn into(self) -> crate::material::DirectionalContainer<'mc> {

crate::material::DirectionalContainer::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Chest into crate::material::DirectionalContainer")

   }
}
impl<'mc> crate::material::DirectionalContainerTrait<'mc> for Chest<'mc> {}
#[repr(C)]
pub struct Sandstone<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Sandstone<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Sandstone<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Sandstone from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Sandstone")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Sandstone object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> SandstoneTrait<'mc> for Sandstone<'mc> {}
pub trait SandstoneTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::Sandstone<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Sandstone"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Sandstone::from_raw(&jni,res
)}
/// Gets the current type of this sandstone
	fn get_type(&self) 
-> Result<crate::SandstoneType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/SandstoneType;");
let res = self.jni_ref().call_method(&self.jni_object(),"getType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::SandstoneType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the type of this sandstone
	fn set_type(&self,val_type: impl Into<crate::SandstoneType<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/SandstoneType;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setType",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::Sandstone<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Sandstone;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Sandstone::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Sandstone<'mc> {
            fn to_string(&self) -> String {
                match SandstoneTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Sandstone.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::MaterialData<'mc>> for Sandstone<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Sandstone into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for Sandstone<'mc> {}
#[repr(C)]
pub struct Rails<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Rails<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Rails<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Rails from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Rails")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Rails object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> RailsTrait<'mc> for Rails<'mc> {}
pub trait RailsTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::Rails<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Rails"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Rails::from_raw(&jni,res
)}

	fn is_on_slope(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isOnSlope",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn is_curve(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCurve",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn direction(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDirection",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Set the direction of these tracks
/// 
/// Note that tracks are bidirectional and that the direction returned is
/// the ascending direction if the track is set on a slope. If it is set as
/// a curve, the corner of the track should be supplied.
	fn set_direction(&self,face: impl Into<crate::block::BlockFace<'mc>>,is_on_slope: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;Z)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Bool(is_on_slope.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setDirection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn clone(&self) 
-> Result<crate::material::Rails<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Rails;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Rails::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Rails<'mc> {
            fn to_string(&self) -> String {
                match RailsTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Rails.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::MaterialData<'mc>> for Rails<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Rails into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for Rails<'mc> {}
#[repr(C)]
pub struct Tree<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Tree<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Tree<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Tree from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Tree")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Tree object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> TreeTrait<'mc> for Tree<'mc> {}
pub trait TreeTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Constructs a tree block of the given tree species, and facing the given
/// direction.
	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,species: std::option::Option<impl Into<crate::TreeSpecies<'mc>>>,dir: std::option::Option<impl Into<crate::block::BlockFace<'mc>>>) 
-> Result<crate::material::Tree<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = species {
sig += "Lorg/bukkit/TreeSpecies;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = dir {
sig += "Lorg/bukkit/block/BlockFace;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Tree"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Tree::from_raw(&jni,res
)}
/// Get direction of the log
	fn direction(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDirection",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Set direction of the log
	fn set_direction(&self,dir: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(dir.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setDirection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::Tree<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Tree;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Tree::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Tree<'mc> {
            fn to_string(&self) -> String {
                match TreeTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Tree.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::Wood<'mc>> for Tree<'mc>{

fn into(self) -> crate::material::Wood<'mc> {

crate::material::Wood::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Tree into crate::material::Wood")

   }
}
impl<'mc> crate::material::WoodTrait<'mc> for Tree<'mc> {}
#[repr(C)]
pub struct Pumpkin<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Pumpkin<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Pumpkin<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Pumpkin from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Pumpkin")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Pumpkin object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PumpkinTrait<'mc> for Pumpkin<'mc> {}
pub trait PumpkinTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::Pumpkin<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Pumpkin"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Pumpkin::from_raw(&jni,res
)}

	fn is_lit(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isLit",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_facing_direction(&self,face: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFacingDirection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn facing(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFacing",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::Pumpkin<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Pumpkin;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Pumpkin::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Pumpkin<'mc> {
            fn to_string(&self) -> String {
                match PumpkinTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Pumpkin.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::Directional<'mc>> for Pumpkin<'mc>{

fn into(self) -> crate::material::Directional<'mc> {

crate::material::Directional::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Pumpkin into crate::material::Directional")

   }
}
impl<'mc> crate::material::DirectionalTrait<'mc> for Pumpkin<'mc> {}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Pumpkin<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Pumpkin into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for Pumpkin<'mc> {}
#[repr(C)]
pub struct Redstone<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Redstone<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Redstone<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Redstone from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Redstone")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Redstone object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> RedstoneTrait<'mc> for Redstone<'mc> {}
pub trait RedstoneTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Gets the current state of this Material, indicating if it's powered or
/// unpowered
	fn is_powered(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isPowered",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct Tripwire<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Tripwire<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Tripwire<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Tripwire from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Tripwire")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Tripwire object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> TripwireTrait<'mc> for Tripwire<'mc> {}
pub trait TripwireTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::Tripwire<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Tripwire"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Tripwire::from_raw(&jni,res
)}
/// Test if tripwire is currently activated
	fn is_activated(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isActivated",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Set tripwire activated state
	fn set_activated(&self,act: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(act.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setActivated",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Test if object triggering this tripwire directly
	fn is_object_triggering(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isObjectTriggering",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Set object triggering state for this tripwire
	fn set_object_triggering(&self,trig: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(trig.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setObjectTriggering",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn clone(&self) 
-> Result<crate::material::Tripwire<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Tripwire;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Tripwire::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Tripwire<'mc> {
            fn to_string(&self) -> String {
                match TripwireTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Tripwire.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::MaterialData<'mc>> for Tripwire<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Tripwire into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for Tripwire<'mc> {}
#[repr(C)]
pub struct Lever<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Lever<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Lever<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Lever from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Lever")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Lever object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> LeverTrait<'mc> for Lever<'mc> {}
pub trait LeverTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::Lever<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Lever"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Lever::from_raw(&jni,res
)}
/// Gets the current state of this Material, indicating if it's powered or
/// unpowered
	fn is_powered(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isPowered",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Set this lever to be powered or not.
	fn set_powered(&self,is_powered: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(is_powered.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setPowered",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the face that this block is attached on
	fn attached_face(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAttachedFace",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the direction this lever is pointing in
	fn set_facing_direction(&self,face: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFacingDirection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::Lever<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Lever;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Lever::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Lever<'mc> {
            fn to_string(&self) -> String {
                match LeverTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Lever.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::Redstone<'mc>> for Lever<'mc>{

fn into(self) -> crate::material::Redstone<'mc> {

crate::material::Redstone::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Lever into crate::material::Redstone")

   }
}
impl<'mc> crate::material::RedstoneTrait<'mc> for Lever<'mc> {}
impl<'mc> Into<crate::material::SimpleAttachableMaterialData<'mc>> for Lever<'mc>{

fn into(self) -> crate::material::SimpleAttachableMaterialData<'mc> {

crate::material::SimpleAttachableMaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Lever into crate::material::SimpleAttachableMaterialData")

   }
}
impl<'mc> crate::material::SimpleAttachableMaterialDataTrait<'mc> for Lever<'mc> {}
#[repr(C)]
pub struct SpawnEgg<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for SpawnEgg<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for SpawnEgg<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate SpawnEgg from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/SpawnEgg")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a SpawnEgg object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> SpawnEggTrait<'mc> for SpawnEgg<'mc> {}
pub trait SpawnEggTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::SpawnEgg<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/SpawnEgg"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::SpawnEgg::from_raw(&jni,res
)}
#[deprecated]
/// Get the type of entity this egg will spawn.
	fn spawned_type(&self) 
-> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/EntityType;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSpawnedType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::EntityType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]
/// Set the type of entity this egg will spawn.
	fn set_spawned_type(&self,val_type: impl Into<crate::entity::EntityType<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/EntityType;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setSpawnedType",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::SpawnEgg<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/SpawnEgg;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::SpawnEgg::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for SpawnEgg<'mc> {
            fn to_string(&self) -> String {
                match SpawnEggTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling SpawnEgg.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::MaterialData<'mc>> for SpawnEgg<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting SpawnEgg into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for SpawnEgg<'mc> {}
#[repr(C)]
pub struct Stairs<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Stairs<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Stairs<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Stairs from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Stairs")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Stairs object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> StairsTrait<'mc> for Stairs<'mc> {}
pub trait StairsTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: impl Into<crate::Material<'mc>>,data: std::option::Option<i8>) 
-> Result<crate::material::Stairs<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Stairs"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Stairs::from_raw(&jni,res
)}

	fn ascending_direction(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAscendingDirection",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn descending_direction(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDescendingDirection",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Set the direction the stair part of the block is facing
	fn set_facing_direction(&self,face: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFacingDirection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn facing(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFacing",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Test if step is inverted
	fn is_inverted(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isInverted",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Set step inverted state
	fn set_inverted(&self,inv: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(inv.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setInverted",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::Stairs<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Stairs;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Stairs::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Stairs<'mc> {
            fn to_string(&self) -> String {
                match StairsTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Stairs.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::Directional<'mc>> for Stairs<'mc>{

fn into(self) -> crate::material::Directional<'mc> {

crate::material::Directional::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Stairs into crate::material::Directional")

   }
}
impl<'mc> crate::material::DirectionalTrait<'mc> for Stairs<'mc> {}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Stairs<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Stairs into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for Stairs<'mc> {}
#[repr(C)]
pub struct DetectorRail<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for DetectorRail<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for DetectorRail<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate DetectorRail from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/DetectorRail")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a DetectorRail object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> DetectorRailTrait<'mc> for DetectorRail<'mc> {}
pub trait DetectorRailTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::DetectorRail<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/DetectorRail"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::DetectorRail::from_raw(&jni,res
)}

	fn is_pressed(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isPressed",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_pressed(&self,is_pressed: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(is_pressed.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setPressed",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn clone(&self) 
-> Result<crate::material::DetectorRail<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/DetectorRail;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::DetectorRail::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::material::PressureSensor<'mc>> for DetectorRail<'mc>{

fn into(self) -> crate::material::PressureSensor<'mc> {

crate::material::PressureSensor::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting DetectorRail into crate::material::PressureSensor")

   }
}
impl<'mc> crate::material::PressureSensorTrait<'mc> for DetectorRail<'mc> {}
impl<'mc> Into<crate::material::ExtendedRails<'mc>> for DetectorRail<'mc>{

fn into(self) -> crate::material::ExtendedRails<'mc> {

crate::material::ExtendedRails::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting DetectorRail into crate::material::ExtendedRails")

   }
}
impl<'mc> crate::material::ExtendedRailsTrait<'mc> for DetectorRail<'mc> {}
#[repr(C)]
pub struct Sign<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Sign<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Sign<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Sign from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Sign")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Sign object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> SignTrait<'mc> for Sign<'mc> {}
pub trait SignTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::Sign<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Sign"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Sign::from_raw(&jni,res
)}
/// Check if this sign is attached to a wall
	fn is_wall_sign(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isWallSign",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets the face that this block is attached on
	fn attached_face(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAttachedFace",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the direction that this sign is currently facing
	fn facing(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFacing",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn set_facing_direction(&self,face: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFacingDirection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::Sign<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Sign;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Sign::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Sign<'mc> {
            fn to_string(&self) -> String {
                match SignTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Sign.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::Attachable<'mc>> for Sign<'mc>{

fn into(self) -> crate::material::Attachable<'mc> {

crate::material::Attachable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Sign into crate::material::Attachable")

   }
}
impl<'mc> crate::material::AttachableTrait<'mc> for Sign<'mc> {}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Sign<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Sign into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for Sign<'mc> {}
#[repr(C)]
pub struct Cake<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Cake<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Cake<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Cake from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Cake")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Cake object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> CakeTrait<'mc> for Cake<'mc> {}
pub trait CakeTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::Cake<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Cake"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Cake::from_raw(&jni,res
)}
/// Gets the number of slices eaten from this cake
	fn slices_eaten(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getSlicesEaten",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Gets the number of slices remaining on this cake
	fn slices_remaining(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getSlicesRemaining",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Sets the number of slices eaten from this cake
	fn set_slices_eaten(&self,n: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(n);
let res = self.jni_ref().call_method(&self.jni_object(),"setSlicesEaten",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Sets the number of slices remaining on this cake
	fn set_slices_remaining(&self,n: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(n);
let res = self.jni_ref().call_method(&self.jni_object(),"setSlicesRemaining",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::Cake<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Cake;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Cake::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Cake<'mc> {
            fn to_string(&self) -> String {
                match CakeTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Cake.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::MaterialData<'mc>> for Cake<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Cake into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for Cake<'mc> {}
#[repr(C)]
pub struct CocoaPlant<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for CocoaPlant<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for CocoaPlant<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate CocoaPlant from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/CocoaPlant")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a CocoaPlant object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> CocoaPlantTrait<'mc> for CocoaPlant<'mc> {}
pub trait CocoaPlantTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,sz: std::option::Option<impl Into<crate::material::CocoaPlantCocoaPlantSize<'mc>>>,dir: std::option::Option<impl Into<crate::block::BlockFace<'mc>>>) 
-> Result<crate::material::CocoaPlant<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = sz {
sig += "Lorg/bukkit/material/CocoaPlant/CocoaPlantSize;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = dir {
sig += "Lorg/bukkit/block/BlockFace;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/CocoaPlant"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::CocoaPlant::from_raw(&jni,res
)}
/// Get size of plant
	fn size(&self) 
-> Result<crate::material::CocoaPlantCocoaPlantSize<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/CocoaPlant/CocoaPlantSize;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSize",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::CocoaPlantCocoaPlantSize::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Set size of plant
	fn set_size(&self,sz: impl Into<crate::material::CocoaPlantCocoaPlantSize<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/material/CocoaPlant/CocoaPlantSize;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(sz.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setSize",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn attached_face(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAttachedFace",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn set_facing_direction(&self,face: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFacingDirection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn facing(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFacing",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn clone(&self) 
-> Result<crate::material::CocoaPlant<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/CocoaPlant;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::CocoaPlant::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for CocoaPlant<'mc> {
            fn to_string(&self) -> String {
                match CocoaPlantTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling CocoaPlant.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::Directional<'mc>> for CocoaPlant<'mc>{

fn into(self) -> crate::material::Directional<'mc> {

crate::material::Directional::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting CocoaPlant into crate::material::Directional")

   }
}
impl<'mc> crate::material::DirectionalTrait<'mc> for CocoaPlant<'mc> {}
impl<'mc> Into<crate::material::Attachable<'mc>> for CocoaPlant<'mc>{

fn into(self) -> crate::material::Attachable<'mc> {

crate::material::Attachable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting CocoaPlant into crate::material::Attachable")

   }
}
impl<'mc> crate::material::AttachableTrait<'mc> for CocoaPlant<'mc> {}
impl<'mc> Into<crate::material::MaterialData<'mc>> for CocoaPlant<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting CocoaPlant into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for CocoaPlant<'mc> {}
#[repr(C)]
pub struct Ladder<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Ladder<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Ladder<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Ladder from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Ladder")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Ladder object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> LadderTrait<'mc> for Ladder<'mc> {}
pub trait LadderTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::Ladder<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Ladder"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Ladder::from_raw(&jni,res
)}
/// Gets the face that this block is attached on
	fn attached_face(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAttachedFace",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the direction this ladder is facing
	fn set_facing_direction(&self,face: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFacingDirection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn clone(&self) 
-> Result<crate::material::Ladder<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Ladder;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Ladder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::material::SimpleAttachableMaterialData<'mc>> for Ladder<'mc>{

fn into(self) -> crate::material::SimpleAttachableMaterialData<'mc> {

crate::material::SimpleAttachableMaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Ladder into crate::material::SimpleAttachableMaterialData")

   }
}
impl<'mc> crate::material::SimpleAttachableMaterialDataTrait<'mc> for Ladder<'mc> {}
#[repr(C)]
pub struct PoweredRail<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PoweredRail<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PoweredRail<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PoweredRail from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/PoweredRail")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PoweredRail object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PoweredRailTrait<'mc> for PoweredRail<'mc> {}
pub trait PoweredRailTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::PoweredRail<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/PoweredRail"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::PoweredRail::from_raw(&jni,res
)}

	fn is_powered(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isPowered",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Set whether this PoweredRail should be powered or not.
	fn set_powered(&self,is_powered: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(is_powered.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setPowered",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn clone(&self) 
-> Result<crate::material::PoweredRail<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/PoweredRail;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::PoweredRail::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::material::Redstone<'mc>> for PoweredRail<'mc>{

fn into(self) -> crate::material::Redstone<'mc> {

crate::material::Redstone::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PoweredRail into crate::material::Redstone")

   }
}
impl<'mc> crate::material::RedstoneTrait<'mc> for PoweredRail<'mc> {}
impl<'mc> Into<crate::material::ExtendedRails<'mc>> for PoweredRail<'mc>{

fn into(self) -> crate::material::ExtendedRails<'mc> {

crate::material::ExtendedRails::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PoweredRail into crate::material::ExtendedRails")

   }
}
impl<'mc> crate::material::ExtendedRailsTrait<'mc> for PoweredRail<'mc> {}
#[repr(C)]
pub struct Openable<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Openable<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Openable<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Openable from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Openable")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Openable object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> OpenableTrait<'mc> for Openable<'mc> {}
pub trait OpenableTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Check to see if the door is open.
	fn is_open(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isOpen",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Configure this door to be either open or closed;
	fn set_open(&self,is_open: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(is_open.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setOpen",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct Directional<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Directional<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Directional<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Directional from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Directional")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Directional object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> DirectionalTrait<'mc> for Directional<'mc> {}
pub trait DirectionalTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Sets the direction that this block is facing in
	fn set_facing_direction(&self,face: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFacingDirection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the direction this block is facing
	fn facing(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFacing",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct Dye<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Dye<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Dye<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Dye from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Dye")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Dye object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> DyeTrait<'mc> for Dye<'mc> {}
pub trait DyeTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::Dye<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Dye"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Dye::from_raw(&jni,res
)}
/// Gets the current color of this dye
	fn color(&self) 
-> Result<crate::DyeColor<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/DyeColor;");
let res = self.jni_ref().call_method(&self.jni_object(),"getColor",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::DyeColor::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the color of this dye
	fn set_color(&self,color: impl Into<crate::DyeColor<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/DyeColor;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(color.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setColor",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::Dye<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Dye;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Dye::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Dye<'mc> {
            fn to_string(&self) -> String {
                match DyeTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Dye.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::Colorable<'mc>> for Dye<'mc>{

fn into(self) -> crate::material::Colorable<'mc> {

crate::material::Colorable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Dye into crate::material::Colorable")

   }
}
impl<'mc> crate::material::ColorableTrait<'mc> for Dye<'mc> {}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Dye<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Dye into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for Dye<'mc> {}
pub enum CocoaPlantCocoaPlantSize<'mc> {
	Small {inner: CocoaPlantCocoaPlantSizeStruct<'mc>},
	Medium {inner: CocoaPlantCocoaPlantSizeStruct<'mc>},
	Large {inner: CocoaPlantCocoaPlantSizeStruct<'mc>},
}
impl<'mc> std::fmt::Display for CocoaPlantCocoaPlantSize<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           CocoaPlantCocoaPlantSize::Small { .. } => f.write_str("SMALL"),
           CocoaPlantCocoaPlantSize::Medium { .. } => f.write_str("MEDIUM"),
           CocoaPlantCocoaPlantSize::Large { .. } => f.write_str("LARGE"),
       }
   }
}

        impl<'mc> CocoaPlantCocoaPlantSizeTrait<'mc> for CocoaPlantCocoaPlantSize<'mc> {}
        
        pub trait CocoaPlantCocoaPlantSizeTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<CocoaPlantCocoaPlantSize<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/material/CocoaPlant/CocoaPlantSize");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/material/CocoaPlant/CocoaPlantSize;",
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
                    
"SMALL" => Ok(CocoaPlantCocoaPlantSize::Small { inner: CocoaPlantCocoaPlantSizeStruct::from_raw(env,obj)?}),
"MEDIUM" => Ok(CocoaPlantCocoaPlantSize::Medium { inner: CocoaPlantCocoaPlantSizeStruct::from_raw(env,obj)?}),
"LARGE" => Ok(CocoaPlantCocoaPlantSize::Large { inner: CocoaPlantCocoaPlantSizeStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct CocoaPlantCocoaPlantSizeStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for CocoaPlantCocoaPlantSize<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::Small { inner } => inner.0.clone(),
Self::Medium { inner } => inner.0.clone(),
Self::Large { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::Small { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Medium { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Large { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for CocoaPlantCocoaPlantSize<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate CocoaPlantCocoaPlantSize from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/CocoaPlant/CocoaPlantSize")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a CocoaPlantCocoaPlantSize object, got {}",
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
                    "SMALL" => Ok(CocoaPlantCocoaPlantSize::Small { inner: CocoaPlantCocoaPlantSizeStruct::from_raw(env,obj)?}),"MEDIUM" => Ok(CocoaPlantCocoaPlantSize::Medium { inner: CocoaPlantCocoaPlantSizeStruct::from_raw(env,obj)?}),"LARGE" => Ok(CocoaPlantCocoaPlantSize::Large { inner: CocoaPlantCocoaPlantSizeStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for CocoaPlantCocoaPlantSizeStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for CocoaPlantCocoaPlantSizeStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate CocoaPlantCocoaPlantSizeStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/CocoaPlant/CocoaPlantSize")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a CocoaPlantCocoaPlantSizeStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> CocoaPlantCocoaPlantSizeStruct<'mc> {

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::material::CocoaPlantCocoaPlantSize<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/CocoaPlant/CocoaPlantSize;");
let cls = jni.find_class("org/bukkit/material/CocoaPlant/CocoaPlantSize"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::material::CocoaPlantCocoaPlantSize::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct PressureSensor<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PressureSensor<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PressureSensor<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PressureSensor from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/PressureSensor")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PressureSensor object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PressureSensorTrait<'mc> for PressureSensor<'mc> {}
pub trait PressureSensorTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn is_pressed(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isPressed",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct NetherWarts<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for NetherWarts<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for NetherWarts<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate NetherWarts from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/NetherWarts")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a NetherWarts object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> NetherWartsTrait<'mc> for NetherWarts<'mc> {}
pub trait NetherWartsTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::NetherWarts<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/NetherWarts"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::NetherWarts::from_raw(&jni,res
)}
/// Gets the current growth state of this nether wart
	fn state(&self) 
-> Result<crate::NetherWartsState<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/NetherWartsState;");
let res = self.jni_ref().call_method(&self.jni_object(),"getState",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::NetherWartsState::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the growth state of this nether wart
	fn set_state(&self,state: impl Into<crate::NetherWartsState<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/NetherWartsState;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(state.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setState",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::NetherWarts<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/NetherWarts;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::NetherWarts::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for NetherWarts<'mc> {
            fn to_string(&self) -> String {
                match NetherWartsTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling NetherWarts.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::MaterialData<'mc>> for NetherWarts<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting NetherWarts into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for NetherWarts<'mc> {}
#[repr(C)]
pub struct TexturedMaterial<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for TexturedMaterial<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for TexturedMaterial<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate TexturedMaterial from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/TexturedMaterial")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a TexturedMaterial object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> TexturedMaterialTrait<'mc> for TexturedMaterial<'mc> {}
pub trait TexturedMaterialTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: impl Into<crate::Material<'mc>>,data: std::option::Option<i8>) 
-> Result<crate::material::TexturedMaterial<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/TexturedMaterial"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::TexturedMaterial::from_raw(&jni,res
)}
/// Gets the current Material this block is made of
	fn material(&self) 
-> Result<crate::Material<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Material;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMaterial",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Material::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the material this block is made of
	fn set_material(&self,material: impl Into<crate::Material<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Material;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(material.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setMaterial",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::TexturedMaterial<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/TexturedMaterial;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::TexturedMaterial::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for TexturedMaterial<'mc> {
            fn to_string(&self) -> String {
                match TexturedMaterialTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling TexturedMaterial.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::MaterialData<'mc>> for TexturedMaterial<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting TexturedMaterial into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for TexturedMaterial<'mc> {}
#[repr(C)]
pub struct Bed<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Bed<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Bed<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Bed from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Bed")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Bed object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BedTrait<'mc> for Bed<'mc> {}
pub trait BedTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::Bed<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Bed"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Bed::from_raw(&jni,res
)}
/// Determine if this block represents the head of the bed
	fn is_head_of_bed(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isHeadOfBed",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Configure this to be either the head or the foot of the bed
	fn set_head_of_bed(&self,is_head_of_bed: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(is_head_of_bed.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setHeadOfBed",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Set which direction the head of the bed is facing. Note that this will
/// only affect one of the two blocks the bed is made of.
	fn set_facing_direction(&self,face: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFacingDirection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the direction that this bed's head is facing toward
	fn facing(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFacing",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::Bed<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Bed;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Bed::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Bed<'mc> {
            fn to_string(&self) -> String {
                match BedTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Bed.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::Directional<'mc>> for Bed<'mc>{

fn into(self) -> crate::material::Directional<'mc> {

crate::material::Directional::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Bed into crate::material::Directional")

   }
}
impl<'mc> crate::material::DirectionalTrait<'mc> for Bed<'mc> {}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Bed<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Bed into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for Bed<'mc> {}
#[repr(C)]
pub struct FurnaceAndDispenser<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for FurnaceAndDispenser<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for FurnaceAndDispenser<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate FurnaceAndDispenser from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/FurnaceAndDispenser")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a FurnaceAndDispenser object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> FurnaceAndDispenserTrait<'mc> for FurnaceAndDispenser<'mc> {}
pub trait FurnaceAndDispenserTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: impl Into<crate::Material<'mc>>,data: std::option::Option<i8>) 
-> Result<crate::material::FurnaceAndDispenser<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/FurnaceAndDispenser"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::FurnaceAndDispenser::from_raw(&jni,res
)}

	fn clone(&self) 
-> Result<crate::material::FurnaceAndDispenser<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/FurnaceAndDispenser;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::FurnaceAndDispenser::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::material::DirectionalContainer<'mc>> for FurnaceAndDispenser<'mc>{

fn into(self) -> crate::material::DirectionalContainer<'mc> {

crate::material::DirectionalContainer::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting FurnaceAndDispenser into crate::material::DirectionalContainer")

   }
}
impl<'mc> crate::material::DirectionalContainerTrait<'mc> for FurnaceAndDispenser<'mc> {}
#[repr(C)]
pub struct EnderChest<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for EnderChest<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for EnderChest<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate EnderChest from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/EnderChest")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EnderChest object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> EnderChestTrait<'mc> for EnderChest<'mc> {}
pub trait EnderChestTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::EnderChest<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/EnderChest"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::EnderChest::from_raw(&jni,res
)}

	fn clone(&self) 
-> Result<crate::material::EnderChest<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/EnderChest;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::EnderChest::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::material::DirectionalContainer<'mc>> for EnderChest<'mc>{

fn into(self) -> crate::material::DirectionalContainer<'mc> {

crate::material::DirectionalContainer::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting EnderChest into crate::material::DirectionalContainer")

   }
}
impl<'mc> crate::material::DirectionalContainerTrait<'mc> for EnderChest<'mc> {}
#[repr(C)]
pub struct DirectionalContainer<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for DirectionalContainer<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for DirectionalContainer<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate DirectionalContainer from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/DirectionalContainer")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a DirectionalContainer object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> DirectionalContainerTrait<'mc> for DirectionalContainer<'mc> {}
pub trait DirectionalContainerTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: impl Into<crate::Material<'mc>>,data: std::option::Option<i8>) 
-> Result<crate::material::DirectionalContainer<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/DirectionalContainer"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::DirectionalContainer::from_raw(&jni,res
)}

	fn set_facing_direction(&self,face: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFacingDirection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn facing(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFacing",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::DirectionalContainer<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/DirectionalContainer;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::DirectionalContainer::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for DirectionalContainer<'mc> {
            fn to_string(&self) -> String {
                match DirectionalContainerTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling DirectionalContainer.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::Directional<'mc>> for DirectionalContainer<'mc>{

fn into(self) -> crate::material::Directional<'mc> {

crate::material::Directional::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting DirectionalContainer into crate::material::Directional")

   }
}
impl<'mc> crate::material::DirectionalTrait<'mc> for DirectionalContainer<'mc> {}
impl<'mc> Into<crate::material::MaterialData<'mc>> for DirectionalContainer<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting DirectionalContainer into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for DirectionalContainer<'mc> {}
#[repr(C)]
pub struct RedstoneTorch<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for RedstoneTorch<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for RedstoneTorch<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate RedstoneTorch from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/RedstoneTorch")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a RedstoneTorch object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> RedstoneTorchTrait<'mc> for RedstoneTorch<'mc> {}
pub trait RedstoneTorchTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::RedstoneTorch<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/RedstoneTorch"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::RedstoneTorch::from_raw(&jni,res
)}
/// Gets the current state of this Material, indicating if it's powered or
/// unpowered
	fn is_powered(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isPowered",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::RedstoneTorch<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/RedstoneTorch;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::RedstoneTorch::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for RedstoneTorch<'mc> {
            fn to_string(&self) -> String {
                match RedstoneTorchTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling RedstoneTorch.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::Redstone<'mc>> for RedstoneTorch<'mc>{

fn into(self) -> crate::material::Redstone<'mc> {

crate::material::Redstone::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting RedstoneTorch into crate::material::Redstone")

   }
}
impl<'mc> crate::material::RedstoneTrait<'mc> for RedstoneTorch<'mc> {}
impl<'mc> Into<crate::material::Torch<'mc>> for RedstoneTorch<'mc>{

fn into(self) -> crate::material::Torch<'mc> {

crate::material::Torch::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting RedstoneTorch into crate::material::Torch")

   }
}
impl<'mc> crate::material::TorchTrait<'mc> for RedstoneTorch<'mc> {}
#[repr(C)]
pub struct Vine<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Vine<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Vine<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Vine from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Vine")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Vine object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> VineTrait<'mc> for Vine<'mc> {}
pub trait VineTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::Vine<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Vine"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Vine::from_raw(&jni,res
)}
/// Check if the vine is attached to the specified face of an adjacent
/// block. You can check two faces at once by passing e.g. {@link
/// BlockFace#NORTH_EAST}.
	fn is_on_face(&self,face: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"isOnFace",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Attach the vine to the specified face of an adjacent block.
	fn put_on_face(&self,face: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"putOnFace",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Detach the vine from the specified face of an adjacent block.
	fn remove_from_face(&self,face: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"removeFromFace",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::Vine<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Vine;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Vine::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Vine<'mc> {
            fn to_string(&self) -> String {
                match VineTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Vine.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::MaterialData<'mc>> for Vine<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Vine into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for Vine<'mc> {}
#[repr(C)]
pub struct Command<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Command<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Command<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Command from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Command")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Command object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> CommandTrait<'mc> for Command<'mc> {}
pub trait CommandTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::Command<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Command"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Command::from_raw(&jni,res
)}
/// Gets the current state of this Material, indicating if it's powered or
/// unpowered
	fn is_powered(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isPowered",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Sets the current state of this Material
	fn set_powered(&self,bool: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(bool.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setPowered",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::Command<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Command;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Command::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Command<'mc> {
            fn to_string(&self) -> String {
                match CommandTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Command.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::Redstone<'mc>> for Command<'mc>{

fn into(self) -> crate::material::Redstone<'mc> {

crate::material::Redstone::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Command into crate::material::Redstone")

   }
}
impl<'mc> crate::material::RedstoneTrait<'mc> for Command<'mc> {}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Command<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Command into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for Command<'mc> {}
#[repr(C)]
pub struct MaterialData<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for MaterialData<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for MaterialData<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate MaterialData from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/MaterialData")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a MaterialData object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> MaterialDataTrait<'mc> for MaterialData<'mc> {}
pub trait MaterialDataTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: impl Into<crate::Material<'mc>>,data: std::option::Option<i8>) 
-> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/MaterialData"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::MaterialData::from_raw(&jni,res
)}
#[deprecated]
/// Gets the raw data in this material
	fn data(&self) 
-> Result<i8, Box<dyn std::error::Error>>

{let sig = String::from("()B");
let res = self.jni_ref().call_method(&self.jni_object(),"getData",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.b()?
)}
#[deprecated]
/// Sets the raw data of this material
	fn set_data(&self,data: i8) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(B)V");
let val_1 = jni::objects::JValueGen::Byte(data);
let res = self.jni_ref().call_method(&self.jni_object(),"setData",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the Material that this MaterialData represents
	fn item_type(&self) 
-> Result<crate::Material<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Material;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItemType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Material::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Creates a new ItemStack based on this MaterialData
	fn to_item_stack(&self,amount: std::option::Option<i32>) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = amount {
sig += "I";
let val_1 = jni::objects::JValueGen::Int(a);
args.push(val_1);
}
sig += ")Lorg/bukkit/inventory/ItemStack;";
let res = self.jni_ref().call_method(&self.jni_object(),"toItemStack",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn hash_code(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"hashCode",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}

	fn equals(&self,obj: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)Z");
let val_1 = jni::objects::JValueGen::Object(obj);
let res = self.jni_ref().call_method(&self.jni_object(),"equals",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn clone(&self) 
-> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/MaterialData;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::MaterialData::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for MaterialData<'mc> {
            fn to_string(&self) -> String {
                match MaterialDataTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling MaterialData.toString: {}",
                        err
                    ),
                }
            }
        }
        
#[repr(C)]
pub struct PistonExtensionMaterial<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PistonExtensionMaterial<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PistonExtensionMaterial<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PistonExtensionMaterial from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/PistonExtensionMaterial")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PistonExtensionMaterial object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PistonExtensionMaterialTrait<'mc> for PistonExtensionMaterial<'mc> {}
pub trait PistonExtensionMaterialTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: impl Into<crate::Material<'mc>>,data: std::option::Option<i8>) 
-> Result<crate::material::PistonExtensionMaterial<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/PistonExtensionMaterial"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::PistonExtensionMaterial::from_raw(&jni,res
)}

	fn set_facing_direction(&self,face: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFacingDirection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn facing(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFacing",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Checks if this piston extension is sticky, and returns true if so
	fn is_sticky(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isSticky",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Sets whether or not this extension is sticky
	fn set_sticky(&self,sticky: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(sticky.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setSticky",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn attached_face(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAttachedFace",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn clone(&self) 
-> Result<crate::material::PistonExtensionMaterial<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/PistonExtensionMaterial;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::PistonExtensionMaterial::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::material::Attachable<'mc>> for PistonExtensionMaterial<'mc>{

fn into(self) -> crate::material::Attachable<'mc> {

crate::material::Attachable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PistonExtensionMaterial into crate::material::Attachable")

   }
}
impl<'mc> crate::material::AttachableTrait<'mc> for PistonExtensionMaterial<'mc> {}
impl<'mc> Into<crate::material::MaterialData<'mc>> for PistonExtensionMaterial<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PistonExtensionMaterial into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for PistonExtensionMaterial<'mc> {}
#[repr(C)]
pub struct Mushroom<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Mushroom<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Mushroom<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Mushroom from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Mushroom")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Mushroom object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> MushroomTrait<'mc> for Mushroom<'mc> {}
pub trait MushroomTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,shroom: impl Into<crate::Material<'mc>>,data: std::option::Option<i8>) 
-> Result<crate::material::Mushroom<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(shroom.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Mushroom"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Mushroom::from_raw(&jni,res
)}

	fn is_stem(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isStem",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
#[deprecated]
/// Sets this to be a mushroom stem.
	fn set_stem(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"setStem",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the mushroom texture of this block.
	fn block_texture(&self) 
-> Result<crate::material::types::MushroomBlockTexture<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/types/MushroomBlockTexture;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBlockTexture",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::types::MushroomBlockTexture::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the mushroom texture of this block.
	fn set_block_texture(&self,texture: impl Into<crate::material::types::MushroomBlockTexture<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/material/types/MushroomBlockTexture;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(texture.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setBlockTexture",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Checks whether a face of the block is painted with cap texture.
	fn is_face_painted(&self,face: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"isFacePainted",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
#[deprecated]
/// Set a face of the block to be painted or not. Note that due to the nature of how the data is stored, setting a face painted or not is not guaranteed to leave the other faces unchanged.
	fn set_face_painted(&self,face: impl Into<crate::block::BlockFace<'mc>>,painted: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;Z)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Bool(painted.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setFacePainted",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn painted_faces(&self) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Set;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPaintedFaces",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::Mushroom<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Mushroom;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Mushroom::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Mushroom<'mc> {
            fn to_string(&self) -> String {
                match MushroomTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Mushroom.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::MaterialData<'mc>> for Mushroom<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Mushroom into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for Mushroom<'mc> {}
#[repr(C)]
pub struct SmoothBrick<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for SmoothBrick<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for SmoothBrick<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate SmoothBrick from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/SmoothBrick")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a SmoothBrick object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> SmoothBrickTrait<'mc> for SmoothBrick<'mc> {}
pub trait SmoothBrickTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::SmoothBrick<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/SmoothBrick"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::SmoothBrick::from_raw(&jni,res
)}

	fn textures(&self) 
-> Result<Vec<crate::Material<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getTextures",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::Material::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}

	fn clone(&self) 
-> Result<crate::material::SmoothBrick<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/SmoothBrick;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::SmoothBrick::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::material::TexturedMaterial<'mc>> for SmoothBrick<'mc>{

fn into(self) -> crate::material::TexturedMaterial<'mc> {

crate::material::TexturedMaterial::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting SmoothBrick into crate::material::TexturedMaterial")

   }
}
impl<'mc> crate::material::TexturedMaterialTrait<'mc> for SmoothBrick<'mc> {}
#[repr(C)]
pub struct WoodenStep<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for WoodenStep<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for WoodenStep<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate WoodenStep from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/WoodenStep")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a WoodenStep object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> WoodenStepTrait<'mc> for WoodenStep<'mc> {}
pub trait WoodenStepTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::WoodenStep<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/WoodenStep"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::WoodenStep::from_raw(&jni,res
)}
/// Test if step is inverted
	fn is_inverted(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isInverted",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Set step inverted state
	fn set_inverted(&self,inv: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(inv.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setInverted",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn clone(&self) 
-> Result<crate::material::WoodenStep<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/WoodenStep;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::WoodenStep::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for WoodenStep<'mc> {
            fn to_string(&self) -> String {
                match WoodenStepTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling WoodenStep.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::Wood<'mc>> for WoodenStep<'mc>{

fn into(self) -> crate::material::Wood<'mc> {

crate::material::Wood::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting WoodenStep into crate::material::Wood")

   }
}
impl<'mc> crate::material::WoodTrait<'mc> for WoodenStep<'mc> {}
#[repr(C)]
pub struct Coal<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Coal<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Coal<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Coal from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Coal")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Coal object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> CoalTrait<'mc> for Coal<'mc> {}
pub trait CoalTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::Coal<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Coal"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Coal::from_raw(&jni,res
)}
/// Gets the current type of this coal
	fn get_type(&self) 
-> Result<crate::CoalType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/CoalType;");
let res = self.jni_ref().call_method(&self.jni_object(),"getType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::CoalType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the type of this coal
	fn set_type(&self,val_type: impl Into<crate::CoalType<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/CoalType;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setType",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::Coal<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Coal;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Coal::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Coal<'mc> {
            fn to_string(&self) -> String {
                match CoalTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Coal.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::MaterialData<'mc>> for Coal<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Coal into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for Coal<'mc> {}
#[repr(C)]
pub struct Cauldron<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Cauldron<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Cauldron<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Cauldron from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Cauldron")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Cauldron object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> CauldronTrait<'mc> for Cauldron<'mc> {}
pub trait CauldronTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::Cauldron<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Cauldron"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Cauldron::from_raw(&jni,res
)}
/// Check if the cauldron is full.
	fn is_full(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isFull",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Check if the cauldron is empty.
	fn is_empty(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::Cauldron<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Cauldron;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Cauldron::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Cauldron<'mc> {
            fn to_string(&self) -> String {
                match CauldronTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Cauldron.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::MaterialData<'mc>> for Cauldron<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Cauldron into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for Cauldron<'mc> {}
#[repr(C)]
pub struct Furnace<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Furnace<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Furnace<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Furnace from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Furnace")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Furnace object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> FurnaceTrait<'mc> for Furnace<'mc> {}
pub trait FurnaceTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::Furnace<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Furnace"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Furnace::from_raw(&jni,res
)}

	fn clone(&self) 
-> Result<crate::material::Furnace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Furnace;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Furnace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::material::FurnaceAndDispenser<'mc>> for Furnace<'mc>{

fn into(self) -> crate::material::FurnaceAndDispenser<'mc> {

crate::material::FurnaceAndDispenser::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Furnace into crate::material::FurnaceAndDispenser")

   }
}
impl<'mc> crate::material::FurnaceAndDispenserTrait<'mc> for Furnace<'mc> {}
#[repr(C)]
pub struct Hopper<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Hopper<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Hopper<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Hopper from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Hopper")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Hopper object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> HopperTrait<'mc> for Hopper<'mc> {}
pub trait HopperTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Constructs a hopper facing the specified direction and either active or
/// not.
	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,facing_direction: std::option::Option<impl Into<crate::block::BlockFace<'mc>>>,is_active: std::option::Option<bool>) 
-> Result<crate::material::Hopper<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = facing_direction {
sig += "Lorg/bukkit/block/BlockFace;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = is_active {
sig += "Z";
let val_2 = jni::objects::JValueGen::Bool(a.into());
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Hopper"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Hopper::from_raw(&jni,res
)}
/// Sets whether the hopper is active or not.
	fn set_active(&self,is_active: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(is_active.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setActive",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Checks whether the hopper is active or not.
	fn is_active(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isActive",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Sets the direction this hopper is facing
	fn set_facing_direction(&self,face: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFacingDirection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the direction this hopper is facing
	fn facing(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFacing",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::Hopper<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Hopper;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Hopper::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Checks if the hopper is powered.
	fn is_powered(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isPowered",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Hopper<'mc> {
            fn to_string(&self) -> String {
                match HopperTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Hopper.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::Directional<'mc>> for Hopper<'mc>{

fn into(self) -> crate::material::Directional<'mc> {

crate::material::Directional::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Hopper into crate::material::Directional")

   }
}
impl<'mc> crate::material::DirectionalTrait<'mc> for Hopper<'mc> {}
impl<'mc> Into<crate::material::Redstone<'mc>> for Hopper<'mc>{

fn into(self) -> crate::material::Redstone<'mc> {

crate::material::Redstone::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Hopper into crate::material::Redstone")

   }
}
impl<'mc> crate::material::RedstoneTrait<'mc> for Hopper<'mc> {}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Hopper<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Hopper into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for Hopper<'mc> {}
#[repr(C)]
pub struct Observer<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Observer<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Observer<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Observer from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Observer")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Observer object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ObserverTrait<'mc> for Observer<'mc> {}
pub trait ObserverTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::Observer<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Observer"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Observer::from_raw(&jni,res
)}

	fn is_powered(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isPowered",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_facing_direction(&self,face: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFacingDirection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn facing(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFacing",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::Observer<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Observer;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Observer::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Observer<'mc> {
            fn to_string(&self) -> String {
                match ObserverTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Observer.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::Directional<'mc>> for Observer<'mc>{

fn into(self) -> crate::material::Directional<'mc> {

crate::material::Directional::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Observer into crate::material::Directional")

   }
}
impl<'mc> crate::material::DirectionalTrait<'mc> for Observer<'mc> {}
impl<'mc> Into<crate::material::Redstone<'mc>> for Observer<'mc>{

fn into(self) -> crate::material::Redstone<'mc> {

crate::material::Redstone::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Observer into crate::material::Redstone")

   }
}
impl<'mc> crate::material::RedstoneTrait<'mc> for Observer<'mc> {}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Observer<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Observer into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for Observer<'mc> {}
#[repr(C)]
pub struct Wool<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Wool<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Wool<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Wool from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Wool")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Wool object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> WoolTrait<'mc> for Wool<'mc> {}
pub trait WoolTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::Wool<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Wool"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Wool::from_raw(&jni,res
)}
/// Gets the current color of this dye
	fn color(&self) 
-> Result<crate::DyeColor<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/DyeColor;");
let res = self.jni_ref().call_method(&self.jni_object(),"getColor",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::DyeColor::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the color of this dye
	fn set_color(&self,color: impl Into<crate::DyeColor<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/DyeColor;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(color.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setColor",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::Wool<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Wool;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Wool::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Wool<'mc> {
            fn to_string(&self) -> String {
                match WoolTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Wool.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::Colorable<'mc>> for Wool<'mc>{

fn into(self) -> crate::material::Colorable<'mc> {

crate::material::Colorable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Wool into crate::material::Colorable")

   }
}
impl<'mc> crate::material::ColorableTrait<'mc> for Wool<'mc> {}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Wool<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Wool into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for Wool<'mc> {}
#[repr(C)]
pub struct Sapling<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Sapling<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Sapling<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Sapling from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Sapling")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Sapling object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> SaplingTrait<'mc> for Sapling<'mc> {}
pub trait SaplingTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Constructs a sapling of the given tree species and if is it instant
/// growable
	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,species: std::option::Option<impl Into<crate::TreeSpecies<'mc>>>,is_instant_growable: std::option::Option<bool>) 
-> Result<crate::material::Sapling<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = species {
sig += "Lorg/bukkit/TreeSpecies;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = is_instant_growable {
sig += "Z";
let val_2 = jni::objects::JValueGen::Bool(a.into());
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Sapling"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Sapling::from_raw(&jni,res
)}
/// Checks if the Sapling would grow when next ticked with bonemeal
	fn is_instant_growable(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isInstantGrowable",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Set whether this sapling will grow when next ticked with bonemeal
	fn set_is_instant_growable(&self,is_instant_growable: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(is_instant_growable.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setIsInstantGrowable",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::Sapling<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Sapling;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Sapling::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Sapling<'mc> {
            fn to_string(&self) -> String {
                match SaplingTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Sapling.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::Wood<'mc>> for Sapling<'mc>{

fn into(self) -> crate::material::Wood<'mc> {

crate::material::Wood::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Sapling into crate::material::Wood")

   }
}
impl<'mc> crate::material::WoodTrait<'mc> for Sapling<'mc> {}
#[repr(C)]
pub struct Button<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Button<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Button<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Button from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Button")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Button object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ButtonTrait<'mc> for Button<'mc> {}
pub trait ButtonTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::Button<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Button"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Button::from_raw(&jni,res
)}
/// Gets the current state of this Material, indicating if it's powered or
/// unpowered
	fn is_powered(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isPowered",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Sets the current state of this button
	fn set_powered(&self,bool: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(bool.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setPowered",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the face that this block is attached on
	fn attached_face(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAttachedFace",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the direction this button is pointing toward
	fn set_facing_direction(&self,face: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFacingDirection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::Button<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Button;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Button::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Button<'mc> {
            fn to_string(&self) -> String {
                match ButtonTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Button.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::Redstone<'mc>> for Button<'mc>{

fn into(self) -> crate::material::Redstone<'mc> {

crate::material::Redstone::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Button into crate::material::Redstone")

   }
}
impl<'mc> crate::material::RedstoneTrait<'mc> for Button<'mc> {}
impl<'mc> Into<crate::material::SimpleAttachableMaterialData<'mc>> for Button<'mc>{

fn into(self) -> crate::material::SimpleAttachableMaterialData<'mc> {

crate::material::SimpleAttachableMaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Button into crate::material::SimpleAttachableMaterialData")

   }
}
impl<'mc> crate::material::SimpleAttachableMaterialDataTrait<'mc> for Button<'mc> {}
#[repr(C)]
pub struct Crops<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Crops<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Crops<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Crops from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Crops")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Crops object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> CropsTrait<'mc> for Crops<'mc> {}
pub trait CropsTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::Crops<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Crops"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Crops::from_raw(&jni,res
)}
/// Gets the current growth state of this crop
/// For crops with only four growth states such as beetroot, only the values SEEDED, SMALL, TALL and RIPE will be
/// returned.
	fn state(&self) 
-> Result<crate::CropState<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/CropState;");
let res = self.jni_ref().call_method(&self.jni_object(),"getState",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::CropState::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the growth state of this crop
/// For crops with only four growth states such as beetroot, the 8 CropStates are mapped into four states:
/// SEEDED, SMALL, TALL and RIPE
/// GERMINATED will change to SEEDED
/// VERY_SMALL will change to SMALL
/// MEDIUM will change to TALL
/// VERY_TALL will change to RIPE
	fn set_state(&self,state: impl Into<crate::CropState<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/CropState;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(state.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setState",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::Crops<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Crops;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Crops::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Crops<'mc> {
            fn to_string(&self) -> String {
                match CropsTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Crops.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::MaterialData<'mc>> for Crops<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Crops into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for Crops<'mc> {}
#[repr(C)]
pub struct Wood<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Wood<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Wood<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Wood from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Wood")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Wood object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> WoodTrait<'mc> for Wood<'mc> {}
pub trait WoodTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::Wood<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Wood"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Wood::from_raw(&jni,res
)}
/// Gets the current species of this wood block
	fn species(&self) 
-> Result<crate::TreeSpecies<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/TreeSpecies;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSpecies",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::TreeSpecies::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the species of this wood block
	fn set_species(&self,species: impl Into<crate::TreeSpecies<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/TreeSpecies;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(species.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setSpecies",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::Wood<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Wood;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Wood::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Wood<'mc> {
            fn to_string(&self) -> String {
                match WoodTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Wood.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::MaterialData<'mc>> for Wood<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Wood into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for Wood<'mc> {}
#[repr(C)]
pub struct PressurePlate<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PressurePlate<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PressurePlate<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PressurePlate from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/PressurePlate")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PressurePlate object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PressurePlateTrait<'mc> for PressurePlate<'mc> {}
pub trait PressurePlateTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::PressurePlate<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/PressurePlate"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::PressurePlate::from_raw(&jni,res
)}

	fn is_pressed(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isPressed",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::PressurePlate<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/PressurePlate;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::PressurePlate::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for PressurePlate<'mc> {
            fn to_string(&self) -> String {
                match PressurePlateTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling PressurePlate.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::PressureSensor<'mc>> for PressurePlate<'mc>{

fn into(self) -> crate::material::PressureSensor<'mc> {

crate::material::PressureSensor::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PressurePlate into crate::material::PressureSensor")

   }
}
impl<'mc> crate::material::PressureSensorTrait<'mc> for PressurePlate<'mc> {}
impl<'mc> Into<crate::material::MaterialData<'mc>> for PressurePlate<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PressurePlate into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for PressurePlate<'mc> {}
#[repr(C)]
pub struct Colorable<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Colorable<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Colorable<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Colorable from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Colorable")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Colorable object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ColorableTrait<'mc> for Colorable<'mc> {}
pub trait ColorableTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Gets the color of this object.
/// 
/// This may be null to represent the default color of an object, if the
/// object has a special default color (e.g Shulkers).
	fn color(&self) 
-> Result<Option<crate::DyeColor<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/DyeColor;");
let res = self.jni_ref().call_method(&self.jni_object(),"getColor",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::DyeColor::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Sets the color of this object to the specified DyeColor.
/// 
/// This may be null to represent the default color of an object, if the
/// object has a special default color (e.g Shulkers).
	fn set_color(&self,color: impl Into<crate::DyeColor<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/DyeColor;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(color.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setColor",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct Diode<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Diode<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Diode<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Diode from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Diode")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Diode object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> DiodeTrait<'mc> for Diode<'mc> {}
pub trait DiodeTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Constructs a diode switched on or off, with the specified delay and
/// facing the specified direction.
	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,facing_direction: std::option::Option<impl Into<crate::block::BlockFace<'mc>>>,delay: std::option::Option<i32>,state: std::option::Option<bool>) 
-> Result<crate::material::Diode<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = facing_direction {
sig += "Lorg/bukkit/block/BlockFace;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = delay {
sig += "I";
let val_2 = jni::objects::JValueGen::Int(a);
args.push(val_2);
}
if let Some(a) = state {
sig += "Z";
let val_3 = jni::objects::JValueGen::Bool(a.into());
args.push(val_3);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Diode"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Diode::from_raw(&jni,res
)}
/// Sets the delay of the repeater.
	fn set_delay(&self,delay: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(delay);
let res = self.jni_ref().call_method(&self.jni_object(),"setDelay",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the delay of the repeater in ticks.
	fn delay(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getDelay",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Sets the direction this diode is facing.
	fn set_facing_direction(&self,face: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFacingDirection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the direction this diode is facing
	fn facing(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFacing",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::Diode<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Diode;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Diode::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Checks if the diode is powered.
	fn is_powered(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isPowered",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Diode<'mc> {
            fn to_string(&self) -> String {
                match DiodeTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Diode.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::Directional<'mc>> for Diode<'mc>{

fn into(self) -> crate::material::Directional<'mc> {

crate::material::Directional::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Diode into crate::material::Directional")

   }
}
impl<'mc> crate::material::DirectionalTrait<'mc> for Diode<'mc> {}
impl<'mc> Into<crate::material::Redstone<'mc>> for Diode<'mc>{

fn into(self) -> crate::material::Redstone<'mc> {

crate::material::Redstone::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Diode into crate::material::Redstone")

   }
}
impl<'mc> crate::material::RedstoneTrait<'mc> for Diode<'mc> {}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Diode<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Diode into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for Diode<'mc> {}
#[repr(C)]
pub struct TrapDoor<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for TrapDoor<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for TrapDoor<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate TrapDoor from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/TrapDoor")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a TrapDoor object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> TrapDoorTrait<'mc> for TrapDoor<'mc> {}
pub trait TrapDoorTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::TrapDoor<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/TrapDoor"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::TrapDoor::from_raw(&jni,res
)}

	fn is_open(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isOpen",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_open(&self,is_open: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(is_open.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setOpen",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Test if trapdoor is inverted
	fn is_inverted(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isInverted",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Set trapdoor inverted state
	fn set_inverted(&self,inv: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(inv.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setInverted",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn attached_face(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAttachedFace",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn set_facing_direction(&self,face: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFacingDirection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::TrapDoor<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/TrapDoor;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::TrapDoor::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for TrapDoor<'mc> {
            fn to_string(&self) -> String {
                match TrapDoorTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling TrapDoor.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::Openable<'mc>> for TrapDoor<'mc>{

fn into(self) -> crate::material::Openable<'mc> {

crate::material::Openable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting TrapDoor into crate::material::Openable")

   }
}
impl<'mc> crate::material::OpenableTrait<'mc> for TrapDoor<'mc> {}
impl<'mc> Into<crate::material::SimpleAttachableMaterialData<'mc>> for TrapDoor<'mc>{

fn into(self) -> crate::material::SimpleAttachableMaterialData<'mc> {

crate::material::SimpleAttachableMaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting TrapDoor into crate::material::SimpleAttachableMaterialData")

   }
}
impl<'mc> crate::material::SimpleAttachableMaterialDataTrait<'mc> for TrapDoor<'mc> {}
#[repr(C)]
pub struct SimpleAttachableMaterialData<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for SimpleAttachableMaterialData<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for SimpleAttachableMaterialData<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate SimpleAttachableMaterialData from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/SimpleAttachableMaterialData")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a SimpleAttachableMaterialData object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> SimpleAttachableMaterialDataTrait<'mc> for SimpleAttachableMaterialData<'mc> {}
pub trait SimpleAttachableMaterialDataTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: impl Into<crate::Material<'mc>>,data: std::option::Option<i8>) 
-> Result<crate::material::SimpleAttachableMaterialData<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/SimpleAttachableMaterialData"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::SimpleAttachableMaterialData::from_raw(&jni,res
)}

	fn facing(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFacing",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::SimpleAttachableMaterialData<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/SimpleAttachableMaterialData;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::SimpleAttachableMaterialData::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for SimpleAttachableMaterialData<'mc> {
            fn to_string(&self) -> String {
                match SimpleAttachableMaterialDataTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling SimpleAttachableMaterialData.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::Attachable<'mc>> for SimpleAttachableMaterialData<'mc>{

fn into(self) -> crate::material::Attachable<'mc> {

crate::material::Attachable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting SimpleAttachableMaterialData into crate::material::Attachable")

   }
}
impl<'mc> crate::material::AttachableTrait<'mc> for SimpleAttachableMaterialData<'mc> {}
impl<'mc> Into<crate::material::MaterialData<'mc>> for SimpleAttachableMaterialData<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting SimpleAttachableMaterialData into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for SimpleAttachableMaterialData<'mc> {}
#[repr(C)]
pub struct MonsterEggs<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for MonsterEggs<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for MonsterEggs<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate MonsterEggs from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/MonsterEggs")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a MonsterEggs object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> MonsterEggsTrait<'mc> for MonsterEggs<'mc> {}
pub trait MonsterEggsTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::MonsterEggs<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/MonsterEggs"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::MonsterEggs::from_raw(&jni,res
)}

	fn textures(&self) 
-> Result<Vec<crate::Material<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getTextures",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::Material::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}

	fn clone(&self) 
-> Result<crate::material::MonsterEggs<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/MonsterEggs;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::MonsterEggs::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::material::TexturedMaterial<'mc>> for MonsterEggs<'mc>{

fn into(self) -> crate::material::TexturedMaterial<'mc> {

crate::material::TexturedMaterial::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting MonsterEggs into crate::material::TexturedMaterial")

   }
}
impl<'mc> crate::material::TexturedMaterialTrait<'mc> for MonsterEggs<'mc> {}
#[repr(C)]
pub struct Banner<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Banner<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Banner<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Banner from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Banner")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Banner object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BannerTrait<'mc> for Banner<'mc> {}
pub trait BannerTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::Banner<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Banner"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Banner::from_raw(&jni,res
)}

	fn is_wall_banner(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isWallBanner",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn attached_face(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAttachedFace",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn facing(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFacing",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn set_facing_direction(&self,face: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFacingDirection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::Banner<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Banner;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Banner::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Banner<'mc> {
            fn to_string(&self) -> String {
                match BannerTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Banner.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::Attachable<'mc>> for Banner<'mc>{

fn into(self) -> crate::material::Attachable<'mc> {

crate::material::Attachable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Banner into crate::material::Attachable")

   }
}
impl<'mc> crate::material::AttachableTrait<'mc> for Banner<'mc> {}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Banner<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Banner into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for Banner<'mc> {}
#[repr(C)]
pub struct Step<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Step<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Step<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Step from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Step")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Step object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> StepTrait<'mc> for Step<'mc> {}
pub trait StepTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::Step<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Step"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Step::from_raw(&jni,res
)}

	fn textures(&self) 
-> Result<Vec<crate::Material<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getTextures",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::Material::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}
/// Test if step is inverted
	fn is_inverted(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isInverted",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Set step inverted state
	fn set_inverted(&self,inv: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(inv.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setInverted",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn clone(&self) 
-> Result<crate::material::Step<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Step;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Step::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Step<'mc> {
            fn to_string(&self) -> String {
                match StepTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Step.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::TexturedMaterial<'mc>> for Step<'mc>{

fn into(self) -> crate::material::TexturedMaterial<'mc> {

crate::material::TexturedMaterial::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Step into crate::material::TexturedMaterial")

   }
}
impl<'mc> crate::material::TexturedMaterialTrait<'mc> for Step<'mc> {}
#[repr(C)]
pub struct Gate<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Gate<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Gate<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Gate from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Gate")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Gate object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> GateTrait<'mc> for Gate<'mc> {}
pub trait GateTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::Gate<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Gate"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Gate::from_raw(&jni,res
)}

	fn set_facing_direction(&self,face: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFacingDirection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn facing(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFacing",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn is_open(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isOpen",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn set_open(&self,is_open: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(is_open.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setOpen",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::Gate<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Gate;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Gate::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Gate<'mc> {
            fn to_string(&self) -> String {
                match GateTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Gate.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::Directional<'mc>> for Gate<'mc>{

fn into(self) -> crate::material::Directional<'mc> {

crate::material::Directional::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Gate into crate::material::Directional")

   }
}
impl<'mc> crate::material::DirectionalTrait<'mc> for Gate<'mc> {}
impl<'mc> Into<crate::material::Openable<'mc>> for Gate<'mc>{

fn into(self) -> crate::material::Openable<'mc> {

crate::material::Openable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Gate into crate::material::Openable")

   }
}
impl<'mc> crate::material::OpenableTrait<'mc> for Gate<'mc> {}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Gate<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Gate into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for Gate<'mc> {}
#[repr(C)]
pub struct Leaves<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Leaves<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Leaves<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Leaves from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Leaves")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Leaves object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> LeavesTrait<'mc> for Leaves<'mc> {}
pub trait LeavesTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Constructs a leaf block of the given tree species and flag for whether
/// this leaf block will disappear when too far from a log.
	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,species: std::option::Option<impl Into<crate::TreeSpecies<'mc>>>,is_decayable: std::option::Option<bool>) 
-> Result<crate::material::Leaves<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = species {
sig += "Lorg/bukkit/TreeSpecies;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = is_decayable {
sig += "Z";
let val_2 = jni::objects::JValueGen::Bool(a.into());
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Leaves"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Leaves::from_raw(&jni,res
)}
/// Checks if this leaf block is in the process of decaying
	fn is_decaying(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isDecaying",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Set whether this leaf block is in the process of decaying
	fn set_decaying(&self,is_decaying: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(is_decaying.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setDecaying",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Checks if this leaf block is permanent or can decay when too far from a
/// log
	fn is_decayable(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isDecayable",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Set whether this leaf block will disappear when too far from a log
	fn set_decayable(&self,is_decayable: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(is_decayable.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setDecayable",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::Leaves<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Leaves;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Leaves::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Leaves<'mc> {
            fn to_string(&self) -> String {
                match LeavesTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Leaves.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::Wood<'mc>> for Leaves<'mc>{

fn into(self) -> crate::material::Wood<'mc> {

crate::material::Wood::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Leaves into crate::material::Wood")

   }
}
impl<'mc> crate::material::WoodTrait<'mc> for Leaves<'mc> {}
#[repr(C)]
pub struct FlowerPot<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for FlowerPot<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for FlowerPot<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate FlowerPot from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/FlowerPot")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a FlowerPot object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> FlowerPotTrait<'mc> for FlowerPot<'mc> {}
pub trait FlowerPotTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::FlowerPot<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/FlowerPot"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::FlowerPot::from_raw(&jni,res
)}
/// Get the material in the flower pot
	fn contents(&self) 
-> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/MaterialData;");
let res = self.jni_ref().call_method(&self.jni_object(),"getContents",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::MaterialData::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Set the contents of the flower pot
	fn set_contents(&self,material_data: impl Into<crate::material::MaterialData<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/material/MaterialData;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(material_data.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setContents",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::FlowerPot<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/FlowerPot;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::FlowerPot::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for FlowerPot<'mc> {
            fn to_string(&self) -> String {
                match FlowerPotTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling FlowerPot.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::MaterialData<'mc>> for FlowerPot<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting FlowerPot into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for FlowerPot<'mc> {}
#[repr(C)]
pub struct PistonBaseMaterial<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PistonBaseMaterial<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PistonBaseMaterial<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PistonBaseMaterial from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/PistonBaseMaterial")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PistonBaseMaterial object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PistonBaseMaterialTrait<'mc> for PistonBaseMaterial<'mc> {}
pub trait PistonBaseMaterialTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]
/// Constructs a PistonBaseMaterial.
	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: impl Into<crate::Material<'mc>>,data: std::option::Option<i8>) 
-> Result<crate::material::PistonBaseMaterial<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/PistonBaseMaterial"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::PistonBaseMaterial::from_raw(&jni,res
)}

	fn set_facing_direction(&self,face: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFacingDirection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn facing(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFacing",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn is_powered(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isPowered",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Sets the current state of this piston
	fn set_powered(&self,powered: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(powered.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setPowered",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Checks if this piston base is sticky, and returns true if so
	fn is_sticky(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isSticky",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn clone(&self) 
-> Result<crate::material::PistonBaseMaterial<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/PistonBaseMaterial;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::PistonBaseMaterial::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::material::Directional<'mc>> for PistonBaseMaterial<'mc>{

fn into(self) -> crate::material::Directional<'mc> {

crate::material::Directional::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PistonBaseMaterial into crate::material::Directional")

   }
}
impl<'mc> crate::material::DirectionalTrait<'mc> for PistonBaseMaterial<'mc> {}
impl<'mc> Into<crate::material::Redstone<'mc>> for PistonBaseMaterial<'mc>{

fn into(self) -> crate::material::Redstone<'mc> {

crate::material::Redstone::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PistonBaseMaterial into crate::material::Redstone")

   }
}
impl<'mc> crate::material::RedstoneTrait<'mc> for PistonBaseMaterial<'mc> {}
impl<'mc> Into<crate::material::MaterialData<'mc>> for PistonBaseMaterial<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PistonBaseMaterial into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for PistonBaseMaterial<'mc> {}
#[repr(C)]
pub struct Skull<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Skull<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Skull<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Skull from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Skull")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Skull object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> SkullTrait<'mc> for Skull<'mc> {}
pub trait SkullTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::Skull<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Skull"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Skull::from_raw(&jni,res
)}

	fn set_facing_direction(&self,face: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFacingDirection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn facing(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFacing",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::Skull<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Skull;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Skull::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Skull<'mc> {
            fn to_string(&self) -> String {
                match SkullTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Skull.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::Directional<'mc>> for Skull<'mc>{

fn into(self) -> crate::material::Directional<'mc> {

crate::material::Directional::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Skull into crate::material::Directional")

   }
}
impl<'mc> crate::material::DirectionalTrait<'mc> for Skull<'mc> {}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Skull<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Skull into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for Skull<'mc> {}
#[repr(C)]
pub struct Torch<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Torch<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Torch<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Torch from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Torch")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Torch object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> TorchTrait<'mc> for Torch<'mc> {}
pub trait TorchTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_type: std::option::Option<impl Into<crate::Material<'mc>>>,data: std::option::Option<i8>) 
-> Result<crate::material::Torch<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = val_type {
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = data {
sig += "B";
let val_2 = jni::objects::JValueGen::Byte(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Torch"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Torch::from_raw(&jni,res
)}
/// Gets the face that this block is attached on
	fn attached_face(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAttachedFace",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn set_facing_direction(&self,face: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFacingDirection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn clone(&self) 
-> Result<crate::material::Torch<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Torch;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Torch::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::material::SimpleAttachableMaterialData<'mc>> for Torch<'mc>{

fn into(self) -> crate::material::SimpleAttachableMaterialData<'mc> {

crate::material::SimpleAttachableMaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Torch into crate::material::SimpleAttachableMaterialData")

   }
}
impl<'mc> crate::material::SimpleAttachableMaterialDataTrait<'mc> for Torch<'mc> {}
#[repr(C)]
pub struct Comparator<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Comparator<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Comparator<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Comparator from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/material/Comparator")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Comparator object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ComparatorTrait<'mc> for Comparator<'mc> {}
pub trait ComparatorTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Constructs a comparator switched on or off, with the specified mode and facing the specified direction.
	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,facing_direction: std::option::Option<impl Into<crate::block::BlockFace<'mc>>>,is_subtraction: std::option::Option<bool>,state: std::option::Option<bool>) 
-> Result<crate::material::Comparator<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = facing_direction {
sig += "Lorg/bukkit/block/BlockFace;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = is_subtraction {
sig += "Z";
let val_2 = jni::objects::JValueGen::Bool(a.into());
args.push(val_2);
}
if let Some(a) = state {
sig += "Z";
let val_3 = jni::objects::JValueGen::Bool(a.into());
args.push(val_3);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/material/Comparator"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::material::Comparator::from_raw(&jni,res
)}
/// Sets whether the comparator is in subtraction mode.
	fn set_subtraction_mode(&self,is_subtraction: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(is_subtraction.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setSubtractionMode",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Checks whether the comparator is in subtraction mode
	fn is_subtraction_mode(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isSubtractionMode",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Sets the direction this comparator is facing
	fn set_facing_direction(&self,face: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFacingDirection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the direction this comparator is facing
	fn facing(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFacing",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

	fn clone(&self) 
-> Result<crate::material::Comparator<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/Comparator;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::Comparator::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Checks if the comparator is powered
	fn is_powered(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isPowered",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Checks if the comparator is being powered
	fn is_being_powered(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isBeingPowered",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Comparator<'mc> {
            fn to_string(&self) -> String {
                match ComparatorTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Comparator.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::material::Directional<'mc>> for Comparator<'mc>{

fn into(self) -> crate::material::Directional<'mc> {

crate::material::Directional::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Comparator into crate::material::Directional")

   }
}
impl<'mc> crate::material::DirectionalTrait<'mc> for Comparator<'mc> {}
impl<'mc> Into<crate::material::Redstone<'mc>> for Comparator<'mc>{

fn into(self) -> crate::material::Redstone<'mc> {

crate::material::Redstone::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Comparator into crate::material::Redstone")

   }
}
impl<'mc> crate::material::RedstoneTrait<'mc> for Comparator<'mc> {}
impl<'mc> Into<crate::material::MaterialData<'mc>> for Comparator<'mc>{

fn into(self) -> crate::material::MaterialData<'mc> {

crate::material::MaterialData::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Comparator into crate::material::MaterialData")

   }
}
impl<'mc> crate::material::MaterialDataTrait<'mc> for Comparator<'mc> {}
pub mod types;
