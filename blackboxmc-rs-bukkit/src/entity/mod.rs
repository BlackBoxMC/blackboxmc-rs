#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// An instantiatable struct that implements Entity. Needed for returning it from Java.
pub struct Entity<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Entity<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Entity from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Entity")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Entity object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Entity<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::metadata::Metadatable<'mc>> for Entity<'mc> {
    fn into(self) -> crate::metadata::Metadatable<'mc> {
        crate::metadata::Metadatable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::command::CommandSender<'mc>> for Entity<'mc> {
    fn into(self) -> crate::command::CommandSender<'mc> {
        crate::command::CommandSender::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::Nameable<'mc>> for Entity<'mc> {
    fn into(self) -> crate::Nameable<'mc> {
        crate::Nameable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::persistence::PersistentDataHolder<'mc>> for Entity<'mc> {
    fn into(self) -> crate::persistence::PersistentDataHolder<'mc> {
        crate::persistence::PersistentDataHolder::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Creature. Needed for returning it from Java.
pub struct Creature<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Creature<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Creature from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Creature")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Creature object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Creature<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Mob<'mc>> for Creature<'mc> {
    fn into(self) -> crate::entity::Mob<'mc> {
        crate::entity::Mob::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Donkey. Needed for returning it from Java.
pub struct Donkey<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Donkey<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Donkey from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Donkey")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Donkey object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Donkey<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::ChestedHorse<'mc>> for Donkey<'mc> {
    fn into(self) -> crate::entity::ChestedHorse<'mc> {
        crate::entity::ChestedHorse::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct FishHookHookState<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for FishHookHookState<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> FishHookHookState<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate FishHookHookState from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "FishHookHookState")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FishHookHookState object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
/// An instantiatable struct that implements Golem. Needed for returning it from Java.
pub struct Golem<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Golem<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Golem from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Golem")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Golem object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Golem<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Creature<'mc>> for Golem<'mc> {
    fn into(self) -> crate::entity::Creature<'mc> {
        crate::entity::Creature::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements EnderDragon. Needed for returning it from Java.
pub struct EnderDragon<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> EnderDragon<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate EnderDragon from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "EnderDragon")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EnderDragon object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for EnderDragon<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::ComplexLivingEntity<'mc>> for EnderDragon<'mc> {
    fn into(self) -> crate::entity::ComplexLivingEntity<'mc> {
        crate::entity::ComplexLivingEntity::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::entity::Boss<'mc>> for EnderDragon<'mc> {
    fn into(self) -> crate::entity::Boss<'mc> {
        crate::entity::Boss::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::entity::Mob<'mc>> for EnderDragon<'mc> {
    fn into(self) -> crate::entity::Mob<'mc> {
        crate::entity::Mob::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::entity::Enemy<'mc>> for EnderDragon<'mc> {
    fn into(self) -> crate::entity::Enemy<'mc> {
        crate::entity::Enemy::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Trident. Needed for returning it from Java.
pub struct Trident<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Trident<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Trident from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Trident")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Trident object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Trident<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::AbstractArrow<'mc>> for Trident<'mc> {
    fn into(self) -> crate::entity::AbstractArrow<'mc> {
        crate::entity::AbstractArrow::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::entity::ThrowableProjectile<'mc>> for Trident<'mc> {
    fn into(self) -> crate::entity::ThrowableProjectile<'mc> {
        crate::entity::ThrowableProjectile::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Phantom. Needed for returning it from Java.
pub struct Phantom<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Phantom<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Phantom from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Phantom")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Phantom object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Phantom<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Flying<'mc>> for Phantom<'mc> {
    fn into(self) -> crate::entity::Flying<'mc> {
        crate::entity::Flying::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::entity::Enemy<'mc>> for Phantom<'mc> {
    fn into(self) -> crate::entity::Enemy<'mc> {
        crate::entity::Enemy::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Piglin. Needed for returning it from Java.
pub struct Piglin<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Piglin<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Piglin from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Piglin")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Piglin object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Piglin<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::PiglinAbstract<'mc>> for Piglin<'mc> {
    fn into(self) -> crate::entity::PiglinAbstract<'mc> {
        crate::entity::PiglinAbstract::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::inventory::InventoryHolder<'mc>> for Piglin<'mc> {
    fn into(self) -> crate::inventory::InventoryHolder<'mc> {
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements AbstractVillager. Needed for returning it from Java.
pub struct AbstractVillager<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> AbstractVillager<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate AbstractVillager from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "AbstractVillager")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AbstractVillager object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for AbstractVillager<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Breedable<'mc>> for AbstractVillager<'mc> {
    fn into(self) -> crate::entity::Breedable<'mc> {
        crate::entity::Breedable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::entity::NPC<'mc>> for AbstractVillager<'mc> {
    fn into(self) -> crate::entity::NPC<'mc> {
        crate::entity::NPC::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::inventory::InventoryHolder<'mc>> for AbstractVillager<'mc> {
    fn into(self) -> crate::inventory::InventoryHolder<'mc> {
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::inventory::Merchant<'mc>> for AbstractVillager<'mc> {
    fn into(self) -> crate::inventory::Merchant<'mc> {
        crate::inventory::Merchant::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Arrow. Needed for returning it from Java.
pub struct Arrow<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Arrow<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Arrow from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Arrow")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Arrow object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Arrow<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::AbstractArrow<'mc>> for Arrow<'mc> {
    fn into(self) -> crate::entity::AbstractArrow<'mc> {
        crate::entity::AbstractArrow::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements LightningStrike. Needed for returning it from Java.
pub struct LightningStrike<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> LightningStrike<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate LightningStrike from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "LightningStrike")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LightningStrike object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for LightningStrike<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Entity<'mc>> for LightningStrike<'mc> {
    fn into(self) -> crate::entity::Entity<'mc> {
        crate::entity::Entity::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements GlowSquid. Needed for returning it from Java.
pub struct GlowSquid<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> GlowSquid<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate GlowSquid from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "GlowSquid")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a GlowSquid object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for GlowSquid<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Squid<'mc>> for GlowSquid<'mc> {
    fn into(self) -> crate::entity::Squid<'mc> {
        crate::entity::Squid::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct WitherHead<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for WitherHead<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> WitherHead<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate WitherHead from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "WitherHead")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a WitherHead object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
/// An instantiatable struct that implements ElderGuardian. Needed for returning it from Java.
pub struct ElderGuardian<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ElderGuardian<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ElderGuardian from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "ElderGuardian")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ElderGuardian object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for ElderGuardian<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Guardian<'mc>> for ElderGuardian<'mc> {
    fn into(self) -> crate::entity::Guardian<'mc> {
        crate::entity::Guardian::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements EvokerFangs. Needed for returning it from Java.
pub struct EvokerFangs<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> EvokerFangs<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate EvokerFangs from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "EvokerFangs")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EvokerFangs object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for EvokerFangs<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Entity<'mc>> for EvokerFangs<'mc> {
    fn into(self) -> crate::entity::Entity<'mc> {
        crate::entity::Entity::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Ravager. Needed for returning it from Java.
pub struct Ravager<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Ravager<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Ravager from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Ravager")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Ravager object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Ravager<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Raider<'mc>> for Ravager<'mc> {
    fn into(self) -> crate::entity::Raider<'mc> {
        crate::entity::Raider::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements WitherSkeleton. Needed for returning it from Java.
pub struct WitherSkeleton<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> WitherSkeleton<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate WitherSkeleton from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "WitherSkeleton")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a WitherSkeleton object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for WitherSkeleton<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::AbstractSkeleton<'mc>> for WitherSkeleton<'mc> {
    fn into(self) -> crate::entity::AbstractSkeleton<'mc> {
        crate::entity::AbstractSkeleton::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Firework. Needed for returning it from Java.
pub struct Firework<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Firework<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Firework from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Firework")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Firework object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Firework<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Projectile<'mc>> for Firework<'mc> {
    fn into(self) -> crate::entity::Projectile<'mc> {
        crate::entity::Projectile::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Chicken. Needed for returning it from Java.
pub struct Chicken<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Chicken<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Chicken from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Chicken")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Chicken object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Chicken<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Animals<'mc>> for Chicken<'mc> {
    fn into(self) -> crate::entity::Animals<'mc> {
        crate::entity::Animals::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub enum SpawnCategoryEnum {
    Monster,
    Animal,
    WaterAnimal,
    WaterAmbient,
    WaterUndergroundCreature,
    Ambient,
    Axolotl,
    Misc,
}
impl std::fmt::Display for SpawnCategoryEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            SpawnCategoryEnum::Monster => f.write_str("MONSTER"),
            SpawnCategoryEnum::Animal => f.write_str("ANIMAL"),
            SpawnCategoryEnum::WaterAnimal => f.write_str("WATER_ANIMAL"),
            SpawnCategoryEnum::WaterAmbient => f.write_str("WATER_AMBIENT"),
            SpawnCategoryEnum::WaterUndergroundCreature => {
                f.write_str("WATER_UNDERGROUND_CREATURE")
            }
            SpawnCategoryEnum::Ambient => f.write_str("AMBIENT"),
            SpawnCategoryEnum::Axolotl => f.write_str("AXOLOTL"),
            SpawnCategoryEnum::Misc => f.write_str("MISC"),
        }
    }
}
pub struct SpawnCategory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub SpawnCategoryEnum,
);
impl<'mc> std::ops::Deref for SpawnCategory<'mc> {
    type Target = SpawnCategoryEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for SpawnCategory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> SpawnCategory<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: SpawnCategoryEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SpawnCategory from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "SpawnCategory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SpawnCategory object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const MONSTER: SpawnCategoryEnum = SpawnCategoryEnum::Monster;
    pub const ANIMAL: SpawnCategoryEnum = SpawnCategoryEnum::Animal;
    pub const WATER_ANIMAL: SpawnCategoryEnum = SpawnCategoryEnum::WaterAnimal;
    pub const WATER_AMBIENT: SpawnCategoryEnum = SpawnCategoryEnum::WaterAmbient;
    pub const WATER_UNDERGROUND_CREATURE: SpawnCategoryEnum =
        SpawnCategoryEnum::WaterUndergroundCreature;
    pub const AMBIENT: SpawnCategoryEnum = SpawnCategoryEnum::Ambient;
    pub const AXOLOTL: SpawnCategoryEnum = SpawnCategoryEnum::Axolotl;
    pub const MISC: SpawnCategoryEnum = SpawnCategoryEnum::Misc;
    pub fn from_string(str: String) -> std::option::Option<SpawnCategoryEnum> {
        match str.as_str() {
            "MONSTER" => Some(SpawnCategoryEnum::Monster),
            "ANIMAL" => Some(SpawnCategoryEnum::Animal),
            "WATER_ANIMAL" => Some(SpawnCategoryEnum::WaterAnimal),
            "WATER_AMBIENT" => Some(SpawnCategoryEnum::WaterAmbient),
            "WATER_UNDERGROUND_CREATURE" => Some(SpawnCategoryEnum::WaterUndergroundCreature),
            "AMBIENT" => Some(SpawnCategoryEnum::Ambient),
            "AXOLOTL" => Some(SpawnCategoryEnum::Axolotl),
            "MISC" => Some(SpawnCategoryEnum::Misc),
            _ => None,
        }
    }
}
pub struct HorseStyle<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for HorseStyle<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> HorseStyle<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate HorseStyle from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "HorseStyle")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a HorseStyle object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
/// An instantiatable struct that implements AbstractArrow. Needed for returning it from Java.
pub struct AbstractArrow<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> AbstractArrow<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate AbstractArrow from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "AbstractArrow")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AbstractArrow object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for AbstractArrow<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Projectile<'mc>> for AbstractArrow<'mc> {
    fn into(self) -> crate::entity::Projectile<'mc> {
        crate::entity::Projectile::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Endermite. Needed for returning it from Java.
pub struct Endermite<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Endermite<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Endermite from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Endermite")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Endermite object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Endermite<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Monster<'mc>> for Endermite<'mc> {
    fn into(self) -> crate::entity::Monster<'mc> {
        crate::entity::Monster::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Vindicator. Needed for returning it from Java.
pub struct Vindicator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Vindicator<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Vindicator from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Vindicator")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Vindicator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Vindicator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Illager<'mc>> for Vindicator<'mc> {
    fn into(self) -> crate::entity::Illager<'mc> {
        crate::entity::Illager::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Warden. Needed for returning it from Java.
pub struct Warden<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Warden<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Warden from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Warden")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Warden object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Warden<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Monster<'mc>> for Warden<'mc> {
    fn into(self) -> crate::entity::Monster<'mc> {
        crate::entity::Monster::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements TraderLlama. Needed for returning it from Java.
pub struct TraderLlama<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> TraderLlama<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate TraderLlama from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "TraderLlama")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TraderLlama object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for TraderLlama<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Llama<'mc>> for TraderLlama<'mc> {
    fn into(self) -> crate::entity::Llama<'mc> {
        crate::entity::Llama::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct AxolotlVariant<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for AxolotlVariant<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> AxolotlVariant<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate AxolotlVariant from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "AxolotlVariant")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AxolotlVariant object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
/// An instantiatable struct that implements TextDisplay. Needed for returning it from Java.
pub struct TextDisplay<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> TextDisplay<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate TextDisplay from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "TextDisplay")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TextDisplay object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for TextDisplay<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Display<'mc>> for TextDisplay<'mc> {
    fn into(self) -> crate::entity::Display<'mc> {
        crate::entity::Display::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Illager. Needed for returning it from Java.
pub struct Illager<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Illager<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Illager from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Illager")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Illager object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Illager<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Raider<'mc>> for Illager<'mc> {
    fn into(self) -> crate::entity::Raider<'mc> {
        crate::entity::Raider::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Fish. Needed for returning it from Java.
pub struct Fish<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Fish<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Fish from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Fish")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Fish object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Fish<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::WaterMob<'mc>> for Fish<'mc> {
    fn into(self) -> crate::entity::WaterMob<'mc> {
        crate::entity::WaterMob::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Wolf. Needed for returning it from Java.
pub struct Wolf<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Wolf<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Wolf from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Wolf")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Wolf object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Wolf<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Tameable<'mc>> for Wolf<'mc> {
    fn into(self) -> crate::entity::Tameable<'mc> {
        crate::entity::Tameable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::entity::Sittable<'mc>> for Wolf<'mc> {
    fn into(self) -> crate::entity::Sittable<'mc> {
        crate::entity::Sittable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct OcelotType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for OcelotType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> OcelotType<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate OcelotType from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "OcelotType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a OcelotType object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
/// An instantiatable struct that implements Wither. Needed for returning it from Java.
pub struct Wither<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Wither<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Wither from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Wither")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Wither object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Wither<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Monster<'mc>> for Wither<'mc> {
    fn into(self) -> crate::entity::Monster<'mc> {
        crate::entity::Monster::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::entity::Boss<'mc>> for Wither<'mc> {
    fn into(self) -> crate::entity::Boss<'mc> {
        crate::entity::Boss::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct FoxType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for FoxType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> FoxType<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate FoxType from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "FoxType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FoxType object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
/// An instantiatable struct that implements Bee. Needed for returning it from Java.
pub struct Bee<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Bee<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Bee from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Bee")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Bee object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Bee<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Animals<'mc>> for Bee<'mc> {
    fn into(self) -> crate::entity::Animals<'mc> {
        crate::entity::Animals::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Damageable. Needed for returning it from Java.
pub struct Damageable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Damageable<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Damageable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Damageable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Damageable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Damageable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Entity<'mc>> for Damageable<'mc> {
    fn into(self) -> crate::entity::Entity<'mc> {
        crate::entity::Entity::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Breedable. Needed for returning it from Java.
pub struct Breedable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Breedable<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Breedable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Breedable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Breedable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Breedable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Ageable<'mc>> for Breedable<'mc> {
    fn into(self) -> crate::entity::Ageable<'mc> {
        crate::entity::Ageable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct PlayerSpigot<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PlayerSpigot<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PlayerSpigot<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PlayerSpigot from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "PlayerSpigot")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PlayerSpigot object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::entity::EntitySpigot<'mc>> for PlayerSpigot<'mc> {
    fn into(self) -> crate::entity::EntitySpigot<'mc> {
        crate::entity::EntitySpigot::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Husk. Needed for returning it from Java.
pub struct Husk<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Husk<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Husk from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Husk")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Husk object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Husk<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Zombie<'mc>> for Husk<'mc> {
    fn into(self) -> crate::entity::Zombie<'mc> {
        crate::entity::Zombie::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements AreaEffectCloud. Needed for returning it from Java.
pub struct AreaEffectCloud<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> AreaEffectCloud<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate AreaEffectCloud from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "AreaEffectCloud")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AreaEffectCloud object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for AreaEffectCloud<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Entity<'mc>> for AreaEffectCloud<'mc> {
    fn into(self) -> crate::entity::Entity<'mc> {
        crate::entity::Entity::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Slime. Needed for returning it from Java.
pub struct Slime<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Slime<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Slime from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Slime")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Slime object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Slime<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Mob<'mc>> for Slime<'mc> {
    fn into(self) -> crate::entity::Mob<'mc> {
        crate::entity::Mob::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::entity::Enemy<'mc>> for Slime<'mc> {
    fn into(self) -> crate::entity::Enemy<'mc> {
        crate::entity::Enemy::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Ambient. Needed for returning it from Java.
pub struct Ambient<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Ambient<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Ambient from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Ambient")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Ambient object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Ambient<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Mob<'mc>> for Ambient<'mc> {
    fn into(self) -> crate::entity::Mob<'mc> {
        crate::entity::Mob::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Llama. Needed for returning it from Java.
pub struct Llama<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Llama<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Llama from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Llama")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Llama object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Llama<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::ChestedHorse<'mc>> for Llama<'mc> {
    fn into(self) -> crate::entity::ChestedHorse<'mc> {
        crate::entity::ChestedHorse::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct WardenAngerLevel<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for WardenAngerLevel<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> WardenAngerLevel<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate WardenAngerLevel from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "WardenAngerLevel")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a WardenAngerLevel object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
/// An instantiatable struct that implements PigZombie. Needed for returning it from Java.
pub struct PigZombie<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> PigZombie<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PigZombie from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "PigZombie")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PigZombie object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for PigZombie<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Zombie<'mc>> for PigZombie<'mc> {
    fn into(self) -> crate::entity::Zombie<'mc> {
        crate::entity::Zombie::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Snowman. Needed for returning it from Java.
pub struct Snowman<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Snowman<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Snowman from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Snowman")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Snowman object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Snowman<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Golem<'mc>> for Snowman<'mc> {
    fn into(self) -> crate::entity::Golem<'mc> {
        crate::entity::Golem::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct RabbitType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for RabbitType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> RabbitType<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate RabbitType from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "RabbitType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a RabbitType object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
/// An instantiatable struct that implements Zoglin. Needed for returning it from Java.
pub struct Zoglin<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Zoglin<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Zoglin from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Zoglin")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Zoglin object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Zoglin<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Monster<'mc>> for Zoglin<'mc> {
    fn into(self) -> crate::entity::Monster<'mc> {
        crate::entity::Monster::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::entity::Ageable<'mc>> for Zoglin<'mc> {
    fn into(self) -> crate::entity::Ageable<'mc> {
        crate::entity::Ageable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Display. Needed for returning it from Java.
pub struct Display<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Display<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Display from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Display")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Display object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Display<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Entity<'mc>> for Display<'mc> {
    fn into(self) -> crate::entity::Entity<'mc> {
        crate::entity::Entity::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct BoatStatus<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for BoatStatus<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BoatStatus<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BoatStatus from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "BoatStatus")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BoatStatus object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
/// An instantiatable struct that implements Minecart. Needed for returning it from Java.
pub struct Minecart<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Minecart<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Minecart from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Minecart")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Minecart object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Minecart<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Vehicle<'mc>> for Minecart<'mc> {
    fn into(self) -> crate::entity::Vehicle<'mc> {
        crate::entity::Vehicle::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Vehicle. Needed for returning it from Java.
pub struct Vehicle<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Vehicle<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Vehicle from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Vehicle")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Vehicle object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Vehicle<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Entity<'mc>> for Vehicle<'mc> {
    fn into(self) -> crate::entity::Entity<'mc> {
        crate::entity::Entity::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements FallingBlock. Needed for returning it from Java.
pub struct FallingBlock<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> FallingBlock<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate FallingBlock from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "FallingBlock")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FallingBlock object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for FallingBlock<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Entity<'mc>> for FallingBlock<'mc> {
    fn into(self) -> crate::entity::Entity<'mc> {
        crate::entity::Entity::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Explosive. Needed for returning it from Java.
pub struct Explosive<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Explosive<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Explosive from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Explosive")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Explosive object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Explosive<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Entity<'mc>> for Explosive<'mc> {
    fn into(self) -> crate::entity::Entity<'mc> {
        crate::entity::Entity::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Hanging. Needed for returning it from Java.
pub struct Hanging<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Hanging<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Hanging from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Hanging")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Hanging object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Hanging<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Entity<'mc>> for Hanging<'mc> {
    fn into(self) -> crate::entity::Entity<'mc> {
        crate::entity::Entity::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::material::Attachable<'mc>> for Hanging<'mc> {
    fn into(self) -> crate::material::Attachable<'mc> {
        crate::material::Attachable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements EnderSignal. Needed for returning it from Java.
pub struct EnderSignal<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> EnderSignal<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate EnderSignal from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "EnderSignal")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EnderSignal object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for EnderSignal<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Entity<'mc>> for EnderSignal<'mc> {
    fn into(self) -> crate::entity::Entity<'mc> {
        crate::entity::Entity::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Enemy. Needed for returning it from Java.
pub struct Enemy<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Enemy<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Enemy from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Enemy")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Enemy object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Enemy<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::LivingEntity<'mc>> for Enemy<'mc> {
    fn into(self) -> crate::entity::LivingEntity<'mc> {
        crate::entity::LivingEntity::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub enum PoseEnum {
    Standing,
    FallFlying,
    Sleeping,
    Swimming,
    SpinAttack,
    Sneaking,
    LongJumping,
    Dying,
    Croaking,
    UsingTongue,
    Sitting,
    Roaring,
    Sniffing,
    Emerging,
    Digging,
}
impl std::fmt::Display for PoseEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            PoseEnum::Standing => f.write_str("STANDING"),
            PoseEnum::FallFlying => f.write_str("FALL_FLYING"),
            PoseEnum::Sleeping => f.write_str("SLEEPING"),
            PoseEnum::Swimming => f.write_str("SWIMMING"),
            PoseEnum::SpinAttack => f.write_str("SPIN_ATTACK"),
            PoseEnum::Sneaking => f.write_str("SNEAKING"),
            PoseEnum::LongJumping => f.write_str("LONG_JUMPING"),
            PoseEnum::Dying => f.write_str("DYING"),
            PoseEnum::Croaking => f.write_str("CROAKING"),
            PoseEnum::UsingTongue => f.write_str("USING_TONGUE"),
            PoseEnum::Sitting => f.write_str("SITTING"),
            PoseEnum::Roaring => f.write_str("ROARING"),
            PoseEnum::Sniffing => f.write_str("SNIFFING"),
            PoseEnum::Emerging => f.write_str("EMERGING"),
            PoseEnum::Digging => f.write_str("DIGGING"),
        }
    }
}
pub struct Pose<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub PoseEnum,
);
impl<'mc> std::ops::Deref for Pose<'mc> {
    type Target = PoseEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for Pose<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Pose<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: PoseEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Pose from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Pose")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Pose object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const STANDING: PoseEnum = PoseEnum::Standing;
    pub const FALL_FLYING: PoseEnum = PoseEnum::FallFlying;
    pub const SLEEPING: PoseEnum = PoseEnum::Sleeping;
    pub const SWIMMING: PoseEnum = PoseEnum::Swimming;
    pub const SPIN_ATTACK: PoseEnum = PoseEnum::SpinAttack;
    pub const SNEAKING: PoseEnum = PoseEnum::Sneaking;
    pub const LONG_JUMPING: PoseEnum = PoseEnum::LongJumping;
    pub const DYING: PoseEnum = PoseEnum::Dying;
    pub const CROAKING: PoseEnum = PoseEnum::Croaking;
    pub const USING_TONGUE: PoseEnum = PoseEnum::UsingTongue;
    pub const SITTING: PoseEnum = PoseEnum::Sitting;
    pub const ROARING: PoseEnum = PoseEnum::Roaring;
    pub const SNIFFING: PoseEnum = PoseEnum::Sniffing;
    pub const EMERGING: PoseEnum = PoseEnum::Emerging;
    pub const DIGGING: PoseEnum = PoseEnum::Digging;
    pub fn from_string(str: String) -> std::option::Option<PoseEnum> {
        match str.as_str() {
            "STANDING" => Some(PoseEnum::Standing),
            "FALL_FLYING" => Some(PoseEnum::FallFlying),
            "SLEEPING" => Some(PoseEnum::Sleeping),
            "SWIMMING" => Some(PoseEnum::Swimming),
            "SPIN_ATTACK" => Some(PoseEnum::SpinAttack),
            "SNEAKING" => Some(PoseEnum::Sneaking),
            "LONG_JUMPING" => Some(PoseEnum::LongJumping),
            "DYING" => Some(PoseEnum::Dying),
            "CROAKING" => Some(PoseEnum::Croaking),
            "USING_TONGUE" => Some(PoseEnum::UsingTongue),
            "SITTING" => Some(PoseEnum::Sitting),
            "ROARING" => Some(PoseEnum::Roaring),
            "SNIFFING" => Some(PoseEnum::Sniffing),
            "EMERGING" => Some(PoseEnum::Emerging),
            "DIGGING" => Some(PoseEnum::Digging),
            _ => None,
        }
    }
}
/// An instantiatable struct that implements Turtle. Needed for returning it from Java.
pub struct Turtle<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Turtle<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Turtle from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Turtle")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Turtle object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Turtle<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Animals<'mc>> for Turtle<'mc> {
    fn into(self) -> crate::entity::Animals<'mc> {
        crate::entity::Animals::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Raider. Needed for returning it from Java.
pub struct Raider<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Raider<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Raider from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Raider")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Raider object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Raider<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Monster<'mc>> for Raider<'mc> {
    fn into(self) -> crate::entity::Monster<'mc> {
        crate::entity::Monster::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Interaction. Needed for returning it from Java.
pub struct Interaction<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Interaction<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Interaction from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Interaction")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Interaction object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Interaction<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Entity<'mc>> for Interaction<'mc> {
    fn into(self) -> crate::entity::Entity<'mc> {
        crate::entity::Entity::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements PiglinBrute. Needed for returning it from Java.
pub struct PiglinBrute<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> PiglinBrute<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PiglinBrute from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "PiglinBrute")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PiglinBrute object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for PiglinBrute<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::PiglinAbstract<'mc>> for PiglinBrute<'mc> {
    fn into(self) -> crate::entity::PiglinAbstract<'mc> {
        crate::entity::PiglinAbstract::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Marker. Needed for returning it from Java.
pub struct Marker<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Marker<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Marker from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Marker")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Marker object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Marker<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Entity<'mc>> for Marker<'mc> {
    fn into(self) -> crate::entity::Entity<'mc> {
        crate::entity::Entity::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct EnderDragonPhase<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EnderDragonPhase<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EnderDragonPhase<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EnderDragonPhase from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "EnderDragonPhase")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EnderDragonPhase object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
/// An instantiatable struct that implements Fox. Needed for returning it from Java.
pub struct Fox<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Fox<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Fox from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Fox")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Fox object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Fox<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Animals<'mc>> for Fox<'mc> {
    fn into(self) -> crate::entity::Animals<'mc> {
        crate::entity::Animals::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::entity::Sittable<'mc>> for Fox<'mc> {
    fn into(self) -> crate::entity::Sittable<'mc> {
        crate::entity::Sittable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements LargeFireball. Needed for returning it from Java.
pub struct LargeFireball<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> LargeFireball<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate LargeFireball from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "LargeFireball")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LargeFireball object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for LargeFireball<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::SizedFireball<'mc>> for LargeFireball<'mc> {
    fn into(self) -> crate::entity::SizedFireball<'mc> {
        crate::entity::SizedFireball::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Ocelot. Needed for returning it from Java.
pub struct Ocelot<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Ocelot<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Ocelot from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Ocelot")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Ocelot object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Ocelot<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Animals<'mc>> for Ocelot<'mc> {
    fn into(self) -> crate::entity::Animals<'mc> {
        crate::entity::Animals::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Drowned. Needed for returning it from Java.
pub struct Drowned<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Drowned<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Drowned from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Drowned")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Drowned object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Drowned<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Zombie<'mc>> for Drowned<'mc> {
    fn into(self) -> crate::entity::Zombie<'mc> {
        crate::entity::Zombie::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct ParrotVariant<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ParrotVariant<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ParrotVariant<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ParrotVariant from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "ParrotVariant")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ParrotVariant object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
/// An instantiatable struct that implements Enderman. Needed for returning it from Java.
pub struct Enderman<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Enderman<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Enderman from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Enderman")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Enderman object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Enderman<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Monster<'mc>> for Enderman<'mc> {
    fn into(self) -> crate::entity::Monster<'mc> {
        crate::entity::Monster::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements ThrownExpBottle. Needed for returning it from Java.
pub struct ThrownExpBottle<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ThrownExpBottle<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ThrownExpBottle from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "ThrownExpBottle")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ThrownExpBottle object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for ThrownExpBottle<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::ThrowableProjectile<'mc>> for ThrownExpBottle<'mc> {
    fn into(self) -> crate::entity::ThrowableProjectile<'mc> {
        crate::entity::ThrowableProjectile::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Allay. Needed for returning it from Java.
pub struct Allay<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Allay<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Allay from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Allay")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Allay object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Allay<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Creature<'mc>> for Allay<'mc> {
    fn into(self) -> crate::entity::Creature<'mc> {
        crate::entity::Creature::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::inventory::InventoryHolder<'mc>> for Allay<'mc> {
    fn into(self) -> crate::inventory::InventoryHolder<'mc> {
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements NPC. Needed for returning it from Java.
pub struct NPC<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> NPC<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate NPC from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "NPC")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a NPC object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for NPC<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Creature<'mc>> for NPC<'mc> {
    fn into(self) -> crate::entity::Creature<'mc> {
        crate::entity::Creature::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Stray. Needed for returning it from Java.
pub struct Stray<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Stray<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Stray from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Stray")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Stray object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Stray<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::AbstractSkeleton<'mc>> for Stray<'mc> {
    fn into(self) -> crate::entity::AbstractSkeleton<'mc> {
        crate::entity::AbstractSkeleton::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct CatType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for CatType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> CatType<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate CatType from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "CatType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CatType object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
/// An instantiatable struct that implements Silverfish. Needed for returning it from Java.
pub struct Silverfish<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Silverfish<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Silverfish from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Silverfish")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Silverfish object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Silverfish<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Monster<'mc>> for Silverfish<'mc> {
    fn into(self) -> crate::entity::Monster<'mc> {
        crate::entity::Monster::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements ArmorStand. Needed for returning it from Java.
pub struct ArmorStand<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ArmorStand<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ArmorStand from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "ArmorStand")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ArmorStand object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for ArmorStand<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::LivingEntity<'mc>> for ArmorStand<'mc> {
    fn into(self) -> crate::entity::LivingEntity<'mc> {
        crate::entity::LivingEntity::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct SkeletonSkeletonType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for SkeletonSkeletonType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> SkeletonSkeletonType<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SkeletonSkeletonType from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "SkeletonSkeletonType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SkeletonSkeletonType object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
/// An instantiatable struct that implements Spellcaster. Needed for returning it from Java.
pub struct Spellcaster<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Spellcaster<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Spellcaster from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Spellcaster")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Spellcaster object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Spellcaster<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Illager<'mc>> for Spellcaster<'mc> {
    fn into(self) -> crate::entity::Illager<'mc> {
        crate::entity::Illager::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements LeashHitch. Needed for returning it from Java.
pub struct LeashHitch<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> LeashHitch<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate LeashHitch from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "LeashHitch")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LeashHitch object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for LeashHitch<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Hanging<'mc>> for LeashHitch<'mc> {
    fn into(self) -> crate::entity::Hanging<'mc> {
        crate::entity::Hanging::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Skeleton. Needed for returning it from Java.
pub struct Skeleton<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Skeleton<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Skeleton from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Skeleton")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Skeleton object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Skeleton<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::AbstractSkeleton<'mc>> for Skeleton<'mc> {
    fn into(self) -> crate::entity::AbstractSkeleton<'mc> {
        crate::entity::AbstractSkeleton::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements SmallFireball. Needed for returning it from Java.
pub struct SmallFireball<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> SmallFireball<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SmallFireball from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "SmallFireball")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SmallFireball object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for SmallFireball<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::SizedFireball<'mc>> for SmallFireball<'mc> {
    fn into(self) -> crate::entity::SizedFireball<'mc> {
        crate::entity::SizedFireball::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements SkeletonHorse. Needed for returning it from Java.
pub struct SkeletonHorse<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> SkeletonHorse<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SkeletonHorse from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "SkeletonHorse")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SkeletonHorse object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for SkeletonHorse<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::AbstractHorse<'mc>> for SkeletonHorse<'mc> {
    fn into(self) -> crate::entity::AbstractHorse<'mc> {
        crate::entity::AbstractHorse::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements PolarBear. Needed for returning it from Java.
pub struct PolarBear<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> PolarBear<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PolarBear from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "PolarBear")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PolarBear object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for PolarBear<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Animals<'mc>> for PolarBear<'mc> {
    fn into(self) -> crate::entity::Animals<'mc> {
        crate::entity::Animals::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Mob. Needed for returning it from Java.
pub struct Mob<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Mob<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Mob from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Mob")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Mob object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Mob<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::LivingEntity<'mc>> for Mob<'mc> {
    fn into(self) -> crate::entity::LivingEntity<'mc> {
        crate::entity::LivingEntity::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::loot::Lootable<'mc>> for Mob<'mc> {
    fn into(self) -> crate::loot::Lootable<'mc> {
        crate::loot::Lootable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements PufferFish. Needed for returning it from Java.
pub struct PufferFish<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> PufferFish<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PufferFish from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "PufferFish")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PufferFish object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for PufferFish<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Fish<'mc>> for PufferFish<'mc> {
    fn into(self) -> crate::entity::Fish<'mc> {
        crate::entity::Fish::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements TNTPrimed. Needed for returning it from Java.
pub struct TNTPrimed<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> TNTPrimed<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate TNTPrimed from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "TNTPrimed")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TNTPrimed object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for TNTPrimed<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Explosive<'mc>> for TNTPrimed<'mc> {
    fn into(self) -> crate::entity::Explosive<'mc> {
        crate::entity::Explosive::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct PandaGene<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PandaGene<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PandaGene<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PandaGene from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "PandaGene")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PandaGene object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
/// An instantiatable struct that implements SplashPotion. Needed for returning it from Java.
pub struct SplashPotion<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> SplashPotion<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SplashPotion from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "SplashPotion")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SplashPotion object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for SplashPotion<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::ThrownPotion<'mc>> for SplashPotion<'mc> {
    fn into(self) -> crate::entity::ThrownPotion<'mc> {
        crate::entity::ThrownPotion::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct EvokerSpell<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EvokerSpell<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EvokerSpell<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate EvokerSpell from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "EvokerSpell")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EvokerSpell object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
/// An instantiatable struct that implements WaterMob. Needed for returning it from Java.
pub struct WaterMob<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> WaterMob<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate WaterMob from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "WaterMob")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a WaterMob object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for WaterMob<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Creature<'mc>> for WaterMob<'mc> {
    fn into(self) -> crate::entity::Creature<'mc> {
        crate::entity::Creature::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Spider. Needed for returning it from Java.
pub struct Spider<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Spider<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Spider from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Spider")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Spider object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Spider<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Monster<'mc>> for Spider<'mc> {
    fn into(self) -> crate::entity::Monster<'mc> {
        crate::entity::Monster::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements ThrowableProjectile. Needed for returning it from Java.
pub struct ThrowableProjectile<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ThrowableProjectile<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ThrowableProjectile from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "ThrowableProjectile")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ThrowableProjectile object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for ThrowableProjectile<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Projectile<'mc>> for ThrowableProjectile<'mc> {
    fn into(self) -> crate::entity::Projectile<'mc> {
        crate::entity::Projectile::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements ComplexEntityPart. Needed for returning it from Java.
pub struct ComplexEntityPart<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ComplexEntityPart<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ComplexEntityPart from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "ComplexEntityPart")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ComplexEntityPart object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for ComplexEntityPart<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Entity<'mc>> for ComplexEntityPart<'mc> {
    fn into(self) -> crate::entity::Entity<'mc> {
        crate::entity::Entity::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Dolphin. Needed for returning it from Java.
pub struct Dolphin<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Dolphin<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Dolphin from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Dolphin")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Dolphin object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Dolphin<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::WaterMob<'mc>> for Dolphin<'mc> {
    fn into(self) -> crate::entity::WaterMob<'mc> {
        crate::entity::WaterMob::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements BlockDisplay. Needed for returning it from Java.
pub struct BlockDisplay<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> BlockDisplay<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BlockDisplay from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "BlockDisplay")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockDisplay object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for BlockDisplay<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Display<'mc>> for BlockDisplay<'mc> {
    fn into(self) -> crate::entity::Display<'mc> {
        crate::entity::Display::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements PiglinAbstract. Needed for returning it from Java.
pub struct PiglinAbstract<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> PiglinAbstract<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PiglinAbstract from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "PiglinAbstract")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PiglinAbstract object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for PiglinAbstract<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Monster<'mc>> for PiglinAbstract<'mc> {
    fn into(self) -> crate::entity::Monster<'mc> {
        crate::entity::Monster::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::entity::Ageable<'mc>> for PiglinAbstract<'mc> {
    fn into(self) -> crate::entity::Ageable<'mc> {
        crate::entity::Ageable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct DisplayBillboard<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for DisplayBillboard<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> DisplayBillboard<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate DisplayBillboard from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "DisplayBillboard")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DisplayBillboard object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
pub struct DisplayBrightness<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for DisplayBrightness<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> DisplayBrightness<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate DisplayBrightness from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "DisplayBrightness")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DisplayBrightness object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
pub struct ItemDisplayItemDisplayTransform<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ItemDisplayItemDisplayTransform<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ItemDisplayItemDisplayTransform<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ItemDisplayItemDisplayTransform from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "ItemDisplayItemDisplayTransform")?;
        if !valid {
            Err(eyre::eyre!(
        "Invalid argument passed. Expected a ItemDisplayItemDisplayTransform object, got {}",
        name
    )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
/// An instantiatable struct that implements Item. Needed for returning it from Java.
pub struct Item<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Item<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Item from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Item")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Item object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Item<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Entity<'mc>> for Item<'mc> {
    fn into(self) -> crate::entity::Entity<'mc> {
        crate::entity::Entity::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Fireball. Needed for returning it from Java.
pub struct Fireball<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Fireball<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Fireball from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Fireball")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Fireball object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Fireball<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Projectile<'mc>> for Fireball<'mc> {
    fn into(self) -> crate::entity::Projectile<'mc> {
        crate::entity::Projectile::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::entity::Explosive<'mc>> for Fireball<'mc> {
    fn into(self) -> crate::entity::Explosive<'mc> {
        crate::entity::Explosive::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements HumanEntity. Needed for returning it from Java.
pub struct HumanEntity<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> HumanEntity<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate HumanEntity from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "HumanEntity")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a HumanEntity object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for HumanEntity<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::LivingEntity<'mc>> for HumanEntity<'mc> {
    fn into(self) -> crate::entity::LivingEntity<'mc> {
        crate::entity::LivingEntity::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::entity::AnimalTamer<'mc>> for HumanEntity<'mc> {
    fn into(self) -> crate::entity::AnimalTamer<'mc> {
        crate::entity::AnimalTamer::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::inventory::InventoryHolder<'mc>> for HumanEntity<'mc> {
    fn into(self) -> crate::inventory::InventoryHolder<'mc> {
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Villager. Needed for returning it from Java.
pub struct Villager<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Villager<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Villager from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Villager")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Villager object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Villager<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::AbstractVillager<'mc>> for Villager<'mc> {
    fn into(self) -> crate::entity::AbstractVillager<'mc> {
        crate::entity::AbstractVillager::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements ThrownPotion. Needed for returning it from Java.
pub struct ThrownPotion<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ThrownPotion<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ThrownPotion from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "ThrownPotion")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ThrownPotion object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for ThrownPotion<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::ThrowableProjectile<'mc>> for ThrownPotion<'mc> {
    fn into(self) -> crate::entity::ThrowableProjectile<'mc> {
        crate::entity::ThrowableProjectile::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Guardian. Needed for returning it from Java.
pub struct Guardian<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Guardian<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Guardian from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Guardian")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Guardian object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Guardian<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Monster<'mc>> for Guardian<'mc> {
    fn into(self) -> crate::entity::Monster<'mc> {
        crate::entity::Monster::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Goat. Needed for returning it from Java.
pub struct Goat<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Goat<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Goat from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Goat")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Goat object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Goat<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Animals<'mc>> for Goat<'mc> {
    fn into(self) -> crate::entity::Animals<'mc> {
        crate::entity::Animals::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Pillager. Needed for returning it from Java.
pub struct Pillager<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Pillager<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Pillager from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Pillager")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Pillager object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Pillager<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Illager<'mc>> for Pillager<'mc> {
    fn into(self) -> crate::entity::Illager<'mc> {
        crate::entity::Illager::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::inventory::InventoryHolder<'mc>> for Pillager<'mc> {
    fn into(self) -> crate::inventory::InventoryHolder<'mc> {
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements MushroomCow. Needed for returning it from Java.
pub struct MushroomCow<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> MushroomCow<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MushroomCow from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "MushroomCow")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MushroomCow object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for MushroomCow<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Cow<'mc>> for MushroomCow<'mc> {
    fn into(self) -> crate::entity::Cow<'mc> {
        crate::entity::Cow::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Sittable. Needed for returning it from Java.
pub struct Sittable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Sittable<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Sittable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Sittable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Sittable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Sittable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements LingeringPotion. Needed for returning it from Java.
pub struct LingeringPotion<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> LingeringPotion<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate LingeringPotion from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "LingeringPotion")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LingeringPotion object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for LingeringPotion<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::ThrownPotion<'mc>> for LingeringPotion<'mc> {
    fn into(self) -> crate::entity::ThrownPotion<'mc> {
        crate::entity::ThrownPotion::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Panda. Needed for returning it from Java.
pub struct Panda<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Panda<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Panda from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Panda")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Panda object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Panda<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Animals<'mc>> for Panda<'mc> {
    fn into(self) -> crate::entity::Animals<'mc> {
        crate::entity::Animals::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::entity::Sittable<'mc>> for Panda<'mc> {
    fn into(self) -> crate::entity::Sittable<'mc> {
        crate::entity::Sittable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Snowball. Needed for returning it from Java.
pub struct Snowball<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Snowball<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Snowball from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Snowball")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Snowball object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Snowball<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::ThrowableProjectile<'mc>> for Snowball<'mc> {
    fn into(self) -> crate::entity::ThrowableProjectile<'mc> {
        crate::entity::ThrowableProjectile::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Flying. Needed for returning it from Java.
pub struct Flying<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Flying<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Flying from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Flying")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Flying object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Flying<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Mob<'mc>> for Flying<'mc> {
    fn into(self) -> crate::entity::Mob<'mc> {
        crate::entity::Mob::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements ItemDisplay. Needed for returning it from Java.
pub struct ItemDisplay<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ItemDisplay<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ItemDisplay from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "ItemDisplay")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ItemDisplay object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for ItemDisplay<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Display<'mc>> for ItemDisplay<'mc> {
    fn into(self) -> crate::entity::Display<'mc> {
        crate::entity::Display::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements CaveSpider. Needed for returning it from Java.
pub struct CaveSpider<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> CaveSpider<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate CaveSpider from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "CaveSpider")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CaveSpider object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for CaveSpider<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Spider<'mc>> for CaveSpider<'mc> {
    fn into(self) -> crate::entity::Spider<'mc> {
        crate::entity::Spider::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements ChestBoat. Needed for returning it from Java.
pub struct ChestBoat<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ChestBoat<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ChestBoat from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "ChestBoat")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ChestBoat object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for ChestBoat<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Boat<'mc>> for ChestBoat<'mc> {
    fn into(self) -> crate::entity::Boat<'mc> {
        crate::entity::Boat::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::inventory::InventoryHolder<'mc>> for ChestBoat<'mc> {
    fn into(self) -> crate::inventory::InventoryHolder<'mc> {
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::loot::Lootable<'mc>> for ChestBoat<'mc> {
    fn into(self) -> crate::loot::Lootable<'mc> {
        crate::loot::Lootable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements ZombieVillager. Needed for returning it from Java.
pub struct ZombieVillager<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ZombieVillager<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ZombieVillager from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "ZombieVillager")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ZombieVillager object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for ZombieVillager<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Zombie<'mc>> for ZombieVillager<'mc> {
    fn into(self) -> crate::entity::Zombie<'mc> {
        crate::entity::Zombie::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements AbstractSkeleton. Needed for returning it from Java.
pub struct AbstractSkeleton<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> AbstractSkeleton<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate AbstractSkeleton from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "AbstractSkeleton")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AbstractSkeleton object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for AbstractSkeleton<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Monster<'mc>> for AbstractSkeleton<'mc> {
    fn into(self) -> crate::entity::Monster<'mc> {
        crate::entity::Monster::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Rabbit. Needed for returning it from Java.
pub struct Rabbit<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Rabbit<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Rabbit from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Rabbit")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Rabbit object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Rabbit<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Animals<'mc>> for Rabbit<'mc> {
    fn into(self) -> crate::entity::Animals<'mc> {
        crate::entity::Animals::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements IronGolem. Needed for returning it from Java.
pub struct IronGolem<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> IronGolem<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate IronGolem from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "IronGolem")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a IronGolem object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for IronGolem<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Golem<'mc>> for IronGolem<'mc> {
    fn into(self) -> crate::entity::Golem<'mc> {
        crate::entity::Golem::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Camel. Needed for returning it from Java.
pub struct Camel<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Camel<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Camel from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Camel")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Camel object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Camel<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::AbstractHorse<'mc>> for Camel<'mc> {
    fn into(self) -> crate::entity::AbstractHorse<'mc> {
        crate::entity::AbstractHorse::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::entity::Sittable<'mc>> for Camel<'mc> {
    fn into(self) -> crate::entity::Sittable<'mc> {
        crate::entity::Sittable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements ShulkerBullet. Needed for returning it from Java.
pub struct ShulkerBullet<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ShulkerBullet<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ShulkerBullet from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "ShulkerBullet")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ShulkerBullet object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for ShulkerBullet<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Projectile<'mc>> for ShulkerBullet<'mc> {
    fn into(self) -> crate::entity::Projectile<'mc> {
        crate::entity::Projectile::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Ageable. Needed for returning it from Java.
pub struct Ageable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Ageable<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Ageable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Ageable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Ageable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Ageable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Creature<'mc>> for Ageable<'mc> {
    fn into(self) -> crate::entity::Creature<'mc> {
        crate::entity::Creature::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements InteractionPreviousInteraction. Needed for returning it from Java.
pub struct InteractionPreviousInteraction<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> InteractionPreviousInteraction<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate InteractionPreviousInteraction from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "InteractionPreviousInteraction")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InteractionPreviousInteraction object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for InteractionPreviousInteraction<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct FrogVariant<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for FrogVariant<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> FrogVariant<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate FrogVariant from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "FrogVariant")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FrogVariant object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::Keyed<'mc>> for FrogVariant<'mc> {
    fn into(self) -> crate::Keyed<'mc> {
        crate::Keyed::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct VillagerProfession<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for VillagerProfession<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> VillagerProfession<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate VillagerProfession from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "VillagerProfession")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a VillagerProfession object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::Keyed<'mc>> for VillagerProfession<'mc> {
    fn into(self) -> crate::Keyed<'mc> {
        crate::Keyed::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Steerable. Needed for returning it from Java.
pub struct Steerable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Steerable<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Steerable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Steerable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Steerable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Steerable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Animals<'mc>> for Steerable<'mc> {
    fn into(self) -> crate::entity::Animals<'mc> {
        crate::entity::Animals::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements TippedArrow. Needed for returning it from Java.
pub struct TippedArrow<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> TippedArrow<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate TippedArrow from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "TippedArrow")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TippedArrow object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for TippedArrow<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Arrow<'mc>> for TippedArrow<'mc> {
    fn into(self) -> crate::entity::Arrow<'mc> {
        crate::entity::Arrow::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements TropicalFish. Needed for returning it from Java.
pub struct TropicalFish<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> TropicalFish<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate TropicalFish from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "TropicalFish")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TropicalFish object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for TropicalFish<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Fish<'mc>> for TropicalFish<'mc> {
    fn into(self) -> crate::entity::Fish<'mc> {
        crate::entity::Fish::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Monster. Needed for returning it from Java.
pub struct Monster<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Monster<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Monster from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Monster")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Monster object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Monster<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Creature<'mc>> for Monster<'mc> {
    fn into(self) -> crate::entity::Creature<'mc> {
        crate::entity::Creature::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::entity::Enemy<'mc>> for Monster<'mc> {
    fn into(self) -> crate::entity::Enemy<'mc> {
        crate::entity::Enemy::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Hoglin. Needed for returning it from Java.
pub struct Hoglin<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Hoglin<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Hoglin from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Hoglin")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Hoglin object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Hoglin<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Animals<'mc>> for Hoglin<'mc> {
    fn into(self) -> crate::entity::Animals<'mc> {
        crate::entity::Animals::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::entity::Enemy<'mc>> for Hoglin<'mc> {
    fn into(self) -> crate::entity::Enemy<'mc> {
        crate::entity::Enemy::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Cod. Needed for returning it from Java.
pub struct Cod<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Cod<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Cod from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Cod")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Cod object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Cod<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Fish<'mc>> for Cod<'mc> {
    fn into(self) -> crate::entity::Fish<'mc> {
        crate::entity::Fish::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Witch. Needed for returning it from Java.
pub struct Witch<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Witch<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Witch from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Witch")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Witch object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Witch<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Raider<'mc>> for Witch<'mc> {
    fn into(self) -> crate::entity::Raider<'mc> {
        crate::entity::Raider::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements EnderPearl. Needed for returning it from Java.
pub struct EnderPearl<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> EnderPearl<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate EnderPearl from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "EnderPearl")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EnderPearl object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for EnderPearl<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::ThrowableProjectile<'mc>> for EnderPearl<'mc> {
    fn into(self) -> crate::entity::ThrowableProjectile<'mc> {
        crate::entity::ThrowableProjectile::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Tameable. Needed for returning it from Java.
pub struct Tameable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Tameable<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Tameable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Tameable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Tameable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Tameable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Animals<'mc>> for Tameable<'mc> {
    fn into(self) -> crate::entity::Animals<'mc> {
        crate::entity::Animals::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Mule. Needed for returning it from Java.
pub struct Mule<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Mule<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Mule from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Mule")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Mule object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Mule<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::ChestedHorse<'mc>> for Mule<'mc> {
    fn into(self) -> crate::entity::ChestedHorse<'mc> {
        crate::entity::ChestedHorse::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct ArmorStandLockType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ArmorStandLockType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ArmorStandLockType<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ArmorStandLockType from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "ArmorStandLockType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ArmorStandLockType object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
/// An instantiatable struct that implements Tadpole. Needed for returning it from Java.
pub struct Tadpole<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Tadpole<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Tadpole from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Tadpole")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Tadpole object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Tadpole<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Fish<'mc>> for Tadpole<'mc> {
    fn into(self) -> crate::entity::Fish<'mc> {
        crate::entity::Fish::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Cow. Needed for returning it from Java.
pub struct Cow<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Cow<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Cow from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Cow")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Cow object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Cow<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Animals<'mc>> for Cow<'mc> {
    fn into(self) -> crate::entity::Animals<'mc> {
        crate::entity::Animals::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements ExperienceOrb. Needed for returning it from Java.
pub struct ExperienceOrb<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ExperienceOrb<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ExperienceOrb from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "ExperienceOrb")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ExperienceOrb object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for ExperienceOrb<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Entity<'mc>> for ExperienceOrb<'mc> {
    fn into(self) -> crate::entity::Entity<'mc> {
        crate::entity::Entity::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct HorseColor<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for HorseColor<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> HorseColor<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate HorseColor from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "HorseColor")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a HorseColor object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
/// An instantiatable struct that implements Pig. Needed for returning it from Java.
pub struct Pig<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Pig<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Pig from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Pig")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Pig object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Pig<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Steerable<'mc>> for Pig<'mc> {
    fn into(self) -> crate::entity::Steerable<'mc> {
        crate::entity::Steerable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::entity::Vehicle<'mc>> for Pig<'mc> {
    fn into(self) -> crate::entity::Vehicle<'mc> {
        crate::entity::Vehicle::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Illusioner. Needed for returning it from Java.
pub struct Illusioner<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Illusioner<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Illusioner from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Illusioner")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Illusioner object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Illusioner<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Spellcaster<'mc>> for Illusioner<'mc> {
    fn into(self) -> crate::entity::Spellcaster<'mc> {
        crate::entity::Spellcaster::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Projectile. Needed for returning it from Java.
pub struct Projectile<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Projectile<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Projectile from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Projectile")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Projectile object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Projectile<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Entity<'mc>> for Projectile<'mc> {
    fn into(self) -> crate::entity::Entity<'mc> {
        crate::entity::Entity::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct EntitySpigot<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntitySpigot<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntitySpigot<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate EntitySpigot from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "EntitySpigot")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntitySpigot object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::command::CommandSenderSpigot<'mc>> for EntitySpigot<'mc> {
    fn into(self) -> crate::command::CommandSenderSpigot<'mc> {
        crate::command::CommandSenderSpigot::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Blaze. Needed for returning it from Java.
pub struct Blaze<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Blaze<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Blaze from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Blaze")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Blaze object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Blaze<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Monster<'mc>> for Blaze<'mc> {
    fn into(self) -> crate::entity::Monster<'mc> {
        crate::entity::Monster::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements AnimalTamer. Needed for returning it from Java.
pub struct AnimalTamer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> AnimalTamer<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate AnimalTamer from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "AnimalTamer")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AnimalTamer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for AnimalTamer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements EnderDragonPart. Needed for returning it from Java.
pub struct EnderDragonPart<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> EnderDragonPart<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EnderDragonPart from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "EnderDragonPart")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EnderDragonPart object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for EnderDragonPart<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::ComplexEntityPart<'mc>> for EnderDragonPart<'mc> {
    fn into(self) -> crate::entity::ComplexEntityPart<'mc> {
        crate::entity::ComplexEntityPart::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::entity::Damageable<'mc>> for EnderDragonPart<'mc> {
    fn into(self) -> crate::entity::Damageable<'mc> {
        crate::entity::Damageable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements WitherSkull. Needed for returning it from Java.
pub struct WitherSkull<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> WitherSkull<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate WitherSkull from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "WitherSkull")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a WitherSkull object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for WitherSkull<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Fireball<'mc>> for WitherSkull<'mc> {
    fn into(self) -> crate::entity::Fireball<'mc> {
        crate::entity::Fireball::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Boat. Needed for returning it from Java.
pub struct Boat<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Boat<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Boat from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Boat")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Boat object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Boat<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Vehicle<'mc>> for Boat<'mc> {
    fn into(self) -> crate::entity::Vehicle<'mc> {
        crate::entity::Vehicle::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements AbstractHorse. Needed for returning it from Java.
pub struct AbstractHorse<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> AbstractHorse<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate AbstractHorse from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "AbstractHorse")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AbstractHorse object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for AbstractHorse<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Vehicle<'mc>> for AbstractHorse<'mc> {
    fn into(self) -> crate::entity::Vehicle<'mc> {
        crate::entity::Vehicle::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::inventory::InventoryHolder<'mc>> for AbstractHorse<'mc> {
    fn into(self) -> crate::inventory::InventoryHolder<'mc> {
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::entity::Tameable<'mc>> for AbstractHorse<'mc> {
    fn into(self) -> crate::entity::Tameable<'mc> {
        crate::entity::Tameable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct TextDisplayTextAlignment<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for TextDisplayTextAlignment<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> TextDisplayTextAlignment<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate TextDisplayTextAlignment from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "TextDisplayTextAlignment")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TextDisplayTextAlignment object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
pub struct BoatType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for BoatType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BoatType<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BoatType from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "BoatType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BoatType object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
/// An instantiatable struct that implements Vex. Needed for returning it from Java.
pub struct Vex<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Vex<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Vex from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Vex")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Vex object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Vex<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Monster<'mc>> for Vex<'mc> {
    fn into(self) -> crate::entity::Monster<'mc> {
        crate::entity::Monster::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements ItemFrame. Needed for returning it from Java.
pub struct ItemFrame<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ItemFrame<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ItemFrame from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "ItemFrame")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ItemFrame object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for ItemFrame<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Hanging<'mc>> for ItemFrame<'mc> {
    fn into(self) -> crate::entity::Hanging<'mc> {
        crate::entity::Hanging::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements SpectralArrow. Needed for returning it from Java.
pub struct SpectralArrow<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> SpectralArrow<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SpectralArrow from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "SpectralArrow")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SpectralArrow object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for SpectralArrow<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::AbstractArrow<'mc>> for SpectralArrow<'mc> {
    fn into(self) -> crate::entity::AbstractArrow<'mc> {
        crate::entity::AbstractArrow::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Boss. Needed for returning it from Java.
pub struct Boss<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Boss<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Boss from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Boss")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Boss object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Boss<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Entity<'mc>> for Boss<'mc> {
    fn into(self) -> crate::entity::Entity<'mc> {
        crate::entity::Entity::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements DragonFireball. Needed for returning it from Java.
pub struct DragonFireball<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> DragonFireball<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate DragonFireball from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "DragonFireball")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DragonFireball object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for DragonFireball<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Fireball<'mc>> for DragonFireball<'mc> {
    fn into(self) -> crate::entity::Fireball<'mc> {
        crate::entity::Fireball::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Strider. Needed for returning it from Java.
pub struct Strider<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Strider<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Strider from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Strider")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Strider object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Strider<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Steerable<'mc>> for Strider<'mc> {
    fn into(self) -> crate::entity::Steerable<'mc> {
        crate::entity::Steerable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::entity::Vehicle<'mc>> for Strider<'mc> {
    fn into(self) -> crate::entity::Vehicle<'mc> {
        crate::entity::Vehicle::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Horse. Needed for returning it from Java.
pub struct Horse<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Horse<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Horse from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Horse")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Horse object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Horse<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::AbstractHorse<'mc>> for Horse<'mc> {
    fn into(self) -> crate::entity::AbstractHorse<'mc> {
        crate::entity::AbstractHorse::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements WanderingTrader. Needed for returning it from Java.
pub struct WanderingTrader<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> WanderingTrader<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate WanderingTrader from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "WanderingTrader")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a WanderingTrader object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for WanderingTrader<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::AbstractVillager<'mc>> for WanderingTrader<'mc> {
    fn into(self) -> crate::entity::AbstractVillager<'mc> {
        crate::entity::AbstractVillager::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct SpellcasterSpell<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for SpellcasterSpell<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> SpellcasterSpell<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SpellcasterSpell from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "SpellcasterSpell")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SpellcasterSpell object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
/// An instantiatable struct that implements Player. Needed for returning it from Java.
pub struct Player<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Player<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Player from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Player")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Player object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Player<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::HumanEntity<'mc>> for Player<'mc> {
    fn into(self) -> crate::entity::HumanEntity<'mc> {
        crate::entity::HumanEntity::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::conversations::Conversable<'mc>> for Player<'mc> {
    fn into(self) -> crate::conversations::Conversable<'mc> {
        crate::conversations::Conversable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::OfflinePlayer<'mc>> for Player<'mc> {
    fn into(self) -> crate::OfflinePlayer<'mc> {
        crate::OfflinePlayer::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::plugin::messaging::PluginMessageRecipient<'mc>> for Player<'mc> {
    fn into(self) -> crate::plugin::messaging::PluginMessageRecipient<'mc> {
        crate::plugin::messaging::PluginMessageRecipient::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements FishHook. Needed for returning it from Java.
pub struct FishHook<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> FishHook<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate FishHook from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "FishHook")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FishHook object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for FishHook<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Projectile<'mc>> for FishHook<'mc> {
    fn into(self) -> crate::entity::Projectile<'mc> {
        crate::entity::Projectile::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Salmon. Needed for returning it from Java.
pub struct Salmon<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Salmon<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Salmon from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Salmon")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Salmon object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Salmon<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Fish<'mc>> for Salmon<'mc> {
    fn into(self) -> crate::entity::Fish<'mc> {
        crate::entity::Fish::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Zombie. Needed for returning it from Java.
pub struct Zombie<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Zombie<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Zombie from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Zombie")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Zombie object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Zombie<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Monster<'mc>> for Zombie<'mc> {
    fn into(self) -> crate::entity::Monster<'mc> {
        crate::entity::Monster::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::entity::Ageable<'mc>> for Zombie<'mc> {
    fn into(self) -> crate::entity::Ageable<'mc> {
        crate::entity::Ageable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Parrot. Needed for returning it from Java.
pub struct Parrot<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Parrot<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Parrot from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Parrot")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Parrot object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Parrot<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Tameable<'mc>> for Parrot<'mc> {
    fn into(self) -> crate::entity::Tameable<'mc> {
        crate::entity::Tameable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::entity::Sittable<'mc>> for Parrot<'mc> {
    fn into(self) -> crate::entity::Sittable<'mc> {
        crate::entity::Sittable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Evoker. Needed for returning it from Java.
pub struct Evoker<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Evoker<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Evoker from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Evoker")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Evoker object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Evoker<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Spellcaster<'mc>> for Evoker<'mc> {
    fn into(self) -> crate::entity::Spellcaster<'mc> {
        crate::entity::Spellcaster::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements EnderCrystal. Needed for returning it from Java.
pub struct EnderCrystal<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> EnderCrystal<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate EnderCrystal from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "EnderCrystal")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EnderCrystal object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for EnderCrystal<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Entity<'mc>> for EnderCrystal<'mc> {
    fn into(self) -> crate::entity::Entity<'mc> {
        crate::entity::Entity::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub enum EntityTypeEnum {
    DroppedItem,
    ExperienceOrb,
    AreaEffectCloud,
    ElderGuardian,
    WitherSkeleton,
    Stray,
    Egg,
    LeashHitch,
    Painting,
    Arrow,
    Snowball,
    Fireball,
    SmallFireball,
    EnderPearl,
    EnderSignal,
    SplashPotion,
    ThrownExpBottle,
    ItemFrame,
    WitherSkull,
    PrimedTnt,
    FallingBlock,
    Firework,
    Husk,
    SpectralArrow,
    ShulkerBullet,
    DragonFireball,
    ZombieVillager,
    SkeletonHorse,
    ZombieHorse,
    ArmorStand,
    Donkey,
    Mule,
    EvokerFangs,
    Evoker,
    Vex,
    Vindicator,
    Illusioner,
    MinecartCommand,
    Boat,
    Minecart,
    MinecartChest,
    MinecartFurnace,
    MinecartTnt,
    MinecartHopper,
    MinecartMobSpawner,
    Creeper,
    Skeleton,
    Spider,
    Giant,
    Zombie,
    Slime,
    Ghast,
    ZombifiedPiglin,
    Enderman,
    CaveSpider,
    Silverfish,
    Blaze,
    MagmaCube,
    EnderDragon,
    Wither,
    Bat,
    Witch,
    Endermite,
    Guardian,
    Shulker,
    Pig,
    Sheep,
    Cow,
    Chicken,
    Squid,
    Wolf,
    MushroomCow,
    Snowman,
    Ocelot,
    IronGolem,
    Horse,
    Rabbit,
    PolarBear,
    Llama,
    LlamaSpit,
    Parrot,
    Villager,
    EnderCrystal,
    Turtle,
    Phantom,
    Trident,
    Cod,
    Salmon,
    Pufferfish,
    TropicalFish,
    Drowned,
    Dolphin,
    Cat,
    Panda,
    Pillager,
    Ravager,
    TraderLlama,
    WanderingTrader,
    Fox,
    Bee,
    Hoglin,
    Piglin,
    Strider,
    Zoglin,
    PiglinBrute,
    Axolotl,
    GlowItemFrame,
    GlowSquid,
    Goat,
    Marker,
    Allay,
    ChestBoat,
    Frog,
    Tadpole,
    Warden,
    Camel,
    BlockDisplay,
    Interaction,
    ItemDisplay,
    Sniffer,
    TextDisplay,
    FishingHook,
    Lightning,
    Player,
    Unknown,
}
impl std::fmt::Display for EntityTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            EntityTypeEnum::DroppedItem => f.write_str("DROPPED_ITEM"),
            EntityTypeEnum::ExperienceOrb => f.write_str("EXPERIENCE_ORB"),
            EntityTypeEnum::AreaEffectCloud => f.write_str("AREA_EFFECT_CLOUD"),
            EntityTypeEnum::ElderGuardian => f.write_str("ELDER_GUARDIAN"),
            EntityTypeEnum::WitherSkeleton => f.write_str("WITHER_SKELETON"),
            EntityTypeEnum::Stray => f.write_str("STRAY"),
            EntityTypeEnum::Egg => f.write_str("EGG"),
            EntityTypeEnum::LeashHitch => f.write_str("LEASH_HITCH"),
            EntityTypeEnum::Painting => f.write_str("PAINTING"),
            EntityTypeEnum::Arrow => f.write_str("ARROW"),
            EntityTypeEnum::Snowball => f.write_str("SNOWBALL"),
            EntityTypeEnum::Fireball => f.write_str("FIREBALL"),
            EntityTypeEnum::SmallFireball => f.write_str("SMALL_FIREBALL"),
            EntityTypeEnum::EnderPearl => f.write_str("ENDER_PEARL"),
            EntityTypeEnum::EnderSignal => f.write_str("ENDER_SIGNAL"),
            EntityTypeEnum::SplashPotion => f.write_str("SPLASH_POTION"),
            EntityTypeEnum::ThrownExpBottle => f.write_str("THROWN_EXP_BOTTLE"),
            EntityTypeEnum::ItemFrame => f.write_str("ITEM_FRAME"),
            EntityTypeEnum::WitherSkull => f.write_str("WITHER_SKULL"),
            EntityTypeEnum::PrimedTnt => f.write_str("PRIMED_TNT"),
            EntityTypeEnum::FallingBlock => f.write_str("FALLING_BLOCK"),
            EntityTypeEnum::Firework => f.write_str("FIREWORK"),
            EntityTypeEnum::Husk => f.write_str("HUSK"),
            EntityTypeEnum::SpectralArrow => f.write_str("SPECTRAL_ARROW"),
            EntityTypeEnum::ShulkerBullet => f.write_str("SHULKER_BULLET"),
            EntityTypeEnum::DragonFireball => f.write_str("DRAGON_FIREBALL"),
            EntityTypeEnum::ZombieVillager => f.write_str("ZOMBIE_VILLAGER"),
            EntityTypeEnum::SkeletonHorse => f.write_str("SKELETON_HORSE"),
            EntityTypeEnum::ZombieHorse => f.write_str("ZOMBIE_HORSE"),
            EntityTypeEnum::ArmorStand => f.write_str("ARMOR_STAND"),
            EntityTypeEnum::Donkey => f.write_str("DONKEY"),
            EntityTypeEnum::Mule => f.write_str("MULE"),
            EntityTypeEnum::EvokerFangs => f.write_str("EVOKER_FANGS"),
            EntityTypeEnum::Evoker => f.write_str("EVOKER"),
            EntityTypeEnum::Vex => f.write_str("VEX"),
            EntityTypeEnum::Vindicator => f.write_str("VINDICATOR"),
            EntityTypeEnum::Illusioner => f.write_str("ILLUSIONER"),
            EntityTypeEnum::MinecartCommand => f.write_str("MINECART_COMMAND"),
            EntityTypeEnum::Boat => f.write_str("BOAT"),
            EntityTypeEnum::Minecart => f.write_str("MINECART"),
            EntityTypeEnum::MinecartChest => f.write_str("MINECART_CHEST"),
            EntityTypeEnum::MinecartFurnace => f.write_str("MINECART_FURNACE"),
            EntityTypeEnum::MinecartTnt => f.write_str("MINECART_TNT"),
            EntityTypeEnum::MinecartHopper => f.write_str("MINECART_HOPPER"),
            EntityTypeEnum::MinecartMobSpawner => f.write_str("MINECART_MOB_SPAWNER"),
            EntityTypeEnum::Creeper => f.write_str("CREEPER"),
            EntityTypeEnum::Skeleton => f.write_str("SKELETON"),
            EntityTypeEnum::Spider => f.write_str("SPIDER"),
            EntityTypeEnum::Giant => f.write_str("GIANT"),
            EntityTypeEnum::Zombie => f.write_str("ZOMBIE"),
            EntityTypeEnum::Slime => f.write_str("SLIME"),
            EntityTypeEnum::Ghast => f.write_str("GHAST"),
            EntityTypeEnum::ZombifiedPiglin => f.write_str("ZOMBIFIED_PIGLIN"),
            EntityTypeEnum::Enderman => f.write_str("ENDERMAN"),
            EntityTypeEnum::CaveSpider => f.write_str("CAVE_SPIDER"),
            EntityTypeEnum::Silverfish => f.write_str("SILVERFISH"),
            EntityTypeEnum::Blaze => f.write_str("BLAZE"),
            EntityTypeEnum::MagmaCube => f.write_str("MAGMA_CUBE"),
            EntityTypeEnum::EnderDragon => f.write_str("ENDER_DRAGON"),
            EntityTypeEnum::Wither => f.write_str("WITHER"),
            EntityTypeEnum::Bat => f.write_str("BAT"),
            EntityTypeEnum::Witch => f.write_str("WITCH"),
            EntityTypeEnum::Endermite => f.write_str("ENDERMITE"),
            EntityTypeEnum::Guardian => f.write_str("GUARDIAN"),
            EntityTypeEnum::Shulker => f.write_str("SHULKER"),
            EntityTypeEnum::Pig => f.write_str("PIG"),
            EntityTypeEnum::Sheep => f.write_str("SHEEP"),
            EntityTypeEnum::Cow => f.write_str("COW"),
            EntityTypeEnum::Chicken => f.write_str("CHICKEN"),
            EntityTypeEnum::Squid => f.write_str("SQUID"),
            EntityTypeEnum::Wolf => f.write_str("WOLF"),
            EntityTypeEnum::MushroomCow => f.write_str("MUSHROOM_COW"),
            EntityTypeEnum::Snowman => f.write_str("SNOWMAN"),
            EntityTypeEnum::Ocelot => f.write_str("OCELOT"),
            EntityTypeEnum::IronGolem => f.write_str("IRON_GOLEM"),
            EntityTypeEnum::Horse => f.write_str("HORSE"),
            EntityTypeEnum::Rabbit => f.write_str("RABBIT"),
            EntityTypeEnum::PolarBear => f.write_str("POLAR_BEAR"),
            EntityTypeEnum::Llama => f.write_str("LLAMA"),
            EntityTypeEnum::LlamaSpit => f.write_str("LLAMA_SPIT"),
            EntityTypeEnum::Parrot => f.write_str("PARROT"),
            EntityTypeEnum::Villager => f.write_str("VILLAGER"),
            EntityTypeEnum::EnderCrystal => f.write_str("ENDER_CRYSTAL"),
            EntityTypeEnum::Turtle => f.write_str("TURTLE"),
            EntityTypeEnum::Phantom => f.write_str("PHANTOM"),
            EntityTypeEnum::Trident => f.write_str("TRIDENT"),
            EntityTypeEnum::Cod => f.write_str("COD"),
            EntityTypeEnum::Salmon => f.write_str("SALMON"),
            EntityTypeEnum::Pufferfish => f.write_str("PUFFERFISH"),
            EntityTypeEnum::TropicalFish => f.write_str("TROPICAL_FISH"),
            EntityTypeEnum::Drowned => f.write_str("DROWNED"),
            EntityTypeEnum::Dolphin => f.write_str("DOLPHIN"),
            EntityTypeEnum::Cat => f.write_str("CAT"),
            EntityTypeEnum::Panda => f.write_str("PANDA"),
            EntityTypeEnum::Pillager => f.write_str("PILLAGER"),
            EntityTypeEnum::Ravager => f.write_str("RAVAGER"),
            EntityTypeEnum::TraderLlama => f.write_str("TRADER_LLAMA"),
            EntityTypeEnum::WanderingTrader => f.write_str("WANDERING_TRADER"),
            EntityTypeEnum::Fox => f.write_str("FOX"),
            EntityTypeEnum::Bee => f.write_str("BEE"),
            EntityTypeEnum::Hoglin => f.write_str("HOGLIN"),
            EntityTypeEnum::Piglin => f.write_str("PIGLIN"),
            EntityTypeEnum::Strider => f.write_str("STRIDER"),
            EntityTypeEnum::Zoglin => f.write_str("ZOGLIN"),
            EntityTypeEnum::PiglinBrute => f.write_str("PIGLIN_BRUTE"),
            EntityTypeEnum::Axolotl => f.write_str("AXOLOTL"),
            EntityTypeEnum::GlowItemFrame => f.write_str("GLOW_ITEM_FRAME"),
            EntityTypeEnum::GlowSquid => f.write_str("GLOW_SQUID"),
            EntityTypeEnum::Goat => f.write_str("GOAT"),
            EntityTypeEnum::Marker => f.write_str("MARKER"),
            EntityTypeEnum::Allay => f.write_str("ALLAY"),
            EntityTypeEnum::ChestBoat => f.write_str("CHEST_BOAT"),
            EntityTypeEnum::Frog => f.write_str("FROG"),
            EntityTypeEnum::Tadpole => f.write_str("TADPOLE"),
            EntityTypeEnum::Warden => f.write_str("WARDEN"),
            EntityTypeEnum::Camel => f.write_str("CAMEL"),
            EntityTypeEnum::BlockDisplay => f.write_str("BLOCK_DISPLAY"),
            EntityTypeEnum::Interaction => f.write_str("INTERACTION"),
            EntityTypeEnum::ItemDisplay => f.write_str("ITEM_DISPLAY"),
            EntityTypeEnum::Sniffer => f.write_str("SNIFFER"),
            EntityTypeEnum::TextDisplay => f.write_str("TEXT_DISPLAY"),
            EntityTypeEnum::FishingHook => f.write_str("FISHING_HOOK"),
            EntityTypeEnum::Lightning => f.write_str("LIGHTNING"),
            EntityTypeEnum::Player => f.write_str("PLAYER"),
            EntityTypeEnum::Unknown => f.write_str("UNKNOWN"),
        }
    }
}
pub struct EntityType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub EntityTypeEnum,
);
impl<'mc> std::ops::Deref for EntityType<'mc> {
    type Target = EntityTypeEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for EntityType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityType<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: EntityTypeEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate EntityType from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "EntityType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityType object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const DROPPED_ITEM: EntityTypeEnum = EntityTypeEnum::DroppedItem;
    pub const EXPERIENCE_ORB: EntityTypeEnum = EntityTypeEnum::ExperienceOrb;
    pub const AREA_EFFECT_CLOUD: EntityTypeEnum = EntityTypeEnum::AreaEffectCloud;
    pub const ELDER_GUARDIAN: EntityTypeEnum = EntityTypeEnum::ElderGuardian;
    pub const WITHER_SKELETON: EntityTypeEnum = EntityTypeEnum::WitherSkeleton;
    pub const STRAY: EntityTypeEnum = EntityTypeEnum::Stray;
    pub const EGG: EntityTypeEnum = EntityTypeEnum::Egg;
    pub const LEASH_HITCH: EntityTypeEnum = EntityTypeEnum::LeashHitch;
    pub const PAINTING: EntityTypeEnum = EntityTypeEnum::Painting;
    pub const ARROW: EntityTypeEnum = EntityTypeEnum::Arrow;
    pub const SNOWBALL: EntityTypeEnum = EntityTypeEnum::Snowball;
    pub const FIREBALL: EntityTypeEnum = EntityTypeEnum::Fireball;
    pub const SMALL_FIREBALL: EntityTypeEnum = EntityTypeEnum::SmallFireball;
    pub const ENDER_PEARL: EntityTypeEnum = EntityTypeEnum::EnderPearl;
    pub const ENDER_SIGNAL: EntityTypeEnum = EntityTypeEnum::EnderSignal;
    pub const SPLASH_POTION: EntityTypeEnum = EntityTypeEnum::SplashPotion;
    pub const THROWN_EXP_BOTTLE: EntityTypeEnum = EntityTypeEnum::ThrownExpBottle;
    pub const ITEM_FRAME: EntityTypeEnum = EntityTypeEnum::ItemFrame;
    pub const WITHER_SKULL: EntityTypeEnum = EntityTypeEnum::WitherSkull;
    pub const PRIMED_TNT: EntityTypeEnum = EntityTypeEnum::PrimedTnt;
    pub const FALLING_BLOCK: EntityTypeEnum = EntityTypeEnum::FallingBlock;
    pub const FIREWORK: EntityTypeEnum = EntityTypeEnum::Firework;
    pub const HUSK: EntityTypeEnum = EntityTypeEnum::Husk;
    pub const SPECTRAL_ARROW: EntityTypeEnum = EntityTypeEnum::SpectralArrow;
    pub const SHULKER_BULLET: EntityTypeEnum = EntityTypeEnum::ShulkerBullet;
    pub const DRAGON_FIREBALL: EntityTypeEnum = EntityTypeEnum::DragonFireball;
    pub const ZOMBIE_VILLAGER: EntityTypeEnum = EntityTypeEnum::ZombieVillager;
    pub const SKELETON_HORSE: EntityTypeEnum = EntityTypeEnum::SkeletonHorse;
    pub const ZOMBIE_HORSE: EntityTypeEnum = EntityTypeEnum::ZombieHorse;
    pub const ARMOR_STAND: EntityTypeEnum = EntityTypeEnum::ArmorStand;
    pub const DONKEY: EntityTypeEnum = EntityTypeEnum::Donkey;
    pub const MULE: EntityTypeEnum = EntityTypeEnum::Mule;
    pub const EVOKER_FANGS: EntityTypeEnum = EntityTypeEnum::EvokerFangs;
    pub const EVOKER: EntityTypeEnum = EntityTypeEnum::Evoker;
    pub const VEX: EntityTypeEnum = EntityTypeEnum::Vex;
    pub const VINDICATOR: EntityTypeEnum = EntityTypeEnum::Vindicator;
    pub const ILLUSIONER: EntityTypeEnum = EntityTypeEnum::Illusioner;
    pub const MINECART_COMMAND: EntityTypeEnum = EntityTypeEnum::MinecartCommand;
    pub const BOAT: EntityTypeEnum = EntityTypeEnum::Boat;
    pub const MINECART: EntityTypeEnum = EntityTypeEnum::Minecart;
    pub const MINECART_CHEST: EntityTypeEnum = EntityTypeEnum::MinecartChest;
    pub const MINECART_FURNACE: EntityTypeEnum = EntityTypeEnum::MinecartFurnace;
    pub const MINECART_TNT: EntityTypeEnum = EntityTypeEnum::MinecartTnt;
    pub const MINECART_HOPPER: EntityTypeEnum = EntityTypeEnum::MinecartHopper;
    pub const MINECART_MOB_SPAWNER: EntityTypeEnum = EntityTypeEnum::MinecartMobSpawner;
    pub const CREEPER: EntityTypeEnum = EntityTypeEnum::Creeper;
    pub const SKELETON: EntityTypeEnum = EntityTypeEnum::Skeleton;
    pub const SPIDER: EntityTypeEnum = EntityTypeEnum::Spider;
    pub const GIANT: EntityTypeEnum = EntityTypeEnum::Giant;
    pub const ZOMBIE: EntityTypeEnum = EntityTypeEnum::Zombie;
    pub const SLIME: EntityTypeEnum = EntityTypeEnum::Slime;
    pub const GHAST: EntityTypeEnum = EntityTypeEnum::Ghast;
    pub const ZOMBIFIED_PIGLIN: EntityTypeEnum = EntityTypeEnum::ZombifiedPiglin;
    pub const ENDERMAN: EntityTypeEnum = EntityTypeEnum::Enderman;
    pub const CAVE_SPIDER: EntityTypeEnum = EntityTypeEnum::CaveSpider;
    pub const SILVERFISH: EntityTypeEnum = EntityTypeEnum::Silverfish;
    pub const BLAZE: EntityTypeEnum = EntityTypeEnum::Blaze;
    pub const MAGMA_CUBE: EntityTypeEnum = EntityTypeEnum::MagmaCube;
    pub const ENDER_DRAGON: EntityTypeEnum = EntityTypeEnum::EnderDragon;
    pub const WITHER: EntityTypeEnum = EntityTypeEnum::Wither;
    pub const BAT: EntityTypeEnum = EntityTypeEnum::Bat;
    pub const WITCH: EntityTypeEnum = EntityTypeEnum::Witch;
    pub const ENDERMITE: EntityTypeEnum = EntityTypeEnum::Endermite;
    pub const GUARDIAN: EntityTypeEnum = EntityTypeEnum::Guardian;
    pub const SHULKER: EntityTypeEnum = EntityTypeEnum::Shulker;
    pub const PIG: EntityTypeEnum = EntityTypeEnum::Pig;
    pub const SHEEP: EntityTypeEnum = EntityTypeEnum::Sheep;
    pub const COW: EntityTypeEnum = EntityTypeEnum::Cow;
    pub const CHICKEN: EntityTypeEnum = EntityTypeEnum::Chicken;
    pub const SQUID: EntityTypeEnum = EntityTypeEnum::Squid;
    pub const WOLF: EntityTypeEnum = EntityTypeEnum::Wolf;
    pub const MUSHROOM_COW: EntityTypeEnum = EntityTypeEnum::MushroomCow;
    pub const SNOWMAN: EntityTypeEnum = EntityTypeEnum::Snowman;
    pub const OCELOT: EntityTypeEnum = EntityTypeEnum::Ocelot;
    pub const IRON_GOLEM: EntityTypeEnum = EntityTypeEnum::IronGolem;
    pub const HORSE: EntityTypeEnum = EntityTypeEnum::Horse;
    pub const RABBIT: EntityTypeEnum = EntityTypeEnum::Rabbit;
    pub const POLAR_BEAR: EntityTypeEnum = EntityTypeEnum::PolarBear;
    pub const LLAMA: EntityTypeEnum = EntityTypeEnum::Llama;
    pub const LLAMA_SPIT: EntityTypeEnum = EntityTypeEnum::LlamaSpit;
    pub const PARROT: EntityTypeEnum = EntityTypeEnum::Parrot;
    pub const VILLAGER: EntityTypeEnum = EntityTypeEnum::Villager;
    pub const ENDER_CRYSTAL: EntityTypeEnum = EntityTypeEnum::EnderCrystal;
    pub const TURTLE: EntityTypeEnum = EntityTypeEnum::Turtle;
    pub const PHANTOM: EntityTypeEnum = EntityTypeEnum::Phantom;
    pub const TRIDENT: EntityTypeEnum = EntityTypeEnum::Trident;
    pub const COD: EntityTypeEnum = EntityTypeEnum::Cod;
    pub const SALMON: EntityTypeEnum = EntityTypeEnum::Salmon;
    pub const PUFFERFISH: EntityTypeEnum = EntityTypeEnum::Pufferfish;
    pub const TROPICAL_FISH: EntityTypeEnum = EntityTypeEnum::TropicalFish;
    pub const DROWNED: EntityTypeEnum = EntityTypeEnum::Drowned;
    pub const DOLPHIN: EntityTypeEnum = EntityTypeEnum::Dolphin;
    pub const CAT: EntityTypeEnum = EntityTypeEnum::Cat;
    pub const PANDA: EntityTypeEnum = EntityTypeEnum::Panda;
    pub const PILLAGER: EntityTypeEnum = EntityTypeEnum::Pillager;
    pub const RAVAGER: EntityTypeEnum = EntityTypeEnum::Ravager;
    pub const TRADER_LLAMA: EntityTypeEnum = EntityTypeEnum::TraderLlama;
    pub const WANDERING_TRADER: EntityTypeEnum = EntityTypeEnum::WanderingTrader;
    pub const FOX: EntityTypeEnum = EntityTypeEnum::Fox;
    pub const BEE: EntityTypeEnum = EntityTypeEnum::Bee;
    pub const HOGLIN: EntityTypeEnum = EntityTypeEnum::Hoglin;
    pub const PIGLIN: EntityTypeEnum = EntityTypeEnum::Piglin;
    pub const STRIDER: EntityTypeEnum = EntityTypeEnum::Strider;
    pub const ZOGLIN: EntityTypeEnum = EntityTypeEnum::Zoglin;
    pub const PIGLIN_BRUTE: EntityTypeEnum = EntityTypeEnum::PiglinBrute;
    pub const AXOLOTL: EntityTypeEnum = EntityTypeEnum::Axolotl;
    pub const GLOW_ITEM_FRAME: EntityTypeEnum = EntityTypeEnum::GlowItemFrame;
    pub const GLOW_SQUID: EntityTypeEnum = EntityTypeEnum::GlowSquid;
    pub const GOAT: EntityTypeEnum = EntityTypeEnum::Goat;
    pub const MARKER: EntityTypeEnum = EntityTypeEnum::Marker;
    pub const ALLAY: EntityTypeEnum = EntityTypeEnum::Allay;
    pub const CHEST_BOAT: EntityTypeEnum = EntityTypeEnum::ChestBoat;
    pub const FROG: EntityTypeEnum = EntityTypeEnum::Frog;
    pub const TADPOLE: EntityTypeEnum = EntityTypeEnum::Tadpole;
    pub const WARDEN: EntityTypeEnum = EntityTypeEnum::Warden;
    pub const CAMEL: EntityTypeEnum = EntityTypeEnum::Camel;
    pub const BLOCK_DISPLAY: EntityTypeEnum = EntityTypeEnum::BlockDisplay;
    pub const INTERACTION: EntityTypeEnum = EntityTypeEnum::Interaction;
    pub const ITEM_DISPLAY: EntityTypeEnum = EntityTypeEnum::ItemDisplay;
    pub const SNIFFER: EntityTypeEnum = EntityTypeEnum::Sniffer;
    pub const TEXT_DISPLAY: EntityTypeEnum = EntityTypeEnum::TextDisplay;
    pub const FISHING_HOOK: EntityTypeEnum = EntityTypeEnum::FishingHook;
    pub const LIGHTNING: EntityTypeEnum = EntityTypeEnum::Lightning;
    pub const PLAYER: EntityTypeEnum = EntityTypeEnum::Player;
    pub const UNKNOWN: EntityTypeEnum = EntityTypeEnum::Unknown;
    pub fn from_string(str: String) -> std::option::Option<EntityTypeEnum> {
        match str.as_str() {
            "DROPPED_ITEM" => Some(EntityTypeEnum::DroppedItem),
            "EXPERIENCE_ORB" => Some(EntityTypeEnum::ExperienceOrb),
            "AREA_EFFECT_CLOUD" => Some(EntityTypeEnum::AreaEffectCloud),
            "ELDER_GUARDIAN" => Some(EntityTypeEnum::ElderGuardian),
            "WITHER_SKELETON" => Some(EntityTypeEnum::WitherSkeleton),
            "STRAY" => Some(EntityTypeEnum::Stray),
            "EGG" => Some(EntityTypeEnum::Egg),
            "LEASH_HITCH" => Some(EntityTypeEnum::LeashHitch),
            "PAINTING" => Some(EntityTypeEnum::Painting),
            "ARROW" => Some(EntityTypeEnum::Arrow),
            "SNOWBALL" => Some(EntityTypeEnum::Snowball),
            "FIREBALL" => Some(EntityTypeEnum::Fireball),
            "SMALL_FIREBALL" => Some(EntityTypeEnum::SmallFireball),
            "ENDER_PEARL" => Some(EntityTypeEnum::EnderPearl),
            "ENDER_SIGNAL" => Some(EntityTypeEnum::EnderSignal),
            "SPLASH_POTION" => Some(EntityTypeEnum::SplashPotion),
            "THROWN_EXP_BOTTLE" => Some(EntityTypeEnum::ThrownExpBottle),
            "ITEM_FRAME" => Some(EntityTypeEnum::ItemFrame),
            "WITHER_SKULL" => Some(EntityTypeEnum::WitherSkull),
            "PRIMED_TNT" => Some(EntityTypeEnum::PrimedTnt),
            "FALLING_BLOCK" => Some(EntityTypeEnum::FallingBlock),
            "FIREWORK" => Some(EntityTypeEnum::Firework),
            "HUSK" => Some(EntityTypeEnum::Husk),
            "SPECTRAL_ARROW" => Some(EntityTypeEnum::SpectralArrow),
            "SHULKER_BULLET" => Some(EntityTypeEnum::ShulkerBullet),
            "DRAGON_FIREBALL" => Some(EntityTypeEnum::DragonFireball),
            "ZOMBIE_VILLAGER" => Some(EntityTypeEnum::ZombieVillager),
            "SKELETON_HORSE" => Some(EntityTypeEnum::SkeletonHorse),
            "ZOMBIE_HORSE" => Some(EntityTypeEnum::ZombieHorse),
            "ARMOR_STAND" => Some(EntityTypeEnum::ArmorStand),
            "DONKEY" => Some(EntityTypeEnum::Donkey),
            "MULE" => Some(EntityTypeEnum::Mule),
            "EVOKER_FANGS" => Some(EntityTypeEnum::EvokerFangs),
            "EVOKER" => Some(EntityTypeEnum::Evoker),
            "VEX" => Some(EntityTypeEnum::Vex),
            "VINDICATOR" => Some(EntityTypeEnum::Vindicator),
            "ILLUSIONER" => Some(EntityTypeEnum::Illusioner),
            "MINECART_COMMAND" => Some(EntityTypeEnum::MinecartCommand),
            "BOAT" => Some(EntityTypeEnum::Boat),
            "MINECART" => Some(EntityTypeEnum::Minecart),
            "MINECART_CHEST" => Some(EntityTypeEnum::MinecartChest),
            "MINECART_FURNACE" => Some(EntityTypeEnum::MinecartFurnace),
            "MINECART_TNT" => Some(EntityTypeEnum::MinecartTnt),
            "MINECART_HOPPER" => Some(EntityTypeEnum::MinecartHopper),
            "MINECART_MOB_SPAWNER" => Some(EntityTypeEnum::MinecartMobSpawner),
            "CREEPER" => Some(EntityTypeEnum::Creeper),
            "SKELETON" => Some(EntityTypeEnum::Skeleton),
            "SPIDER" => Some(EntityTypeEnum::Spider),
            "GIANT" => Some(EntityTypeEnum::Giant),
            "ZOMBIE" => Some(EntityTypeEnum::Zombie),
            "SLIME" => Some(EntityTypeEnum::Slime),
            "GHAST" => Some(EntityTypeEnum::Ghast),
            "ZOMBIFIED_PIGLIN" => Some(EntityTypeEnum::ZombifiedPiglin),
            "ENDERMAN" => Some(EntityTypeEnum::Enderman),
            "CAVE_SPIDER" => Some(EntityTypeEnum::CaveSpider),
            "SILVERFISH" => Some(EntityTypeEnum::Silverfish),
            "BLAZE" => Some(EntityTypeEnum::Blaze),
            "MAGMA_CUBE" => Some(EntityTypeEnum::MagmaCube),
            "ENDER_DRAGON" => Some(EntityTypeEnum::EnderDragon),
            "WITHER" => Some(EntityTypeEnum::Wither),
            "BAT" => Some(EntityTypeEnum::Bat),
            "WITCH" => Some(EntityTypeEnum::Witch),
            "ENDERMITE" => Some(EntityTypeEnum::Endermite),
            "GUARDIAN" => Some(EntityTypeEnum::Guardian),
            "SHULKER" => Some(EntityTypeEnum::Shulker),
            "PIG" => Some(EntityTypeEnum::Pig),
            "SHEEP" => Some(EntityTypeEnum::Sheep),
            "COW" => Some(EntityTypeEnum::Cow),
            "CHICKEN" => Some(EntityTypeEnum::Chicken),
            "SQUID" => Some(EntityTypeEnum::Squid),
            "WOLF" => Some(EntityTypeEnum::Wolf),
            "MUSHROOM_COW" => Some(EntityTypeEnum::MushroomCow),
            "SNOWMAN" => Some(EntityTypeEnum::Snowman),
            "OCELOT" => Some(EntityTypeEnum::Ocelot),
            "IRON_GOLEM" => Some(EntityTypeEnum::IronGolem),
            "HORSE" => Some(EntityTypeEnum::Horse),
            "RABBIT" => Some(EntityTypeEnum::Rabbit),
            "POLAR_BEAR" => Some(EntityTypeEnum::PolarBear),
            "LLAMA" => Some(EntityTypeEnum::Llama),
            "LLAMA_SPIT" => Some(EntityTypeEnum::LlamaSpit),
            "PARROT" => Some(EntityTypeEnum::Parrot),
            "VILLAGER" => Some(EntityTypeEnum::Villager),
            "ENDER_CRYSTAL" => Some(EntityTypeEnum::EnderCrystal),
            "TURTLE" => Some(EntityTypeEnum::Turtle),
            "PHANTOM" => Some(EntityTypeEnum::Phantom),
            "TRIDENT" => Some(EntityTypeEnum::Trident),
            "COD" => Some(EntityTypeEnum::Cod),
            "SALMON" => Some(EntityTypeEnum::Salmon),
            "PUFFERFISH" => Some(EntityTypeEnum::Pufferfish),
            "TROPICAL_FISH" => Some(EntityTypeEnum::TropicalFish),
            "DROWNED" => Some(EntityTypeEnum::Drowned),
            "DOLPHIN" => Some(EntityTypeEnum::Dolphin),
            "CAT" => Some(EntityTypeEnum::Cat),
            "PANDA" => Some(EntityTypeEnum::Panda),
            "PILLAGER" => Some(EntityTypeEnum::Pillager),
            "RAVAGER" => Some(EntityTypeEnum::Ravager),
            "TRADER_LLAMA" => Some(EntityTypeEnum::TraderLlama),
            "WANDERING_TRADER" => Some(EntityTypeEnum::WanderingTrader),
            "FOX" => Some(EntityTypeEnum::Fox),
            "BEE" => Some(EntityTypeEnum::Bee),
            "HOGLIN" => Some(EntityTypeEnum::Hoglin),
            "PIGLIN" => Some(EntityTypeEnum::Piglin),
            "STRIDER" => Some(EntityTypeEnum::Strider),
            "ZOGLIN" => Some(EntityTypeEnum::Zoglin),
            "PIGLIN_BRUTE" => Some(EntityTypeEnum::PiglinBrute),
            "AXOLOTL" => Some(EntityTypeEnum::Axolotl),
            "GLOW_ITEM_FRAME" => Some(EntityTypeEnum::GlowItemFrame),
            "GLOW_SQUID" => Some(EntityTypeEnum::GlowSquid),
            "GOAT" => Some(EntityTypeEnum::Goat),
            "MARKER" => Some(EntityTypeEnum::Marker),
            "ALLAY" => Some(EntityTypeEnum::Allay),
            "CHEST_BOAT" => Some(EntityTypeEnum::ChestBoat),
            "FROG" => Some(EntityTypeEnum::Frog),
            "TADPOLE" => Some(EntityTypeEnum::Tadpole),
            "WARDEN" => Some(EntityTypeEnum::Warden),
            "CAMEL" => Some(EntityTypeEnum::Camel),
            "BLOCK_DISPLAY" => Some(EntityTypeEnum::BlockDisplay),
            "INTERACTION" => Some(EntityTypeEnum::Interaction),
            "ITEM_DISPLAY" => Some(EntityTypeEnum::ItemDisplay),
            "SNIFFER" => Some(EntityTypeEnum::Sniffer),
            "TEXT_DISPLAY" => Some(EntityTypeEnum::TextDisplay),
            "FISHING_HOOK" => Some(EntityTypeEnum::FishingHook),
            "LIGHTNING" => Some(EntityTypeEnum::Lightning),
            "PLAYER" => Some(EntityTypeEnum::Player),
            "UNKNOWN" => Some(EntityTypeEnum::Unknown),
            _ => None,
        }
    }
}
/// An instantiatable struct that implements ZombieHorse. Needed for returning it from Java.
pub struct ZombieHorse<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ZombieHorse<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ZombieHorse from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "ZombieHorse")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ZombieHorse object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for ZombieHorse<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::AbstractHorse<'mc>> for ZombieHorse<'mc> {
    fn into(self) -> crate::entity::AbstractHorse<'mc> {
        crate::entity::AbstractHorse::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements ChestedHorse. Needed for returning it from Java.
pub struct ChestedHorse<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ChestedHorse<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ChestedHorse from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "ChestedHorse")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ChestedHorse object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for ChestedHorse<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::AbstractHorse<'mc>> for ChestedHorse<'mc> {
    fn into(self) -> crate::entity::AbstractHorse<'mc> {
        crate::entity::AbstractHorse::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Cat. Needed for returning it from Java.
pub struct Cat<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Cat<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Cat from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Cat")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Cat object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Cat<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Tameable<'mc>> for Cat<'mc> {
    fn into(self) -> crate::entity::Tameable<'mc> {
        crate::entity::Tameable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::entity::Sittable<'mc>> for Cat<'mc> {
    fn into(self) -> crate::entity::Sittable<'mc> {
        crate::entity::Sittable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct VillagerType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for VillagerType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> VillagerType<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate VillagerType from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "VillagerType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a VillagerType object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::Keyed<'mc>> for VillagerType<'mc> {
    fn into(self) -> crate::Keyed<'mc> {
        crate::Keyed::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Axolotl. Needed for returning it from Java.
pub struct Axolotl<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Axolotl<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Axolotl from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Axolotl")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Axolotl object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Axolotl<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Animals<'mc>> for Axolotl<'mc> {
    fn into(self) -> crate::entity::Animals<'mc> {
        crate::entity::Animals::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Giant. Needed for returning it from Java.
pub struct Giant<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Giant<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Giant from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Giant")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Giant object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Giant<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Monster<'mc>> for Giant<'mc> {
    fn into(self) -> crate::entity::Monster<'mc> {
        crate::entity::Monster::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Creeper. Needed for returning it from Java.
pub struct Creeper<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Creeper<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Creeper from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Creeper")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Creeper object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Creeper<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Monster<'mc>> for Creeper<'mc> {
    fn into(self) -> crate::entity::Monster<'mc> {
        crate::entity::Monster::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements ComplexLivingEntity. Needed for returning it from Java.
pub struct ComplexLivingEntity<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ComplexLivingEntity<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ComplexLivingEntity from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "ComplexLivingEntity")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ComplexLivingEntity object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for ComplexLivingEntity<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::LivingEntity<'mc>> for ComplexLivingEntity<'mc> {
    fn into(self) -> crate::entity::LivingEntity<'mc> {
        crate::entity::LivingEntity::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct LlamaColor<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for LlamaColor<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> LlamaColor<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate LlamaColor from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "LlamaColor")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LlamaColor object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
pub struct HorseVariant<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for HorseVariant<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> HorseVariant<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate HorseVariant from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "HorseVariant")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a HorseVariant object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
/// An instantiatable struct that implements Shulker. Needed for returning it from Java.
pub struct Shulker<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Shulker<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Shulker from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Shulker")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Shulker object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Shulker<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Golem<'mc>> for Shulker<'mc> {
    fn into(self) -> crate::entity::Golem<'mc> {
        crate::entity::Golem::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::material::Colorable<'mc>> for Shulker<'mc> {
    fn into(self) -> crate::material::Colorable<'mc> {
        crate::material::Colorable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::entity::Enemy<'mc>> for Shulker<'mc> {
    fn into(self) -> crate::entity::Enemy<'mc> {
        crate::entity::Enemy::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements LivingEntity. Needed for returning it from Java.
pub struct LivingEntity<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> LivingEntity<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate LivingEntity from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "LivingEntity")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LivingEntity object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for LivingEntity<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::attribute::Attributable<'mc>> for LivingEntity<'mc> {
    fn into(self) -> crate::attribute::Attributable<'mc> {
        crate::attribute::Attributable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::entity::Damageable<'mc>> for LivingEntity<'mc> {
    fn into(self) -> crate::entity::Damageable<'mc> {
        crate::entity::Damageable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::projectiles::ProjectileSource<'mc>> for LivingEntity<'mc> {
    fn into(self) -> crate::projectiles::ProjectileSource<'mc> {
        crate::projectiles::ProjectileSource::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct AbstractArrowPickupStatus<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for AbstractArrowPickupStatus<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> AbstractArrowPickupStatus<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate AbstractArrowPickupStatus from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "AbstractArrowPickupStatus")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AbstractArrowPickupStatus object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
/// An instantiatable struct that implements Squid. Needed for returning it from Java.
pub struct Squid<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Squid<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Squid from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Squid")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Squid object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Squid<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::WaterMob<'mc>> for Squid<'mc> {
    fn into(self) -> crate::entity::WaterMob<'mc> {
        crate::entity::WaterMob::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct LightningStrikeSpigot<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for LightningStrikeSpigot<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> LightningStrikeSpigot<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate LightningStrikeSpigot from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "LightningStrikeSpigot")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LightningStrikeSpigot object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::entity::EntitySpigot<'mc>> for LightningStrikeSpigot<'mc> {
    fn into(self) -> crate::entity::EntitySpigot<'mc> {
        crate::entity::EntitySpigot::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements MagmaCube. Needed for returning it from Java.
pub struct MagmaCube<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> MagmaCube<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MagmaCube from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "MagmaCube")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MagmaCube object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for MagmaCube<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Slime<'mc>> for MagmaCube<'mc> {
    fn into(self) -> crate::entity::Slime<'mc> {
        crate::entity::Slime::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Egg. Needed for returning it from Java.
pub struct Egg<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Egg<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Egg from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Egg")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Egg object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Egg<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::ThrowableProjectile<'mc>> for Egg<'mc> {
    fn into(self) -> crate::entity::ThrowableProjectile<'mc> {
        crate::entity::ThrowableProjectile::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Frog. Needed for returning it from Java.
pub struct Frog<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Frog<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Frog from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Frog")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Frog object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Frog<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Animals<'mc>> for Frog<'mc> {
    fn into(self) -> crate::entity::Animals<'mc> {
        crate::entity::Animals::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct MushroomCowVariant<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for MushroomCowVariant<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> MushroomCowVariant<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate MushroomCowVariant from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "MushroomCowVariant")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MushroomCowVariant object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
/// An instantiatable struct that implements SizedFireball. Needed for returning it from Java.
pub struct SizedFireball<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> SizedFireball<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SizedFireball from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "SizedFireball")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SizedFireball object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for SizedFireball<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Fireball<'mc>> for SizedFireball<'mc> {
    fn into(self) -> crate::entity::Fireball<'mc> {
        crate::entity::Fireball::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Sniffer. Needed for returning it from Java.
pub struct Sniffer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Sniffer<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Sniffer from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Sniffer")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Sniffer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Sniffer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Animals<'mc>> for Sniffer<'mc> {
    fn into(self) -> crate::entity::Animals<'mc> {
        crate::entity::Animals::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements GlowItemFrame. Needed for returning it from Java.
pub struct GlowItemFrame<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> GlowItemFrame<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate GlowItemFrame from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "GlowItemFrame")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a GlowItemFrame object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for GlowItemFrame<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::ItemFrame<'mc>> for GlowItemFrame<'mc> {
    fn into(self) -> crate::entity::ItemFrame<'mc> {
        crate::entity::ItemFrame::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Painting. Needed for returning it from Java.
pub struct Painting<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Painting<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Painting from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Painting")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Painting object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Painting<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Hanging<'mc>> for Painting<'mc> {
    fn into(self) -> crate::entity::Hanging<'mc> {
        crate::entity::Hanging::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct SnifferState<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for SnifferState<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> SnifferState<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SnifferState from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "SnifferState")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SnifferState object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
/// An instantiatable struct that implements Animals. Needed for returning it from Java.
pub struct Animals<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Animals<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Animals from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Animals")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Animals object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Animals<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Breedable<'mc>> for Animals<'mc> {
    fn into(self) -> crate::entity::Breedable<'mc> {
        crate::entity::Breedable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Bat. Needed for returning it from Java.
pub struct Bat<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Bat<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Bat from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Bat")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Bat object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Bat<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Ambient<'mc>> for Bat<'mc> {
    fn into(self) -> crate::entity::Ambient<'mc> {
        crate::entity::Ambient::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements LlamaSpit. Needed for returning it from Java.
pub struct LlamaSpit<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> LlamaSpit<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate LlamaSpit from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "LlamaSpit")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LlamaSpit object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for LlamaSpit<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Projectile<'mc>> for LlamaSpit<'mc> {
    fn into(self) -> crate::entity::Projectile<'mc> {
        crate::entity::Projectile::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Ghast. Needed for returning it from Java.
pub struct Ghast<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Ghast<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Ghast from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Ghast")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Ghast object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Ghast<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Flying<'mc>> for Ghast<'mc> {
    fn into(self) -> crate::entity::Flying<'mc> {
        crate::entity::Flying::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::entity::Enemy<'mc>> for Ghast<'mc> {
    fn into(self) -> crate::entity::Enemy<'mc> {
        crate::entity::Enemy::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Sheep. Needed for returning it from Java.
pub struct Sheep<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Sheep<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Sheep from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Sheep")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Sheep object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Sheep<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Animals<'mc>> for Sheep<'mc> {
    fn into(self) -> crate::entity::Animals<'mc> {
        crate::entity::Animals::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::material::Colorable<'mc>> for Sheep<'mc> {
    fn into(self) -> crate::material::Colorable<'mc> {
        crate::material::Colorable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct TropicalFishPattern<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for TropicalFishPattern<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> TropicalFishPattern<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate TropicalFishPattern from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "TropicalFishPattern")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TropicalFishPattern object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
pub enum EntityCategoryEnum {
    None,
    Undead,
    Arthropod,
    Illager,
    Water,
}
impl std::fmt::Display for EntityCategoryEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            EntityCategoryEnum::None => f.write_str("NONE"),
            EntityCategoryEnum::Undead => f.write_str("UNDEAD"),
            EntityCategoryEnum::Arthropod => f.write_str("ARTHROPOD"),
            EntityCategoryEnum::Illager => f.write_str("ILLAGER"),
            EntityCategoryEnum::Water => f.write_str("WATER"),
        }
    }
}
pub struct EntityCategory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub EntityCategoryEnum,
);
impl<'mc> std::ops::Deref for EntityCategory<'mc> {
    type Target = EntityCategoryEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for EntityCategory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntityCategory<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: EntityCategoryEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityCategory from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "EntityCategory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityCategory object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const NONE: EntityCategoryEnum = EntityCategoryEnum::None;
    pub const UNDEAD: EntityCategoryEnum = EntityCategoryEnum::Undead;
    pub const ARTHROPOD: EntityCategoryEnum = EntityCategoryEnum::Arthropod;
    pub const ILLAGER: EntityCategoryEnum = EntityCategoryEnum::Illager;
    pub const WATER: EntityCategoryEnum = EntityCategoryEnum::Water;
    pub fn from_string(str: String) -> std::option::Option<EntityCategoryEnum> {
        match str.as_str() {
            "NONE" => Some(EntityCategoryEnum::None),
            "UNDEAD" => Some(EntityCategoryEnum::Undead),
            "ARTHROPOD" => Some(EntityCategoryEnum::Arthropod),
            "ILLAGER" => Some(EntityCategoryEnum::Illager),
            "WATER" => Some(EntityCategoryEnum::Water),
            _ => None,
        }
    }
}
pub mod memory;
pub mod minecart;
