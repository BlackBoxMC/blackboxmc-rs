#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;/*org/bukkit/command/mod.rs*/

#[repr(C)]
pub struct BlockCommandSender<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockCommandSender<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockCommandSender<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockCommandSender from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/command/BlockCommandSender")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockCommandSender object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockCommandSenderTrait<'mc> for BlockCommandSender<'mc> {}
pub trait BlockCommandSenderTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Returns the block this command sender belongs to
	fn block(&self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/Block;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBlock",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::command::CommandSender<'mc>> for BlockCommandSender<'mc>{

fn into(self) -> crate::command::CommandSender<'mc> {

crate::command::CommandSender::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockCommandSender into crate::command::CommandSender")

   }
}
impl<'mc> crate::command::CommandSenderTrait<'mc> for BlockCommandSender<'mc> {}
#[repr(C)]
pub struct TabExecutor<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for TabExecutor<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for TabExecutor<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate TabExecutor from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/command/TabExecutor")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a TabExecutor object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> TabExecutorTrait<'mc> for TabExecutor<'mc> {}
pub trait TabExecutorTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::command::TabCompleter<'mc>> for TabExecutor<'mc>{

fn into(self) -> crate::command::TabCompleter<'mc> {

crate::command::TabCompleter::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting TabExecutor into crate::command::TabCompleter")

   }
}
impl<'mc> crate::command::TabCompleterTrait<'mc> for TabExecutor<'mc> {}
impl<'mc> Into<crate::command::CommandExecutor<'mc>> for TabExecutor<'mc>{

fn into(self) -> crate::command::CommandExecutor<'mc> {

crate::command::CommandExecutor::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting TabExecutor into crate::command::CommandExecutor")

   }
}
impl<'mc> crate::command::CommandExecutorTrait<'mc> for TabExecutor<'mc> {}
#[repr(C)]
pub struct PluginCommandYamlParser<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PluginCommandYamlParser<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PluginCommandYamlParser<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PluginCommandYamlParser from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/command/PluginCommandYamlParser")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PluginCommandYamlParser object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PluginCommandYamlParserTrait<'mc> for PluginCommandYamlParser<'mc> {}
pub trait PluginCommandYamlParserTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::command::PluginCommandYamlParser<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()V");
let cls = jni.find_class("org/bukkit/command/PluginCommandYamlParser"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![]);
let res = 
jni.translate_error_no_gen(res)?;
crate::command::PluginCommandYamlParser::from_raw(&jni,res
)}

	fn parse(jni: &blackboxmc_general::SharedJNIEnv<'mc>,plugin: impl Into<crate::plugin::Plugin<'mc>>) 
-> Result<Vec<crate::command::Command<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/Plugin;)Ljava/util/List;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/command/PluginCommandYamlParser"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"parse",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&jni, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::command::Command::from_raw(&jni,obj,)?);
};
Ok(
new_vec
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct FormattedCommandAlias<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for FormattedCommandAlias<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for FormattedCommandAlias<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate FormattedCommandAlias from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/command/FormattedCommandAlias")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a FormattedCommandAlias object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> FormattedCommandAliasTrait<'mc> for FormattedCommandAlias<'mc> {}
pub trait FormattedCommandAliasTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,alias: impl Into<String>,format_strings: impl Into<String>) 
-> Result<crate::command::FormattedCommandAlias<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(alias.into())?));
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(format_strings.into())?));
let cls = jni.find_class("org/bukkit/command/FormattedCommandAlias"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::command::FormattedCommandAlias::from_raw(&jni,res
)}

	fn execute(&self,sender: impl Into<crate::command::CommandSender<'mc>>,command_label: impl Into<String>,val_args: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/command/CommandSender;Ljava/lang/String;Ljava/lang/String;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(sender.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(command_label.into())?));
let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(val_args.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"execute",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
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
impl<'mc> Into<crate::command::Command<'mc>> for FormattedCommandAlias<'mc>{

fn into(self) -> crate::command::Command<'mc> {

crate::command::Command::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting FormattedCommandAlias into crate::command::Command")

   }
}
impl<'mc> crate::command::CommandTrait<'mc> for FormattedCommandAlias<'mc> {}
#[repr(C)]
pub struct CommandMap<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for CommandMap<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for CommandMap<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate CommandMap from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/command/CommandMap")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a CommandMap object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> CommandMapTrait<'mc> for CommandMap<'mc> {}
pub trait CommandMapTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Registers all the commands belonging to a certain plugin.
/// 
/// Caller can use:-
/// <ul>
/// <li>command.getName() to determine the label registered for this
/// command
/// <li>command.getAliases() to determine the aliases which where
/// registered
/// </ul>
	fn register_all(&self,fallback_prefix: impl Into<String>,commands: Vec<jni::objects::JObject<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;Ljava/util/List;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(fallback_prefix.into())?));
let raw_val_2 = self.jni_ref().new_object("java/util/ArrayList", "()V", vec![])?;
for v in commands{
let map_val_0 = jni::objects::JValueGen::Object(v);
self.jni_ref().call_method(&raw_val_2,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_2 = jni::objects::JValueGen::Object(raw_val_2);
let res = self.jni_ref().call_method(&self.jni_object(),"registerAll",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Registers a command. Returns true on success; false if name is already
/// taken and fallback had to be used.
/// 
/// Caller can use:-
/// <ul>
/// <li>command.getName() to determine the label registered for this
/// command
/// <li>command.getAliases() to determine the aliases which where
/// registered
/// </ul>
	fn register(&self,label: impl Into<String>,fallback_prefix: impl Into<String>,command: std::option::Option<impl Into<crate::command::Command<'mc>>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(label.into())?));
args.push(val_1);
sig += "Ljava/lang/String;";
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(fallback_prefix.into())?));
args.push(val_2);
if let Some(a) = command {
sig += "Lorg/bukkit/command/Command;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_3);
}
sig += ")Z";
let res = self.jni_ref().call_method(&self.jni_object(),"register",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Looks for the requested command and executes it if found.
	fn dispatch(&self,sender: impl Into<crate::command::CommandSender<'mc>>,cmd_line: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/command/CommandSender;Ljava/lang/String;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(sender.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(cmd_line.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"dispatch",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Clears all registered commands.
	fn clear_commands(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"clearCommands",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the command registered to the specified name
	fn get_command(&self,name: impl Into<String>) 
-> Result<Option<crate::command::Command<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lorg/bukkit/command/Command;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(name.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getCommand",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::command::Command::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Looks for the requested command and executes an appropriate
/// tab-completer if found. This method will also tab-complete partial
/// commands.
	fn tab_complete(&self,sender: impl Into<crate::command::CommandSender<'mc>>,cmd_line: impl Into<String>,location: std::option::Option<impl Into<crate::Location<'mc>>>) 
-> Result<Option<Vec<String>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/command/CommandSender;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(sender.into().jni_object().clone())});
args.push(val_1);
sig += "Ljava/lang/String;";
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(cmd_line.into())?));
args.push(val_2);
if let Some(a) = location {
sig += "Lorg/bukkit/Location;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_3);
}
sig += ")Ljava/util/List;";
let res = self.jni_ref().call_method(&self.jni_object(),"tabComplete",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?.to_string_lossy().to_string());
};
Ok(
Some(
new_vec
)
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct PluginIdentifiableCommand<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PluginIdentifiableCommand<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PluginIdentifiableCommand<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PluginIdentifiableCommand from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/command/PluginIdentifiableCommand")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PluginIdentifiableCommand object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PluginIdentifiableCommandTrait<'mc> for PluginIdentifiableCommand<'mc> {}
pub trait PluginIdentifiableCommandTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Gets the owner of this PluginIdentifiableCommand.
	fn plugin(&self) 
-> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/plugin/Plugin;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPlugin",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::plugin::Plugin::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct MultipleCommandAlias<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for MultipleCommandAlias<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for MultipleCommandAlias<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate MultipleCommandAlias from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/command/MultipleCommandAlias")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a MultipleCommandAlias object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> MultipleCommandAliasTrait<'mc> for MultipleCommandAlias<'mc> {}
pub trait MultipleCommandAliasTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,name: impl Into<String>,commands: impl Into<crate::command::Command<'mc>>) 
-> Result<crate::command::MultipleCommandAlias<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;Lorg/bukkit/command/Command;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(name.into())?));
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(commands.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/command/MultipleCommandAlias"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::command::MultipleCommandAlias::from_raw(&jni,res
)}
/// Gets the commands associated with the multi-command alias.
	fn commands(&self) 
-> Result<crate::command::Command<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/command/Command;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCommands",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::command::Command::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn execute(&self,sender: impl Into<crate::command::CommandSender<'mc>>,command_label: impl Into<String>,val_args: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/command/CommandSender;Ljava/lang/String;Ljava/lang/String;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(sender.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(command_label.into())?));
let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(val_args.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"execute",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
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
impl<'mc> Into<crate::command::Command<'mc>> for MultipleCommandAlias<'mc>{

fn into(self) -> crate::command::Command<'mc> {

crate::command::Command::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting MultipleCommandAlias into crate::command::Command")

   }
}
impl<'mc> crate::command::CommandTrait<'mc> for MultipleCommandAlias<'mc> {}
#[repr(C)]
pub struct ConsoleCommandSender<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ConsoleCommandSender<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ConsoleCommandSender<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ConsoleCommandSender from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/command/ConsoleCommandSender")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ConsoleCommandSender object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ConsoleCommandSenderTrait<'mc> for ConsoleCommandSender<'mc> {}
pub trait ConsoleCommandSenderTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::command::CommandSender<'mc>> for ConsoleCommandSender<'mc>{

fn into(self) -> crate::command::CommandSender<'mc> {

crate::command::CommandSender::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting ConsoleCommandSender into crate::command::CommandSender")

   }
}
impl<'mc> crate::command::CommandSenderTrait<'mc> for ConsoleCommandSender<'mc> {}
impl<'mc> Into<crate::conversations::Conversable<'mc>> for ConsoleCommandSender<'mc>{

fn into(self) -> crate::conversations::Conversable<'mc> {

crate::conversations::Conversable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting ConsoleCommandSender into crate::conversations::Conversable")

   }
}
impl<'mc> crate::conversations::ConversableTrait<'mc> for ConsoleCommandSender<'mc> {}
#[repr(C)]
pub struct RemoteConsoleCommandSender<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for RemoteConsoleCommandSender<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for RemoteConsoleCommandSender<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate RemoteConsoleCommandSender from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/command/RemoteConsoleCommandSender")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a RemoteConsoleCommandSender object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> RemoteConsoleCommandSenderTrait<'mc> for RemoteConsoleCommandSender<'mc> {}
pub trait RemoteConsoleCommandSenderTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Gets the socket address of this remote sender.
	fn address(&self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/net/SocketAddress;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAddress",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.l()?
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::command::CommandSender<'mc>> for RemoteConsoleCommandSender<'mc>{

fn into(self) -> crate::command::CommandSender<'mc> {

crate::command::CommandSender::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting RemoteConsoleCommandSender into crate::command::CommandSender")

   }
}
impl<'mc> crate::command::CommandSenderTrait<'mc> for RemoteConsoleCommandSender<'mc> {}
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
            let (valid, name) = env.validate_name(&obj, "org/bukkit/command/Command")?;
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
/// Executed on tab completion for this command, returning a list of
/// options the player can tab through.
	fn tab_complete(&self,sender: impl Into<crate::command::CommandSender<'mc>>,alias: impl Into<String>,val_args: impl Into<String>,location: std::option::Option<impl Into<crate::Location<'mc>>>) 
-> Result<Vec<String>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/command/CommandSender;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(sender.into().jni_object().clone())});
args.push(val_1);
sig += "Ljava/lang/String;";
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(alias.into())?));
args.push(val_2);
sig += "Ljava/lang/String;";
let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(val_args.into())?));
args.push(val_3);
if let Some(a) = location {
sig += "Lorg/bukkit/Location;";
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_4);
}
sig += ")Ljava/util/List;";
let res = self.jni_ref().call_method(&self.jni_object(),"tabComplete",sig.as_str(),args);
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
/// Returns the name of this command
	fn name(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Sets the name of this command.
/// 
/// May only be used before registering the command.
/// Will return true if the new name is set, and false
/// if the command has already been registered.
	fn set_name(&self,name: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Z");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(name.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setName",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets the permission required by users to be able to perform this
/// command
	fn permission(&self) 
-> Result<Option<String>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPermission",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)
)}
/// Sets the permission required by users to be able to perform this
/// command
	fn set_permission(&self,permission: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(permission.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setPermission",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Tests the given {@link CommandSender} to see if they can perform this
/// command.
/// 
/// If they do not have permission, they will be informed that they cannot
/// do this.
	fn test_permission(&self,target: impl Into<crate::command::CommandSender<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/command/CommandSender;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(target.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"testPermission",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Tests the given {@link CommandSender} to see if they can perform this
/// command.
/// 
/// No error is sent to the sender.
	fn test_permission_silent(&self,target: impl Into<crate::command::CommandSender<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/command/CommandSender;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(target.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"testPermissionSilent",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Returns the label for this command
	fn label(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLabel",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Sets the label of this command.
/// 
/// May only be used before registering the command.
/// Will return true if the new name is set, and false
/// if the command has already been registered.
	fn set_label(&self,name: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Z");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(name.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setLabel",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Registers this command to a CommandMap.
/// Once called it only allows changes the registered CommandMap
	fn register(&self,command_map: impl Into<crate::command::CommandMap<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/command/CommandMap;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(command_map.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"register",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Unregisters this command from the passed CommandMap applying any
/// outstanding changes
	fn unregister(&self,command_map: impl Into<crate::command::CommandMap<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/command/CommandMap;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(command_map.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"unregister",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Returns the current registered state of this command
	fn is_registered(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isRegistered",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Returns a list of active aliases of this command
	fn aliases(&self) 
-> Result<Vec<String>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAliases",sig.as_str(),vec![]);
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
/// Returns a message to be displayed on a failed permission check for this
/// command
	fn permission_message(&self) 
-> Result<Option<String>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPermissionMessage",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)
)}
/// Gets a brief description of this command
	fn description(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDescription",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Gets an example usage of this command
	fn usage(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getUsage",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Sets the list of aliases to request on registration for this command.
/// This is not effective outside of defining aliases in the {@link
/// PluginDescriptionFile#getCommands()} (under the
/// `<code>aliases</code>' node) is equivalent to this method.
	fn set_aliases(&self,aliases: Vec<jni::objects::JObject<'mc>>) 
-> Result<crate::command::Command<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/util/List;)Lorg/bukkit/command/Command;");
let raw_val_1 = self.jni_ref().new_object("java/util/ArrayList", "()V", vec![])?;
for v in aliases{
let map_val_0 = jni::objects::JValueGen::Object(v);
self.jni_ref().call_method(&raw_val_1,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_1 = jni::objects::JValueGen::Object(raw_val_1);
let res = self.jni_ref().call_method(&self.jni_object(),"setAliases",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::command::Command::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets a brief description of this command. Defining a description in the
/// {@link PluginDescriptionFile#getCommands()} (under the
/// `<code>description</code>' node) is equivalent to this method.
	fn set_description(&self,description: impl Into<String>) 
-> Result<crate::command::Command<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lorg/bukkit/command/Command;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(description.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setDescription",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::command::Command::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the message sent when a permission check fails
	fn set_permission_message(&self,permission_message: impl Into<String>) 
-> Result<crate::command::Command<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lorg/bukkit/command/Command;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(permission_message.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setPermissionMessage",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::command::Command::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the example usage of this command
	fn set_usage(&self,usage: impl Into<String>) 
-> Result<crate::command::Command<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lorg/bukkit/command/Command;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(usage.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setUsage",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::command::Command::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn broadcast_command_message(jni: &blackboxmc_general::SharedJNIEnv<'mc>,source: impl Into<crate::command::CommandSender<'mc>>,message: impl Into<String>,send_to_source: std::option::Option<bool>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/command/CommandSender;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(source.into().jni_object().clone())});
args.push(val_1);
sig += "Ljava/lang/String;";
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(message.into())?));
args.push(val_2);
if let Some(a) = send_to_source {
sig += "Z";
let val_3 = jni::objects::JValueGen::Bool(a.into());
args.push(val_3);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/command/Command"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"broadcastCommandMessage",
sig.as_str(),args);
jni.translate_error(res)?;
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
        
#[repr(C)]
pub struct PluginCommand<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PluginCommand<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PluginCommand<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PluginCommand from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/command/PluginCommand")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PluginCommand object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PluginCommandTrait<'mc> for PluginCommand<'mc> {}
pub trait PluginCommandTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Executes the command, returning its success
	fn execute(&self,sender: impl Into<crate::command::CommandSender<'mc>>,command_label: impl Into<String>,val_args: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/command/CommandSender;Ljava/lang/String;Ljava/lang/String;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(sender.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(command_label.into())?));
let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(val_args.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"execute",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Sets the {@link CommandExecutor} to run when parsing this command
	fn set_executor(&self,executor: impl Into<crate::command::CommandExecutor<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/command/CommandExecutor;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(executor.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setExecutor",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the {@link CommandExecutor} associated with this command
	fn executor(&self) 
-> Result<crate::command::CommandExecutor<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/command/CommandExecutor;");
let res = self.jni_ref().call_method(&self.jni_object(),"getExecutor",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::command::CommandExecutor::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the {@link TabCompleter} to run when tab-completing this command.
/// 
/// If no TabCompleter is specified, and the command's executor implements
/// TabCompleter, then the executor will be used for tab completion.
	fn set_tab_completer(&self,completer: impl Into<crate::command::TabCompleter<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/command/TabCompleter;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(completer.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setTabCompleter",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the {@link TabCompleter} associated with this command.
	fn tab_completer(&self) 
-> Result<Option<crate::command::TabCompleter<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/command/TabCompleter;");
let res = self.jni_ref().call_method(&self.jni_object(),"getTabCompleter",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::command::TabCompleter::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Gets the owner of this PluginCommand
	fn plugin(&self) 
-> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/plugin/Plugin;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPlugin",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::plugin::Plugin::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// {@inheritDoc}
/// 
/// Delegates to the tab completer if present.
/// 
/// If it is not present or returns null, will delegate to the current
/// command executor if it implements {@link TabCompleter}. If a non-null
/// list has not been found, will default to standard player name
/// completion in {@link
/// Command#tabComplete(CommandSender, String, String[])}.
/// 
/// This method does not consider permissions.
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

        impl<'mc> std::string::ToString for PluginCommand<'mc> {
            fn to_string(&self) -> String {
                match PluginCommandTrait::internal_to_string(self) {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling PluginCommand.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::command::PluginIdentifiableCommand<'mc>> for PluginCommand<'mc>{

fn into(self) -> crate::command::PluginIdentifiableCommand<'mc> {

crate::command::PluginIdentifiableCommand::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PluginCommand into crate::command::PluginIdentifiableCommand")

   }
}
impl<'mc> crate::command::PluginIdentifiableCommandTrait<'mc> for PluginCommand<'mc> {}
impl<'mc> Into<crate::command::Command<'mc>> for PluginCommand<'mc>{

fn into(self) -> crate::command::Command<'mc> {

crate::command::Command::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PluginCommand into crate::command::Command")

   }
}
impl<'mc> crate::command::CommandTrait<'mc> for PluginCommand<'mc> {}
#[repr(C)]
pub struct CommandExecutor<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for CommandExecutor<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for CommandExecutor<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate CommandExecutor from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/command/CommandExecutor")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a CommandExecutor object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> CommandExecutorTrait<'mc> for CommandExecutor<'mc> {}
pub trait CommandExecutorTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Executes the given command, returning its success.
/// 
/// If false is returned, then the "usage" plugin.yml entry for this command
/// (if defined) will be sent to the player.
	fn on_command(&self,sender: impl Into<crate::command::CommandSender<'mc>>,command: impl Into<crate::command::Command<'mc>>,label: impl Into<String>,val_args: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/command/CommandSender;Lorg/bukkit/command/Command;Ljava/lang/String;Ljava/lang/String;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(sender.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(command.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(label.into())?));
let val_4 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(val_args.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"onCommand",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4)]);
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
pub struct ProxiedCommandSender<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ProxiedCommandSender<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ProxiedCommandSender<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ProxiedCommandSender from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/command/ProxiedCommandSender")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ProxiedCommandSender object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ProxiedCommandSenderTrait<'mc> for ProxiedCommandSender<'mc> {}
pub trait ProxiedCommandSenderTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Returns the CommandSender which triggered this proxied command
	fn caller(&self) 
-> Result<crate::command::CommandSender<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/command/CommandSender;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCaller",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::command::CommandSender::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Returns the CommandSender which is being used to call the command
	fn callee(&self) 
-> Result<crate::command::CommandSender<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/command/CommandSender;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCallee",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::command::CommandSender::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::command::CommandSender<'mc>> for ProxiedCommandSender<'mc>{

fn into(self) -> crate::command::CommandSender<'mc> {

crate::command::CommandSender::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting ProxiedCommandSender into crate::command::CommandSender")

   }
}
impl<'mc> crate::command::CommandSenderTrait<'mc> for ProxiedCommandSender<'mc> {}
#[repr(C)]
pub struct SimpleCommandMap<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for SimpleCommandMap<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for SimpleCommandMap<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate SimpleCommandMap from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/command/SimpleCommandMap")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a SimpleCommandMap object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> SimpleCommandMapTrait<'mc> for SimpleCommandMap<'mc> {}
pub trait SimpleCommandMapTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,server: impl Into<crate::Server<'mc>>) 
-> Result<crate::command::SimpleCommandMap<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Server;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(server.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/command/SimpleCommandMap"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::command::SimpleCommandMap::from_raw(&jni,res
)}

	fn set_fallback_commands(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"setFallbackCommands",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// {@inheritDoc}
	fn register_all(&self,fallback_prefix: impl Into<String>,commands: Vec<jni::objects::JObject<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;Ljava/util/List;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(fallback_prefix.into())?));
let raw_val_2 = self.jni_ref().new_object("java/util/ArrayList", "()V", vec![])?;
for v in commands{
let map_val_0 = jni::objects::JValueGen::Object(v);
self.jni_ref().call_method(&raw_val_2,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_2 = jni::objects::JValueGen::Object(raw_val_2);
let res = self.jni_ref().call_method(&self.jni_object(),"registerAll",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// {@inheritDoc}
	fn register(&self,label: impl Into<String>,fallback_prefix: impl Into<String>,command: std::option::Option<impl Into<crate::command::Command<'mc>>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(label.into())?));
args.push(val_1);
sig += "Ljava/lang/String;";
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(fallback_prefix.into())?));
args.push(val_2);
if let Some(a) = command {
sig += "Lorg/bukkit/command/Command;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_3);
}
sig += ")Z";
let res = self.jni_ref().call_method(&self.jni_object(),"register",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// {@inheritDoc}
	fn dispatch(&self,sender: impl Into<crate::command::CommandSender<'mc>>,command_line: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/command/CommandSender;Ljava/lang/String;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(sender.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(command_line.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"dispatch",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn clear_commands(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"clearCommands",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn get_command(&self,name: impl Into<String>) 
-> Result<Option<crate::command::Command<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lorg/bukkit/command/Command;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(name.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getCommand",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::command::Command::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

	fn tab_complete(&self,sender: impl Into<crate::command::CommandSender<'mc>>,cmd_line: impl Into<String>,location: std::option::Option<impl Into<crate::Location<'mc>>>) 
-> Result<Option<Vec<String>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/command/CommandSender;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(sender.into().jni_object().clone())});
args.push(val_1);
sig += "Ljava/lang/String;";
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(cmd_line.into())?));
args.push(val_2);
if let Some(a) = location {
sig += "Lorg/bukkit/Location;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_3);
}
sig += ")Ljava/util/List;";
let res = self.jni_ref().call_method(&self.jni_object(),"tabComplete",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?.to_string_lossy().to_string());
};
Ok(
Some(
new_vec
)
)}

	fn commands(&self) 
-> Result<Vec<crate::command::Command<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Collection;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCommands",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(),res.l()?)?;let iter = col.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::command::Command::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}

	fn register_server_aliases(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"registerServerAliases",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::command::CommandMap<'mc>> for SimpleCommandMap<'mc>{

fn into(self) -> crate::command::CommandMap<'mc> {

crate::command::CommandMap::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting SimpleCommandMap into crate::command::CommandMap")

   }
}
impl<'mc> crate::command::CommandMapTrait<'mc> for SimpleCommandMap<'mc> {}
#[repr(C)]
pub struct CommandException<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for CommandException<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for CommandException<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate CommandException from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/command/CommandException")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a CommandException object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> CommandExceptionTrait<'mc> for CommandException<'mc> {}
pub trait CommandExceptionTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct CommandSender<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for CommandSender<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for CommandSender<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate CommandSender from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/command/CommandSender")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a CommandSender object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> CommandSenderTrait<'mc> for CommandSender<'mc> {}
pub trait CommandSenderTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Sends this sender multiple messages
	fn send_message(&self,sender: impl Into<blackboxmc_java::util::JavaUUID<'mc>>,messages: std::option::Option<impl Into<String>>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/util/UUID;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(sender.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = messages {
sig += "Ljava/lang/String;";
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(a.into())?));
args.push(val_2);
}
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"sendMessage",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Returns the server instance that this command is running on
	fn server(&self) 
-> Result<crate::Server<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Server;");
let res = self.jni_ref().call_method(&self.jni_object(),"getServer",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Server::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the name of this command sender
	fn name(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getName",sig.as_str(),vec![]);
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
impl<'mc> Into<crate::permissions::Permissible<'mc>> for CommandSender<'mc>{

fn into(self) -> crate::permissions::Permissible<'mc> {

crate::permissions::Permissible::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting CommandSender into crate::permissions::Permissible")

   }
}
impl<'mc> crate::permissions::PermissibleTrait<'mc> for CommandSender<'mc> {}
#[repr(C)]
pub struct TabCompleter<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for TabCompleter<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for TabCompleter<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate TabCompleter from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/command/TabCompleter")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a TabCompleter object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> TabCompleterTrait<'mc> for TabCompleter<'mc> {}
pub trait TabCompleterTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Requests a list of possible completions for a command argument.
	fn on_tab_complete(&self,sender: impl Into<crate::command::CommandSender<'mc>>,command: impl Into<crate::command::Command<'mc>>,label: impl Into<String>,val_args: impl Into<String>) 
-> Result<Option<Vec<String>>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/command/CommandSender;Lorg/bukkit/command/Command;Ljava/lang/String;Ljava/lang/String;)Ljava/util/List;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(sender.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(command.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(label.into())?));
let val_4 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(val_args.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"onTabComplete",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?.to_string_lossy().to_string());
};
Ok(
Some(
new_vec
)
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
pub mod defaults;
