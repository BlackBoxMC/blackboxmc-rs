#![allow(deprecated)]
use crate::JNIRaw;
/// An instantiatable struct that implements ProjectileSource. Needed for returning it from Java.
pub struct ProjectileSource<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ProjectileSource<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ProjectileSource from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("ProjectileSource") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ProjectileSource object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn launch_projectile_with_class(
        &mut self,
        arg0: std::option::Option<jni::objects::JClass<'mc>>,
        arg1: std::option::Option<impl Into<crate::bukkit::util::Vector<'mc>>>,
    ) -> Result<crate::bukkit::entity::Projectile<'mc>, Box<dyn std::error::Error>> {
        let val_0 = arg0.unwrap();
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "launchProjectile",
            "(Ljava/lang/Class;Lorg/bukkit/util/Vector;)Lorg/bukkit/entity/Projectile;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = {
            crate::bukkit::entity::Projectile(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for ProjectileSource<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements BlockProjectileSource. Needed for returning it from Java.
pub struct BlockProjectileSource<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> BlockProjectileSource<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate BlockProjectileSource from null object."
            )
            .into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("BlockProjectileSource") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockProjectileSource object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn block(
        &mut self,
    ) -> Result<crate::bukkit::block::Block<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlock",
            "()Lorg/bukkit/block/Block;",
            &[],
        )?;
        let ret = {
            crate::bukkit::block::Block(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn launch_projectile_with_class(
        &mut self,
        arg0: std::option::Option<jni::objects::JClass<'mc>>,
        arg1: std::option::Option<impl Into<crate::bukkit::util::Vector<'mc>>>,
    ) -> Result<crate::bukkit::entity::Projectile<'mc>, Box<dyn std::error::Error>> {
        let val_0 = arg0.unwrap();
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "launchProjectile",
            "(Ljava/lang/Class;Lorg/bukkit/util/Vector;)Lorg/bukkit/entity/Projectile;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = {
            crate::bukkit::entity::Projectile(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for BlockProjectileSource<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::projectiles::ProjectileSource<'mc>> for BlockProjectileSource<'mc> {
    fn into(self) -> crate::bukkit::projectiles::ProjectileSource<'mc> {
        crate::bukkit::projectiles::ProjectileSource::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
