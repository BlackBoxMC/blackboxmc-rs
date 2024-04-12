#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;/*org/bukkit/command/defaults/mod.rs*/

#[repr(C)]
pub struct ReloadCommand<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ReloadCommand<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ReloadCommand<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ReloadCommand from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/command/defaults/ReloadCommand")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ReloadCommand object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ReloadCommandTrait<'mc> for ReloadCommand<'mc> {}
pub trait ReloadCommandTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,name: impl Into<String>) 
-> Result<crate::command::defaults::ReloadCommand<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(name.into())?));
let cls = jni.find_class("org/bukkit/command/defaults/ReloadCommand"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::command::defaults::ReloadCommand::from_raw(&jni,res
)}

	fn execute(&self,sender: impl Into<crate::command::CommandSender<'mc>>,current_alias: impl Into<String>,val_args: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/command/CommandSender;Ljava/lang/String;Ljava/lang/String;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(sender.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(current_alias.into())?));
let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(val_args.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"execute",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn tab_complete(&self,sender: impl Into<crate::command::CommandSender<'mc>>,alias: impl Into<String>,val_args: impl Into<String>) 
-> Result<Vec<String>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/command/CommandSender;Ljava/lang/String;Ljava/lang/String;)Ljava/util/List;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(sender.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(alias.into())?));
let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(val_args.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"tabComplete",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?.to_string_lossy().to_string());
};
Ok(
new_vec
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::command::defaults::BukkitCommand<'mc>> for ReloadCommand<'mc>{

fn into(self) -> crate::command::defaults::BukkitCommand<'mc> {

crate::command::defaults::BukkitCommand::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting ReloadCommand into crate::command::defaults::BukkitCommand")

   }
}
impl<'mc> crate::command::defaults::BukkitCommandTrait<'mc> for ReloadCommand<'mc> {}
#[repr(C)]
pub struct TimingsCommand<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for TimingsCommand<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for TimingsCommand<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate TimingsCommand from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/command/defaults/TimingsCommand")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a TimingsCommand object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> TimingsCommandTrait<'mc> for TimingsCommand<'mc> {}
pub trait TimingsCommandTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,name: impl Into<String>) 
-> Result<crate::command::defaults::TimingsCommand<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(name.into())?));
let cls = jni.find_class("org/bukkit/command/defaults/TimingsCommand"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::command::defaults::TimingsCommand::from_raw(&jni,res
)}

	fn execute(&self,sender: impl Into<crate::command::CommandSender<'mc>>,current_alias: impl Into<String>,val_args: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/command/CommandSender;Ljava/lang/String;Ljava/lang/String;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(sender.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(current_alias.into())?));
let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(val_args.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"execute",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn tab_complete(&self,sender: impl Into<crate::command::CommandSender<'mc>>,alias: impl Into<String>,val_args: impl Into<String>) 
-> Result<Vec<String>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/command/CommandSender;Ljava/lang/String;Ljava/lang/String;)Ljava/util/List;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(sender.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(alias.into())?));
let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(val_args.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"tabComplete",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?.to_string_lossy().to_string());
};
Ok(
new_vec
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::command::defaults::BukkitCommand<'mc>> for TimingsCommand<'mc>{

fn into(self) -> crate::command::defaults::BukkitCommand<'mc> {

crate::command::defaults::BukkitCommand::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting TimingsCommand into crate::command::defaults::BukkitCommand")

   }
}
impl<'mc> crate::command::defaults::BukkitCommandTrait<'mc> for TimingsCommand<'mc> {}
#[repr(C)]
pub struct PluginsCommand<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PluginsCommand<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PluginsCommand<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PluginsCommand from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/command/defaults/PluginsCommand")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PluginsCommand object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PluginsCommandTrait<'mc> for PluginsCommand<'mc> {}
pub trait PluginsCommandTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,name: impl Into<String>) 
-> Result<crate::command::defaults::PluginsCommand<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(name.into())?));
let cls = jni.find_class("org/bukkit/command/defaults/PluginsCommand"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::command::defaults::PluginsCommand::from_raw(&jni,res
)}

	fn execute(&self,sender: impl Into<crate::command::CommandSender<'mc>>,current_alias: impl Into<String>,val_args: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/command/CommandSender;Ljava/lang/String;Ljava/lang/String;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(sender.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(current_alias.into())?));
let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(val_args.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"execute",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn tab_complete(&self,sender: impl Into<crate::command::CommandSender<'mc>>,alias: impl Into<String>,val_args: impl Into<String>) 
-> Result<Vec<String>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/command/CommandSender;Ljava/lang/String;Ljava/lang/String;)Ljava/util/List;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(sender.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(alias.into())?));
let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(val_args.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"tabComplete",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?.to_string_lossy().to_string());
};
Ok(
new_vec
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::command::defaults::BukkitCommand<'mc>> for PluginsCommand<'mc>{

fn into(self) -> crate::command::defaults::BukkitCommand<'mc> {

crate::command::defaults::BukkitCommand::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PluginsCommand into crate::command::defaults::BukkitCommand")

   }
}
impl<'mc> crate::command::defaults::BukkitCommandTrait<'mc> for PluginsCommand<'mc> {}
#[repr(C)]
pub struct VersionCommand<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for VersionCommand<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for VersionCommand<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate VersionCommand from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/command/defaults/VersionCommand")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a VersionCommand object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> VersionCommandTrait<'mc> for VersionCommand<'mc> {}
pub trait VersionCommandTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,name: impl Into<String>) 
-> Result<crate::command::defaults::VersionCommand<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(name.into())?));
let cls = jni.find_class("org/bukkit/command/defaults/VersionCommand"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::command::defaults::VersionCommand::from_raw(&jni,res
)}

	fn execute(&self,sender: impl Into<crate::command::CommandSender<'mc>>,current_alias: impl Into<String>,val_args: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/command/CommandSender;Ljava/lang/String;Ljava/lang/String;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(sender.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(current_alias.into())?));
let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(val_args.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"execute",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn tab_complete(&self,sender: impl Into<crate::command::CommandSender<'mc>>,alias: impl Into<String>,val_args: impl Into<String>) 
-> Result<Vec<String>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/command/CommandSender;Ljava/lang/String;Ljava/lang/String;)Ljava/util/List;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(sender.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(alias.into())?));
let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(val_args.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"tabComplete",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?.to_string_lossy().to_string());
};
Ok(
new_vec
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::command::defaults::BukkitCommand<'mc>> for VersionCommand<'mc>{

fn into(self) -> crate::command::defaults::BukkitCommand<'mc> {

crate::command::defaults::BukkitCommand::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting VersionCommand into crate::command::defaults::BukkitCommand")

   }
}
impl<'mc> crate::command::defaults::BukkitCommandTrait<'mc> for VersionCommand<'mc> {}
#[repr(C)]
pub struct BukkitCommand<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BukkitCommand<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BukkitCommand<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BukkitCommand from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/command/defaults/BukkitCommand")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BukkitCommand object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BukkitCommandTrait<'mc> for BukkitCommand<'mc> {}
pub trait BukkitCommandTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::command::Command<'mc>> for BukkitCommand<'mc>{

fn into(self) -> crate::command::Command<'mc> {

crate::command::Command::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BukkitCommand into crate::command::Command")

   }
}
impl<'mc> crate::command::CommandTrait<'mc> for BukkitCommand<'mc> {}
#[repr(C)]
pub struct HelpCommand<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for HelpCommand<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for HelpCommand<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate HelpCommand from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/command/defaults/HelpCommand")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a HelpCommand object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> HelpCommandTrait<'mc> for HelpCommand<'mc> {}
pub trait HelpCommandTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::command::defaults::HelpCommand<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()V");
let cls = jni.find_class("org/bukkit/command/defaults/HelpCommand"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![]);
let res = 
jni.translate_error_no_gen(res)?;
crate::command::defaults::HelpCommand::from_raw(&jni,res
)}

	fn execute(&self,sender: impl Into<crate::command::CommandSender<'mc>>,current_alias: impl Into<String>,val_args: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/command/CommandSender;Ljava/lang/String;Ljava/lang/String;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(sender.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(current_alias.into())?));
let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(val_args.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"execute",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn tab_complete(&self,sender: impl Into<crate::command::CommandSender<'mc>>,alias: impl Into<String>,val_args: impl Into<String>) 
-> Result<Vec<String>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/command/CommandSender;Ljava/lang/String;Ljava/lang/String;)Ljava/util/List;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(sender.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(alias.into())?));
let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(val_args.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"tabComplete",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?.to_string_lossy().to_string());
};
Ok(
new_vec
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::command::defaults::BukkitCommand<'mc>> for HelpCommand<'mc>{

fn into(self) -> crate::command::defaults::BukkitCommand<'mc> {

crate::command::defaults::BukkitCommand::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting HelpCommand into crate::command::defaults::BukkitCommand")

   }
}
impl<'mc> crate::command::defaults::BukkitCommandTrait<'mc> for HelpCommand<'mc> {}
