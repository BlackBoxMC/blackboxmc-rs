#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct StructurePiece<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for StructurePiece<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for StructurePiece<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate StructurePiece from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/generator/structure/StructurePiece")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a StructurePiece object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> StructurePiece<'mc> {
	pub fn bounding_box(&self) 
-> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::util::BoundingBox;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBoundingBox",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::BoundingBox::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct GeneratedStructure<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for GeneratedStructure<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for GeneratedStructure<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate GeneratedStructure from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/generator/structure/GeneratedStructure")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a GeneratedStructure object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> GeneratedStructure<'mc> {
	pub fn bounding_box(&self) 
-> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::util::BoundingBox;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBoundingBox",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::BoundingBox::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn structure(&self) 
-> Result<crate::generator::structure::Structure<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::generator::structure::Structure;");
let res = self.jni_ref().call_method(&self.jni_object(),"getStructure",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::generator::structure::Structure::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn pieces(&self) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()LVec;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPieces",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(),res.l()?)?;let iter = col.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn persistent_data_container(&self) 
-> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::persistence::PersistentDataContainer;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPersistentDataContainer",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::persistence::PersistentDataHolder<'mc>> for GeneratedStructure<'mc>{

fn into(self) -> crate::persistence::PersistentDataHolder<'mc> {

crate::persistence::PersistentDataHolder::from_raw(&self.jni_ref(), self.1).expect("Error converting GeneratedStructure into crate::persistence::PersistentDataHolder")

   }
}
#[repr(C)]
pub struct StructureType<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for StructureType<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for StructureType<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate StructureType from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/generator/structure/StructureType")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a StructureType object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> StructureType<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::generator::structure::StructureType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()V");
let cls = jni.find_class("org/bukkit/generator/structure/StructureType"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![]);
let res = 
jni.translate_error_no_gen(res)?;
crate::generator::structure::StructureType::from_raw(&jni,res
)}
	pub fn key(&self) 
-> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::NamespacedKey;");
let res = self.jni_ref().call_method(&self.jni_object(),"getKey",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::NamespacedKey::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
// SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::Keyed<'mc>> for StructureType<'mc>{

fn into(self) -> crate::Keyed<'mc> {

crate::Keyed::from_raw(&self.jni_ref(), self.1).expect("Error converting StructureType into crate::Keyed")

   }
}
#[repr(C)]
pub struct Structure<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Structure<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Structure<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Structure from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/generator/structure/Structure")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Structure object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> Structure<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::generator::structure::Structure<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()V");
let cls = jni.find_class("org/bukkit/generator/structure/Structure"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![]);
let res = 
jni.translate_error_no_gen(res)?;
crate::generator::structure::Structure::from_raw(&jni,res
)}
	pub fn key(&self) 
-> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::NamespacedKey;");
let res = self.jni_ref().call_method(&self.jni_object(),"getKey",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::NamespacedKey::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
// SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::Keyed<'mc>> for Structure<'mc>{

fn into(self) -> crate::Keyed<'mc> {

crate::Keyed::from_raw(&self.jni_ref(), self.1).expect("Error converting Structure into crate::Keyed")

   }
}
