#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;/*org/bukkit/ban/mod.rs*/

#[repr(C)]
pub struct ProfileBanList<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ProfileBanList<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ProfileBanList<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ProfileBanList from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/ban/ProfileBanList")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ProfileBanList object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ProfileBanListTrait<'mc> for ProfileBanList<'mc> {}
pub trait ProfileBanListTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// {@inheritDoc}
	fn add_ban(&self,target: impl Into<crate::profile::PlayerProfile<'mc>>,reason: impl Into<String>,expires: impl Into<blackboxmc_java::util::JavaDate<'mc>>,source: impl Into<String>) 
-> Result<Option<crate::BanEntry<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/profile/PlayerProfile;Ljava/lang/String;Ljava/util/Date;Ljava/lang/String;)Lorg/bukkit/BanEntry;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(target.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(reason.into())?));
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(expires.into().jni_object().clone())});
let val_4 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(source.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"addBan",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::BanEntry::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::BanList<'mc>> for ProfileBanList<'mc>{

fn into(self) -> crate::BanList<'mc> {

crate::BanList::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting ProfileBanList into crate::BanList")

   }
}
impl<'mc> crate::BanListTrait<'mc> for ProfileBanList<'mc> {}
#[repr(C)]
pub struct IpBanList<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for IpBanList<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for IpBanList<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate IpBanList from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/ban/IpBanList")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a IpBanList object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> IpBanListTrait<'mc> for IpBanList<'mc> {}
pub trait IpBanListTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::BanList<'mc>> for IpBanList<'mc>{

fn into(self) -> crate::BanList<'mc> {

crate::BanList::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting IpBanList into crate::BanList")

   }
}
impl<'mc> crate::BanListTrait<'mc> for IpBanList<'mc> {}
