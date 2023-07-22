#![allow(deprecated)]
use crate::JNIRaw;
pub struct Item<'mc>(pub(crate) crate::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> crate::JNIRaw<'mc> for Item<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Item<'mc> {
	pub fn new(jni: crate::SharedJNIEnv<'mc>,arg0: impl Into<String>,arg1: i32,arg2: impl Into<crate::bungee::bungee::api::chat::ItemTag<'mc>>) 
-> Result<crate::bungee::bungee::api::chat::hover::content::Item<'mc>, Box<dyn std::error::Error>>

{let val_0 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
let val_1 = jni::objects::JValueGen::Int(arg1.into());
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().1.clone())};
let cls = &jni.find_class("net/md_5/bungee/api/chat/hover/content/Item")?;
let res = jni.new_object(cls,
"(Ljava/lang/String;ILnet/md_5/bungee/api/chat/ItemTag;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
let ret = {
crate::bungee::bungee::api::chat::hover::content::Item(jni,res
)
};
Ok(ret)}
pub fn from_raw(env: &crate::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate Item from null object.")
    .into());
}
let cls = env.jni.borrow().get_object_class(&obj)?;
let name_raw = env
    .call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
let oh = name_raw.l()?.into();
let what = env
    .get_string(&oh)?;
let name = what.to_string_lossy();
if !name.ends_with("Item") {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a Item object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj))
}
}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res =
self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)])?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res =
self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[])?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res =
self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[])?;
Ok(res.i().unwrap())}
	pub fn id(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res =
self.jni_ref().call_method(&self.jni_object(),"getId","()Ljava/lang/String;",&[])?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn count(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res =
self.jni_ref().call_method(&self.jni_object(),"getCount","()I",&[])?;
Ok(res.i().unwrap())}
	pub fn tag(&mut self) 
-> Result<crate::bungee::bungee::api::chat::ItemTag<'mc>, Box<dyn std::error::Error>>

{let res =
self.jni_ref().call_method(&self.jni_object(),"getTag","()Lnet/md_5/bungee/api/chat/ItemTag;",&[])?;
let ret = {
crate::bungee::bungee::api::chat::ItemTag(self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)
};
Ok(ret)}
	pub fn set_tag(&mut self,arg0: impl Into<crate::bungee::bungee::api::chat::ItemTag<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone())};
self.jni_ref().call_method(&self.jni_object(),"setTag","(Lnet/md_5/bungee/api/chat/ItemTag;)V",&[jni::objects::JValueGen::from(&val_0)])?;
Ok(())}
	pub fn set_count(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Int(arg0.into());
self.jni_ref().call_method(&self.jni_object(),"setCount","(I)V",&[jni::objects::JValueGen::from(&val_0)])?;
Ok(())}
	pub fn required_action(&mut self) 
-> Result<crate::bungee::bungee::api::chat::HoverEventAction<'mc>, Box<dyn std::error::Error>>

{let res =
self.jni_ref().call_method(&self.jni_object(),"requiredAction","()Lnet/md_5/bungee/api/chat/HoverEvent$Action;",&[])?;
let ret = {
crate::bungee::bungee::api::chat::HoverEventAction(self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)
};
Ok(ret)}
	pub fn set_id(&mut self,arg0: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
self.jni_ref().call_method(&self.jni_object(),"setId","(Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_0)])?;
Ok(())}
	pub fn assert_action(&mut self,arg0: impl Into<crate::bungee::bungee::api::chat::HoverEventAction<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone())};
self.jni_ref().call_method(&self.jni_object(),"assertAction","(Lnet/md_5/bungee/api/chat/HoverEvent$Action;)V",&[jni::objects::JValueGen::from(&val_0)])?;
Ok(())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res =
self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[])?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[])?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[])?;
Ok(())}
}
impl<'mc> Into<crate::bungee::bungee::api::chat::hover::content::Content<'mc>> for Item<'mc> {
   fn into(self) -> crate::bungee::bungee::api::chat::hover::content::Content<'mc> {
       crate::bungee::bungee::api::chat::hover::content::Content::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct Entity<'mc>(pub(crate) crate::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> crate::JNIRaw<'mc> for Entity<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Entity<'mc> {
	pub fn new(jni: crate::SharedJNIEnv<'mc>,arg0: impl Into<String>,arg1: impl Into<String>,arg2: impl Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>) 
-> Result<crate::bungee::bungee::api::chat::hover::content::Entity<'mc>, Box<dyn std::error::Error>>

{let val_0 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
let val_1 = jni::objects::JObject::from(jni.new_string(arg1.into()).unwrap());
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().1.clone())};
let cls = &jni.find_class("net/md_5/bungee/api/chat/hover/content/Entity")?;
let res = jni.new_object(cls,
"(Ljava/lang/String;Ljava/lang/String;Lnet/md_5/bungee/api/chat/BaseComponent;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
let ret = {
crate::bungee::bungee::api::chat::hover::content::Entity(jni,res
)
};
Ok(ret)}
pub fn from_raw(env: &crate::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate Entity from null object.")
    .into());
}
let cls = env.jni.borrow().get_object_class(&obj)?;
let name_raw = env
    .call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
let oh = name_raw.l()?.into();
let what = env
    .get_string(&oh)?;
let name = what.to_string_lossy();
if !name.ends_with("Entity") {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a Entity object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj))
}
}
	pub fn name(&mut self) 
-> Result<crate::bungee::bungee::api::chat::BaseComponent<'mc>, Box<dyn std::error::Error>>

{let res =
self.jni_ref().call_method(&self.jni_object(),"getName","()Lnet/md_5/bungee/api/chat/BaseComponent;",&[])?;
let ret = {
crate::bungee::bungee::api::chat::BaseComponent(self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)
};
Ok(ret)}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res =
self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)])?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res =
self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[])?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res =
self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[])?;
Ok(res.i().unwrap())}
	pub fn set_name(&mut self,arg0: impl Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone())};
self.jni_ref().call_method(&self.jni_object(),"setName","(Lnet/md_5/bungee/api/chat/BaseComponent;)V",&[jni::objects::JValueGen::from(&val_0)])?;
Ok(())}
	pub fn id(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res =
self.jni_ref().call_method(&self.jni_object(),"getId","()Ljava/lang/String;",&[])?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn get_type(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res =
self.jni_ref().call_method(&self.jni_object(),"getType","()Ljava/lang/String;",&[])?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn set_type(&mut self,arg0: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
self.jni_ref().call_method(&self.jni_object(),"setType","(Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_0)])?;
Ok(())}
	pub fn required_action(&mut self) 
-> Result<crate::bungee::bungee::api::chat::HoverEventAction<'mc>, Box<dyn std::error::Error>>

{let res =
self.jni_ref().call_method(&self.jni_object(),"requiredAction","()Lnet/md_5/bungee/api/chat/HoverEvent$Action;",&[])?;
let ret = {
crate::bungee::bungee::api::chat::HoverEventAction(self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)
};
Ok(ret)}
	pub fn set_id(&mut self,arg0: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
self.jni_ref().call_method(&self.jni_object(),"setId","(Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_0)])?;
Ok(())}
	pub fn assert_action(&mut self,arg0: impl Into<crate::bungee::bungee::api::chat::HoverEventAction<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone())};
self.jni_ref().call_method(&self.jni_object(),"assertAction","(Lnet/md_5/bungee/api/chat/HoverEvent$Action;)V",&[jni::objects::JValueGen::from(&val_0)])?;
Ok(())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res =
self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[])?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[])?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[])?;
Ok(())}
}
impl<'mc> Into<crate::bungee::bungee::api::chat::hover::content::Content<'mc>> for Entity<'mc> {
   fn into(self) -> crate::bungee::bungee::api::chat::hover::content::Content<'mc> {
       crate::bungee::bungee::api::chat::hover::content::Content::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct ItemSerializer<'mc>(pub(crate) crate::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> crate::JNIRaw<'mc> for ItemSerializer<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ItemSerializer<'mc> {
	pub fn new(jni: crate::SharedJNIEnv<'mc>) 
-> Result<crate::bungee::bungee::api::chat::hover::content::ItemSerializer<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("net/md_5/bungee/api/chat/hover/content/ItemSerializer")?;
let res = jni.new_object(cls,
"()V",&[])?;
let ret = {
crate::bungee::bungee::api::chat::hover::content::ItemSerializer(jni,res
)
};
Ok(ret)}
pub fn from_raw(env: &crate::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate ItemSerializer from null object.")
    .into());
}
let cls = env.jni.borrow().get_object_class(&obj)?;
let name_raw = env
    .call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
let oh = name_raw.l()?.into();
let what = env
    .get_string(&oh)?;
let name = what.to_string_lossy();
if !name.ends_with("ItemSerializer") {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a ItemSerializer object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj))
}
}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res =
self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)])?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res =
self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[])?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res =
self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[])?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res =
self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[])?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[])?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[])?;
Ok(())}
}
pub struct TextSerializer<'mc>(pub(crate) crate::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> crate::JNIRaw<'mc> for TextSerializer<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> TextSerializer<'mc> {
	pub fn new(jni: crate::SharedJNIEnv<'mc>) 
-> Result<crate::bungee::bungee::api::chat::hover::content::TextSerializer<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("net/md_5/bungee/api/chat/hover/content/TextSerializer")?;
let res = jni.new_object(cls,
"()V",&[])?;
let ret = {
crate::bungee::bungee::api::chat::hover::content::TextSerializer(jni,res
)
};
Ok(ret)}
pub fn from_raw(env: &crate::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate TextSerializer from null object.")
    .into());
}
let cls = env.jni.borrow().get_object_class(&obj)?;
let name_raw = env
    .call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
let oh = name_raw.l()?.into();
let what = env
    .get_string(&oh)?;
let name = what.to_string_lossy();
if !name.ends_with("TextSerializer") {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a TextSerializer object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj))
}
}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res =
self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)])?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res =
self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[])?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res =
self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[])?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res =
self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[])?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[])?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[])?;
Ok(())}
}
pub struct Content<'mc>(pub(crate) crate::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> crate::JNIRaw<'mc> for Content<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Content<'mc> {
	pub fn new(jni: crate::SharedJNIEnv<'mc>) 
-> Result<crate::bungee::bungee::api::chat::hover::content::Content<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("net/md_5/bungee/api/chat/hover/content/Content")?;
let res = jni.new_object(cls,
"()V",&[])?;
let ret = {
crate::bungee::bungee::api::chat::hover::content::Content(jni,res
)
};
Ok(ret)}
pub fn from_raw(env: &crate::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate Content from null object.")
    .into());
}
let cls = env.jni.borrow().get_object_class(&obj)?;
let name_raw = env
    .call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
let oh = name_raw.l()?.into();
let what = env
    .get_string(&oh)?;
let name = what.to_string_lossy();
if !name.ends_with("Content") {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a Content object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj))
}
}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res =
self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)])?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res =
self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[])?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res =
self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[])?;
Ok(res.i().unwrap())}
	pub fn assert_action(&mut self,arg0: impl Into<crate::bungee::bungee::api::chat::HoverEventAction<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone())};
self.jni_ref().call_method(&self.jni_object(),"assertAction","(Lnet/md_5/bungee/api/chat/HoverEvent$Action;)V",&[jni::objects::JValueGen::from(&val_0)])?;
Ok(())}
	pub fn required_action(&mut self) 
-> Result<crate::bungee::bungee::api::chat::HoverEventAction<'mc>, Box<dyn std::error::Error>>

{let res =
self.jni_ref().call_method(&self.jni_object(),"requiredAction","()Lnet/md_5/bungee/api/chat/HoverEvent$Action;",&[])?;
let ret = {
crate::bungee::bungee::api::chat::HoverEventAction(self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)
};
Ok(ret)}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res =
self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[])?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[])?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[])?;
Ok(())}
}
pub struct Text<'mc>(pub(crate) crate::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> crate::JNIRaw<'mc> for Text<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Text<'mc> {
	pub fn new_with_base_components(jni: crate::SharedJNIEnv<'mc>,arg0: std::option::Option<impl Into<String>>) 
-> Result<crate::bungee::bungee::api::chat::hover::content::Text<'mc>, Box<dyn std::error::Error>>

{let val_0 = jni::objects::JObject::from(jni.new_string(arg0.unwrap().into()).unwrap());
let cls = &jni.find_class("net/md_5/bungee/api/chat/hover/content/Text")?;
let res = jni.new_object(cls,
"(Ljava/lang/String;)V",&[jni::objects::JValueGen::from(&val_0)])?;
let ret = {
crate::bungee::bungee::api::chat::hover::content::Text(jni,res
)
};
Ok(ret)}
pub fn from_raw(env: &crate::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate Text from null object.")
    .into());
}
let cls = env.jni.borrow().get_object_class(&obj)?;
let name_raw = env
    .call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
let oh = name_raw.l()?.into();
let what = env
    .get_string(&oh)?;
let name = what.to_string_lossy();
if !name.ends_with("Text") {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a Text object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj))
}
}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res =
self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)])?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res =
self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[])?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res =
self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[])?;
Ok(res.i().unwrap())}
	pub fn value(&mut self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let res =
self.jni_ref().call_method(&self.jni_object(),"getValue","()Ljava/lang/Object;",&[])?;
Ok(res.l().unwrap())}
	pub fn required_action(&mut self) 
-> Result<crate::bungee::bungee::api::chat::HoverEventAction<'mc>, Box<dyn std::error::Error>>

{let res =
self.jni_ref().call_method(&self.jni_object(),"requiredAction","()Lnet/md_5/bungee/api/chat/HoverEvent$Action;",&[])?;
let ret = {
crate::bungee::bungee::api::chat::HoverEventAction(self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)
};
Ok(ret)}
	pub fn assert_action(&mut self,arg0: impl Into<crate::bungee::bungee::api::chat::HoverEventAction<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone())};
self.jni_ref().call_method(&self.jni_object(),"assertAction","(Lnet/md_5/bungee/api/chat/HoverEvent$Action;)V",&[jni::objects::JValueGen::from(&val_0)])?;
Ok(())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res =
self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[])?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[])?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[])?;
Ok(())}
}
impl<'mc> Into<crate::bungee::bungee::api::chat::hover::content::Content<'mc>> for Text<'mc> {
   fn into(self) -> crate::bungee::bungee::api::chat::hover::content::Content<'mc> {
       crate::bungee::bungee::api::chat::hover::content::Content::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct EntitySerializer<'mc>(pub(crate) crate::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> crate::JNIRaw<'mc> for EntitySerializer<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntitySerializer<'mc> {
	pub fn new(jni: crate::SharedJNIEnv<'mc>) 
-> Result<crate::bungee::bungee::api::chat::hover::content::EntitySerializer<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("net/md_5/bungee/api/chat/hover/content/EntitySerializer")?;
let res = jni.new_object(cls,
"()V",&[])?;
let ret = {
crate::bungee::bungee::api::chat::hover::content::EntitySerializer(jni,res
)
};
Ok(ret)}
pub fn from_raw(env: &crate::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate EntitySerializer from null object.")
    .into());
}
let cls = env.jni.borrow().get_object_class(&obj)?;
let name_raw = env
    .call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
let oh = name_raw.l()?.into();
let what = env
    .get_string(&oh)?;
let name = what.to_string_lossy();
if !name.ends_with("EntitySerializer") {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a EntitySerializer object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj))
}
}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res =
self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)])?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res =
self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[])?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res =
self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[])?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res =
self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[])?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[])?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[])?;
Ok(())}
}
