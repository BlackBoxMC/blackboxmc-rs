#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct Configuration<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Configuration<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Configuration<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Configuration from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/configuration/Configuration")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Configuration object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> Configuration<'mc> {
	pub fn add_default(&self,path: impl Into<String>,value: jni::objects::JObject<'mc>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;Ljava/lang/Object;)L();");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let val_2 = jni::objects::JValueGen::Object(value);
let res = self.jni_ref().call_method(&self.jni_object(),"addDefault",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn add_defaults_with_defaults(&self,defaults: impl Into<crate::configuration::Configuration<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/configuration/Configuration;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(defaults.into().jni_object().clone())});
args.push(val_1);
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"addDefaults",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn set_defaults(&self,defaults: impl Into<crate::configuration::Configuration<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/configuration/Configuration;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(defaults.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setDefaults",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn defaults(&self) 
-> Result<Option<crate::configuration::Configuration<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::configuration::Configuration;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDefaults",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::configuration::Configuration::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn options(&self) 
-> Result<crate::configuration::ConfigurationOptions<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::configuration::ConfigurationOptions;");
let res = self.jni_ref().call_method(&self.jni_object(),"options",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::configuration::ConfigurationOptions::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_keys(&self,deep: bool) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Z)Lblackboxmc_java::util::Set;");
let val_1 = jni::objects::JValueGen::Bool(deep.into());
let res = self.jni_ref().call_method(&self.jni_object(),"getKeys",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_values(&self,deep: bool) 
-> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Z)Lblackboxmc_java::util::Map;");
let val_1 = jni::objects::JValueGen::Bool(deep.into());
let res = self.jni_ref().call_method(&self.jni_object(),"getValues",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn contains_with_path(&self,path: impl Into<String>,ignore_default: std::option::Option<bool>) 
-> Result<bool, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = ignore_default {
sig += "Z";
let val_2 = jni::objects::JValueGen::Bool(a.into());
args.push(val_2);
}
sig += ")Z";
let res = self.jni_ref().call_method(&self.jni_object(),"contains",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn is_set(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isSet",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn current_path(&self) 
-> Result<Option<String>, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCurrentPath",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)
)}
	pub fn name(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
	pub fn root(&self) 
-> Result<Option<crate::configuration::Configuration<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::configuration::Configuration;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRoot",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::configuration::Configuration::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn parent(&self) 
-> Result<Option<crate::configuration::ConfigurationSection<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::configuration::ConfigurationSection;");
let res = self.jni_ref().call_method(&self.jni_object(),"getParent",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn get_with_path(&self,path: impl Into<String>,def: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Ljava/lang/Object;";
let val_2 = jni::objects::JValueGen::Object(a);
args.push(val_2);
}
sig += ")Ljava/lang/Object;";
let res = self.jni_ref().call_method(&self.jni_object(),"get",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
res.l()?
)
)}
	pub fn set(&self,path: impl Into<String>,value: jni::objects::JObject<'mc>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;Ljava/lang/Object;)L();");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let val_2 = jni::objects::JValueGen::Object(value);
let res = self.jni_ref().call_method(&self.jni_object(),"set",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn create_section_with_path(&self,path: impl Into<String>,map: std::option::Option<impl Into<blackboxmc_java::util::JavaMap<'mc>>>) 
-> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = map {
sig += "Ljava/util/Map;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")Lorg/bukkit/configuration/ConfigurationSection;";
let res = self.jni_ref().call_method(&self.jni_object(),"createSection",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_string_with_path(&self,path: impl Into<String>,def: std::option::Option<impl Into<String>>) 
-> Result<Option<String>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Ljava/lang/String;";
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(a.into())?));
args.push(val_2);
}
sig += ")Ljava/lang/String;";
let res = self.jni_ref().call_method(&self.jni_object(),"getString",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)
)}
	pub fn is_string(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isString",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_int_with_path(&self,path: impl Into<String>,def: std::option::Option<i32>) 
-> Result<i32, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "I";
let val_2 = jni::objects::JValueGen::Int(a);
args.push(val_2);
}
sig += ")I";
let res = self.jni_ref().call_method(&self.jni_object(),"getInt",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn is_int(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isInt",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_boolean_with_path(&self,path: impl Into<String>,def: std::option::Option<bool>) 
-> Result<bool, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Z";
let val_2 = jni::objects::JValueGen::Bool(a.into());
args.push(val_2);
}
sig += ")Z";
let res = self.jni_ref().call_method(&self.jni_object(),"getBoolean",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn is_boolean(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isBoolean",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_double_with_path(&self,path: impl Into<String>,def: std::option::Option<f64>) 
-> Result<f64, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "D";
let val_2 = jni::objects::JValueGen::Double(a);
args.push(val_2);
}
sig += ")D";
let res = self.jni_ref().call_method(&self.jni_object(),"getDouble",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn is_double(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isDouble",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_long_with_path(&self,path: impl Into<String>,def: std::option::Option<i64>) 
-> Result<i64, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "J";
let val_2 = jni::objects::JValueGen::Long(a);
args.push(val_2);
}
sig += ")J";
let res = self.jni_ref().call_method(&self.jni_object(),"getLong",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.j()?
)}
	pub fn is_long(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isLong",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_list_with_path(&self,path: impl Into<String>,def: std::option::Option<Vec<jni::objects::JObject<'mc>>>) 
-> Result<Option<Vec<jni::objects::JObject<'mc>>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Ljava/util/List;";
let raw_val_2 = self.jni_ref().new_object("java/util/ArrayList", "()V", vec![])?;
for v in a{
sig += "Ldef/jni::objects::JObject;";
		let map_val_0 = jni::objects::JValueGen::Object(v);
self.jni_ref().call_method(&raw_val_2,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_2 = jni::objects::JValueGen::Object(raw_val_2);
args.push(val_2);
}
sig += ")Ljava/util/List;";
let res = self.jni_ref().call_method(&self.jni_object(),"getList",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
Some(
new_vec
)
)}
	pub fn is_list(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_string_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getStringList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_integer_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getIntegerList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_boolean_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getBooleanList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_double_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getDoubleList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_float_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getFloatList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_long_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getLongList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_byte_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getByteList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_character_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getCharacterList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_short_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getShortList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_map_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getMapList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_object_with_path(&self,path: impl Into<String>,clazz: jni::objects::JClass<'mc>,def: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
sig += "Ljava/lang/Class;";
let val_2 = jni::objects::JValueGen::Object(clazz.into());
args.push(val_2);
if let Some(a) = def {
sig += "Ljava/lang/Object;";
let val_3 = jni::objects::JValueGen::Object(a);
args.push(val_3);
}
sig += ")Ljava/lang/Object;";
let res = self.jni_ref().call_method(&self.jni_object(),"getObject",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
res.l()?
)
)}
	pub fn get_serializable_with_path(&self,path: impl Into<String>,clazz: jni::objects::JClass<'mc>,def: std::option::Option<impl Into<crate::configuration::serialization::ConfigurationSerializable<'mc>>>) 
-> Result<Option<crate::configuration::serialization::ConfigurationSerializable<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
sig += "Ljava/lang/Class;";
let val_2 = jni::objects::JValueGen::Object(clazz.into());
args.push(val_2);
if let Some(a) = def {
sig += "Lorg/bukkit/configuration/serialization/ConfigurationSerializable;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_3);
}
sig += ")Lorg/bukkit/configuration/serialization/ConfigurationSerializable;";
let res = self.jni_ref().call_method(&self.jni_object(),"getSerializable",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::configuration::serialization::ConfigurationSerializable::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn get_vector_with_path(&self,path: impl Into<String>,def: std::option::Option<impl Into<crate::util::Vector<'mc>>>) 
-> Result<Option<crate::util::Vector<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Lorg/bukkit/util/Vector;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")Lorg/bukkit/util/Vector;";
let res = self.jni_ref().call_method(&self.jni_object(),"getVector",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn is_vector(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isVector",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_offline_player_with_path(&self,path: impl Into<String>,def: std::option::Option<impl Into<crate::OfflinePlayer<'mc>>>) 
-> Result<Option<crate::OfflinePlayer<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Lorg/bukkit/OfflinePlayer;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")Lorg/bukkit/OfflinePlayer;";
let res = self.jni_ref().call_method(&self.jni_object(),"getOfflinePlayer",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::OfflinePlayer::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn is_offline_player(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isOfflinePlayer",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_item_stack_with_path(&self,path: impl Into<String>,def: std::option::Option<impl Into<crate::inventory::ItemStack<'mc>>>) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")Lorg/bukkit/inventory/ItemStack;";
let res = self.jni_ref().call_method(&self.jni_object(),"getItemStack",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn is_item_stack(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isItemStack",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_color_with_path(&self,path: impl Into<String>,def: std::option::Option<impl Into<crate::Color<'mc>>>) 
-> Result<Option<crate::Color<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Lorg/bukkit/Color;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")Lorg/bukkit/Color;";
let res = self.jni_ref().call_method(&self.jni_object(),"getColor",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::Color::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn is_color(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isColor",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_location_with_path(&self,path: impl Into<String>,def: std::option::Option<impl Into<crate::Location<'mc>>>) 
-> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Lorg/bukkit/Location;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")Lorg/bukkit/Location;";
let res = self.jni_ref().call_method(&self.jni_object(),"getLocation",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn is_location(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isLocation",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_configuration_section(&self,path: impl Into<String>) 
-> Result<Option<crate::configuration::ConfigurationSection<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lcrate::configuration::ConfigurationSection;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getConfigurationSection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn is_configuration_section(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isConfigurationSection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn default_section(&self) 
-> Result<Option<crate::configuration::ConfigurationSection<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::configuration::ConfigurationSection;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDefaultSection",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn get_comments(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getComments",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_inline_comments(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getInlineComments",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn set_comments(&self,path: impl Into<String>,comments: Vec<jni::objects::JObject<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;Ljava/util/List;)L();");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let raw_val_2 = self.jni_ref().new_object("java/util/ArrayList", "()V", vec![])?;
for v in comments{
let map_val_0 = jni::objects::JValueGen::Object(v);
self.jni_ref().call_method(&raw_val_2,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_2 = jni::objects::JValueGen::Object(raw_val_2);
let res = self.jni_ref().call_method(&self.jni_object(),"setComments",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn set_inline_comments(&self,path: impl Into<String>,comments: Vec<jni::objects::JObject<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;Ljava/util/List;)L();");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let raw_val_2 = self.jni_ref().new_object("java/util/ArrayList", "()V", vec![])?;
for v in comments{
let map_val_0 = jni::objects::JValueGen::Object(v);
self.jni_ref().call_method(&raw_val_2,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_2 = jni::objects::JValueGen::Object(raw_val_2);
let res = self.jni_ref().call_method(&self.jni_object(),"setInlineComments",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::configuration::ConfigurationSection<'mc>> for Configuration<'mc>{

fn into(self) -> crate::configuration::ConfigurationSection<'mc> {

crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), self.1).expect("Error converting Configuration into crate::configuration::ConfigurationSection")

   }
}
#[repr(C)]
pub struct MemoryConfigurationOptions<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for MemoryConfigurationOptions<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for MemoryConfigurationOptions<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate MemoryConfigurationOptions from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/configuration/MemoryConfigurationOptions")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a MemoryConfigurationOptions object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> MemoryConfigurationOptions<'mc> {
	pub fn configuration(&self) 
-> Result<crate::configuration::MemoryConfiguration<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::configuration::MemoryConfiguration;");
let res = self.jni_ref().call_method(&self.jni_object(),"configuration",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::configuration::MemoryConfiguration::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn copy_defaults(&self,value: bool) 
-> Result<crate::configuration::MemoryConfigurationOptions<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Z)Lcrate::configuration::MemoryConfigurationOptions;");
let val_1 = jni::objects::JValueGen::Bool(value.into());
let res = self.jni_ref().call_method(&self.jni_object(),"copyDefaults",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::configuration::MemoryConfigurationOptions::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn path_separator(&self,value: u16) 
-> Result<crate::configuration::MemoryConfigurationOptions<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(C)Lcrate::configuration::MemoryConfigurationOptions;");
let val_1 = jni::objects::JValueGen::Char(value);
let res = self.jni_ref().call_method(&self.jni_object(),"pathSeparator",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::configuration::MemoryConfigurationOptions::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
// SUPER CLASS: ConfigurationOptions

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::configuration::ConfigurationOptions<'mc>> for MemoryConfigurationOptions<'mc>{

fn into(self) -> crate::configuration::ConfigurationOptions<'mc> {

crate::configuration::ConfigurationOptions::from_raw(&self.jni_ref(), self.1).expect("Error converting MemoryConfigurationOptions into crate::configuration::ConfigurationOptions")

   }
}
#[repr(C)]
pub struct InvalidConfigurationException<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for InvalidConfigurationException<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for InvalidConfigurationException<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate InvalidConfigurationException from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/configuration/InvalidConfigurationException")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a InvalidConfigurationException object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> InvalidConfigurationException<'mc> {
// SUPER CLASS: Exception

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct MemoryConfiguration<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for MemoryConfiguration<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for MemoryConfiguration<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate MemoryConfiguration from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/configuration/MemoryConfiguration")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a MemoryConfiguration object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> MemoryConfiguration<'mc> {
	pub fn new_with_defaults(jni: &blackboxmc_general::SharedJNIEnv<'mc>,defaults: std::option::Option<impl Into<crate::configuration::Configuration<'mc>>>) 
-> Result<crate::configuration::MemoryConfiguration<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = defaults {
sig += "Lorg/bukkit/configuration/Configuration;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/configuration/MemoryConfiguration"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::configuration::MemoryConfiguration::from_raw(&jni,res
)}
	pub fn add_defaults_with_defaults(&self,defaults: impl Into<crate::configuration::Configuration<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/configuration/Configuration;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(defaults.into().jni_object().clone())});
args.push(val_1);
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"addDefaults",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn set_defaults(&self,defaults: impl Into<crate::configuration::Configuration<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/configuration/Configuration;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(defaults.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setDefaults",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn defaults(&self) 
-> Result<Option<crate::configuration::Configuration<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::configuration::Configuration;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDefaults",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::configuration::Configuration::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn parent(&self) 
-> Result<Option<crate::configuration::ConfigurationSection<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::configuration::ConfigurationSection;");
let res = self.jni_ref().call_method(&self.jni_object(),"getParent",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn options(&self) 
-> Result<crate::configuration::MemoryConfigurationOptions<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::configuration::MemoryConfigurationOptions;");
let res = self.jni_ref().call_method(&self.jni_object(),"options",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::configuration::MemoryConfigurationOptions::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn add_default(&self,path: impl Into<String>,value: jni::objects::JObject<'mc>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;Ljava/lang/Object;)L();");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let val_2 = jni::objects::JValueGen::Object(value);
let res = self.jni_ref().call_method(&self.jni_object(),"addDefault",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn get_keys(&self,deep: bool) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Z)Lblackboxmc_java::util::Set;");
let val_1 = jni::objects::JValueGen::Bool(deep.into());
let res = self.jni_ref().call_method(&self.jni_object(),"getKeys",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_values(&self,deep: bool) 
-> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Z)Lblackboxmc_java::util::Map;");
let val_1 = jni::objects::JValueGen::Bool(deep.into());
let res = self.jni_ref().call_method(&self.jni_object(),"getValues",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn contains_with_path(&self,path: impl Into<String>,ignore_default: std::option::Option<bool>) 
-> Result<bool, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = ignore_default {
sig += "Z";
let val_2 = jni::objects::JValueGen::Bool(a.into());
args.push(val_2);
}
sig += ")Z";
let res = self.jni_ref().call_method(&self.jni_object(),"contains",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn is_set(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isSet",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn current_path(&self) 
-> Result<Option<String>, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCurrentPath",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)
)}
	pub fn name(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
	pub fn root(&self) 
-> Result<Option<crate::configuration::Configuration<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::configuration::Configuration;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRoot",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::configuration::Configuration::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn get_with_path(&self,path: impl Into<String>,def: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Ljava/lang/Object;";
let val_2 = jni::objects::JValueGen::Object(a);
args.push(val_2);
}
sig += ")Ljava/lang/Object;";
let res = self.jni_ref().call_method(&self.jni_object(),"get",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
res.l()?
)
)}
	pub fn set(&self,path: impl Into<String>,value: jni::objects::JObject<'mc>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;Ljava/lang/Object;)L();");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let val_2 = jni::objects::JValueGen::Object(value);
let res = self.jni_ref().call_method(&self.jni_object(),"set",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn create_section_with_path(&self,path: impl Into<String>,map: std::option::Option<impl Into<blackboxmc_java::util::JavaMap<'mc>>>) 
-> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = map {
sig += "Ljava/util/Map;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")Lorg/bukkit/configuration/ConfigurationSection;";
let res = self.jni_ref().call_method(&self.jni_object(),"createSection",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_string_with_path(&self,path: impl Into<String>,def: std::option::Option<impl Into<String>>) 
-> Result<Option<String>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Ljava/lang/String;";
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(a.into())?));
args.push(val_2);
}
sig += ")Ljava/lang/String;";
let res = self.jni_ref().call_method(&self.jni_object(),"getString",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)
)}
	pub fn is_string(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isString",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_int_with_path(&self,path: impl Into<String>,def: std::option::Option<i32>) 
-> Result<i32, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "I";
let val_2 = jni::objects::JValueGen::Int(a);
args.push(val_2);
}
sig += ")I";
let res = self.jni_ref().call_method(&self.jni_object(),"getInt",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn is_int(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isInt",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_boolean_with_path(&self,path: impl Into<String>,def: std::option::Option<bool>) 
-> Result<bool, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Z";
let val_2 = jni::objects::JValueGen::Bool(a.into());
args.push(val_2);
}
sig += ")Z";
let res = self.jni_ref().call_method(&self.jni_object(),"getBoolean",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn is_boolean(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isBoolean",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_double_with_path(&self,path: impl Into<String>,def: std::option::Option<f64>) 
-> Result<f64, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "D";
let val_2 = jni::objects::JValueGen::Double(a);
args.push(val_2);
}
sig += ")D";
let res = self.jni_ref().call_method(&self.jni_object(),"getDouble",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn is_double(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isDouble",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_long_with_path(&self,path: impl Into<String>,def: std::option::Option<i64>) 
-> Result<i64, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "J";
let val_2 = jni::objects::JValueGen::Long(a);
args.push(val_2);
}
sig += ")J";
let res = self.jni_ref().call_method(&self.jni_object(),"getLong",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.j()?
)}
	pub fn is_long(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isLong",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_list_with_path(&self,path: impl Into<String>,def: std::option::Option<Vec<jni::objects::JObject<'mc>>>) 
-> Result<Option<Vec<jni::objects::JObject<'mc>>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Ljava/util/List;";
let raw_val_2 = self.jni_ref().new_object("java/util/ArrayList", "()V", vec![])?;
for v in a{
sig += "Ldef/jni::objects::JObject;";
		let map_val_0 = jni::objects::JValueGen::Object(v);
self.jni_ref().call_method(&raw_val_2,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_2 = jni::objects::JValueGen::Object(raw_val_2);
args.push(val_2);
}
sig += ")Ljava/util/List;";
let res = self.jni_ref().call_method(&self.jni_object(),"getList",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
Some(
new_vec
)
)}
	pub fn is_list(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_string_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getStringList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_integer_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getIntegerList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_boolean_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getBooleanList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_double_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getDoubleList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_float_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getFloatList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_long_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getLongList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_byte_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getByteList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_character_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getCharacterList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_short_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getShortList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_map_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getMapList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_object_with_path(&self,path: impl Into<String>,clazz: jni::objects::JClass<'mc>,def: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
sig += "Ljava/lang/Class;";
let val_2 = jni::objects::JValueGen::Object(clazz.into());
args.push(val_2);
if let Some(a) = def {
sig += "Ljava/lang/Object;";
let val_3 = jni::objects::JValueGen::Object(a);
args.push(val_3);
}
sig += ")Ljava/lang/Object;";
let res = self.jni_ref().call_method(&self.jni_object(),"getObject",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
res.l()?
)
)}
	pub fn get_serializable_with_path(&self,path: impl Into<String>,clazz: jni::objects::JClass<'mc>,def: std::option::Option<impl Into<crate::configuration::serialization::ConfigurationSerializable<'mc>>>) 
-> Result<Option<crate::configuration::serialization::ConfigurationSerializable<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
sig += "Ljava/lang/Class;";
let val_2 = jni::objects::JValueGen::Object(clazz.into());
args.push(val_2);
if let Some(a) = def {
sig += "Lorg/bukkit/configuration/serialization/ConfigurationSerializable;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_3);
}
sig += ")Lorg/bukkit/configuration/serialization/ConfigurationSerializable;";
let res = self.jni_ref().call_method(&self.jni_object(),"getSerializable",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::configuration::serialization::ConfigurationSerializable::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn get_vector_with_path(&self,path: impl Into<String>,def: std::option::Option<impl Into<crate::util::Vector<'mc>>>) 
-> Result<Option<crate::util::Vector<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Lorg/bukkit/util/Vector;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")Lorg/bukkit/util/Vector;";
let res = self.jni_ref().call_method(&self.jni_object(),"getVector",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn is_vector(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isVector",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_offline_player_with_path(&self,path: impl Into<String>,def: std::option::Option<impl Into<crate::OfflinePlayer<'mc>>>) 
-> Result<Option<crate::OfflinePlayer<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Lorg/bukkit/OfflinePlayer;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")Lorg/bukkit/OfflinePlayer;";
let res = self.jni_ref().call_method(&self.jni_object(),"getOfflinePlayer",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::OfflinePlayer::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn is_offline_player(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isOfflinePlayer",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_item_stack_with_path(&self,path: impl Into<String>,def: std::option::Option<impl Into<crate::inventory::ItemStack<'mc>>>) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")Lorg/bukkit/inventory/ItemStack;";
let res = self.jni_ref().call_method(&self.jni_object(),"getItemStack",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn is_item_stack(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isItemStack",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_color_with_path(&self,path: impl Into<String>,def: std::option::Option<impl Into<crate::Color<'mc>>>) 
-> Result<Option<crate::Color<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Lorg/bukkit/Color;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")Lorg/bukkit/Color;";
let res = self.jni_ref().call_method(&self.jni_object(),"getColor",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::Color::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn is_color(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isColor",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_location_with_path(&self,path: impl Into<String>,def: std::option::Option<impl Into<crate::Location<'mc>>>) 
-> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Lorg/bukkit/Location;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")Lorg/bukkit/Location;";
let res = self.jni_ref().call_method(&self.jni_object(),"getLocation",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn is_location(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isLocation",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_configuration_section(&self,path: impl Into<String>) 
-> Result<Option<crate::configuration::ConfigurationSection<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lcrate::configuration::ConfigurationSection;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getConfigurationSection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn is_configuration_section(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isConfigurationSection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn default_section(&self) 
-> Result<Option<crate::configuration::ConfigurationSection<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::configuration::ConfigurationSection;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDefaultSection",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn get_comments(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getComments",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_inline_comments(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getInlineComments",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn set_comments(&self,path: impl Into<String>,comments: Vec<jni::objects::JObject<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;Ljava/util/List;)L();");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let raw_val_2 = self.jni_ref().new_object("java/util/ArrayList", "()V", vec![])?;
for v in comments{
let map_val_0 = jni::objects::JValueGen::Object(v);
self.jni_ref().call_method(&raw_val_2,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_2 = jni::objects::JValueGen::Object(raw_val_2);
let res = self.jni_ref().call_method(&self.jni_object(),"setComments",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn set_inline_comments(&self,path: impl Into<String>,comments: Vec<jni::objects::JObject<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;Ljava/util/List;)L();");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let raw_val_2 = self.jni_ref().new_object("java/util/ArrayList", "()V", vec![])?;
for v in comments{
let map_val_0 = jni::objects::JValueGen::Object(v);
self.jni_ref().call_method(&raw_val_2,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_2 = jni::objects::JValueGen::Object(raw_val_2);
let res = self.jni_ref().call_method(&self.jni_object(),"setInlineComments",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
// SUPER CLASS: MemorySection

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::configuration::Configuration<'mc>> for MemoryConfiguration<'mc>{

fn into(self) -> crate::configuration::Configuration<'mc> {

crate::configuration::Configuration::from_raw(&self.jni_ref(), self.1).expect("Error converting MemoryConfiguration into crate::configuration::Configuration")

   }
}
impl<'mc> Into<crate::configuration::MemorySection<'mc>> for MemoryConfiguration<'mc>{

fn into(self) -> crate::configuration::MemorySection<'mc> {

crate::configuration::MemorySection::from_raw(&self.jni_ref(), self.1).expect("Error converting MemoryConfiguration into crate::configuration::MemorySection")

   }
}
#[repr(C)]
pub struct ConfigurationSection<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ConfigurationSection<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ConfigurationSection<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ConfigurationSection from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/configuration/ConfigurationSection")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ConfigurationSection object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ConfigurationSection<'mc> {
	pub fn get_keys(&self,deep: bool) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Z)Lblackboxmc_java::util::Set;");
let val_1 = jni::objects::JValueGen::Bool(deep.into());
let res = self.jni_ref().call_method(&self.jni_object(),"getKeys",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_values(&self,deep: bool) 
-> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Z)Lblackboxmc_java::util::Map;");
let val_1 = jni::objects::JValueGen::Bool(deep.into());
let res = self.jni_ref().call_method(&self.jni_object(),"getValues",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn contains_with_path(&self,path: impl Into<String>,ignore_default: std::option::Option<bool>) 
-> Result<bool, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = ignore_default {
sig += "Z";
let val_2 = jni::objects::JValueGen::Bool(a.into());
args.push(val_2);
}
sig += ")Z";
let res = self.jni_ref().call_method(&self.jni_object(),"contains",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn is_set(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isSet",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn current_path(&self) 
-> Result<Option<String>, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCurrentPath",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)
)}
	pub fn name(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
	pub fn root(&self) 
-> Result<Option<crate::configuration::Configuration<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::configuration::Configuration;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRoot",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::configuration::Configuration::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn parent(&self) 
-> Result<Option<crate::configuration::ConfigurationSection<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::configuration::ConfigurationSection;");
let res = self.jni_ref().call_method(&self.jni_object(),"getParent",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn get_with_path(&self,path: impl Into<String>,def: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Ljava/lang/Object;";
let val_2 = jni::objects::JValueGen::Object(a);
args.push(val_2);
}
sig += ")Ljava/lang/Object;";
let res = self.jni_ref().call_method(&self.jni_object(),"get",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
res.l()?
)
)}
	pub fn set(&self,path: impl Into<String>,value: jni::objects::JObject<'mc>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;Ljava/lang/Object;)L();");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let val_2 = jni::objects::JValueGen::Object(value);
let res = self.jni_ref().call_method(&self.jni_object(),"set",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn create_section_with_path(&self,path: impl Into<String>,map: std::option::Option<impl Into<blackboxmc_java::util::JavaMap<'mc>>>) 
-> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = map {
sig += "Ljava/util/Map;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")Lorg/bukkit/configuration/ConfigurationSection;";
let res = self.jni_ref().call_method(&self.jni_object(),"createSection",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_string_with_path(&self,path: impl Into<String>,def: std::option::Option<impl Into<String>>) 
-> Result<Option<String>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Ljava/lang/String;";
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(a.into())?));
args.push(val_2);
}
sig += ")Ljava/lang/String;";
let res = self.jni_ref().call_method(&self.jni_object(),"getString",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)
)}
	pub fn is_string(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isString",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_int_with_path(&self,path: impl Into<String>,def: std::option::Option<i32>) 
-> Result<i32, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "I";
let val_2 = jni::objects::JValueGen::Int(a);
args.push(val_2);
}
sig += ")I";
let res = self.jni_ref().call_method(&self.jni_object(),"getInt",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn is_int(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isInt",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_boolean_with_path(&self,path: impl Into<String>,def: std::option::Option<bool>) 
-> Result<bool, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Z";
let val_2 = jni::objects::JValueGen::Bool(a.into());
args.push(val_2);
}
sig += ")Z";
let res = self.jni_ref().call_method(&self.jni_object(),"getBoolean",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn is_boolean(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isBoolean",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_double_with_path(&self,path: impl Into<String>,def: std::option::Option<f64>) 
-> Result<f64, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "D";
let val_2 = jni::objects::JValueGen::Double(a);
args.push(val_2);
}
sig += ")D";
let res = self.jni_ref().call_method(&self.jni_object(),"getDouble",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn is_double(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isDouble",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_long_with_path(&self,path: impl Into<String>,def: std::option::Option<i64>) 
-> Result<i64, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "J";
let val_2 = jni::objects::JValueGen::Long(a);
args.push(val_2);
}
sig += ")J";
let res = self.jni_ref().call_method(&self.jni_object(),"getLong",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.j()?
)}
	pub fn is_long(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isLong",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_list_with_path(&self,path: impl Into<String>,def: std::option::Option<Vec<jni::objects::JObject<'mc>>>) 
-> Result<Option<Vec<jni::objects::JObject<'mc>>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Ljava/util/List;";
let raw_val_2 = self.jni_ref().new_object("java/util/ArrayList", "()V", vec![])?;
for v in a{
sig += "Ldef/jni::objects::JObject;";
		let map_val_0 = jni::objects::JValueGen::Object(v);
self.jni_ref().call_method(&raw_val_2,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_2 = jni::objects::JValueGen::Object(raw_val_2);
args.push(val_2);
}
sig += ")Ljava/util/List;";
let res = self.jni_ref().call_method(&self.jni_object(),"getList",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
Some(
new_vec
)
)}
	pub fn is_list(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_string_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getStringList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_integer_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getIntegerList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_boolean_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getBooleanList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_double_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getDoubleList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_float_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getFloatList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_long_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getLongList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_byte_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getByteList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_character_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getCharacterList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_short_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getShortList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_map_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getMapList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_object_with_path(&self,path: impl Into<String>,clazz: jni::objects::JClass<'mc>,def: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
sig += "Ljava/lang/Class;";
let val_2 = jni::objects::JValueGen::Object(clazz.into());
args.push(val_2);
if let Some(a) = def {
sig += "Ljava/lang/Object;";
let val_3 = jni::objects::JValueGen::Object(a);
args.push(val_3);
}
sig += ")Ljava/lang/Object;";
let res = self.jni_ref().call_method(&self.jni_object(),"getObject",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
res.l()?
)
)}
	pub fn get_serializable_with_path(&self,path: impl Into<String>,clazz: jni::objects::JClass<'mc>,def: std::option::Option<impl Into<crate::configuration::serialization::ConfigurationSerializable<'mc>>>) 
-> Result<Option<crate::configuration::serialization::ConfigurationSerializable<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
sig += "Ljava/lang/Class;";
let val_2 = jni::objects::JValueGen::Object(clazz.into());
args.push(val_2);
if let Some(a) = def {
sig += "Lorg/bukkit/configuration/serialization/ConfigurationSerializable;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_3);
}
sig += ")Lorg/bukkit/configuration/serialization/ConfigurationSerializable;";
let res = self.jni_ref().call_method(&self.jni_object(),"getSerializable",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::configuration::serialization::ConfigurationSerializable::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn get_vector_with_path(&self,path: impl Into<String>,def: std::option::Option<impl Into<crate::util::Vector<'mc>>>) 
-> Result<Option<crate::util::Vector<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Lorg/bukkit/util/Vector;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")Lorg/bukkit/util/Vector;";
let res = self.jni_ref().call_method(&self.jni_object(),"getVector",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn is_vector(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isVector",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_offline_player_with_path(&self,path: impl Into<String>,def: std::option::Option<impl Into<crate::OfflinePlayer<'mc>>>) 
-> Result<Option<crate::OfflinePlayer<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Lorg/bukkit/OfflinePlayer;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")Lorg/bukkit/OfflinePlayer;";
let res = self.jni_ref().call_method(&self.jni_object(),"getOfflinePlayer",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::OfflinePlayer::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn is_offline_player(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isOfflinePlayer",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_item_stack_with_path(&self,path: impl Into<String>,def: std::option::Option<impl Into<crate::inventory::ItemStack<'mc>>>) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")Lorg/bukkit/inventory/ItemStack;";
let res = self.jni_ref().call_method(&self.jni_object(),"getItemStack",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn is_item_stack(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isItemStack",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_color_with_path(&self,path: impl Into<String>,def: std::option::Option<impl Into<crate::Color<'mc>>>) 
-> Result<Option<crate::Color<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Lorg/bukkit/Color;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")Lorg/bukkit/Color;";
let res = self.jni_ref().call_method(&self.jni_object(),"getColor",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::Color::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn is_color(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isColor",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_location_with_path(&self,path: impl Into<String>,def: std::option::Option<impl Into<crate::Location<'mc>>>) 
-> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Lorg/bukkit/Location;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")Lorg/bukkit/Location;";
let res = self.jni_ref().call_method(&self.jni_object(),"getLocation",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn is_location(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isLocation",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_configuration_section(&self,path: impl Into<String>) 
-> Result<Option<crate::configuration::ConfigurationSection<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lcrate::configuration::ConfigurationSection;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getConfigurationSection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn is_configuration_section(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isConfigurationSection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn default_section(&self) 
-> Result<Option<crate::configuration::ConfigurationSection<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::configuration::ConfigurationSection;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDefaultSection",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn add_default(&self,path: impl Into<String>,value: jni::objects::JObject<'mc>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;Ljava/lang/Object;)L();");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let val_2 = jni::objects::JValueGen::Object(value);
let res = self.jni_ref().call_method(&self.jni_object(),"addDefault",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn get_comments(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getComments",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_inline_comments(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getInlineComments",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn set_comments(&self,path: impl Into<String>,comments: Vec<jni::objects::JObject<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;Ljava/util/List;)L();");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let raw_val_2 = self.jni_ref().new_object("java/util/ArrayList", "()V", vec![])?;
for v in comments{
let map_val_0 = jni::objects::JValueGen::Object(v);
self.jni_ref().call_method(&raw_val_2,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_2 = jni::objects::JValueGen::Object(raw_val_2);
let res = self.jni_ref().call_method(&self.jni_object(),"setComments",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn set_inline_comments(&self,path: impl Into<String>,comments: Vec<jni::objects::JObject<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;Ljava/util/List;)L();");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let raw_val_2 = self.jni_ref().new_object("java/util/ArrayList", "()V", vec![])?;
for v in comments{
let map_val_0 = jni::objects::JValueGen::Object(v);
self.jni_ref().call_method(&raw_val_2,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_2 = jni::objects::JValueGen::Object(raw_val_2);
let res = self.jni_ref().call_method(&self.jni_object(),"setInlineComments",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct ConfigurationOptions<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ConfigurationOptions<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ConfigurationOptions<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ConfigurationOptions from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/configuration/ConfigurationOptions")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ConfigurationOptions object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ConfigurationOptions<'mc> {
	pub fn configuration(&self) 
-> Result<crate::configuration::Configuration<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::configuration::Configuration;");
let res = self.jni_ref().call_method(&self.jni_object(),"configuration",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::configuration::Configuration::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn path_separator_with_value(&self,value: std::option::Option<u16>) 
-> Result<crate::configuration::ConfigurationOptions<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = value {
sig += "C";
let val_1 = jni::objects::JValueGen::Char(a);
args.push(val_1);
}
sig += ")Lorg/bukkit/configuration/ConfigurationOptions;";
let res = self.jni_ref().call_method(&self.jni_object(),"pathSeparator",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::configuration::ConfigurationOptions::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn copy_defaults_with_value(&self,value: std::option::Option<bool>) 
-> Result<crate::configuration::ConfigurationOptions<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = value {
sig += "Z";
let val_1 = jni::objects::JValueGen::Bool(a.into());
args.push(val_1);
}
sig += ")Lorg/bukkit/configuration/ConfigurationOptions;";
let res = self.jni_ref().call_method(&self.jni_object(),"copyDefaults",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::configuration::ConfigurationOptions::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
// SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct MemorySection<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for MemorySection<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for MemorySection<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate MemorySection from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/configuration/MemorySection")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a MemorySection object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> MemorySection<'mc> {
	pub fn get_keys(&self,deep: bool) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Z)Lblackboxmc_java::util::Set;");
let val_1 = jni::objects::JValueGen::Bool(deep.into());
let res = self.jni_ref().call_method(&self.jni_object(),"getKeys",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_values(&self,deep: bool) 
-> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Z)Lblackboxmc_java::util::Map;");
let val_1 = jni::objects::JValueGen::Bool(deep.into());
let res = self.jni_ref().call_method(&self.jni_object(),"getValues",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn contains_with_path(&self,path: impl Into<String>,ignore_default: std::option::Option<bool>) 
-> Result<bool, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = ignore_default {
sig += "Z";
let val_2 = jni::objects::JValueGen::Bool(a.into());
args.push(val_2);
}
sig += ")Z";
let res = self.jni_ref().call_method(&self.jni_object(),"contains",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn is_set(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isSet",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn current_path(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCurrentPath",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
	pub fn name(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
	pub fn root(&self) 
-> Result<Option<crate::configuration::Configuration<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::configuration::Configuration;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRoot",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::configuration::Configuration::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn parent(&self) 
-> Result<Option<crate::configuration::ConfigurationSection<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::configuration::ConfigurationSection;");
let res = self.jni_ref().call_method(&self.jni_object(),"getParent",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn add_default(&self,path: impl Into<String>,value: jni::objects::JObject<'mc>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;Ljava/lang/Object;)L();");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let val_2 = jni::objects::JValueGen::Object(value);
let res = self.jni_ref().call_method(&self.jni_object(),"addDefault",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn default_section(&self) 
-> Result<Option<crate::configuration::ConfigurationSection<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::configuration::ConfigurationSection;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDefaultSection",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn set(&self,path: impl Into<String>,value: jni::objects::JObject<'mc>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;Ljava/lang/Object;)L();");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let val_2 = jni::objects::JValueGen::Object(value);
let res = self.jni_ref().call_method(&self.jni_object(),"set",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn get_with_path(&self,path: impl Into<String>,def: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Ljava/lang/Object;";
let val_2 = jni::objects::JValueGen::Object(a);
args.push(val_2);
}
sig += ")Ljava/lang/Object;";
let res = self.jni_ref().call_method(&self.jni_object(),"get",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
res.l()?
)
)}
	pub fn create_section_with_path(&self,path: impl Into<String>,map: std::option::Option<impl Into<blackboxmc_java::util::JavaMap<'mc>>>) 
-> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = map {
sig += "Ljava/util/Map;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")Lorg/bukkit/configuration/ConfigurationSection;";
let res = self.jni_ref().call_method(&self.jni_object(),"createSection",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_string_with_path(&self,path: impl Into<String>,def: std::option::Option<impl Into<String>>) 
-> Result<Option<String>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Ljava/lang/String;";
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(a.into())?));
args.push(val_2);
}
sig += ")Ljava/lang/String;";
let res = self.jni_ref().call_method(&self.jni_object(),"getString",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)
)}
	pub fn is_string(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isString",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_int_with_path(&self,path: impl Into<String>,def: std::option::Option<i32>) 
-> Result<i32, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "I";
let val_2 = jni::objects::JValueGen::Int(a);
args.push(val_2);
}
sig += ")I";
let res = self.jni_ref().call_method(&self.jni_object(),"getInt",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn is_int(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isInt",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_boolean_with_path(&self,path: impl Into<String>,def: std::option::Option<bool>) 
-> Result<bool, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Z";
let val_2 = jni::objects::JValueGen::Bool(a.into());
args.push(val_2);
}
sig += ")Z";
let res = self.jni_ref().call_method(&self.jni_object(),"getBoolean",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn is_boolean(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isBoolean",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_double_with_path(&self,path: impl Into<String>,def: std::option::Option<f64>) 
-> Result<f64, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "D";
let val_2 = jni::objects::JValueGen::Double(a);
args.push(val_2);
}
sig += ")D";
let res = self.jni_ref().call_method(&self.jni_object(),"getDouble",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn is_double(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isDouble",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_long_with_path(&self,path: impl Into<String>,def: std::option::Option<i64>) 
-> Result<i64, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "J";
let val_2 = jni::objects::JValueGen::Long(a);
args.push(val_2);
}
sig += ")J";
let res = self.jni_ref().call_method(&self.jni_object(),"getLong",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.j()?
)}
	pub fn is_long(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isLong",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_list_with_path(&self,path: impl Into<String>,def: std::option::Option<Vec<jni::objects::JObject<'mc>>>) 
-> Result<Option<Vec<jni::objects::JObject<'mc>>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Ljava/util/List;";
let raw_val_2 = self.jni_ref().new_object("java/util/ArrayList", "()V", vec![])?;
for v in a{
sig += "Ldef/jni::objects::JObject;";
		let map_val_0 = jni::objects::JValueGen::Object(v);
self.jni_ref().call_method(&raw_val_2,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_2 = jni::objects::JValueGen::Object(raw_val_2);
args.push(val_2);
}
sig += ")Ljava/util/List;";
let res = self.jni_ref().call_method(&self.jni_object(),"getList",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
Some(
new_vec
)
)}
	pub fn is_list(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_string_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getStringList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_integer_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getIntegerList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_boolean_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getBooleanList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_double_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getDoubleList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_float_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getFloatList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_long_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getLongList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_byte_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getByteList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_character_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getCharacterList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_short_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getShortList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_map_list(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getMapList",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_object_with_path(&self,path: impl Into<String>,clazz: jni::objects::JClass<'mc>,def: std::option::Option<jni::objects::JObject<'mc>>) 
-> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
sig += "Ljava/lang/Class;";
let val_2 = jni::objects::JValueGen::Object(clazz.into());
args.push(val_2);
if let Some(a) = def {
sig += "Ljava/lang/Object;";
let val_3 = jni::objects::JValueGen::Object(a);
args.push(val_3);
}
sig += ")Ljava/lang/Object;";
let res = self.jni_ref().call_method(&self.jni_object(),"getObject",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
res.l()?
)
)}
	pub fn get_serializable_with_path(&self,path: impl Into<String>,clazz: jni::objects::JClass<'mc>,def: std::option::Option<impl Into<crate::configuration::serialization::ConfigurationSerializable<'mc>>>) 
-> Result<Option<crate::configuration::serialization::ConfigurationSerializable<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
sig += "Ljava/lang/Class;";
let val_2 = jni::objects::JValueGen::Object(clazz.into());
args.push(val_2);
if let Some(a) = def {
sig += "Lorg/bukkit/configuration/serialization/ConfigurationSerializable;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_3);
}
sig += ")Lorg/bukkit/configuration/serialization/ConfigurationSerializable;";
let res = self.jni_ref().call_method(&self.jni_object(),"getSerializable",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::configuration::serialization::ConfigurationSerializable::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn get_vector_with_path(&self,path: impl Into<String>,def: std::option::Option<impl Into<crate::util::Vector<'mc>>>) 
-> Result<Option<crate::util::Vector<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Lorg/bukkit/util/Vector;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")Lorg/bukkit/util/Vector;";
let res = self.jni_ref().call_method(&self.jni_object(),"getVector",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn is_vector(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isVector",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_offline_player_with_path(&self,path: impl Into<String>,def: std::option::Option<impl Into<crate::OfflinePlayer<'mc>>>) 
-> Result<Option<crate::OfflinePlayer<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Lorg/bukkit/OfflinePlayer;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")Lorg/bukkit/OfflinePlayer;";
let res = self.jni_ref().call_method(&self.jni_object(),"getOfflinePlayer",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::OfflinePlayer::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn is_offline_player(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isOfflinePlayer",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_item_stack_with_path(&self,path: impl Into<String>,def: std::option::Option<impl Into<crate::inventory::ItemStack<'mc>>>) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")Lorg/bukkit/inventory/ItemStack;";
let res = self.jni_ref().call_method(&self.jni_object(),"getItemStack",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn is_item_stack(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isItemStack",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_color_with_path(&self,path: impl Into<String>,def: std::option::Option<impl Into<crate::Color<'mc>>>) 
-> Result<Option<crate::Color<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Lorg/bukkit/Color;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")Lorg/bukkit/Color;";
let res = self.jni_ref().call_method(&self.jni_object(),"getColor",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::Color::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn is_color(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isColor",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_location_with_path(&self,path: impl Into<String>,def: std::option::Option<impl Into<crate::Location<'mc>>>) 
-> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
args.push(val_1);
if let Some(a) = def {
sig += "Lorg/bukkit/Location;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")Lorg/bukkit/Location;";
let res = self.jni_ref().call_method(&self.jni_object(),"getLocation",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn is_location(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isLocation",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get_configuration_section(&self,path: impl Into<String>) 
-> Result<Option<crate::configuration::ConfigurationSection<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lcrate::configuration::ConfigurationSection;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getConfigurationSection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn is_configuration_section(&self,path: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"isConfigurationSection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn create_path_with_section(jni: &blackboxmc_general::SharedJNIEnv<'mc>,section: impl Into<crate::configuration::ConfigurationSection<'mc>>,key: impl Into<String>,relative_to: std::option::Option<impl Into<crate::configuration::ConfigurationSection<'mc>>>) 
-> Result<String, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/configuration/ConfigurationSection;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(section.into().jni_object().clone())});
args.push(val_1);
sig += "Ljava/lang/String;";
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(key.into())?));
args.push(val_2);
if let Some(a) = relative_to {
sig += "Lorg/bukkit/configuration/ConfigurationSection;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_3);
}
sig += ")Ljava/lang/String;";
let cls = jni.find_class("org/bukkit/configuration/MemorySection"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"createPath",
sig.as_str(),args);
let res = 
jni.translate_error(res)?;
Ok(
jni.get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
	pub fn get_comments(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getComments",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn get_inline_comments(&self,path: impl Into<String>) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)LVec;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"getInlineComments",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn set_comments(&self,path: impl Into<String>,comments: Vec<jni::objects::JObject<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;Ljava/util/List;)L();");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let raw_val_2 = self.jni_ref().new_object("java/util/ArrayList", "()V", vec![])?;
for v in comments{
let map_val_0 = jni::objects::JValueGen::Object(v);
self.jni_ref().call_method(&raw_val_2,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_2 = jni::objects::JValueGen::Object(raw_val_2);
let res = self.jni_ref().call_method(&self.jni_object(),"setComments",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn set_inline_comments(&self,path: impl Into<String>,comments: Vec<jni::objects::JObject<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;Ljava/util/List;)L();");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(path.into())?));
let raw_val_2 = self.jni_ref().new_object("java/util/ArrayList", "()V", vec![])?;
for v in comments{
let map_val_0 = jni::objects::JValueGen::Object(v);
self.jni_ref().call_method(&raw_val_2,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_2 = jni::objects::JValueGen::Object(raw_val_2);
let res = self.jni_ref().call_method(&self.jni_object(),"setInlineComments",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[doc(hidden)]
	pub fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
// SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for MemorySection<'mc> {
            fn to_string(&self) -> String {
                match &self.internal_to_string() {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling MemorySection.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::configuration::ConfigurationSection<'mc>> for MemorySection<'mc>{

fn into(self) -> crate::configuration::ConfigurationSection<'mc> {

crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), self.1).expect("Error converting MemorySection into crate::configuration::ConfigurationSection")

   }
}
pub mod file;
pub mod serialization;
