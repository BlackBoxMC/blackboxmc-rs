#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;/*org/bukkit/packs/mod.rs*/

#[repr(C)]
pub struct DataPackManager<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for DataPackManager<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for DataPackManager<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate DataPackManager from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/packs/DataPackManager")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a DataPackManager object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> DataPackManagerTrait<'mc> for DataPackManager<'mc> {}
pub trait DataPackManagerTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Return all the available {@link DataPack}s on the server.
	fn data_packs(&self) 
-> Result<Vec<crate::packs::DataPack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Collection;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDataPacks",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(),res.l()?)?;let iter = col.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::packs::DataPack::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}
/// Gets a {@link DataPack} by its key.
	fn get_data_pack(&self,data_pack_key: impl Into<crate::NamespacedKey<'mc>>) 
-> Result<Option<crate::packs::DataPack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/NamespacedKey;)Lorg/bukkit/packs/DataPack;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(data_pack_key.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"getDataPack",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::packs::DataPack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Return all the enabled {@link DataPack} in the World.
	fn get_enabled_data_packs(&self,world: impl Into<crate::World<'mc>>) 
-> Result<Vec<crate::packs::DataPack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/World;)Ljava/util/Collection;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(world.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"getEnabledDataPacks",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(),res.l()?)?;let iter = col.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::packs::DataPack::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}
/// Return all the disabled {@link DataPack} in the World.
	fn get_disabled_data_packs(&self,world: impl Into<crate::World<'mc>>) 
-> Result<Vec<crate::packs::DataPack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/World;)Ljava/util/Collection;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(world.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"getDisabledDataPacks",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(),res.l()?)?;let iter = col.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::packs::DataPack::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}
/// Gets if the EntityType is enabled for use by the Features in World.
	fn is_enabled_by_feature(&self,entity_type: impl Into<crate::entity::EntityType<'mc>>,world: impl Into<crate::World<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/entity/EntityType;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(entity_type.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/World;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(world.into().jni_object().clone())});
args.push(val_2);
sig += ")Z";
let res = self.jni_ref().call_method(&self.jni_object(),"isEnabledByFeature",sig.as_str(),args);
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
pub enum DataPackSource<'mc> {
	Default {inner: DataPackSourceStruct<'mc>},
	BuiltIn {inner: DataPackSourceStruct<'mc>},
	Feature {inner: DataPackSourceStruct<'mc>},
	World {inner: DataPackSourceStruct<'mc>},
	Server {inner: DataPackSourceStruct<'mc>},
}
impl<'mc> std::fmt::Display for DataPackSource<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           DataPackSource::Default { .. } => f.write_str("DEFAULT"),
           DataPackSource::BuiltIn { .. } => f.write_str("BUILT_IN"),
           DataPackSource::Feature { .. } => f.write_str("FEATURE"),
           DataPackSource::World { .. } => f.write_str("WORLD"),
           DataPackSource::Server { .. } => f.write_str("SERVER"),
       }
   }
}

        impl<'mc> DataPackSourceTrait<'mc> for DataPackSource<'mc> {}
        
        pub trait DataPackSourceTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<DataPackSource<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/packs/DataPack/Source");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/packs/DataPack/Source;",
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
                    
"DEFAULT" => Ok(DataPackSource::Default { inner: DataPackSourceStruct::from_raw(env,obj)?}),
"BUILT_IN" => Ok(DataPackSource::BuiltIn { inner: DataPackSourceStruct::from_raw(env,obj)?}),
"FEATURE" => Ok(DataPackSource::Feature { inner: DataPackSourceStruct::from_raw(env,obj)?}),
"WORLD" => Ok(DataPackSource::World { inner: DataPackSourceStruct::from_raw(env,obj)?}),
"SERVER" => Ok(DataPackSource::Server { inner: DataPackSourceStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct DataPackSourceStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for DataPackSource<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::Default { inner } => inner.0.clone(),
Self::BuiltIn { inner } => inner.0.clone(),
Self::Feature { inner } => inner.0.clone(),
Self::World { inner } => inner.0.clone(),
Self::Server { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::Default { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::BuiltIn { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Feature { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::World { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Server { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for DataPackSource<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate DataPackSource from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/packs/DataPack/Source")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a DataPackSource object, got {}",
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
                    "DEFAULT" => Ok(DataPackSource::Default { inner: DataPackSourceStruct::from_raw(env,obj)?}),"BUILT_IN" => Ok(DataPackSource::BuiltIn { inner: DataPackSourceStruct::from_raw(env,obj)?}),"FEATURE" => Ok(DataPackSource::Feature { inner: DataPackSourceStruct::from_raw(env,obj)?}),"WORLD" => Ok(DataPackSource::World { inner: DataPackSourceStruct::from_raw(env,obj)?}),"SERVER" => Ok(DataPackSource::Server { inner: DataPackSourceStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for DataPackSourceStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for DataPackSourceStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate DataPackSourceStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/packs/DataPack/Source")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a DataPackSourceStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> DataPackSourceStruct<'mc> {

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::packs::DataPackSource<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/packs/DataPack/Source;");
let cls = jni.find_class("org/bukkit/packs/DataPack/Source"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::packs::DataPackSource::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct ResourcePack<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ResourcePack<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ResourcePack<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ResourcePack from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/packs/ResourcePack")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ResourcePack object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ResourcePackTrait<'mc> for ResourcePack<'mc> {}
pub trait ResourcePackTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Gets the id of the resource pack.
	fn id(&self) 
-> Result<blackboxmc_java::util::JavaUUID<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/UUID;");
let res = self.jni_ref().call_method(&self.jni_object(),"getId",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaUUID::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the url of the resource pack.
	fn url(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getUrl",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Gets the hash of the resource pack.
	fn hash(&self) 
-> Result<Option<String>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHash",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)
)}
/// Gets the prompt to show of the resource pack.
	fn prompt(&self) 
-> Result<Option<String>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPrompt",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)
)}
/// Gets if the resource pack is required by the server.
	fn is_required(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isRequired",sig.as_str(),vec![]);
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
pub enum DataPackCompatibility<'mc> {
	New {inner: DataPackCompatibilityStruct<'mc>},
	Old {inner: DataPackCompatibilityStruct<'mc>},
	Compatible {inner: DataPackCompatibilityStruct<'mc>},
}
impl<'mc> std::fmt::Display for DataPackCompatibility<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           DataPackCompatibility::New { .. } => f.write_str("NEW"),
           DataPackCompatibility::Old { .. } => f.write_str("OLD"),
           DataPackCompatibility::Compatible { .. } => f.write_str("COMPATIBLE"),
       }
   }
}

        impl<'mc> DataPackCompatibilityTrait<'mc> for DataPackCompatibility<'mc> {}
        
        pub trait DataPackCompatibilityTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<DataPackCompatibility<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/packs/DataPack/Compatibility");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/packs/DataPack/Compatibility;",
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
                    
"NEW" => Ok(DataPackCompatibility::New { inner: DataPackCompatibilityStruct::from_raw(env,obj)?}),
"OLD" => Ok(DataPackCompatibility::Old { inner: DataPackCompatibilityStruct::from_raw(env,obj)?}),
"COMPATIBLE" => Ok(DataPackCompatibility::Compatible { inner: DataPackCompatibilityStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct DataPackCompatibilityStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for DataPackCompatibility<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::New { inner } => inner.0.clone(),
Self::Old { inner } => inner.0.clone(),
Self::Compatible { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::New { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Old { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Compatible { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for DataPackCompatibility<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate DataPackCompatibility from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/packs/DataPack/Compatibility")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a DataPackCompatibility object, got {}",
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
                    "NEW" => Ok(DataPackCompatibility::New { inner: DataPackCompatibilityStruct::from_raw(env,obj)?}),"OLD" => Ok(DataPackCompatibility::Old { inner: DataPackCompatibilityStruct::from_raw(env,obj)?}),"COMPATIBLE" => Ok(DataPackCompatibility::Compatible { inner: DataPackCompatibilityStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for DataPackCompatibilityStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for DataPackCompatibilityStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate DataPackCompatibilityStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/packs/DataPack/Compatibility")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a DataPackCompatibilityStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> DataPackCompatibilityStruct<'mc> {

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::packs::DataPackCompatibility<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/packs/DataPack/Compatibility;");
let cls = jni.find_class("org/bukkit/packs/DataPack/Compatibility"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::packs::DataPackCompatibility::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct DataPack<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for DataPack<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for DataPack<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate DataPack from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/packs/DataPack")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a DataPack object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> DataPackTrait<'mc> for DataPack<'mc> {}
pub trait DataPackTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Gets the title of the data pack.
	fn title(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getTitle",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Gets the description of the data pack.
	fn description(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDescription",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Gets the pack format.
/// 
/// Pack formats are non-standard and unrelated to the version of Minecraft. For
/// a list of known pack versions, see the
/// <a href="https://minecraft.wiki/w/Data_pack#Pack_format">Minecraft Wiki</a>.
	fn pack_format(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getPackFormat",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Gets the minimum supported pack format. If the data pack does not specify a
/// minimum supported format, {@link #getPackFormat()} is returned.
/// 
/// Pack formats are non-standard and unrelated to the version of Minecraft. For
/// a list of known pack versions, see the
/// <a href="https://minecraft.wiki/w/Data_pack#Pack_format">Minecraft Wiki</a>.
	fn min_supported_pack_format(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getMinSupportedPackFormat",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Gets the maximum supported pack format. If the data pack does not specify a
/// maximum supported format, {@link #getPackFormat()} is returned.
/// 
/// Pack formats are non-standard and unrelated to the version of Minecraft. For
/// a list of known pack versions, see the
/// <a href="https://minecraft.wiki/w/Data_pack#Pack_format">Minecraft Wiki</a>.
	fn max_supported_pack_format(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getMaxSupportedPackFormat",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Gets if the data pack is enabled on the server.
	fn is_enabled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isEnabled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets if the data pack is required on the server.
	fn is_required(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isRequired",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets the compatibility of this data pack with the server.
	fn compatibility(&self) 
-> Result<crate::packs::DataPackCompatibility<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/packs/DataPack/Compatibility;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCompatibility",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::packs::DataPackCompatibility::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets a set of features requested by this data pack.
	fn requested_features(&self) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Set;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRequestedFeatures",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the source of this data pack.
	fn source(&self) 
-> Result<crate::packs::DataPackSource<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/packs/DataPack/Source;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSource",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::packs::DataPackSource::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::Keyed<'mc>> for DataPack<'mc>{

fn into(self) -> crate::Keyed<'mc> {

crate::Keyed::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting DataPack into crate::Keyed")

   }
}
impl<'mc> crate::KeyedTrait<'mc> for DataPack<'mc> {}
