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

{let sig = String::from("(Ljava/lang/String;ILnet/md_5/bungee/api/chat/ItemTag;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(arg0.into())?));
let val_2 = jni::objects::JValueGen::Int(arg1.into());
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())});
let cls = jni.find_class("net/md_5/bungee/api/chat/hover/content/Item"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = jni.translate_error_no_gen(res)?;
crate::bungee::api::chat::hover::content::Item::from_raw(&jni,res
)}
//


	pub fn count(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getCount",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
//


	pub fn tag(&mut self) 
-> Result<crate::bungee::api::chat::ItemTag<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lnet/md_5/bungee/api/chat/ItemTag;");
let res = self.jni_ref().call_method(&self.jni_object(),"getTag",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::bungee::api::chat::ItemTag::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
//


	pub fn set_id(&mut self,arg0: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(arg0.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setId",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn set_tag(&mut self,arg0: impl Into<crate::bungee::api::chat::ItemTag<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lnet/md_5/bungee/api/chat/ItemTag;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setTag",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn required_action(&mut self) 
-> Result<crate::bungee::api::chat::HoverEventAction<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lnet/md_5/bungee/api/chat/HoverEvent$Action;");
let res = self.jni_ref().call_method(&self.jni_object(),"requiredAction",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]); let variant = self.jni_ref().translate_error(variant)?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::bungee::api::chat::HoverEventAction::from_raw(&self.jni_ref(),raw_obj
, crate::bungee::api::chat::HoverEventAction::from_string(variant_str).ok_or(eyre::eyre!("String gaven for variant was invalid"))?
)}
//


	pub fn set_count(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCount",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)Z");
let val_1 = jni::objects::JValueGen::Object(arg0);
let res = self.jni_ref().call_method(&self.jni_object(),"equals",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
//


#[doc(hidden)]
	pub fn internal_to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
//


	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"hashCode",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
//


	pub fn id(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getId",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
//


	pub fn assert_action(&mut self,arg0: impl Into<crate::bungee::api::chat::HoverEventAction<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lnet/md_5/bungee/api/chat/HoverEvent$Action;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"assertAction",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = arg0 {
sig += "J";
let val_1 = jni::objects::JValueGen::Long(a.into());
args.push(val_1);
}
if let Some(a) = arg1 {
sig += "I";
let val_2 = jni::objects::JValueGen::Int(a.into());
args.push(val_2);
}
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"wait",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/Class;");
let res = self.jni_ref().call_method(&self.jni_object(),"getClass",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
unsafe {jni::objects::JClass::from_raw(res.as_jni().l)}
)}
//


	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"notify",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
}

        impl<'mc> std::string::ToString for Item<'mc> {
            fn to_string(&self) -> String {
                match &self.internal_to_string() {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Item.toString: {}",
                        err
                    ),
                }
            }
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

{let sig = String::from("(Ljava/lang/String;Ljava/lang/String;Lnet/md_5/bungee/api/chat/BaseComponent;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(arg0.into())?));
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(arg1.into())?));
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())});
let cls = jni.find_class("net/md_5/bungee/api/chat/hover/content/Entity"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = jni.translate_error_no_gen(res)?;
crate::bungee::api::chat::hover::content::Entity::from_raw(&jni,res
)}
//


	pub fn set_id(&mut self,arg0: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(arg0.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setId",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn set_type(&mut self,arg0: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(arg0.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setType",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn required_action(&mut self) 
-> Result<crate::bungee::api::chat::HoverEventAction<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lnet/md_5/bungee/api/chat/HoverEvent$Action;");
let res = self.jni_ref().call_method(&self.jni_object(),"requiredAction",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]); let variant = self.jni_ref().translate_error(variant)?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::bungee::api::chat::HoverEventAction::from_raw(&self.jni_ref(),raw_obj
, crate::bungee::api::chat::HoverEventAction::from_string(variant_str).ok_or(eyre::eyre!("String gaven for variant was invalid"))?
)}
//


	pub fn name(&mut self) 
-> Result<crate::bungee::api::chat::BaseComponent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lnet/md_5/bungee/api/chat/BaseComponent;");
let res = self.jni_ref().call_method(&self.jni_object(),"getName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::bungee::api::chat::BaseComponent::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
//


	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)Z");
let val_1 = jni::objects::JValueGen::Object(arg0);
let res = self.jni_ref().call_method(&self.jni_object(),"equals",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
//


#[doc(hidden)]
	pub fn internal_to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
//


	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"hashCode",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
//


	pub fn set_name(&mut self,arg0: impl Into<crate::bungee::api::chat::BaseComponent<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lnet/md_5/bungee/api/chat/BaseComponent;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setName",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn id(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getId",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
//


	pub fn get_type(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
//


	pub fn assert_action(&mut self,arg0: impl Into<crate::bungee::api::chat::HoverEventAction<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lnet/md_5/bungee/api/chat/HoverEvent$Action;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"assertAction",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = arg0 {
sig += "J";
let val_1 = jni::objects::JValueGen::Long(a.into());
args.push(val_1);
}
if let Some(a) = arg1 {
sig += "I";
let val_2 = jni::objects::JValueGen::Int(a.into());
args.push(val_2);
}
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"wait",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/Class;");
let res = self.jni_ref().call_method(&self.jni_object(),"getClass",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
unsafe {jni::objects::JClass::from_raw(res.as_jni().l)}
)}
//


	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"notify",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
}

        impl<'mc> std::string::ToString for Entity<'mc> {
            fn to_string(&self) -> String {
                match &self.internal_to_string() {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Entity.toString: {}",
                        err
                    ),
                }
            }
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

{let sig = String::from("()V");
let cls = jni.find_class("net/md_5/bungee/api/chat/hover/content/ItemSerializer"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![]);
let res = jni.translate_error_no_gen(res)?;
crate::bungee::api::chat::hover::content::ItemSerializer::from_raw(&jni,res
)}
//


	pub fn serialize_with_item(&mut self,arg0: jni::objects::JObject<'mc>,arg1: jni::objects::JObject<'mc>,arg2: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/Object;";
let val_1 = jni::objects::JValueGen::Object(arg0);
args.push(val_1);
sig += "Ljava/lang/reflect/Type;";
let val_2 = jni::objects::JValueGen::Object(arg1);
args.push(val_2);
if let Some(a) = arg2 {
sig += "Lcom/google/gson/JsonSerializationContext;";
let val_3 = jni::objects::JValueGen::Object(a);
args.push(val_3);
}
sig += ")Lcom/google/gson/JsonElement;";
let res = self.jni_ref().call_method(&self.jni_object(),"serialize",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l()?)}
//


	pub fn deserialize_with_json_element(&mut self,arg0: jni::objects::JObject<'mc>,arg1: jni::objects::JObject<'mc>,arg2: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<crate::bungee::api::chat::hover::content::Item<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lcom/google/gson/JsonElement;";
let val_1 = jni::objects::JValueGen::Object(arg0);
args.push(val_1);
sig += "Ljava/lang/reflect/Type;";
let val_2 = jni::objects::JValueGen::Object(arg1);
args.push(val_2);
if let Some(a) = arg2 {
sig += "Lcom/google/gson/JsonDeserializationContext;";
let val_3 = jni::objects::JValueGen::Object(a);
args.push(val_3);
}
sig += ")Lnet/md_5/bungee/api/chat/hover/content/Item;";
let res = self.jni_ref().call_method(&self.jni_object(),"deserialize",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::bungee::api::chat::hover::content::Item::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
//


	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = arg0 {
sig += "J";
let val_1 = jni::objects::JValueGen::Long(a.into());
args.push(val_1);
}
if let Some(a) = arg1 {
sig += "I";
let val_2 = jni::objects::JValueGen::Int(a.into());
args.push(val_2);
}
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"wait",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)Z");
let val_1 = jni::objects::JValueGen::Object(arg0);
let res = self.jni_ref().call_method(&self.jni_object(),"equals",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
//


#[doc(hidden)]
	pub fn internal_to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
//


	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"hashCode",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
//


	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/Class;");
let res = self.jni_ref().call_method(&self.jni_object(),"getClass",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
unsafe {jni::objects::JClass::from_raw(res.as_jni().l)}
)}
//


	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"notify",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
}

        impl<'mc> std::string::ToString for ItemSerializer<'mc> {
            fn to_string(&self) -> String {
                match &self.internal_to_string() {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling ItemSerializer.toString: {}",
                        err
                    ),
                }
            }
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

{let sig = String::from("()V");
let cls = jni.find_class("net/md_5/bungee/api/chat/hover/content/TextSerializer"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![]);
let res = jni.translate_error_no_gen(res)?;
crate::bungee::api::chat::hover::content::TextSerializer::from_raw(&jni,res
)}
//


	pub fn serialize_with_text(&mut self,arg0: jni::objects::JObject<'mc>,arg1: jni::objects::JObject<'mc>,arg2: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/Object;";
let val_1 = jni::objects::JValueGen::Object(arg0);
args.push(val_1);
sig += "Ljava/lang/reflect/Type;";
let val_2 = jni::objects::JValueGen::Object(arg1);
args.push(val_2);
if let Some(a) = arg2 {
sig += "Lcom/google/gson/JsonSerializationContext;";
let val_3 = jni::objects::JValueGen::Object(a);
args.push(val_3);
}
sig += ")Lcom/google/gson/JsonElement;";
let res = self.jni_ref().call_method(&self.jni_object(),"serialize",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l()?)}
//


	pub fn deserialize_with_json_element(&mut self,arg0: jni::objects::JObject<'mc>,arg1: jni::objects::JObject<'mc>,arg2: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<crate::bungee::api::chat::hover::content::Text<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lcom/google/gson/JsonElement;";
let val_1 = jni::objects::JValueGen::Object(arg0);
args.push(val_1);
sig += "Ljava/lang/reflect/Type;";
let val_2 = jni::objects::JValueGen::Object(arg1);
args.push(val_2);
if let Some(a) = arg2 {
sig += "Lcom/google/gson/JsonDeserializationContext;";
let val_3 = jni::objects::JValueGen::Object(a);
args.push(val_3);
}
sig += ")Lnet/md_5/bungee/api/chat/hover/content/Text;";
let res = self.jni_ref().call_method(&self.jni_object(),"deserialize",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::bungee::api::chat::hover::content::Text::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
//


	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = arg0 {
sig += "J";
let val_1 = jni::objects::JValueGen::Long(a.into());
args.push(val_1);
}
if let Some(a) = arg1 {
sig += "I";
let val_2 = jni::objects::JValueGen::Int(a.into());
args.push(val_2);
}
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"wait",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)Z");
let val_1 = jni::objects::JValueGen::Object(arg0);
let res = self.jni_ref().call_method(&self.jni_object(),"equals",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
//


#[doc(hidden)]
	pub fn internal_to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
//


	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"hashCode",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
//


	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/Class;");
let res = self.jni_ref().call_method(&self.jni_object(),"getClass",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
unsafe {jni::objects::JClass::from_raw(res.as_jni().l)}
)}
//


	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"notify",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
}

        impl<'mc> std::string::ToString for TextSerializer<'mc> {
            fn to_string(&self) -> String {
                match &self.internal_to_string() {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling TextSerializer.toString: {}",
                        err
                    ),
                }
            }
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

{let sig = String::from("()V");
let cls = jni.find_class("net/md_5/bungee/api/chat/hover/content/Content"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![]);
let res = jni.translate_error_no_gen(res)?;
crate::bungee::api::chat::hover::content::Content::from_raw(&jni,res
)}
//


	pub fn assert_action(&mut self,arg0: impl Into<crate::bungee::api::chat::HoverEventAction<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lnet/md_5/bungee/api/chat/HoverEvent$Action;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"assertAction",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn required_action(&mut self) 
-> Result<crate::bungee::api::chat::HoverEventAction<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lnet/md_5/bungee/api/chat/HoverEvent$Action;");
let res = self.jni_ref().call_method(&self.jni_object(),"requiredAction",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]); let variant = self.jni_ref().translate_error(variant)?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::bungee::api::chat::HoverEventAction::from_raw(&self.jni_ref(),raw_obj
, crate::bungee::api::chat::HoverEventAction::from_string(variant_str).ok_or(eyre::eyre!("String gaven for variant was invalid"))?
)}
//


	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)Z");
let val_1 = jni::objects::JValueGen::Object(arg0);
let res = self.jni_ref().call_method(&self.jni_object(),"equals",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
//


#[doc(hidden)]
	pub fn internal_to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
//


	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"hashCode",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
//


	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = arg0 {
sig += "J";
let val_1 = jni::objects::JValueGen::Long(a.into());
args.push(val_1);
}
if let Some(a) = arg1 {
sig += "I";
let val_2 = jni::objects::JValueGen::Int(a.into());
args.push(val_2);
}
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"wait",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/Class;");
let res = self.jni_ref().call_method(&self.jni_object(),"getClass",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
unsafe {jni::objects::JClass::from_raw(res.as_jni().l)}
)}
//


	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"notify",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
}

        impl<'mc> std::string::ToString for Content<'mc> {
            fn to_string(&self) -> String {
                match &self.internal_to_string() {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Content.toString: {}",
                        err
                    ),
                }
            }
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

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = arg0 {
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(a.into())?));
args.push(val_1);
}
sig += ")V";
let cls = jni.find_class("net/md_5/bungee/api/chat/hover/content/Text"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = jni.translate_error_no_gen(res)?;
crate::bungee::api::chat::hover::content::Text::from_raw(&jni,res
)}
//


	pub fn required_action(&mut self) 
-> Result<crate::bungee::api::chat::HoverEventAction<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lnet/md_5/bungee/api/chat/HoverEvent$Action;");
let res = self.jni_ref().call_method(&self.jni_object(),"requiredAction",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]); let variant = self.jni_ref().translate_error(variant)?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::bungee::api::chat::HoverEventAction::from_raw(&self.jni_ref(),raw_obj
, crate::bungee::api::chat::HoverEventAction::from_string(variant_str).ok_or(eyre::eyre!("String gaven for variant was invalid"))?
)}
//


	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)Z");
let val_1 = jni::objects::JValueGen::Object(arg0);
let res = self.jni_ref().call_method(&self.jni_object(),"equals",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
//


#[doc(hidden)]
	pub fn internal_to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
//


	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"hashCode",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
//


	pub fn value(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/Object;");
let res = self.jni_ref().call_method(&self.jni_object(),"getValue",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l()?)}
//


	pub fn assert_action(&mut self,arg0: impl Into<crate::bungee::api::chat::HoverEventAction<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lnet/md_5/bungee/api/chat/HoverEvent$Action;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"assertAction",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = arg0 {
sig += "J";
let val_1 = jni::objects::JValueGen::Long(a.into());
args.push(val_1);
}
if let Some(a) = arg1 {
sig += "I";
let val_2 = jni::objects::JValueGen::Int(a.into());
args.push(val_2);
}
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"wait",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/Class;");
let res = self.jni_ref().call_method(&self.jni_object(),"getClass",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
unsafe {jni::objects::JClass::from_raw(res.as_jni().l)}
)}
//


	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"notify",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
}

        impl<'mc> std::string::ToString for Text<'mc> {
            fn to_string(&self) -> String {
                match &self.internal_to_string() {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Text.toString: {}",
                        err
                    ),
                }
            }
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

{let sig = String::from("()V");
let cls = jni.find_class("net/md_5/bungee/api/chat/hover/content/EntitySerializer"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![]);
let res = jni.translate_error_no_gen(res)?;
crate::bungee::api::chat::hover::content::EntitySerializer::from_raw(&jni,res
)}
//


	pub fn serialize_with_entity(&mut self,arg0: jni::objects::JObject<'mc>,arg1: jni::objects::JObject<'mc>,arg2: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/Object;";
let val_1 = jni::objects::JValueGen::Object(arg0);
args.push(val_1);
sig += "Ljava/lang/reflect/Type;";
let val_2 = jni::objects::JValueGen::Object(arg1);
args.push(val_2);
if let Some(a) = arg2 {
sig += "Lcom/google/gson/JsonSerializationContext;";
let val_3 = jni::objects::JValueGen::Object(a);
args.push(val_3);
}
sig += ")Lcom/google/gson/JsonElement;";
let res = self.jni_ref().call_method(&self.jni_object(),"serialize",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.l()?)}
//


	pub fn deserialize_with_json_element(&mut self,arg0: jni::objects::JObject<'mc>,arg1: jni::objects::JObject<'mc>,arg2: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<crate::bungee::api::chat::hover::content::Entity<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lcom/google/gson/JsonElement;";
let val_1 = jni::objects::JValueGen::Object(arg0);
args.push(val_1);
sig += "Ljava/lang/reflect/Type;";
let val_2 = jni::objects::JValueGen::Object(arg1);
args.push(val_2);
if let Some(a) = arg2 {
sig += "Lcom/google/gson/JsonDeserializationContext;";
let val_3 = jni::objects::JValueGen::Object(a);
args.push(val_3);
}
sig += ")Lnet/md_5/bungee/api/chat/hover/content/Entity;";
let res = self.jni_ref().call_method(&self.jni_object(),"deserialize",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::bungee::api::chat::hover::content::Entity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
//


	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = arg0 {
sig += "J";
let val_1 = jni::objects::JValueGen::Long(a.into());
args.push(val_1);
}
if let Some(a) = arg1 {
sig += "I";
let val_2 = jni::objects::JValueGen::Int(a.into());
args.push(val_2);
}
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"wait",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)Z");
let val_1 = jni::objects::JValueGen::Object(arg0);
let res = self.jni_ref().call_method(&self.jni_object(),"equals",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
//


#[doc(hidden)]
	pub fn internal_to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
//


	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"hashCode",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
//


	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/Class;");
let res = self.jni_ref().call_method(&self.jni_object(),"getClass",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
unsafe {jni::objects::JClass::from_raw(res.as_jni().l)}
)}
//


	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"notify",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
//


	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
}

        impl<'mc> std::string::ToString for EntitySerializer<'mc> {
            fn to_string(&self) -> String {
                match &self.internal_to_string() {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling EntitySerializer.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<jni::objects::JObject<'mc>> for EntitySerializer<'mc>{

fn into(self) -> jni::objects::JObject<'mc> {

self.1
   }
}
