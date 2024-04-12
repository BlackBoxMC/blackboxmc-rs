#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;/*org/bukkit/projectiles/mod.rs*/

#[repr(C)]
pub struct BlockProjectileSource<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockProjectileSource<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockProjectileSource<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockProjectileSource from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/projectiles/BlockProjectileSource")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockProjectileSource object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockProjectileSourceTrait<'mc> for BlockProjectileSource<'mc> {}
pub trait BlockProjectileSourceTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Gets the block this projectile source belongs to.
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
impl<'mc> Into<crate::projectiles::ProjectileSource<'mc>> for BlockProjectileSource<'mc>{

fn into(self) -> crate::projectiles::ProjectileSource<'mc> {

crate::projectiles::ProjectileSource::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockProjectileSource into crate::projectiles::ProjectileSource")

   }
}
impl<'mc> crate::projectiles::ProjectileSourceTrait<'mc> for BlockProjectileSource<'mc> {}
#[repr(C)]
pub struct ProjectileSource<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ProjectileSource<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ProjectileSource<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ProjectileSource from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/projectiles/ProjectileSource")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ProjectileSource object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ProjectileSourceTrait<'mc> for ProjectileSource<'mc> {}
pub trait ProjectileSourceTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Launches a {@link Projectile} from the ProjectileSource with an
/// initial velocity.
	fn launch_projectile(&self,projectile: jni::objects::JClass<'mc>,velocity: std::option::Option<impl Into<crate::util::Vector<'mc>>>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/Class;";
let val_1 = jni::objects::JValueGen::Object(projectile.into());
args.push(val_1);
if let Some(a) = velocity {
sig += "Lorg/bukkit/util/Vector;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")LT;";
let res = self.jni_ref().call_method(&self.jni_object(),"launchProjectile",sig.as_str(),args);
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
