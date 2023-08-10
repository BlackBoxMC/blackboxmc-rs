#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;

pub struct Item<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Item<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Item<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate Item from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "net/md_5/bungee/api/chat/hover/content/Item")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a Item object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<String>,arg1: i32,arg2: impl Into<crate::bungee::api::chat::ItemTag<'mc>>) 
-> Result<crate::bungee::api::chat::hover::content::Item<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let cls = jni.find_class("net/md_5/bungee/api/chat/hover/content/Item"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
"(Ljava/lang/String;ILnet/md_5/bungee/api/chat/ItemTag;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
let res = jni.translate_error_no_gen(res)?;
crate::bungee::api::chat::hover::content::Item::from_raw(&jni,res
)}
//


	pub fn count(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCount","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
//


	pub fn tag(&mut self) 
-> Result<crate::bungee::api::chat::ItemTag<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getTag","()Lnet/md_5/bungee/api/chat/ItemTag;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::bungee::api::chat::ItemTag::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
//


	pub fn set_id(&mut self,arg0: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into())?);
let res = self.jni_ref().call_method(&self.jni_object(),"setId","(Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn set_tag(&mut self,arg0: impl Into<crate::bungee::api::chat::ItemTag<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setTag","(Lnet/md_5/bungee/api/chat/ItemTag;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn required_action(&mut self) 
-> Result<crate::bungee::api::chat::HoverEventAction<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"requiredAction","()Lnet/md_5/bungee/api/chat/HoverEvent$Action;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]); let variant = self.jni_ref().translate_error(variant)?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::bungee::api::chat::HoverEventAction::from_raw(&self.jni_ref(),raw_obj
, crate::bungee::api::chat::HoverEventAction::from_string(variant_str).ok_or(eyre::eyre!("String gaven for variant was invalid"))?
)}
//


	pub fn set_count(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCount","(I)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
//


	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
//


	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
//


	pub fn id(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getId","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
//


	pub fn assert_action(&mut self,arg0: impl Into<crate::bungee::api::chat::HoverEventAction<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"assertAction","(Lnet/md_5/bungee/api/chat/HoverEvent$Action;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?.into());
let val_2 = jni::objects::JValueGen::Int(arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?.into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
unsafe {jni::objects::JClass::from_raw(res.as_jni().l)}
)}
//


	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
}
impl<'mc> Into<crate::bungee::api::chat::hover::content::Content<'mc>> for Item<'mc>{

fn into(self) -> crate::bungee::api::chat::hover::content::Content<'mc> {

crate::bungee::api::chat::hover::content::Content::from_raw(&self.jni_ref(), self.1).expect("Error converting Item into crate::bungee::api::chat::hover::content::Content")

   }
}

pub struct Entity<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Entity<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Entity<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate Entity from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "net/md_5/bungee/api/chat/hover/content/Entity")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a Entity object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<String>,arg1: impl Into<String>,arg2: impl Into<crate::bungee::api::chat::BaseComponent<'mc>>) 
-> Result<crate::bungee::api::chat::hover::content::Entity<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
let val_2 = jni::objects::JObject::from(jni.new_string(arg1.into())?);
let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let cls = jni.find_class("net/md_5/bungee/api/chat/hover/content/Entity"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
"(Ljava/lang/String;Ljava/lang/String;Lnet/md_5/bungee/api/chat/BaseComponent;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
let res = jni.translate_error_no_gen(res)?;
crate::bungee::api::chat::hover::content::Entity::from_raw(&jni,res
)}
//


	pub fn set_id(&mut self,arg0: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into())?);
let res = self.jni_ref().call_method(&self.jni_object(),"setId","(Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn set_type(&mut self,arg0: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into())?);
let res = self.jni_ref().call_method(&self.jni_object(),"setType","(Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn required_action(&mut self) 
-> Result<crate::bungee::api::chat::HoverEventAction<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"requiredAction","()Lnet/md_5/bungee/api/chat/HoverEvent$Action;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]); let variant = self.jni_ref().translate_error(variant)?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::bungee::api::chat::HoverEventAction::from_raw(&self.jni_ref(),raw_obj
, crate::bungee::api::chat::HoverEventAction::from_string(variant_str).ok_or(eyre::eyre!("String gaven for variant was invalid"))?
)}
//


	pub fn name(&mut self) 
-> Result<crate::bungee::api::chat::BaseComponent<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getName","()Lnet/md_5/bungee/api/chat/BaseComponent;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::bungee::api::chat::BaseComponent::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
//


	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
//


	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
//


	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
//


	pub fn set_name(&mut self,arg0: impl Into<crate::bungee::api::chat::BaseComponent<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setName","(Lnet/md_5/bungee/api/chat/BaseComponent;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn id(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getId","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
//


	pub fn get_type(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getType","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
//


	pub fn assert_action(&mut self,arg0: impl Into<crate::bungee::api::chat::HoverEventAction<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"assertAction","(Lnet/md_5/bungee/api/chat/HoverEvent$Action;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?.into());
let val_2 = jni::objects::JValueGen::Int(arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?.into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
unsafe {jni::objects::JClass::from_raw(res.as_jni().l)}
)}
//


	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
}
impl<'mc> Into<crate::bungee::api::chat::hover::content::Content<'mc>> for Entity<'mc>{

fn into(self) -> crate::bungee::api::chat::hover::content::Content<'mc> {

crate::bungee::api::chat::hover::content::Content::from_raw(&self.jni_ref(), self.1).expect("Error converting Entity into crate::bungee::api::chat::hover::content::Content")

   }
}

pub struct ItemSerializer<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ItemSerializer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ItemSerializer<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate ItemSerializer from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "net/md_5/bungee/api/chat/hover/content/ItemSerializer")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a ItemSerializer object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::bungee::api::chat::hover::content::ItemSerializer<'mc>, Box<dyn std::error::Error>>

{let cls = jni.find_class("net/md_5/bungee/api/chat/hover/content/ItemSerializer"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
"()V",&[]);
let res = jni.translate_error_no_gen(res)?;
crate::bungee::api::chat::hover::content::ItemSerializer::from_raw(&jni,res
)}
//


	pub fn serialize_with_item(&mut self,arg0: jni::objects::JObject<'mc>,arg1: jni::objects::JObject<'mc>,arg2: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let val_2 = arg1;
let val_3 = arg2.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?;
let res = self.jni_ref().call_method(&self.jni_object(),"serialize","(Ljava/lang/Object;Ljava/lang/reflect/Type;Lcom/google/gson/JsonSerializationContext;)Lcom/google/gson/JsonElement;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l()?)}
//


	pub fn deserialize_with_json_element(&mut self,arg0: jni::objects::JObject<'mc>,arg1: jni::objects::JObject<'mc>,arg2: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<crate::bungee::api::chat::hover::content::Item<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let val_2 = arg1;
let val_3 = arg2.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?;
let res = self.jni_ref().call_method(&self.jni_object(),"deserialize","(Lcom/google/gson/JsonElement;Ljava/lang/reflect/Type;Lcom/google/gson/JsonDeserializationContext;)Lnet/md_5/bungee/api/chat/hover/content/Item;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::bungee::api::chat::hover::content::Item::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
//


	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?.into());
let val_2 = jni::objects::JValueGen::Int(arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?.into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
//


	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
//


	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
//


	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
unsafe {jni::objects::JClass::from_raw(res.as_jni().l)}
)}
//


	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
}
impl<'mc> Into<jni::objects::JObject<'mc>> for ItemSerializer<'mc>{

fn into(self) -> jni::objects::JObject<'mc> {

self.1
   }
}

pub struct TextSerializer<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for TextSerializer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> TextSerializer<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate TextSerializer from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "net/md_5/bungee/api/chat/hover/content/TextSerializer")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a TextSerializer object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::bungee::api::chat::hover::content::TextSerializer<'mc>, Box<dyn std::error::Error>>

{let cls = jni.find_class("net/md_5/bungee/api/chat/hover/content/TextSerializer"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
"()V",&[]);
let res = jni.translate_error_no_gen(res)?;
crate::bungee::api::chat::hover::content::TextSerializer::from_raw(&jni,res
)}
//


	pub fn serialize_with_text(&mut self,arg0: jni::objects::JObject<'mc>,arg1: jni::objects::JObject<'mc>,arg2: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let val_2 = arg1;
let val_3 = arg2.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?;
let res = self.jni_ref().call_method(&self.jni_object(),"serialize","(Ljava/lang/Object;Ljava/lang/reflect/Type;Lcom/google/gson/JsonSerializationContext;)Lcom/google/gson/JsonElement;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l()?)}
//


	pub fn deserialize_with_json_element(&mut self,arg0: jni::objects::JObject<'mc>,arg1: jni::objects::JObject<'mc>,arg2: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<crate::bungee::api::chat::hover::content::Text<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let val_2 = arg1;
let val_3 = arg2.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?;
let res = self.jni_ref().call_method(&self.jni_object(),"deserialize","(Lcom/google/gson/JsonElement;Ljava/lang/reflect/Type;Lcom/google/gson/JsonDeserializationContext;)Lnet/md_5/bungee/api/chat/hover/content/Text;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::bungee::api::chat::hover::content::Text::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
//


	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?.into());
let val_2 = jni::objects::JValueGen::Int(arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?.into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
//


	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
//


	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
//


	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
unsafe {jni::objects::JClass::from_raw(res.as_jni().l)}
)}
//


	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
}
impl<'mc> Into<jni::objects::JObject<'mc>> for TextSerializer<'mc>{

fn into(self) -> jni::objects::JObject<'mc> {

self.1
   }
}

pub struct Content<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Content<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Content<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate Content from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "net/md_5/bungee/api/chat/hover/content/Content")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a Content object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::bungee::api::chat::hover::content::Content<'mc>, Box<dyn std::error::Error>>

{let cls = jni.find_class("net/md_5/bungee/api/chat/hover/content/Content"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
"()V",&[]);
let res = jni.translate_error_no_gen(res)?;
crate::bungee::api::chat::hover::content::Content::from_raw(&jni,res
)}
//


	pub fn assert_action(&mut self,arg0: impl Into<crate::bungee::api::chat::HoverEventAction<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"assertAction","(Lnet/md_5/bungee/api/chat/HoverEvent$Action;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn required_action(&mut self) 
-> Result<crate::bungee::api::chat::HoverEventAction<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"requiredAction","()Lnet/md_5/bungee/api/chat/HoverEvent$Action;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]); let variant = self.jni_ref().translate_error(variant)?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::bungee::api::chat::HoverEventAction::from_raw(&self.jni_ref(),raw_obj
, crate::bungee::api::chat::HoverEventAction::from_string(variant_str).ok_or(eyre::eyre!("String gaven for variant was invalid"))?
)}
//


	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
//


	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
//


	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
//


	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?.into());
let val_2 = jni::objects::JValueGen::Int(arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?.into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
unsafe {jni::objects::JClass::from_raw(res.as_jni().l)}
)}
//


	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
}

pub struct Text<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Text<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Text<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate Text from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "net/md_5/bungee/api/chat/hover/content/Text")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a Text object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_base_components(jni: &blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<String>>) 
-> Result<crate::bungee::api::chat::hover::content::Text<'mc>, Box<dyn std::error::Error>>

{let val_1 = jni::objects::JObject::from(jni.new_string(arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?.into())?);
let cls = jni.find_class("net/md_5/bungee/api/chat/hover/content/Text"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
"(Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_1)]);
let res = jni.translate_error_no_gen(res)?;
crate::bungee::api::chat::hover::content::Text::from_raw(&jni,res
)}
//


	pub fn required_action(&mut self) 
-> Result<crate::bungee::api::chat::HoverEventAction<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"requiredAction","()Lnet/md_5/bungee/api/chat/HoverEvent$Action;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]); let variant = self.jni_ref().translate_error(variant)?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::bungee::api::chat::HoverEventAction::from_raw(&self.jni_ref(),raw_obj
, crate::bungee::api::chat::HoverEventAction::from_string(variant_str).ok_or(eyre::eyre!("String gaven for variant was invalid"))?
)}
//


	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
//


	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
//


	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
//


	pub fn value(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getValue","()Ljava/lang/Object;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l()?)}
//


	pub fn assert_action(&mut self,arg0: impl Into<crate::bungee::api::chat::HoverEventAction<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"assertAction","(Lnet/md_5/bungee/api/chat/HoverEvent$Action;)V",&[jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?.into());
let val_2 = jni::objects::JValueGen::Int(arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?.into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
unsafe {jni::objects::JClass::from_raw(res.as_jni().l)}
)}
//


	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
}
impl<'mc> Into<crate::bungee::api::chat::hover::content::Content<'mc>> for Text<'mc>{

fn into(self) -> crate::bungee::api::chat::hover::content::Content<'mc> {

crate::bungee::api::chat::hover::content::Content::from_raw(&self.jni_ref(), self.1).expect("Error converting Text into crate::bungee::api::chat::hover::content::Content")

   }
}

pub struct EntitySerializer<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntitySerializer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntitySerializer<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntitySerializer from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "net/md_5/bungee/api/chat/hover/content/EntitySerializer")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntitySerializer object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::bungee::api::chat::hover::content::EntitySerializer<'mc>, Box<dyn std::error::Error>>

{let cls = jni.find_class("net/md_5/bungee/api/chat/hover/content/EntitySerializer"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
"()V",&[]);
let res = jni.translate_error_no_gen(res)?;
crate::bungee::api::chat::hover::content::EntitySerializer::from_raw(&jni,res
)}
//


	pub fn serialize_with_entity(&mut self,arg0: jni::objects::JObject<'mc>,arg1: jni::objects::JObject<'mc>,arg2: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let val_2 = arg1;
let val_3 = arg2.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?;
let res = self.jni_ref().call_method(&self.jni_object(),"serialize","(Ljava/lang/Object;Ljava/lang/reflect/Type;Lcom/google/gson/JsonSerializationContext;)Lcom/google/gson/JsonElement;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l()?)}
//


	pub fn deserialize_with_json_element(&mut self,arg0: jni::objects::JObject<'mc>,arg1: jni::objects::JObject<'mc>,arg2: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<crate::bungee::api::chat::hover::content::Entity<'mc>, Box<dyn std::error::Error>>

{let val_1 = arg0;
let val_2 = arg1;
let val_3 = arg2.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?;
let res = self.jni_ref().call_method(&self.jni_object(),"deserialize","(Lcom/google/gson/JsonElement;Ljava/lang/reflect/Type;Lcom/google/gson/JsonDeserializationContext;)Lnet/md_5/bungee/api/chat/hover/content/Entity;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::bungee::api::chat::hover::content::Entity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
//


	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_1 = jni::objects::JValueGen::Long(arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?.into());
let val_2 = jni::objects::JValueGen::Int(arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?.into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_1 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
//


	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
//


	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
//


	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
unsafe {jni::objects::JClass::from_raw(res.as_jni().l)}
)}
//


	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
}
impl<'mc> Into<jni::objects::JObject<'mc>> for EntitySerializer<'mc>{

fn into(self) -> jni::objects::JObject<'mc> {

self.1
   }
}
