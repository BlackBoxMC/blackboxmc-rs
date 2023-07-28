#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// An instantiatable struct that implements MetadataStore. Needed for returning it from Java.
pub struct MetadataStore<'mc, T>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where T: JNIRaw<'mc>;
impl<'mc, T> MetadataStore<'mc, T> where T: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate MetadataStore from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "MetadataStore")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a MetadataStore object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn set_metadata(&mut self,arg0: T,arg1: impl Into<&'mc String>,arg2: impl Into<&'mc crate::metadata::MetadataValue<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = arg0;
let val_2 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setMetadata","(Ljava/lang/Object;Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn get_metadata(&mut self,arg0: T,arg1: impl Into<&'mc String>) 
-> Result<blackboxmc_java::JavaList<'mc, orgMetadataValue>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let val_2 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"getMetadata","(Ljava/lang/Object;Ljava/lang/String;)Ljava/util/List;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn has_metadata(&mut self,arg0: T,arg1: impl Into<&'mc String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let val_2 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"hasMetadata","(Ljava/lang/Object;Ljava/lang/String;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove_metadata(&mut self,arg0: T,arg1: impl Into<&'mc String>,arg2: impl Into<&'mc crate::plugin::Plugin<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = arg0;
let val_2 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"removeMetadata","(Ljava/lang/Object;Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn invalidate_all(&mut self,arg0: impl Into<&'mc crate::plugin::Plugin<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"invalidateAll","(Lorg/bukkit/plugin/Plugin;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc, T> JNIRaw<'mc> for MetadataStore<'mc, T> where T: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements MetadataValue. Needed for returning it from Java.
pub struct MetadataValue<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> MetadataValue<'mc> {
pub fn from_extendable(
    env: &blackboxmc_general::SharedJNIEnv<'mc>,
    plugin: &'mc crate::plugin::Plugin,
    address: i32,
    lib_name: String,
    name: String,
) -> Result<Self, Box<dyn std::error::Error>> {
    let obj = unsafe { plugin.new_extendable(address, "MetadataValue", name, lib_name) }?;
    Self::from_raw(env, obj)
}
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate MetadataValue from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "MetadataValue")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a MetadataValue object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn value(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"value","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn as_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn as_int(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asInt","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn owning_plugin(&mut self) 
-> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getOwningPlugin","()Lorg/bukkit/plugin/Plugin;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::plugin::Plugin::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn invalidate(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"invalidate","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn as_float(&mut self) 
-> Result<f32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asFloat","()F",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.f().unwrap())}
	pub fn as_double(&mut self) 
-> Result<f64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asDouble","()D",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
	pub fn as_long(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asLong","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn as_short(&mut self) 
-> Result<i16, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asShort","()S",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.s().unwrap())}
	pub fn as_byte(&mut self) 
-> Result<i8, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asByte","()B",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.b().unwrap())}
	pub fn as_boolean(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asBoolean","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
}
impl<'mc> JNIRaw<'mc> for MetadataValue<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements Metadatable. Needed for returning it from Java.
pub struct Metadatable<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> Metadatable<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate Metadatable from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "Metadatable")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a Metadatable object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn set_metadata(&mut self,arg0: impl Into<&'mc String>,arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setMetadata","(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn get_metadata(&mut self,arg0: impl Into<&'mc String>) 
-> Result<blackboxmc_java::JavaList<'mc, orgMetadataValue>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"getMetadata","(Ljava/lang/String;)Ljava/util/List;",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn has_metadata(&mut self,arg0: impl Into<&'mc String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"hasMetadata","(Ljava/lang/String;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove_metadata(&mut self,arg0: impl Into<&'mc String>,arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"removeMetadata","(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> JNIRaw<'mc> for Metadatable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct MetadataStoreBase<'mc, T>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>) where T: JNIRaw<'mc>;
impl<'mc, T> blackboxmc_general::JNIRaw<'mc> for MetadataStoreBase<'mc, T> where T: JNIRaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, T> MetadataStoreBase<'mc, T> where T: JNIRaw<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate MetadataStoreBase from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "MetadataStoreBase")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a MetadataStoreBase object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::metadata::MetadataStoreBase<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/metadata/MetadataStoreBase")?;
let res = jni.new_object(cls,
"()V",&[])?;
crate::metadata::MetadataStoreBase::from_raw(&jni,res
)}
	pub fn set_metadata(&mut self,arg0: T,arg1: impl Into<&'mc String>,arg2: impl Into<&'mc crate::metadata::MetadataValue<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = arg0;
let val_2 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setMetadata","(Ljava/lang/Object;Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn get_metadata(&mut self,arg0: T,arg1: impl Into<&'mc String>) 
-> Result<blackboxmc_java::JavaList<'mc, orgMetadataValue>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let val_2 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"getMetadata","(Ljava/lang/Object;Ljava/lang/String;)Ljava/util/List;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn has_metadata(&mut self,arg0: T,arg1: impl Into<&'mc String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let val_2 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
let res = self.jni_ref().call_method(&self.jni_object(),"hasMetadata","(Ljava/lang/Object;Ljava/lang/String;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn remove_metadata(&mut self,arg0: T,arg1: impl Into<&'mc String>,arg2: impl Into<&'mc crate::plugin::Plugin<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = arg0;
let val_2 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"removeMetadata","(Ljava/lang/Object;Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn invalidate_all(&mut self,arg0: impl Into<&'mc crate::plugin::Plugin<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"invalidateAll","(Lorg/bukkit/plugin/Plugin;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
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
pub struct LazyMetadataValue<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
pub struct LazyMetadataValueCacheStrategy<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for LazyMetadataValueCacheStrategy<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> LazyMetadataValueCacheStrategy<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate LazyMetadataValueCacheStrategy from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "LazyMetadataValueCacheStrategy")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a LazyMetadataValueCacheStrategy object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn value_of_with_string(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<jni::objects::JClass<'mc>>,arg1: std::option::Option<impl Into<&'mc String>>) 
-> Result<T, Box<dyn std::error::Error>>

{let val_1 = arg0.unwrap();
let val_2 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
let cls = &jni.find_class("java/lang/Enum")?;
let res = jni.call_static_method(cls,"valueOf",
"(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
let mut obj = res.l()?;
T::from_raw(&jni,obj
)}
	pub fn name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"name","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
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

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
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
impl<'mc> blackboxmc_general::JNIRaw<'mc> for LazyMetadataValue<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> LazyMetadataValue<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate LazyMetadataValue from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "LazyMetadataValue")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a LazyMetadataValue object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_plugin(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::plugin::Plugin<'mc>>,arg1: std::option::Option<impl Into<&'mc crate::metadata::LazyMetadataValueCacheStrategy<'mc>>>,arg2: std::option::Option<impl Into<&'mc blackboxmc_java::concurrent::JavaCallable<'mc, javaObject>>>) 
-> Result<crate::metadata::LazyMetadataValue<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/metadata/LazyMetadataValue")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/plugin/Plugin;Lorg/bukkit/metadata/LazyMetadataValue$CacheStrategy;Ljava/util/concurrent/Callable;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
crate::metadata::LazyMetadataValue::from_raw(&jni,res
)}
	pub fn value(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"value","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn invalidate(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"invalidate","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn as_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn as_int(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asInt","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn owning_plugin(&mut self) 
-> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getOwningPlugin","()Lorg/bukkit/plugin/Plugin;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::plugin::Plugin::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn as_float(&mut self) 
-> Result<f32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asFloat","()F",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.f().unwrap())}
	pub fn as_double(&mut self) 
-> Result<f64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asDouble","()D",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
	pub fn as_long(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asLong","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn as_short(&mut self) 
-> Result<i16, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asShort","()S",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.s().unwrap())}
	pub fn as_byte(&mut self) 
-> Result<i8, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asByte","()B",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.b().unwrap())}
	pub fn as_boolean(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asBoolean","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
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
impl<'mc> Into<crate::metadata::MetadataValueAdapter<'mc>> for LazyMetadataValue<'mc>{
   fn into(self) -> crate::metadata::MetadataValueAdapter<'mc> {
       crate::metadata::MetadataValueAdapter::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct FixedMetadataValue<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for FixedMetadataValue<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> FixedMetadataValue<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate FixedMetadataValue from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "FixedMetadataValue")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a FixedMetadataValue object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::plugin::Plugin<'mc>>,arg1: jni::objects::JObject<'mc>) 
-> Result<crate::metadata::FixedMetadataValue<'mc>, Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_2 = arg1;
let cls = &jni.find_class("org/bukkit/metadata/FixedMetadataValue")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/plugin/Plugin;Ljava/lang/Object;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::metadata::FixedMetadataValue::from_raw(&jni,res
)}
	pub fn value(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"value","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn invalidate(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"invalidate","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn as_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn as_int(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asInt","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn owning_plugin(&mut self) 
-> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getOwningPlugin","()Lorg/bukkit/plugin/Plugin;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::plugin::Plugin::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn as_float(&mut self) 
-> Result<f32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asFloat","()F",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.f().unwrap())}
	pub fn as_double(&mut self) 
-> Result<f64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asDouble","()D",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
	pub fn as_long(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asLong","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn as_short(&mut self) 
-> Result<i16, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asShort","()S",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.s().unwrap())}
	pub fn as_byte(&mut self) 
-> Result<i8, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asByte","()B",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.b().unwrap())}
	pub fn as_boolean(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asBoolean","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
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
impl<'mc> Into<crate::metadata::LazyMetadataValue<'mc>> for FixedMetadataValue<'mc>{
   fn into(self) -> crate::metadata::LazyMetadataValue<'mc> {
       crate::metadata::LazyMetadataValue::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct MetadataValueAdapter<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for MetadataValueAdapter<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> MetadataValueAdapter<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate MetadataValueAdapter from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "MetadataValueAdapter")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a MetadataValueAdapter object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn as_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn as_int(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asInt","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn owning_plugin(&mut self) 
-> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getOwningPlugin","()Lorg/bukkit/plugin/Plugin;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::plugin::Plugin::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn as_float(&mut self) 
-> Result<f32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asFloat","()F",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.f().unwrap())}
	pub fn as_double(&mut self) 
-> Result<f64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asDouble","()D",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.d().unwrap())}
	pub fn as_long(&mut self) 
-> Result<i64, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asLong","()J",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.j().unwrap())}
	pub fn as_short(&mut self) 
-> Result<i16, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asShort","()S",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.s().unwrap())}
	pub fn as_byte(&mut self) 
-> Result<i8, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asByte","()B",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.b().unwrap())}
	pub fn as_boolean(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"asBoolean","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
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
	pub fn value(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"value","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l().unwrap())}
	pub fn invalidate(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"invalidate","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::metadata::MetadataValue<'mc>> for MetadataValueAdapter<'mc>{
   fn into(self) -> crate::metadata::MetadataValue<'mc> {
       crate::metadata::MetadataValue::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
