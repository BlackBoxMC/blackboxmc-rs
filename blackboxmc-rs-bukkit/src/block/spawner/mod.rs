#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct SpawnerEntry<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for SpawnerEntry<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for SpawnerEntry<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate SpawnerEntry from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/block/spawner/SpawnerEntry")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a SpawnerEntry object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> SpawnerEntry<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,snapshot: impl Into<crate::entity::EntitySnapshot<'mc>>,spawn_weight: i32,spawn_rule: impl Into<crate::block::spawner::SpawnRule<'mc>>) 
-> Result<crate::block::spawner::SpawnerEntry<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/EntitySnapshot;ILorg/bukkit/block/spawner/SpawnRule;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(snapshot.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Int(spawn_weight);
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(spawn_rule.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/block/spawner/SpawnerEntry"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::block::spawner::SpawnerEntry::from_raw(&jni,res
)}
	pub fn snapshot(&self) 
-> Result<crate::entity::EntitySnapshot<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::entity::EntitySnapshot;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSnapshot",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::EntitySnapshot::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_snapshot(&self,snapshot: impl Into<crate::entity::EntitySnapshot<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/EntitySnapshot;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(snapshot.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setSnapshot",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn spawn_weight(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSpawnWeight",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn set_spawn_weight(&self,spawn_weight: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)L();");
let val_1 = jni::objects::JValueGen::Int(spawn_weight);
let res = self.jni_ref().call_method(&self.jni_object(),"setSpawnWeight",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn spawn_rule(&self) 
-> Result<Option<crate::block::spawner::SpawnRule<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::block::spawner::SpawnRule;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSpawnRule",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::block::spawner::SpawnRule::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn set_spawn_rule(&self,spawn_rule: impl Into<crate::block::spawner::SpawnRule<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/spawner/SpawnRule;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(spawn_rule.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setSpawnRule",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
// SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct SpawnRule<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for SpawnRule<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for SpawnRule<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate SpawnRule from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/block/spawner/SpawnRule")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a SpawnRule object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> SpawnRule<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,min_block_light: i32,max_block_light: i32,min_sky_light: i32,max_sky_light: i32) 
-> Result<crate::block::spawner::SpawnRule<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(IIII)V");
let val_1 = jni::objects::JValueGen::Int(min_block_light);
let val_2 = jni::objects::JValueGen::Int(max_block_light);
let val_3 = jni::objects::JValueGen::Int(min_sky_light);
let val_4 = jni::objects::JValueGen::Int(max_sky_light);
let cls = jni.find_class("org/bukkit/block/spawner/SpawnRule"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::block::spawner::SpawnRule::from_raw(&jni,res
)}
	pub fn min_block_light(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMinBlockLight",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn set_min_block_light(&self,min_block_light: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)L();");
let val_1 = jni::objects::JValueGen::Int(min_block_light);
let res = self.jni_ref().call_method(&self.jni_object(),"setMinBlockLight",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn max_block_light(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMaxBlockLight",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn set_max_block_light(&self,max_block_light: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)L();");
let val_1 = jni::objects::JValueGen::Int(max_block_light);
let res = self.jni_ref().call_method(&self.jni_object(),"setMaxBlockLight",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn min_sky_light(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMinSkyLight",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn set_min_sky_light(&self,min_sky_light: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)L();");
let val_1 = jni::objects::JValueGen::Int(min_sky_light);
let res = self.jni_ref().call_method(&self.jni_object(),"setMinSkyLight",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn max_sky_light(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMaxSkyLight",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn set_max_sky_light(&self,max_sky_light: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)L();");
let val_1 = jni::objects::JValueGen::Int(max_sky_light);
let res = self.jni_ref().call_method(&self.jni_object(),"setMaxSkyLight",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn equals(&self,obj: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(obj);
let res = self.jni_ref().call_method(&self.jni_object(),"equals",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn hash_code(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"hashCode",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn clone(&self) 
-> Result<crate::block::spawner::SpawnRule<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::block::spawner::SpawnRule;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::spawner::SpawnRule::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn serialize(&self) 
-> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lblackboxmc_java::util::Map;");
let res = self.jni_ref().call_method(&self.jni_object(),"serialize",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn deserialize(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_args: impl Into<blackboxmc_java::util::JavaMap<'mc>>) 
-> Result<crate::block::spawner::SpawnRule<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/util/Map;)Lcrate::block::spawner::SpawnRule;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_args.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/block/spawner/SpawnRule"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"deserialize",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::block::spawner::SpawnRule::from_raw(&jni,obj
)}
// SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::configuration::serialization::ConfigurationSerializable<'mc>> for SpawnRule<'mc>{

fn into(self) -> crate::configuration::serialization::ConfigurationSerializable<'mc> {

crate::configuration::serialization::ConfigurationSerializable::from_raw(&self.jni_ref(), self.1).expect("Error converting SpawnRule into crate::configuration::serialization::ConfigurationSerializable")

   }
}
