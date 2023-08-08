#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// An instantiatable struct that implements ProjectileSource. Needed for returning it from Java.
pub struct ProjectileSource<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ProjectileSource<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ProjectileSource from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "ProjectileSource")?;
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
    pub fn launch_projectile_with_class(
        &mut self,
        arg0: std::option::Option<jni::objects::JClass<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::util::Vector<'mc>>>,
    ) -> Result<crate::entity::Projectile<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "launchProjectile",
            "(Ljava/lang/Class;Lorg/bukkit/util/Vector;)Lorg/bukkit/entity/Projectile;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Projectile::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for ProjectileSource<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements BlockProjectileSource. Needed for returning it from Java.
pub struct BlockProjectileSource<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> BlockProjectileSource<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate BlockProjectileSource from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "BlockProjectileSource")?;
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
    pub fn block(&mut self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlock",
            "()Lorg/bukkit/block/Block;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Block::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn launch_projectile_with_class(
        &mut self,
        arg0: std::option::Option<jni::objects::JClass<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::util::Vector<'mc>>>,
    ) -> Result<crate::entity::Projectile<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "launchProjectile",
            "(Ljava/lang/Class;Lorg/bukkit/util/Vector;)Lorg/bukkit/entity/Projectile;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Projectile::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for BlockProjectileSource<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::projectiles::ProjectileSource<'mc>> for BlockProjectileSource<'mc> {
    fn into(self) -> crate::projectiles::ProjectileSource<'mc> {
        crate::projectiles::ProjectileSource::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
