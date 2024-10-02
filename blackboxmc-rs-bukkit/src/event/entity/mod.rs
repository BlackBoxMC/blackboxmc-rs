#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct PotionSplashEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PotionSplashEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PotionSplashEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PotionSplashEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/PotionSplashEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PotionSplashEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PotionSplashEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        potion: impl Into<crate::entity::ThrownPotion<'mc>>,
        hit_entity: impl Into<crate::entity::Entity<'mc>>,
        hit_block: std::option::Option<impl Into<crate::block::Block<'mc>>>,
        hit_face: std::option::Option<impl Into<crate::block::BlockFace<'mc>>>,
        affected_entities: std::option::Option<impl Into<blackboxmc_java::util::JavaMap<'mc>>>,
    ) -> Result<crate::event::entity::PotionSplashEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/ThrownPotion;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(potion.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/entity/Entity;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(hit_entity.into().jni_object().clone())
        });
        args.push(val_2);
        if let Some(a) = hit_block {
            sig += "Lorg/bukkit/block/Block;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        if let Some(a) = hit_face {
            sig += "Lorg/bukkit/block/BlockFace;";
            let val_4 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_4);
        }
        if let Some(a) = affected_entities {
            sig += "Ljava/util/Map;";
            let val_5 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_5);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/PotionSplashEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::PotionSplashEvent::from_raw(&jni, res)
    }

    pub fn entity(&self) -> Result<crate::entity::ThrownPotion<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/ThrownPotion;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::ThrownPotion::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the potion which caused this event
    pub fn potion(&self) -> Result<crate::entity::ThrownPotion<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/ThrownPotion;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPotion", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::ThrownPotion::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Retrieves a list of all effected entities
    pub fn affected_entities(
        &self,
    ) -> Result<Vec<crate::entity::LivingEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Collection;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAffectedEntities",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = col.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::LivingEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    /// Gets the intensity of the potion's effects for given entity; This
    /// depends on the distance to the impact center
    pub fn get_intensity(
        &self,
        entity: impl Into<crate::entity::LivingEntity<'mc>>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/LivingEntity;)D");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(entity.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getIntensity",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Overwrites the intensity for a given entity
    pub fn set_intensity(
        &self,
        entity: impl Into<crate::entity::LivingEntity<'mc>>,
        intensity: f64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/LivingEntity;D)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(entity.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Double(intensity);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setIntensity",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/PotionSplashEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.ProjectileHitEvent ( ['getEntity', 'getPotion', 'getAffectedEntities', 'getIntensity', 'setIntensity', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the block that was hit, if it was a block that was hit.
    pub fn hit_block(
        &self,
    ) -> Result<Option<crate::block::Block<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::ProjectileHitEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::ProjectileHitEvent = temp_clone.into();
        real.hit_block()
    }
    /// Gets the block face that was hit, if it was a block that was hit and the
    /// face was provided in the event.
    pub fn hit_block_face(
        &self,
    ) -> Result<Option<crate::block::BlockFace<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::ProjectileHitEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::ProjectileHitEvent = temp_clone.into();
        real.hit_block_face()
    }
    /// Gets the entity that was hit, if it was an entity that was hit.
    pub fn hit_entity(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::ProjectileHitEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::ProjectileHitEvent = temp_clone.into();
        real.hit_entity()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PotionSplashEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PotionSplashEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::ProjectileHitEvent<'mc>> for PotionSplashEvent<'mc> {
    fn into(self) -> crate::event::entity::ProjectileHitEvent<'mc> {
        crate::event::entity::ProjectileHitEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting PotionSplashEvent into crate::event::entity::ProjectileHitEvent",
        )
    }
}
#[repr(C)]
pub struct EntityTargetLivingEntityEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityTargetLivingEntityEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityTargetLivingEntityEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityTargetLivingEntityEvent from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityTargetLivingEntityEvent",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityTargetLivingEntityEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityTargetLivingEntityEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        entity: impl Into<crate::entity::Entity<'mc>>,
        target: impl Into<crate::entity::LivingEntity<'mc>>,
        reason: impl Into<crate::event::entity::EntityTargetEventTargetReason<'mc>>,
    ) -> Result<crate::event::entity::EntityTargetLivingEntityEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/entity/Entity;Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/event/entity/EntityTargetEvent/TargetReason;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(entity.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(target.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(reason.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityTargetLivingEntityEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityTargetLivingEntityEvent::from_raw(&jni, res)
    }

    pub fn target(
        &self,
    ) -> Result<Option<crate::entity::LivingEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/LivingEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTarget", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::LivingEntity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Set the Entity that you want the mob to target.
    ///
    /// It is possible to be null, null will cause the entity to be
    /// target-less.
    ///
    /// Must be a LivingEntity, or null.
    pub fn set_target(
        &self,
        target: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(target.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTarget",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityTargetEvent ( ['getTarget', 'setTarget'])

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityTargetEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityTargetEvent = temp_clone.into();
        real.is_cancelled()
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityTargetEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityTargetEvent = temp_clone.into();
        real.set_cancelled(cancel)
    }
    /// Returns the reason for the targeting
    pub fn reason(
        &self,
    ) -> Result<crate::event::entity::EntityTargetEventTargetReason<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = crate::event::entity::EntityTargetEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityTargetEvent = temp_clone.into();
        real.reason()
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        crate::event::entity::EntityTargetEvent::handler_list(jni)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntityTargetEvent<'mc>>
    for EntityTargetLivingEntityEvent<'mc>
{
    fn into(self) -> crate::event::entity::EntityTargetEvent<'mc> {
        crate::event::entity::EntityTargetEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting EntityTargetLivingEntityEvent into crate::event::entity::EntityTargetEvent")
    }
}
#[repr(C)]
pub struct EntityDamageEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityDamageEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityDamageEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityDamageEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/EntityDamageEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityDamageEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityDamageEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        damagee: impl Into<crate::entity::Entity<'mc>>,
        cause: impl Into<crate::event::entity::EntityDamageEventDamageCause<'mc>>,
        damage_source: impl Into<crate::damage::DamageSource<'mc>>,
        modifiers: std::option::Option<impl Into<blackboxmc_java::util::JavaMap<'mc>>>,
        modifier_functions: std::option::Option<impl Into<blackboxmc_java::util::JavaMap<'mc>>>,
    ) -> Result<crate::event::entity::EntityDamageEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(damagee.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/event/entity/EntityDamageEvent/DamageCause;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(cause.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "Lorg/bukkit/damage/DamageSource;";
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(damage_source.into().jni_object().clone())
        });
        args.push(val_3);
        if let Some(a) = modifiers {
            sig += "Ljava/util/Map;";
            let val_4 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_4);
        }
        if let Some(a) = modifier_functions {
            sig += "Ljava/util/Map;";
            let val_5 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_5);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/EntityDamageEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityDamageEvent::from_raw(&jni, res)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the original damage for the specified modifier, as defined at this
    /// event's construction.
    pub fn get_original_damage(
        &self,
        val_type: impl Into<crate::event::entity::EntityDamageEventDamageModifier<'mc>>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/event/entity/EntityDamageEvent/DamageModifier;)D");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_type.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOriginalDamage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Sets the damage for the specified modifier.
    pub fn set_damage(
        &self,
        val_type: impl Into<crate::event::entity::EntityDamageEventDamageModifier<'mc>>,
        damage: std::option::Option<f64>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/event/entity/EntityDamageEvent/DamageModifier;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_type.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = damage {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a);
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setDamage", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the raw amount of damage caused by the event
    pub fn damage(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDamage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// This checks to see if a particular modifier is valid for this event's
    /// caller, such that, {@link #setDamage(DamageModifier, double)} will not
    /// throw an {@link UnsupportedOperationException}.
    ///
    /// {@link DamageModifier#BASE} is always applicable.
    pub fn is_applicable(
        &self,
        val_type: impl Into<crate::event::entity::EntityDamageEventDamageModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/event/entity/EntityDamageEvent/DamageModifier;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_type.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isApplicable",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the amount of damage caused by the event after all damage
    /// reduction is applied.
    pub fn final_damage(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFinalDamage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the cause of the damage.
    ///
    /// While a DamageCause may indicate a specific Bukkit-assigned cause of damage,
    /// {@link #getDamageSource()} may expose additional types of damage such as custom
    /// damage types provided by data packs, as well as any direct or indirect entities,
    /// locations, or other contributing factors to the damage being inflicted. The
    /// alternative is generally preferred, but DamageCauses provided to this event
    /// should largely encompass most common use cases for developers if a simple cause
    /// is required.
    pub fn cause(
        &self,
    ) -> Result<crate::event::entity::EntityDamageEventDamageCause<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/event/entity/EntityDamageEvent/DamageCause;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCause", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::entity::EntityDamageEventDamageCause::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the source of damage.
    pub fn damage_source(
        &self,
    ) -> Result<crate::damage::DamageSource<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/damage/DamageSource;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDamageSource", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::damage::DamageSource::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityDamageEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['isCancelled', 'setCancelled', 'getOriginalDamage', 'setDamage', 'getDamage', 'isApplicable', 'getFinalDamage', 'getCause', 'getDamageSource', 'getHandlers', 'getHandlerList'])
    /// Returns the Entity involved in this event
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityDamageEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityDamageEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityDamageEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityDamageEvent into crate::event::entity::EntityEvent")
    }
}
pub enum EntityDamageEventDamageModifier<'mc> {
    Base {
        inner: EntityDamageEventDamageModifierStruct<'mc>,
    },
    Freezing {
        inner: EntityDamageEventDamageModifierStruct<'mc>,
    },
    HardHat {
        inner: EntityDamageEventDamageModifierStruct<'mc>,
    },
    Blocking {
        inner: EntityDamageEventDamageModifierStruct<'mc>,
    },
    Armor {
        inner: EntityDamageEventDamageModifierStruct<'mc>,
    },
    Resistance {
        inner: EntityDamageEventDamageModifierStruct<'mc>,
    },
    Magic {
        inner: EntityDamageEventDamageModifierStruct<'mc>,
    },
    Absorption {
        inner: EntityDamageEventDamageModifierStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for EntityDamageEventDamageModifier<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityDamageEventDamageModifier::Base { .. } => f.write_str("BASE"),
            EntityDamageEventDamageModifier::Freezing { .. } => f.write_str("FREEZING"),
            EntityDamageEventDamageModifier::HardHat { .. } => f.write_str("HARD_HAT"),
            EntityDamageEventDamageModifier::Blocking { .. } => f.write_str("BLOCKING"),
            EntityDamageEventDamageModifier::Armor { .. } => f.write_str("ARMOR"),
            EntityDamageEventDamageModifier::Resistance { .. } => f.write_str("RESISTANCE"),
            EntityDamageEventDamageModifier::Magic { .. } => f.write_str("MAGIC"),
            EntityDamageEventDamageModifier::Absorption { .. } => f.write_str("ABSORPTION"),
        }
    }
}
impl<'mc> std::ops::Deref for EntityDamageEventDamageModifier<'mc> {
    type Target = EntityDamageEventDamageModifierStruct<'mc>;
    fn deref(&self) -> &<EntityDamageEventDamageModifier<'mc> as std::ops::Deref>::Target {
        match self {
            EntityDamageEventDamageModifier::Base { inner } => inner,
            EntityDamageEventDamageModifier::Freezing { inner } => inner,
            EntityDamageEventDamageModifier::HardHat { inner } => inner,
            EntityDamageEventDamageModifier::Blocking { inner } => inner,
            EntityDamageEventDamageModifier::Armor { inner } => inner,
            EntityDamageEventDamageModifier::Resistance { inner } => inner,
            EntityDamageEventDamageModifier::Magic { inner } => inner,
            EntityDamageEventDamageModifier::Absorption { inner } => inner,
        }
    }
}

impl<'mc> EntityDamageEventDamageModifier<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<EntityDamageEventDamageModifier<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/EntityDamageEvent/DamageModifier");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/EntityDamageEvent/DamageModifier;",
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = env.translate_error(res)?;
        let obj = res.l()?;
        let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = env.translate_error(variant)?;
        let variant_str = env
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        match variant_str.as_str() {
            "BASE" => Ok(EntityDamageEventDamageModifier::Base {
                inner: EntityDamageEventDamageModifierStruct::from_raw(env, obj)?,
            }),
            "FREEZING" => Ok(EntityDamageEventDamageModifier::Freezing {
                inner: EntityDamageEventDamageModifierStruct::from_raw(env, obj)?,
            }),
            "HARD_HAT" => Ok(EntityDamageEventDamageModifier::HardHat {
                inner: EntityDamageEventDamageModifierStruct::from_raw(env, obj)?,
            }),
            "BLOCKING" => Ok(EntityDamageEventDamageModifier::Blocking {
                inner: EntityDamageEventDamageModifierStruct::from_raw(env, obj)?,
            }),
            "ARMOR" => Ok(EntityDamageEventDamageModifier::Armor {
                inner: EntityDamageEventDamageModifierStruct::from_raw(env, obj)?,
            }),
            "RESISTANCE" => Ok(EntityDamageEventDamageModifier::Resistance {
                inner: EntityDamageEventDamageModifierStruct::from_raw(env, obj)?,
            }),
            "MAGIC" => Ok(EntityDamageEventDamageModifier::Magic {
                inner: EntityDamageEventDamageModifierStruct::from_raw(env, obj)?,
            }),
            "ABSORPTION" => Ok(EntityDamageEventDamageModifier::Absorption {
                inner: EntityDamageEventDamageModifierStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct EntityDamageEventDamageModifierStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityDamageEventDamageModifier<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Base { inner } => inner.0.clone(),
            Self::Freezing { inner } => inner.0.clone(),
            Self::HardHat { inner } => inner.0.clone(),
            Self::Blocking { inner } => inner.0.clone(),
            Self::Armor { inner } => inner.0.clone(),
            Self::Resistance { inner } => inner.0.clone(),
            Self::Magic { inner } => inner.0.clone(),
            Self::Absorption { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Base { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Freezing { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::HardHat { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Blocking { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Armor { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Resistance { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Magic { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Absorption { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityDamageEventDamageModifier<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityDamageEventDamageModifier from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityDamageEvent/DamageModifier",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntityDamageEventDamageModifier object, got {}",
                    name
                )
                .into())
        } else {
            let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
            let variant = env.translate_error(variant)?;
            let variant_str = env
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            match variant_str.as_str() {
                "BASE" => Ok(EntityDamageEventDamageModifier::Base {
                    inner: EntityDamageEventDamageModifierStruct::from_raw(env, obj)?,
                }),
                "FREEZING" => Ok(EntityDamageEventDamageModifier::Freezing {
                    inner: EntityDamageEventDamageModifierStruct::from_raw(env, obj)?,
                }),
                "HARD_HAT" => Ok(EntityDamageEventDamageModifier::HardHat {
                    inner: EntityDamageEventDamageModifierStruct::from_raw(env, obj)?,
                }),
                "BLOCKING" => Ok(EntityDamageEventDamageModifier::Blocking {
                    inner: EntityDamageEventDamageModifierStruct::from_raw(env, obj)?,
                }),
                "ARMOR" => Ok(EntityDamageEventDamageModifier::Armor {
                    inner: EntityDamageEventDamageModifierStruct::from_raw(env, obj)?,
                }),
                "RESISTANCE" => Ok(EntityDamageEventDamageModifier::Resistance {
                    inner: EntityDamageEventDamageModifierStruct::from_raw(env, obj)?,
                }),
                "MAGIC" => Ok(EntityDamageEventDamageModifier::Magic {
                    inner: EntityDamageEventDamageModifierStruct::from_raw(env, obj)?,
                }),
                "ABSORPTION" => Ok(EntityDamageEventDamageModifier::Absorption {
                    inner: EntityDamageEventDamageModifierStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for EntityDamageEventDamageModifierStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityDamageEventDamageModifierStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityDamageEventDamageModifierStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityDamageEvent/DamageModifier",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntityDamageEventDamageModifierStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityDamageEventDamageModifierStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<
        crate::event::entity::EntityDamageEventDamageModifier<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Lorg/bukkit/event/entity/EntityDamageEvent/DamageModifier;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityDamageEvent/DamageModifier");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::entity::EntityDamageEventDamageModifier::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub enum EntityDamageEventDamageCause<'mc> {
    Kill {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    WorldBorder {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Contact {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    EntityAttack {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    EntitySweepAttack {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Projectile {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Suffocation {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Fall {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Fire {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    FireTick {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Melting {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Lava {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Drowning {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    BlockExplosion {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    EntityExplosion {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Void {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Lightning {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Suicide {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Starvation {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Poison {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Magic {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Wither {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    FallingBlock {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Thorns {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    DragonBreath {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Custom {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    FlyIntoWall {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    HotFloor {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Campfire {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Cramming {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Dryout {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    Freeze {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
    SonicBoom {
        inner: EntityDamageEventDamageCauseStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for EntityDamageEventDamageCause<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityDamageEventDamageCause::Kill { .. } => f.write_str("KILL"),
            EntityDamageEventDamageCause::WorldBorder { .. } => f.write_str("WORLD_BORDER"),
            EntityDamageEventDamageCause::Contact { .. } => f.write_str("CONTACT"),
            EntityDamageEventDamageCause::EntityAttack { .. } => f.write_str("ENTITY_ATTACK"),
            EntityDamageEventDamageCause::EntitySweepAttack { .. } => {
                f.write_str("ENTITY_SWEEP_ATTACK")
            }
            EntityDamageEventDamageCause::Projectile { .. } => f.write_str("PROJECTILE"),
            EntityDamageEventDamageCause::Suffocation { .. } => f.write_str("SUFFOCATION"),
            EntityDamageEventDamageCause::Fall { .. } => f.write_str("FALL"),
            EntityDamageEventDamageCause::Fire { .. } => f.write_str("FIRE"),
            EntityDamageEventDamageCause::FireTick { .. } => f.write_str("FIRE_TICK"),
            EntityDamageEventDamageCause::Melting { .. } => f.write_str("MELTING"),
            EntityDamageEventDamageCause::Lava { .. } => f.write_str("LAVA"),
            EntityDamageEventDamageCause::Drowning { .. } => f.write_str("DROWNING"),
            EntityDamageEventDamageCause::BlockExplosion { .. } => f.write_str("BLOCK_EXPLOSION"),
            EntityDamageEventDamageCause::EntityExplosion { .. } => f.write_str("ENTITY_EXPLOSION"),
            EntityDamageEventDamageCause::Void { .. } => f.write_str("VOID"),
            EntityDamageEventDamageCause::Lightning { .. } => f.write_str("LIGHTNING"),
            EntityDamageEventDamageCause::Suicide { .. } => f.write_str("SUICIDE"),
            EntityDamageEventDamageCause::Starvation { .. } => f.write_str("STARVATION"),
            EntityDamageEventDamageCause::Poison { .. } => f.write_str("POISON"),
            EntityDamageEventDamageCause::Magic { .. } => f.write_str("MAGIC"),
            EntityDamageEventDamageCause::Wither { .. } => f.write_str("WITHER"),
            EntityDamageEventDamageCause::FallingBlock { .. } => f.write_str("FALLING_BLOCK"),
            EntityDamageEventDamageCause::Thorns { .. } => f.write_str("THORNS"),
            EntityDamageEventDamageCause::DragonBreath { .. } => f.write_str("DRAGON_BREATH"),
            EntityDamageEventDamageCause::Custom { .. } => f.write_str("CUSTOM"),
            EntityDamageEventDamageCause::FlyIntoWall { .. } => f.write_str("FLY_INTO_WALL"),
            EntityDamageEventDamageCause::HotFloor { .. } => f.write_str("HOT_FLOOR"),
            EntityDamageEventDamageCause::Campfire { .. } => f.write_str("CAMPFIRE"),
            EntityDamageEventDamageCause::Cramming { .. } => f.write_str("CRAMMING"),
            EntityDamageEventDamageCause::Dryout { .. } => f.write_str("DRYOUT"),
            EntityDamageEventDamageCause::Freeze { .. } => f.write_str("FREEZE"),
            EntityDamageEventDamageCause::SonicBoom { .. } => f.write_str("SONIC_BOOM"),
        }
    }
}
impl<'mc> std::ops::Deref for EntityDamageEventDamageCause<'mc> {
    type Target = EntityDamageEventDamageCauseStruct<'mc>;
    fn deref(&self) -> &<EntityDamageEventDamageCause<'mc> as std::ops::Deref>::Target {
        match self {
            EntityDamageEventDamageCause::Kill { inner } => inner,
            EntityDamageEventDamageCause::WorldBorder { inner } => inner,
            EntityDamageEventDamageCause::Contact { inner } => inner,
            EntityDamageEventDamageCause::EntityAttack { inner } => inner,
            EntityDamageEventDamageCause::EntitySweepAttack { inner } => inner,
            EntityDamageEventDamageCause::Projectile { inner } => inner,
            EntityDamageEventDamageCause::Suffocation { inner } => inner,
            EntityDamageEventDamageCause::Fall { inner } => inner,
            EntityDamageEventDamageCause::Fire { inner } => inner,
            EntityDamageEventDamageCause::FireTick { inner } => inner,
            EntityDamageEventDamageCause::Melting { inner } => inner,
            EntityDamageEventDamageCause::Lava { inner } => inner,
            EntityDamageEventDamageCause::Drowning { inner } => inner,
            EntityDamageEventDamageCause::BlockExplosion { inner } => inner,
            EntityDamageEventDamageCause::EntityExplosion { inner } => inner,
            EntityDamageEventDamageCause::Void { inner } => inner,
            EntityDamageEventDamageCause::Lightning { inner } => inner,
            EntityDamageEventDamageCause::Suicide { inner } => inner,
            EntityDamageEventDamageCause::Starvation { inner } => inner,
            EntityDamageEventDamageCause::Poison { inner } => inner,
            EntityDamageEventDamageCause::Magic { inner } => inner,
            EntityDamageEventDamageCause::Wither { inner } => inner,
            EntityDamageEventDamageCause::FallingBlock { inner } => inner,
            EntityDamageEventDamageCause::Thorns { inner } => inner,
            EntityDamageEventDamageCause::DragonBreath { inner } => inner,
            EntityDamageEventDamageCause::Custom { inner } => inner,
            EntityDamageEventDamageCause::FlyIntoWall { inner } => inner,
            EntityDamageEventDamageCause::HotFloor { inner } => inner,
            EntityDamageEventDamageCause::Campfire { inner } => inner,
            EntityDamageEventDamageCause::Cramming { inner } => inner,
            EntityDamageEventDamageCause::Dryout { inner } => inner,
            EntityDamageEventDamageCause::Freeze { inner } => inner,
            EntityDamageEventDamageCause::SonicBoom { inner } => inner,
        }
    }
}

impl<'mc> EntityDamageEventDamageCause<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<EntityDamageEventDamageCause<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/EntityDamageEvent/DamageCause");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/EntityDamageEvent/DamageCause;",
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = env.translate_error(res)?;
        let obj = res.l()?;
        let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = env.translate_error(variant)?;
        let variant_str = env
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        match variant_str.as_str() {
            "KILL" => Ok(EntityDamageEventDamageCause::Kill {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "WORLD_BORDER" => Ok(EntityDamageEventDamageCause::WorldBorder {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "CONTACT" => Ok(EntityDamageEventDamageCause::Contact {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "ENTITY_ATTACK" => Ok(EntityDamageEventDamageCause::EntityAttack {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "ENTITY_SWEEP_ATTACK" => Ok(EntityDamageEventDamageCause::EntitySweepAttack {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "PROJECTILE" => Ok(EntityDamageEventDamageCause::Projectile {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "SUFFOCATION" => Ok(EntityDamageEventDamageCause::Suffocation {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "FALL" => Ok(EntityDamageEventDamageCause::Fall {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "FIRE" => Ok(EntityDamageEventDamageCause::Fire {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "FIRE_TICK" => Ok(EntityDamageEventDamageCause::FireTick {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "MELTING" => Ok(EntityDamageEventDamageCause::Melting {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "LAVA" => Ok(EntityDamageEventDamageCause::Lava {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "DROWNING" => Ok(EntityDamageEventDamageCause::Drowning {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "BLOCK_EXPLOSION" => Ok(EntityDamageEventDamageCause::BlockExplosion {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "ENTITY_EXPLOSION" => Ok(EntityDamageEventDamageCause::EntityExplosion {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "VOID" => Ok(EntityDamageEventDamageCause::Void {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "LIGHTNING" => Ok(EntityDamageEventDamageCause::Lightning {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "SUICIDE" => Ok(EntityDamageEventDamageCause::Suicide {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "STARVATION" => Ok(EntityDamageEventDamageCause::Starvation {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "POISON" => Ok(EntityDamageEventDamageCause::Poison {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "MAGIC" => Ok(EntityDamageEventDamageCause::Magic {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "WITHER" => Ok(EntityDamageEventDamageCause::Wither {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "FALLING_BLOCK" => Ok(EntityDamageEventDamageCause::FallingBlock {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "THORNS" => Ok(EntityDamageEventDamageCause::Thorns {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "DRAGON_BREATH" => Ok(EntityDamageEventDamageCause::DragonBreath {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "CUSTOM" => Ok(EntityDamageEventDamageCause::Custom {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "FLY_INTO_WALL" => Ok(EntityDamageEventDamageCause::FlyIntoWall {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "HOT_FLOOR" => Ok(EntityDamageEventDamageCause::HotFloor {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "CAMPFIRE" => Ok(EntityDamageEventDamageCause::Campfire {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "CRAMMING" => Ok(EntityDamageEventDamageCause::Cramming {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "DRYOUT" => Ok(EntityDamageEventDamageCause::Dryout {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "FREEZE" => Ok(EntityDamageEventDamageCause::Freeze {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),
            "SONIC_BOOM" => Ok(EntityDamageEventDamageCause::SonicBoom {
                inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct EntityDamageEventDamageCauseStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityDamageEventDamageCause<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Kill { inner } => inner.0.clone(),
            Self::WorldBorder { inner } => inner.0.clone(),
            Self::Contact { inner } => inner.0.clone(),
            Self::EntityAttack { inner } => inner.0.clone(),
            Self::EntitySweepAttack { inner } => inner.0.clone(),
            Self::Projectile { inner } => inner.0.clone(),
            Self::Suffocation { inner } => inner.0.clone(),
            Self::Fall { inner } => inner.0.clone(),
            Self::Fire { inner } => inner.0.clone(),
            Self::FireTick { inner } => inner.0.clone(),
            Self::Melting { inner } => inner.0.clone(),
            Self::Lava { inner } => inner.0.clone(),
            Self::Drowning { inner } => inner.0.clone(),
            Self::BlockExplosion { inner } => inner.0.clone(),
            Self::EntityExplosion { inner } => inner.0.clone(),
            Self::Void { inner } => inner.0.clone(),
            Self::Lightning { inner } => inner.0.clone(),
            Self::Suicide { inner } => inner.0.clone(),
            Self::Starvation { inner } => inner.0.clone(),
            Self::Poison { inner } => inner.0.clone(),
            Self::Magic { inner } => inner.0.clone(),
            Self::Wither { inner } => inner.0.clone(),
            Self::FallingBlock { inner } => inner.0.clone(),
            Self::Thorns { inner } => inner.0.clone(),
            Self::DragonBreath { inner } => inner.0.clone(),
            Self::Custom { inner } => inner.0.clone(),
            Self::FlyIntoWall { inner } => inner.0.clone(),
            Self::HotFloor { inner } => inner.0.clone(),
            Self::Campfire { inner } => inner.0.clone(),
            Self::Cramming { inner } => inner.0.clone(),
            Self::Dryout { inner } => inner.0.clone(),
            Self::Freeze { inner } => inner.0.clone(),
            Self::SonicBoom { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Kill { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::WorldBorder { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Contact { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::EntityAttack { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EntitySweepAttack { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Projectile { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Suffocation { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Fall { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Fire { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::FireTick { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Melting { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Lava { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Drowning { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::BlockExplosion { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EntityExplosion { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Void { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Lightning { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Suicide { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Starvation { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Poison { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Magic { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Wither { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::FallingBlock { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Thorns { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::DragonBreath { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Custom { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::FlyIntoWall { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::HotFloor { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Campfire { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Cramming { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Dryout { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Freeze { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SonicBoom { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityDamageEventDamageCause<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityDamageEventDamageCause from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityDamageEvent/DamageCause",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityDamageEventDamageCause object, got {}",
                name
            )
            .into())
        } else {
            let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
            let variant = env.translate_error(variant)?;
            let variant_str = env
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            match variant_str.as_str() {
                "KILL" => Ok(EntityDamageEventDamageCause::Kill {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "WORLD_BORDER" => Ok(EntityDamageEventDamageCause::WorldBorder {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "CONTACT" => Ok(EntityDamageEventDamageCause::Contact {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "ENTITY_ATTACK" => Ok(EntityDamageEventDamageCause::EntityAttack {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "ENTITY_SWEEP_ATTACK" => Ok(EntityDamageEventDamageCause::EntitySweepAttack {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "PROJECTILE" => Ok(EntityDamageEventDamageCause::Projectile {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "SUFFOCATION" => Ok(EntityDamageEventDamageCause::Suffocation {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "FALL" => Ok(EntityDamageEventDamageCause::Fall {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "FIRE" => Ok(EntityDamageEventDamageCause::Fire {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "FIRE_TICK" => Ok(EntityDamageEventDamageCause::FireTick {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "MELTING" => Ok(EntityDamageEventDamageCause::Melting {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "LAVA" => Ok(EntityDamageEventDamageCause::Lava {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "DROWNING" => Ok(EntityDamageEventDamageCause::Drowning {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "BLOCK_EXPLOSION" => Ok(EntityDamageEventDamageCause::BlockExplosion {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "ENTITY_EXPLOSION" => Ok(EntityDamageEventDamageCause::EntityExplosion {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "VOID" => Ok(EntityDamageEventDamageCause::Void {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "LIGHTNING" => Ok(EntityDamageEventDamageCause::Lightning {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "SUICIDE" => Ok(EntityDamageEventDamageCause::Suicide {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "STARVATION" => Ok(EntityDamageEventDamageCause::Starvation {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "POISON" => Ok(EntityDamageEventDamageCause::Poison {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "MAGIC" => Ok(EntityDamageEventDamageCause::Magic {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "WITHER" => Ok(EntityDamageEventDamageCause::Wither {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "FALLING_BLOCK" => Ok(EntityDamageEventDamageCause::FallingBlock {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "THORNS" => Ok(EntityDamageEventDamageCause::Thorns {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "DRAGON_BREATH" => Ok(EntityDamageEventDamageCause::DragonBreath {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "CUSTOM" => Ok(EntityDamageEventDamageCause::Custom {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "FLY_INTO_WALL" => Ok(EntityDamageEventDamageCause::FlyIntoWall {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "HOT_FLOOR" => Ok(EntityDamageEventDamageCause::HotFloor {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "CAMPFIRE" => Ok(EntityDamageEventDamageCause::Campfire {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "CRAMMING" => Ok(EntityDamageEventDamageCause::Cramming {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "DRYOUT" => Ok(EntityDamageEventDamageCause::Dryout {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "FREEZE" => Ok(EntityDamageEventDamageCause::Freeze {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                "SONIC_BOOM" => Ok(EntityDamageEventDamageCause::SonicBoom {
                    inner: EntityDamageEventDamageCauseStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for EntityDamageEventDamageCauseStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityDamageEventDamageCauseStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityDamageEventDamageCauseStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityDamageEvent/DamageCause",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntityDamageEventDamageCauseStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityDamageEventDamageCauseStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::entity::EntityDamageEventDamageCause<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/event/entity/EntityDamageEvent/DamageCause;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityDamageEvent/DamageCause");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::entity::EntityDamageEventDamageCause::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct SheepRegrowWoolEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SheepRegrowWoolEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SheepRegrowWoolEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SheepRegrowWoolEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/SheepRegrowWoolEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SheepRegrowWoolEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SheepRegrowWoolEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        sheep: impl Into<crate::entity::Sheep<'mc>>,
    ) -> Result<crate::event::entity::SheepRegrowWoolEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Sheep;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(sheep.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/SheepRegrowWoolEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::SheepRegrowWoolEvent::from_raw(&jni, res)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn entity(&self) -> Result<crate::entity::Sheep<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Sheep;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Sheep::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/SheepRegrowWoolEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['isCancelled', 'setCancelled', 'getEntity', 'getHandlers', 'getHandlerList'])
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for SheepRegrowWoolEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SheepRegrowWoolEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for SheepRegrowWoolEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SheepRegrowWoolEvent into crate::event::entity::EntityEvent")
    }
}
#[repr(C)]
pub struct FoodLevelChangeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for FoodLevelChangeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for FoodLevelChangeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate FoodLevelChangeEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/FoodLevelChangeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FoodLevelChangeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> FoodLevelChangeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        what: impl Into<crate::entity::HumanEntity<'mc>>,
        level: i32,
        item: std::option::Option<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<crate::event::entity::FoodLevelChangeEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/HumanEntity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(what.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "I";
        let val_2 = jni::objects::JValueGen::Int(level);
        args.push(val_2);
        if let Some(a) = item {
            sig += "Lorg/bukkit/inventory/ItemStack;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/FoodLevelChangeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::FoodLevelChangeEvent::from_raw(&jni, res)
    }

    pub fn entity(&self) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/HumanEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::HumanEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the item that triggered this event, if any.
    pub fn item(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getItem", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Gets the resultant food level that the entity involved in this event
    /// should be set to.
    ///
    /// Where 20 is a full food bar and 0 is an empty one.
    pub fn food_level(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFoodLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the resultant food level that the entity involved in this event
    /// should be set to
    pub fn set_food_level(&self, level: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(level);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFoodLevel",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/FoodLevelChangeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getEntity', 'getItem', 'getFoodLevel', 'setFoodLevel', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for FoodLevelChangeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting FoodLevelChangeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for FoodLevelChangeEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting FoodLevelChangeEvent into crate::event::entity::EntityEvent")
    }
}
#[repr(C)]
pub struct EntityAirChangeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityAirChangeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityAirChangeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityAirChangeEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityAirChangeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityAirChangeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityAirChangeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        what: impl Into<crate::entity::Entity<'mc>>,
        amount: i32,
    ) -> Result<crate::event::entity::EntityAirChangeEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;I)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(what.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(amount);
        let cls = jni.find_class("org/bukkit/event/entity/EntityAirChangeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityAirChangeEvent::from_raw(&jni, res)
    }
    /// Gets the amount of air the entity has left (measured in ticks).
    pub fn amount(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAmount", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the amount of air remaining for the entity (measured in ticks.
    pub fn set_amount(&self, amount: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(amount);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAmount",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancelled: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancelled.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityAirChangeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getAmount', 'setAmount', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Returns the Entity involved in this event
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityAirChangeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityAirChangeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityAirChangeEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityAirChangeEvent into crate::event::entity::EntityEvent")
    }
}
#[repr(C)]
pub struct ExplosionPrimeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ExplosionPrimeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ExplosionPrimeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ExplosionPrimeEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/ExplosionPrimeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ExplosionPrimeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ExplosionPrimeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        what: impl Into<crate::entity::Entity<'mc>>,
        radius: std::option::Option<f32>,
        fire: std::option::Option<bool>,
    ) -> Result<crate::event::entity::ExplosionPrimeEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(what.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = radius {
            sig += "F";
            let val_2 = jni::objects::JValueGen::Float(a);
            args.push(val_2);
        }
        if let Some(a) = fire {
            sig += "Z";
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/ExplosionPrimeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::ExplosionPrimeEvent::from_raw(&jni, res)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the radius of the explosion
    pub fn radius(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRadius", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// Sets the radius of the explosion
    pub fn set_radius(&self, radius: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(radius);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRadius",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether this explosion will create fire or not
    pub fn fire(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFire", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether this explosion will create fire or not
    pub fn set_fire(&self, fire: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(fire.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFire",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/ExplosionPrimeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['isCancelled', 'setCancelled', 'getRadius', 'setRadius', 'getFire', 'setFire', 'getHandlers', 'getHandlerList'])
    /// Returns the Entity involved in this event
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for ExplosionPrimeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ExplosionPrimeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for ExplosionPrimeEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ExplosionPrimeEvent into crate::event::entity::EntityEvent")
    }
}
#[repr(C)]
pub struct EntityTransformEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityTransformEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityTransformEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityTransformEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityTransformEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityTransformEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityTransformEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        original: impl Into<crate::entity::Entity<'mc>>,
        converted_list: Vec<jni::objects::JObject<'mc>>,
        transform_reason: impl Into<crate::event::entity::EntityTransformEventTransformReason<'mc>>,
    ) -> Result<crate::event::entity::EntityTransformEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;Ljava/util/List;Lorg/bukkit/event/entity/EntityTransformEvent/TransformReason;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(original.into().jni_object().clone())
        });
        let raw_val_2 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
        for v in converted_list {
            let map_val_0 = jni::objects::JValueGen::Object(v);
            jni.call_method(
                &raw_val_2,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(transform_reason.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityTransformEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityTransformEvent::from_raw(&jni, res)
    }
    /// Gets the entity that the original entity was transformed to.
    /// This returns the first entity in the transformed entity list.
    pub fn transformed_entity(
        &self,
    ) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTransformedEntity",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the entities that the original entity was transformed to.
    pub fn transformed_entities(
        &self,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTransformedEntities",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::Entity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    /// Gets the reason for the conversion that has occurred.
    pub fn transform_reason(
        &self,
    ) -> Result<
        crate::event::entity::EntityTransformEventTransformReason<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Lorg/bukkit/event/entity/EntityTransformEvent/TransformReason;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTransformReason",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::entity::EntityTransformEventTransformReason::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityTransformEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getTransformedEntity', 'getTransformedEntities', 'getTransformReason', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Returns the Entity involved in this event
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityTransformEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityTransformEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityTransformEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityTransformEvent into crate::event::entity::EntityEvent")
    }
}
pub enum EntityTransformEventTransformReason<'mc> {
    Cured {
        inner: EntityTransformEventTransformReasonStruct<'mc>,
    },
    Frozen {
        inner: EntityTransformEventTransformReasonStruct<'mc>,
    },
    Infection {
        inner: EntityTransformEventTransformReasonStruct<'mc>,
    },
    Drowned {
        inner: EntityTransformEventTransformReasonStruct<'mc>,
    },
    Sheared {
        inner: EntityTransformEventTransformReasonStruct<'mc>,
    },
    Lightning {
        inner: EntityTransformEventTransformReasonStruct<'mc>,
    },
    Split {
        inner: EntityTransformEventTransformReasonStruct<'mc>,
    },
    PiglinZombified {
        inner: EntityTransformEventTransformReasonStruct<'mc>,
    },
    Metamorphosis {
        inner: EntityTransformEventTransformReasonStruct<'mc>,
    },
    Unknown {
        inner: EntityTransformEventTransformReasonStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for EntityTransformEventTransformReason<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityTransformEventTransformReason::Cured { .. } => f.write_str("CURED"),
            EntityTransformEventTransformReason::Frozen { .. } => f.write_str("FROZEN"),
            EntityTransformEventTransformReason::Infection { .. } => f.write_str("INFECTION"),
            EntityTransformEventTransformReason::Drowned { .. } => f.write_str("DROWNED"),
            EntityTransformEventTransformReason::Sheared { .. } => f.write_str("SHEARED"),
            EntityTransformEventTransformReason::Lightning { .. } => f.write_str("LIGHTNING"),
            EntityTransformEventTransformReason::Split { .. } => f.write_str("SPLIT"),
            EntityTransformEventTransformReason::PiglinZombified { .. } => {
                f.write_str("PIGLIN_ZOMBIFIED")
            }
            EntityTransformEventTransformReason::Metamorphosis { .. } => {
                f.write_str("METAMORPHOSIS")
            }
            EntityTransformEventTransformReason::Unknown { .. } => f.write_str("UNKNOWN"),
        }
    }
}
impl<'mc> std::ops::Deref for EntityTransformEventTransformReason<'mc> {
    type Target = EntityTransformEventTransformReasonStruct<'mc>;
    fn deref(&self) -> &<EntityTransformEventTransformReason<'mc> as std::ops::Deref>::Target {
        match self {
            EntityTransformEventTransformReason::Cured { inner } => inner,
            EntityTransformEventTransformReason::Frozen { inner } => inner,
            EntityTransformEventTransformReason::Infection { inner } => inner,
            EntityTransformEventTransformReason::Drowned { inner } => inner,
            EntityTransformEventTransformReason::Sheared { inner } => inner,
            EntityTransformEventTransformReason::Lightning { inner } => inner,
            EntityTransformEventTransformReason::Split { inner } => inner,
            EntityTransformEventTransformReason::PiglinZombified { inner } => inner,
            EntityTransformEventTransformReason::Metamorphosis { inner } => inner,
            EntityTransformEventTransformReason::Unknown { inner } => inner,
        }
    }
}

impl<'mc> EntityTransformEventTransformReason<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<EntityTransformEventTransformReason<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/EntityTransformEvent/TransformReason");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/EntityTransformEvent/TransformReason;",
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = env.translate_error(res)?;
        let obj = res.l()?;
        let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = env.translate_error(variant)?;
        let variant_str = env
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        match variant_str.as_str() {
            "CURED" => Ok(EntityTransformEventTransformReason::Cured {
                inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
            }),
            "FROZEN" => Ok(EntityTransformEventTransformReason::Frozen {
                inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
            }),
            "INFECTION" => Ok(EntityTransformEventTransformReason::Infection {
                inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
            }),
            "DROWNED" => Ok(EntityTransformEventTransformReason::Drowned {
                inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
            }),
            "SHEARED" => Ok(EntityTransformEventTransformReason::Sheared {
                inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
            }),
            "LIGHTNING" => Ok(EntityTransformEventTransformReason::Lightning {
                inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
            }),
            "SPLIT" => Ok(EntityTransformEventTransformReason::Split {
                inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
            }),
            "PIGLIN_ZOMBIFIED" => Ok(EntityTransformEventTransformReason::PiglinZombified {
                inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
            }),
            "METAMORPHOSIS" => Ok(EntityTransformEventTransformReason::Metamorphosis {
                inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
            }),
            "UNKNOWN" => Ok(EntityTransformEventTransformReason::Unknown {
                inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct EntityTransformEventTransformReasonStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityTransformEventTransformReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Cured { inner } => inner.0.clone(),
            Self::Frozen { inner } => inner.0.clone(),
            Self::Infection { inner } => inner.0.clone(),
            Self::Drowned { inner } => inner.0.clone(),
            Self::Sheared { inner } => inner.0.clone(),
            Self::Lightning { inner } => inner.0.clone(),
            Self::Split { inner } => inner.0.clone(),
            Self::PiglinZombified { inner } => inner.0.clone(),
            Self::Metamorphosis { inner } => inner.0.clone(),
            Self::Unknown { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Cured { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Frozen { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Infection { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Drowned { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Sheared { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Lightning { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Split { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::PiglinZombified { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Metamorphosis { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Unknown { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityTransformEventTransformReason<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityTransformEventTransformReason from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityTransformEvent/TransformReason",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntityTransformEventTransformReason object, got {}",
                    name
                )
                .into())
        } else {
            let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
            let variant = env.translate_error(variant)?;
            let variant_str = env
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            match variant_str.as_str() {
                "CURED" => Ok(EntityTransformEventTransformReason::Cured {
                    inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
                }),
                "FROZEN" => Ok(EntityTransformEventTransformReason::Frozen {
                    inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
                }),
                "INFECTION" => Ok(EntityTransformEventTransformReason::Infection {
                    inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
                }),
                "DROWNED" => Ok(EntityTransformEventTransformReason::Drowned {
                    inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
                }),
                "SHEARED" => Ok(EntityTransformEventTransformReason::Sheared {
                    inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
                }),
                "LIGHTNING" => Ok(EntityTransformEventTransformReason::Lightning {
                    inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
                }),
                "SPLIT" => Ok(EntityTransformEventTransformReason::Split {
                    inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
                }),
                "PIGLIN_ZOMBIFIED" => Ok(EntityTransformEventTransformReason::PiglinZombified {
                    inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
                }),
                "METAMORPHOSIS" => Ok(EntityTransformEventTransformReason::Metamorphosis {
                    inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
                }),
                "UNKNOWN" => Ok(EntityTransformEventTransformReason::Unknown {
                    inner: EntityTransformEventTransformReasonStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for EntityTransformEventTransformReasonStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityTransformEventTransformReasonStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityTransformEventTransformReasonStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityTransformEvent/TransformReason",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntityTransformEventTransformReasonStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityTransformEventTransformReasonStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<
        crate::event::entity::EntityTransformEventTransformReason<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Lorg/bukkit/event/entity/EntityTransformEvent/TransformReason;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityTransformEvent/TransformReason");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::entity::EntityTransformEventTransformReason::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct EntityPortalEnterEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityPortalEnterEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityPortalEnterEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityPortalEnterEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityPortalEnterEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityPortalEnterEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityPortalEnterEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        entity: impl Into<crate::entity::Entity<'mc>>,
        location: impl Into<crate::Location<'mc>>,
    ) -> Result<crate::event::entity::EntityPortalEnterEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;Lorg/bukkit/Location;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(entity.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(location.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityPortalEnterEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityPortalEnterEvent::from_raw(&jni, res)
    }
    /// Gets the portal block the entity is touching
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLocation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityPortalEnterEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getLocation', 'getHandlers', 'getHandlerList'])
    /// Returns the Entity involved in this event
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityPortalEnterEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting EntityPortalEnterEvent into crate::event::entity::EntityEvent",
        )
    }
}
#[repr(C)]
pub struct VillagerAcquireTradeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for VillagerAcquireTradeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for VillagerAcquireTradeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate VillagerAcquireTradeEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/VillagerAcquireTradeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a VillagerAcquireTradeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> VillagerAcquireTradeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        what: impl Into<crate::entity::AbstractVillager<'mc>>,
        recipe: impl Into<crate::inventory::MerchantRecipe<'mc>>,
    ) -> Result<crate::event::entity::VillagerAcquireTradeEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from(
            "(Lorg/bukkit/entity/AbstractVillager;Lorg/bukkit/inventory/MerchantRecipe;)V",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(what.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(recipe.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/VillagerAcquireTradeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::VillagerAcquireTradeEvent::from_raw(&jni, res)
    }
    /// Get the recipe to be acquired.
    pub fn recipe(
        &self,
    ) -> Result<crate::inventory::MerchantRecipe<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/MerchantRecipe;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRecipe", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::MerchantRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Set the recipe to be acquired.
    pub fn set_recipe(
        &self,
        recipe: impl Into<crate::inventory::MerchantRecipe<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/MerchantRecipe;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(recipe.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRecipe",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn entity(
        &self,
    ) -> Result<crate::entity::AbstractVillager<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/AbstractVillager;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::AbstractVillager::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/VillagerAcquireTradeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getRecipe', 'setRecipe', 'isCancelled', 'setCancelled', 'getEntity', 'getHandlers', 'getHandlerList'])
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for VillagerAcquireTradeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting VillagerAcquireTradeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for VillagerAcquireTradeEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting VillagerAcquireTradeEvent into crate::event::entity::EntityEvent",
        )
    }
}
#[repr(C)]
pub struct EntityTeleportEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityTeleportEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityTeleportEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityTeleportEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityTeleportEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityTeleportEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityTeleportEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        what: impl Into<crate::entity::Entity<'mc>>,
        from: impl Into<crate::Location<'mc>>,
        to: impl Into<crate::Location<'mc>>,
    ) -> Result<crate::event::entity::EntityTeleportEvent<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/entity/Entity;Lorg/bukkit/Location;Lorg/bukkit/Location;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(what.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(from.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(to.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityTeleportEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityTeleportEvent::from_raw(&jni, res)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the location that this entity moved from
    pub fn from(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFrom", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the location that this entity moved from
    pub fn set_from(
        &self,
        from: impl Into<crate::Location<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Location;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(from.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFrom",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the location that this entity moved to
    pub fn to(&self) -> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTo", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }
    /// Sets the location that this entity moved to
    pub fn set_to(
        &self,
        to: impl Into<crate::Location<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Location;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(to.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTo",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityTeleportEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['isCancelled', 'setCancelled', 'getFrom', 'setFrom', 'getTo', 'setTo', 'getHandlers', 'getHandlerList'])
    /// Returns the Entity involved in this event
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityTeleportEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityTeleportEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityTeleportEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityTeleportEvent into crate::event::entity::EntityEvent")
    }
}
#[repr(C)]
pub struct TrialSpawnerSpawnEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for TrialSpawnerSpawnEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for TrialSpawnerSpawnEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate TrialSpawnerSpawnEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/TrialSpawnerSpawnEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TrialSpawnerSpawnEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> TrialSpawnerSpawnEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        spawnee: impl Into<crate::entity::Entity<'mc>>,
        spawner: impl Into<crate::block::TrialSpawner<'mc>>,
    ) -> Result<crate::event::entity::TrialSpawnerSpawnEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;Lorg/bukkit/block/TrialSpawner;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(spawnee.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(spawner.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/TrialSpawnerSpawnEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::TrialSpawnerSpawnEvent::from_raw(&jni, res)
    }

    pub fn trial_spawner(
        &self,
    ) -> Result<crate::block::TrialSpawner<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/TrialSpawner;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTrialSpawner", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::TrialSpawner::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: org.bukkit.event.entity.EntitySpawnEvent ( ['getTrialSpawner'])

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntitySpawnEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntitySpawnEvent = temp_clone.into();
        real.is_cancelled()
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntitySpawnEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntitySpawnEvent = temp_clone.into();
        real.set_cancelled(cancel)
    }
    /// Gets the location at which the entity is spawning.
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntitySpawnEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntitySpawnEvent = temp_clone.into();
        real.location()
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        crate::event::entity::EntitySpawnEvent::handler_list(jni)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntitySpawnEvent<'mc>> for TrialSpawnerSpawnEvent<'mc> {
    fn into(self) -> crate::event::entity::EntitySpawnEvent<'mc> {
        crate::event::entity::EntitySpawnEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting TrialSpawnerSpawnEvent into crate::event::entity::EntitySpawnEvent",
        )
    }
}
#[repr(C)]
pub struct ArrowBodyCountChangeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ArrowBodyCountChangeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ArrowBodyCountChangeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ArrowBodyCountChangeEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/ArrowBodyCountChangeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ArrowBodyCountChangeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ArrowBodyCountChangeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        entity: impl Into<crate::entity::LivingEntity<'mc>>,
        old_amount: i32,
        new_amount: i32,
        is_reset: bool,
    ) -> Result<crate::event::entity::ArrowBodyCountChangeEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/entity/LivingEntity;IIZ)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(entity.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(old_amount);
        let val_3 = jni::objects::JValueGen::Int(new_amount);
        let val_4 = jni::objects::JValueGen::Bool(is_reset.into());
        let cls = jni.find_class("org/bukkit/event/entity/ArrowBodyCountChangeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::ArrowBodyCountChangeEvent::from_raw(&jni, res)
    }
    /// Whether the event was called because the entity was reset.
    pub fn is_reset(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isReset", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the old amount of arrows in the entity's body.
    pub fn old_amount(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getOldAmount", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Get the new amount of arrows in the entity's body.
    pub fn new_amount(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getNewAmount", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the final amount of arrows in the entity's body.
    pub fn set_new_amount(&self, new_amount: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(new_amount);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setNewAmount",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn entity(&self) -> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/LivingEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::LivingEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/ArrowBodyCountChangeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['isReset', 'getOldAmount', 'getNewAmount', 'setNewAmount', 'getEntity', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for ArrowBodyCountChangeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ArrowBodyCountChangeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for ArrowBodyCountChangeEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting ArrowBodyCountChangeEvent into crate::event::entity::EntityEvent",
        )
    }
}
#[repr(C)]
pub struct EntityEnterLoveModeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityEnterLoveModeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityEnterLoveModeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityEnterLoveModeEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityEnterLoveModeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityEnterLoveModeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityEnterLoveModeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        animal_in_love: impl Into<crate::entity::Animals<'mc>>,
        human_entity: impl Into<crate::entity::HumanEntity<'mc>>,
        ticks_in_love: i32,
    ) -> Result<crate::event::entity::EntityEnterLoveModeEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/entity/Animals;Lorg/bukkit/entity/HumanEntity;I)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(animal_in_love.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(human_entity.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Int(ticks_in_love);
        let cls = jni.find_class("org/bukkit/event/entity/EntityEnterLoveModeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityEnterLoveModeEvent::from_raw(&jni, res)
    }
    /// Gets the animal that is entering love mode.
    pub fn entity(&self) -> Result<crate::entity::Animals<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Animals;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Animals::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the Human Entity that caused the animal to enter love mode.
    pub fn human_entity(
        &self,
    ) -> Result<Option<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/HumanEntity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHumanEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::HumanEntity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Gets the amount of ticks that the animal will fall in love for.
    pub fn ticks_in_love(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTicksInLove", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the amount of ticks that the animal will fall in love for.
    pub fn set_ticks_in_love(&self, ticks_in_love: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(ticks_in_love);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTicksInLove",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityEnterLoveModeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getEntity', 'getHumanEntity', 'getTicksInLove', 'setTicksInLove', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityEnterLoveModeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityEnterLoveModeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityEnterLoveModeEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting EntityEnterLoveModeEvent into crate::event::entity::EntityEvent",
        )
    }
}
#[repr(C)]
pub struct PiglinBarterEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PiglinBarterEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PiglinBarterEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PiglinBarterEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/PiglinBarterEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PiglinBarterEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PiglinBarterEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        what: impl Into<crate::entity::Piglin<'mc>>,
        input: impl Into<crate::inventory::ItemStack<'mc>>,
        outcome: Vec<jni::objects::JObject<'mc>>,
    ) -> Result<crate::event::entity::PiglinBarterEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/entity/Piglin;Lorg/bukkit/inventory/ItemStack;Ljava/util/List;)V",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(what.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(input.into().jni_object().clone())
        });
        let raw_val_3 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
        for v in outcome {
            let map_val_0 = jni::objects::JValueGen::Object(v);
            jni.call_method(
                &raw_val_3,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_3 = jni::objects::JValueGen::Object(raw_val_3);
        let cls = jni.find_class("org/bukkit/event/entity/PiglinBarterEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::PiglinBarterEvent::from_raw(&jni, res)
    }

    pub fn entity(&self) -> Result<crate::entity::Piglin<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Piglin;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Piglin::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the input of the barter.
    pub fn input(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getInput", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns a mutable list representing the outcome of the barter.
    pub fn outcome(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getOutcome", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::inventory::ItemStack::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/PiglinBarterEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getEntity', 'getInput', 'getOutcome', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PiglinBarterEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PiglinBarterEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for PiglinBarterEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PiglinBarterEvent into crate::event::entity::EntityEvent")
    }
}
#[repr(C)]
pub struct CreeperPowerEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CreeperPowerEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CreeperPowerEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate CreeperPowerEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/CreeperPowerEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CreeperPowerEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CreeperPowerEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        creeper: impl Into<crate::entity::Creeper<'mc>>,
        bolt: impl Into<crate::entity::LightningStrike<'mc>>,
        cause: std::option::Option<
            impl Into<crate::event::entity::CreeperPowerEventPowerCause<'mc>>,
        >,
    ) -> Result<crate::event::entity::CreeperPowerEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Creeper;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(creeper.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/entity/LightningStrike;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(bolt.into().jni_object().clone())
        });
        args.push(val_2);
        if let Some(a) = cause {
            sig += "Lorg/bukkit/event/entity/CreeperPowerEvent/PowerCause;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/CreeperPowerEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::CreeperPowerEvent::from_raw(&jni, res)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn entity(&self) -> Result<crate::entity::Creeper<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Creeper;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Creeper::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the lightning bolt which is striking the Creeper.
    pub fn lightning(
        &self,
    ) -> Result<Option<crate::entity::LightningStrike<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/LightningStrike;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLightning", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::LightningStrike::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Gets the cause of the creeper being (un)powered.
    pub fn cause(
        &self,
    ) -> Result<crate::event::entity::CreeperPowerEventPowerCause<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/event/entity/CreeperPowerEvent/PowerCause;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCause", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::entity::CreeperPowerEventPowerCause::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/CreeperPowerEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['isCancelled', 'setCancelled', 'getEntity', 'getLightning', 'getCause', 'getHandlers', 'getHandlerList'])
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for CreeperPowerEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CreeperPowerEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for CreeperPowerEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CreeperPowerEvent into crate::event::entity::EntityEvent")
    }
}
pub enum CreeperPowerEventPowerCause<'mc> {
    Lightning {
        inner: CreeperPowerEventPowerCauseStruct<'mc>,
    },
    SetOn {
        inner: CreeperPowerEventPowerCauseStruct<'mc>,
    },
    SetOff {
        inner: CreeperPowerEventPowerCauseStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for CreeperPowerEventPowerCause<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreeperPowerEventPowerCause::Lightning { .. } => f.write_str("LIGHTNING"),
            CreeperPowerEventPowerCause::SetOn { .. } => f.write_str("SET_ON"),
            CreeperPowerEventPowerCause::SetOff { .. } => f.write_str("SET_OFF"),
        }
    }
}
impl<'mc> std::ops::Deref for CreeperPowerEventPowerCause<'mc> {
    type Target = CreeperPowerEventPowerCauseStruct<'mc>;
    fn deref(&self) -> &<CreeperPowerEventPowerCause<'mc> as std::ops::Deref>::Target {
        match self {
            CreeperPowerEventPowerCause::Lightning { inner } => inner,
            CreeperPowerEventPowerCause::SetOn { inner } => inner,
            CreeperPowerEventPowerCause::SetOff { inner } => inner,
        }
    }
}

impl<'mc> CreeperPowerEventPowerCause<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<CreeperPowerEventPowerCause<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/CreeperPowerEvent/PowerCause");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/CreeperPowerEvent/PowerCause;",
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = env.translate_error(res)?;
        let obj = res.l()?;
        let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = env.translate_error(variant)?;
        let variant_str = env
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        match variant_str.as_str() {
            "LIGHTNING" => Ok(CreeperPowerEventPowerCause::Lightning {
                inner: CreeperPowerEventPowerCauseStruct::from_raw(env, obj)?,
            }),
            "SET_ON" => Ok(CreeperPowerEventPowerCause::SetOn {
                inner: CreeperPowerEventPowerCauseStruct::from_raw(env, obj)?,
            }),
            "SET_OFF" => Ok(CreeperPowerEventPowerCause::SetOff {
                inner: CreeperPowerEventPowerCauseStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct CreeperPowerEventPowerCauseStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CreeperPowerEventPowerCause<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Lightning { inner } => inner.0.clone(),
            Self::SetOn { inner } => inner.0.clone(),
            Self::SetOff { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Lightning { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SetOn { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SetOff { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CreeperPowerEventPowerCause<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate CreeperPowerEventPowerCause from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/CreeperPowerEvent/PowerCause")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CreeperPowerEventPowerCause object, got {}",
                name
            )
            .into())
        } else {
            let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
            let variant = env.translate_error(variant)?;
            let variant_str = env
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            match variant_str.as_str() {
                "LIGHTNING" => Ok(CreeperPowerEventPowerCause::Lightning {
                    inner: CreeperPowerEventPowerCauseStruct::from_raw(env, obj)?,
                }),
                "SET_ON" => Ok(CreeperPowerEventPowerCause::SetOn {
                    inner: CreeperPowerEventPowerCauseStruct::from_raw(env, obj)?,
                }),
                "SET_OFF" => Ok(CreeperPowerEventPowerCause::SetOff {
                    inner: CreeperPowerEventPowerCauseStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for CreeperPowerEventPowerCauseStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CreeperPowerEventPowerCauseStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate CreeperPowerEventPowerCauseStruct from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/CreeperPowerEvent/PowerCause")?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a CreeperPowerEventPowerCauseStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CreeperPowerEventPowerCauseStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::entity::CreeperPowerEventPowerCause<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/event/entity/CreeperPowerEvent/PowerCause;");
        let cls = jni.find_class("org/bukkit/event/entity/CreeperPowerEvent/PowerCause");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::entity::CreeperPowerEventPowerCause::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct EntityKnockbackEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityKnockbackEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityKnockbackEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityKnockbackEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityKnockbackEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityKnockbackEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityKnockbackEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        entity: impl Into<crate::entity::LivingEntity<'mc>>,
        cause: impl Into<crate::event::entity::EntityKnockbackEventKnockbackCause<'mc>>,
        force: f64,
        raw_knockback: impl Into<crate::util::Vector<'mc>>,
        knockback: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<crate::event::entity::EntityKnockbackEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/event/entity/EntityKnockbackEvent/KnockbackCause;DLorg/bukkit/util/Vector;Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(entity.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(cause.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Double(force);
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(raw_knockback.into().jni_object().clone())
        });
        let val_5 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(knockback.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityKnockbackEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
                jni::objects::JValueGen::from(val_5),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityKnockbackEvent::from_raw(&jni, res)
    }

    pub fn entity(&self) -> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/LivingEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::LivingEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the cause of the knockback.
    pub fn cause(
        &self,
    ) -> Result<
        crate::event::entity::EntityKnockbackEventKnockbackCause<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Lorg/bukkit/event/entity/EntityKnockbackEvent/KnockbackCause;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCause", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::entity::EntityKnockbackEventKnockbackCause::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )
    }
    /// Gets the raw force of the knockback.
    ///
    /// This value is based on factors such as the {@link Enchantment#KNOCKBACK}
    /// level of an attacker and the
    /// {@link Attribute#GENERIC_KNOCKBACK_RESISTANCE} of the entity.
    pub fn force(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getForce", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the raw knockback force that will be applied to the entity.
    ///
    /// This value is read-only, changes made to it <b>will not</b> have any
    /// effect on the final knockback received.
    pub fn knockback(&self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getKnockback", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the force that will be applied to the entity.
    ///
    /// In contrast to {@link EntityKnockbackEvent#getKnockback()} this value is
    /// affected by the entities current velocity and whether they are touching
    /// the ground.
    ///
    /// <b>Note:</b> this method returns a copy, changes must be applied with
    /// {@link #setFinalKnockback(Vector)}.
    pub fn final_knockback(&self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFinalKnockback",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the force that will be applied to the entity.
    pub fn set_final_knockback(
        &self,
        knockback: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(knockback.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFinalKnockback",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityKnockbackEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getEntity', 'getCause', 'getForce', 'getKnockback', 'getFinalKnockback', 'setFinalKnockback', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityKnockbackEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityKnockbackEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityKnockbackEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityKnockbackEvent into crate::event::entity::EntityEvent")
    }
}
pub enum EntityKnockbackEventKnockbackCause<'mc> {
    Damage {
        inner: EntityKnockbackEventKnockbackCauseStruct<'mc>,
    },
    EntityAttack {
        inner: EntityKnockbackEventKnockbackCauseStruct<'mc>,
    },
    Explosion {
        inner: EntityKnockbackEventKnockbackCauseStruct<'mc>,
    },
    ShieldBlock {
        inner: EntityKnockbackEventKnockbackCauseStruct<'mc>,
    },
    SweepAttack {
        inner: EntityKnockbackEventKnockbackCauseStruct<'mc>,
    },
    Unknown {
        inner: EntityKnockbackEventKnockbackCauseStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for EntityKnockbackEventKnockbackCause<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityKnockbackEventKnockbackCause::Damage { .. } => f.write_str("DAMAGE"),
            EntityKnockbackEventKnockbackCause::EntityAttack { .. } => f.write_str("ENTITY_ATTACK"),
            EntityKnockbackEventKnockbackCause::Explosion { .. } => f.write_str("EXPLOSION"),
            EntityKnockbackEventKnockbackCause::ShieldBlock { .. } => f.write_str("SHIELD_BLOCK"),
            EntityKnockbackEventKnockbackCause::SweepAttack { .. } => f.write_str("SWEEP_ATTACK"),
            EntityKnockbackEventKnockbackCause::Unknown { .. } => f.write_str("UNKNOWN"),
        }
    }
}
impl<'mc> std::ops::Deref for EntityKnockbackEventKnockbackCause<'mc> {
    type Target = EntityKnockbackEventKnockbackCauseStruct<'mc>;
    fn deref(&self) -> &<EntityKnockbackEventKnockbackCause<'mc> as std::ops::Deref>::Target {
        match self {
            EntityKnockbackEventKnockbackCause::Damage { inner } => inner,
            EntityKnockbackEventKnockbackCause::EntityAttack { inner } => inner,
            EntityKnockbackEventKnockbackCause::Explosion { inner } => inner,
            EntityKnockbackEventKnockbackCause::ShieldBlock { inner } => inner,
            EntityKnockbackEventKnockbackCause::SweepAttack { inner } => inner,
            EntityKnockbackEventKnockbackCause::Unknown { inner } => inner,
        }
    }
}

impl<'mc> EntityKnockbackEventKnockbackCause<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<EntityKnockbackEventKnockbackCause<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/EntityKnockbackEvent/KnockbackCause");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/EntityKnockbackEvent/KnockbackCause;",
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = env.translate_error(res)?;
        let obj = res.l()?;
        let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = env.translate_error(variant)?;
        let variant_str = env
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        match variant_str.as_str() {
            "DAMAGE" => Ok(EntityKnockbackEventKnockbackCause::Damage {
                inner: EntityKnockbackEventKnockbackCauseStruct::from_raw(env, obj)?,
            }),
            "ENTITY_ATTACK" => Ok(EntityKnockbackEventKnockbackCause::EntityAttack {
                inner: EntityKnockbackEventKnockbackCauseStruct::from_raw(env, obj)?,
            }),
            "EXPLOSION" => Ok(EntityKnockbackEventKnockbackCause::Explosion {
                inner: EntityKnockbackEventKnockbackCauseStruct::from_raw(env, obj)?,
            }),
            "SHIELD_BLOCK" => Ok(EntityKnockbackEventKnockbackCause::ShieldBlock {
                inner: EntityKnockbackEventKnockbackCauseStruct::from_raw(env, obj)?,
            }),
            "SWEEP_ATTACK" => Ok(EntityKnockbackEventKnockbackCause::SweepAttack {
                inner: EntityKnockbackEventKnockbackCauseStruct::from_raw(env, obj)?,
            }),
            "UNKNOWN" => Ok(EntityKnockbackEventKnockbackCause::Unknown {
                inner: EntityKnockbackEventKnockbackCauseStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct EntityKnockbackEventKnockbackCauseStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityKnockbackEventKnockbackCause<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Damage { inner } => inner.0.clone(),
            Self::EntityAttack { inner } => inner.0.clone(),
            Self::Explosion { inner } => inner.0.clone(),
            Self::ShieldBlock { inner } => inner.0.clone(),
            Self::SweepAttack { inner } => inner.0.clone(),
            Self::Unknown { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Damage { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::EntityAttack { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Explosion { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ShieldBlock { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SweepAttack { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Unknown { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityKnockbackEventKnockbackCause<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityKnockbackEventKnockbackCause from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityKnockbackEvent/KnockbackCause",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntityKnockbackEventKnockbackCause object, got {}",
                    name
                )
                .into())
        } else {
            let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
            let variant = env.translate_error(variant)?;
            let variant_str = env
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            match variant_str.as_str() {
                "DAMAGE" => Ok(EntityKnockbackEventKnockbackCause::Damage {
                    inner: EntityKnockbackEventKnockbackCauseStruct::from_raw(env, obj)?,
                }),
                "ENTITY_ATTACK" => Ok(EntityKnockbackEventKnockbackCause::EntityAttack {
                    inner: EntityKnockbackEventKnockbackCauseStruct::from_raw(env, obj)?,
                }),
                "EXPLOSION" => Ok(EntityKnockbackEventKnockbackCause::Explosion {
                    inner: EntityKnockbackEventKnockbackCauseStruct::from_raw(env, obj)?,
                }),
                "SHIELD_BLOCK" => Ok(EntityKnockbackEventKnockbackCause::ShieldBlock {
                    inner: EntityKnockbackEventKnockbackCauseStruct::from_raw(env, obj)?,
                }),
                "SWEEP_ATTACK" => Ok(EntityKnockbackEventKnockbackCause::SweepAttack {
                    inner: EntityKnockbackEventKnockbackCauseStruct::from_raw(env, obj)?,
                }),
                "UNKNOWN" => Ok(EntityKnockbackEventKnockbackCause::Unknown {
                    inner: EntityKnockbackEventKnockbackCauseStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for EntityKnockbackEventKnockbackCauseStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityKnockbackEventKnockbackCauseStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityKnockbackEventKnockbackCauseStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityKnockbackEvent/KnockbackCause",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntityKnockbackEventKnockbackCauseStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityKnockbackEventKnockbackCauseStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<
        crate::event::entity::EntityKnockbackEventKnockbackCause<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Lorg/bukkit/event/entity/EntityKnockbackEvent/KnockbackCause;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityKnockbackEvent/KnockbackCause");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::entity::EntityKnockbackEventKnockbackCause::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct EntityShootBowEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityShootBowEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityShootBowEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityShootBowEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityShootBowEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityShootBowEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityShootBowEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        shooter: impl Into<crate::entity::LivingEntity<'mc>>,
        bow: impl Into<crate::inventory::ItemStack<'mc>>,
        consumable: impl Into<crate::inventory::ItemStack<'mc>>,
        projectile: impl Into<crate::entity::Entity<'mc>>,
        hand: impl Into<crate::inventory::EquipmentSlot<'mc>>,
        force: f32,
        consume_item: bool,
    ) -> Result<crate::event::entity::EntityShootBowEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/entity/Entity;Lorg/bukkit/inventory/EquipmentSlot;FZ)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(shooter.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(bow.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(consumable.into().jni_object().clone())
        });
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(projectile.into().jni_object().clone())
        });
        let val_5 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(hand.into().jni_object().clone())
        });
        let val_6 = jni::objects::JValueGen::Float(force);
        let val_7 = jni::objects::JValueGen::Bool(consume_item.into());
        let cls = jni.find_class("org/bukkit/event/entity/EntityShootBowEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
                jni::objects::JValueGen::from(val_5),
                jni::objects::JValueGen::from(val_6),
                jni::objects::JValueGen::from(val_7),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityShootBowEvent::from_raw(&jni, res)
    }

    pub fn entity(&self) -> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/LivingEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::LivingEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the bow ItemStack used to fire the arrow.
    pub fn bow(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBow", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Get the ItemStack to be consumed in this event (if any).
    /// For instance, bows will consume an arrow ItemStack in a player's
    /// inventory.
    pub fn consumable(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getConsumable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Gets the projectile which will be launched by this event
    pub fn projectile(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getProjectile", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Replaces the projectile which will be launched
    pub fn set_projectile(
        &self,
        projectile: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(projectile.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setProjectile",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the hand from which the bow was shot.
    pub fn hand(&self) -> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/EquipmentSlot;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHand", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the force the arrow was launched with
    pub fn force(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getForce", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    #[deprecated]
    /// Set whether or not the consumable item should be consumed in this event. If set to false, it is recommended that a call to {@link Player#updateInventory()} is made as the client may disagree with the server's decision to not consume a consumable item.This value is ignored for entities where items are not required (skeletons, pillagers, etc.) or with crossbows (as no item is being consumed).
    pub fn set_consume_item(&self, consume_item: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(consume_item.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setConsumeItem",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get whether or not the consumable item should be consumed in this event.
    pub fn should_consume_item(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "shouldConsumeItem",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityShootBowEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getEntity', 'getBow', 'getConsumable', 'getProjectile', 'setProjectile', 'getHand', 'getForce', 'setConsumeItem', 'shouldConsumeItem', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityShootBowEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityShootBowEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityShootBowEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityShootBowEvent into crate::event::entity::EntityEvent")
    }
}
#[repr(C)]
pub struct EntityUnleashEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityUnleashEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityUnleashEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityUnleashEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityUnleashEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityUnleashEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityUnleashEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        entity: impl Into<crate::entity::Entity<'mc>>,
        reason: impl Into<crate::event::entity::EntityUnleashEventUnleashReason<'mc>>,
    ) -> Result<crate::event::entity::EntityUnleashEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;Lorg/bukkit/event/entity/EntityUnleashEvent/UnleashReason;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(entity.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(reason.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityUnleashEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityUnleashEvent::from_raw(&jni, res)
    }
    /// Returns the reason for the unleashing.
    pub fn reason(
        &self,
    ) -> Result<
        crate::event::entity::EntityUnleashEventUnleashReason<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Lorg/bukkit/event/entity/EntityUnleashEvent/UnleashReason;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getReason", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::entity::EntityUnleashEventUnleashReason::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityUnleashEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getReason', 'getHandlers', 'getHandlerList'])
    /// Returns the Entity involved in this event
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityUnleashEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityUnleashEvent into crate::event::entity::EntityEvent")
    }
}
pub enum EntityUnleashEventUnleashReason<'mc> {
    HolderGone {
        inner: EntityUnleashEventUnleashReasonStruct<'mc>,
    },
    PlayerUnleash {
        inner: EntityUnleashEventUnleashReasonStruct<'mc>,
    },
    Distance {
        inner: EntityUnleashEventUnleashReasonStruct<'mc>,
    },
    Unknown {
        inner: EntityUnleashEventUnleashReasonStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for EntityUnleashEventUnleashReason<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityUnleashEventUnleashReason::HolderGone { .. } => f.write_str("HOLDER_GONE"),
            EntityUnleashEventUnleashReason::PlayerUnleash { .. } => f.write_str("PLAYER_UNLEASH"),
            EntityUnleashEventUnleashReason::Distance { .. } => f.write_str("DISTANCE"),
            EntityUnleashEventUnleashReason::Unknown { .. } => f.write_str("UNKNOWN"),
        }
    }
}
impl<'mc> std::ops::Deref for EntityUnleashEventUnleashReason<'mc> {
    type Target = EntityUnleashEventUnleashReasonStruct<'mc>;
    fn deref(&self) -> &<EntityUnleashEventUnleashReason<'mc> as std::ops::Deref>::Target {
        match self {
            EntityUnleashEventUnleashReason::HolderGone { inner } => inner,
            EntityUnleashEventUnleashReason::PlayerUnleash { inner } => inner,
            EntityUnleashEventUnleashReason::Distance { inner } => inner,
            EntityUnleashEventUnleashReason::Unknown { inner } => inner,
        }
    }
}

impl<'mc> EntityUnleashEventUnleashReason<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<EntityUnleashEventUnleashReason<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/EntityUnleashEvent/UnleashReason");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/EntityUnleashEvent/UnleashReason;",
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = env.translate_error(res)?;
        let obj = res.l()?;
        let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = env.translate_error(variant)?;
        let variant_str = env
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        match variant_str.as_str() {
            "HOLDER_GONE" => Ok(EntityUnleashEventUnleashReason::HolderGone {
                inner: EntityUnleashEventUnleashReasonStruct::from_raw(env, obj)?,
            }),
            "PLAYER_UNLEASH" => Ok(EntityUnleashEventUnleashReason::PlayerUnleash {
                inner: EntityUnleashEventUnleashReasonStruct::from_raw(env, obj)?,
            }),
            "DISTANCE" => Ok(EntityUnleashEventUnleashReason::Distance {
                inner: EntityUnleashEventUnleashReasonStruct::from_raw(env, obj)?,
            }),
            "UNKNOWN" => Ok(EntityUnleashEventUnleashReason::Unknown {
                inner: EntityUnleashEventUnleashReasonStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct EntityUnleashEventUnleashReasonStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityUnleashEventUnleashReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::HolderGone { inner } => inner.0.clone(),
            Self::PlayerUnleash { inner } => inner.0.clone(),
            Self::Distance { inner } => inner.0.clone(),
            Self::Unknown { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::HolderGone { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::PlayerUnleash { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Distance { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Unknown { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityUnleashEventUnleashReason<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityUnleashEventUnleashReason from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityUnleashEvent/UnleashReason",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntityUnleashEventUnleashReason object, got {}",
                    name
                )
                .into())
        } else {
            let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
            let variant = env.translate_error(variant)?;
            let variant_str = env
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            match variant_str.as_str() {
                "HOLDER_GONE" => Ok(EntityUnleashEventUnleashReason::HolderGone {
                    inner: EntityUnleashEventUnleashReasonStruct::from_raw(env, obj)?,
                }),
                "PLAYER_UNLEASH" => Ok(EntityUnleashEventUnleashReason::PlayerUnleash {
                    inner: EntityUnleashEventUnleashReasonStruct::from_raw(env, obj)?,
                }),
                "DISTANCE" => Ok(EntityUnleashEventUnleashReason::Distance {
                    inner: EntityUnleashEventUnleashReasonStruct::from_raw(env, obj)?,
                }),
                "UNKNOWN" => Ok(EntityUnleashEventUnleashReason::Unknown {
                    inner: EntityUnleashEventUnleashReasonStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for EntityUnleashEventUnleashReasonStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityUnleashEventUnleashReasonStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityUnleashEventUnleashReasonStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityUnleashEvent/UnleashReason",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntityUnleashEventUnleashReasonStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityUnleashEventUnleashReasonStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<
        crate::event::entity::EntityUnleashEventUnleashReason<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Lorg/bukkit/event/entity/EntityUnleashEvent/UnleashReason;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityUnleashEvent/UnleashReason");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::entity::EntityUnleashEventUnleashReason::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct EntityPickupItemEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityPickupItemEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityPickupItemEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityPickupItemEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityPickupItemEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityPickupItemEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityPickupItemEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        entity: impl Into<crate::entity::LivingEntity<'mc>>,
        item: impl Into<crate::entity::Item<'mc>>,
        remaining: i32,
    ) -> Result<crate::event::entity::EntityPickupItemEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/entity/Item;I)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(entity.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Int(remaining);
        let cls = jni.find_class("org/bukkit/event/entity/EntityPickupItemEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityPickupItemEvent::from_raw(&jni, res)
    }

    pub fn entity(&self) -> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/LivingEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::LivingEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the Item picked up by the entity.
    pub fn item(&self) -> Result<crate::entity::Item<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Item;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getItem", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Item::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the amount remaining on the ground, if any
    pub fn remaining(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRemaining", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityPickupItemEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getEntity', 'getItem', 'getRemaining', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityPickupItemEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityPickupItemEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityPickupItemEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityPickupItemEvent into crate::event::entity::EntityEvent")
    }
}
#[repr(C)]
pub struct EntityCombustByBlockEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityCombustByBlockEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityCombustByBlockEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityCombustByBlockEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityCombustByBlockEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityCombustByBlockEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityCombustByBlockEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        combuster: impl Into<crate::block::Block<'mc>>,
        combustee: impl Into<crate::entity::Entity<'mc>>,
        duration: f32,
    ) -> Result<crate::event::entity::EntityCombustByBlockEvent<'mc>, Box<dyn std::error::Error>>
    {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/block/Block;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(combuster.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/entity/Entity;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(combustee.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "F";
        let val_3 = jni::objects::JValueGen::Float(duration);
        args.push(val_3);
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/EntityCombustByBlockEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityCombustByBlockEvent::from_raw(&jni, res)
    }
    /// The combuster can be lava or a block that is on fire.
    ///
    /// WARNING: block may be null.
    pub fn combuster(
        &self,
    ) -> Result<Option<crate::block::Block<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/Block;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCombuster", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::block::Block::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityCombustEvent ( ['getCombuster'])

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityCombustEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityCombustEvent = temp_clone.into();
        real.is_cancelled()
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityCombustEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityCombustEvent = temp_clone.into();
        real.set_cancelled(cancel)
    }

    pub fn duration(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityCombustEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityCombustEvent = temp_clone.into();
        real.duration()
    }
    /// The number of seconds the combustee should be alight for.This value will only ever increase the combustion time, not decrease existing combustion times.
    pub fn set_duration(&self, duration: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityCombustEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityCombustEvent = temp_clone.into();
        real.set_duration(duration)
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        crate::event::entity::EntityCombustEvent::handler_list(jni)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntityCombustEvent<'mc>> for EntityCombustByBlockEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityCombustEvent<'mc> {
        crate::event::entity::EntityCombustEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting EntityCombustByBlockEvent into crate::event::entity::EntityCombustEvent")
    }
}
#[repr(C)]
pub struct PigZombieAngerEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PigZombieAngerEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PigZombieAngerEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PigZombieAngerEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/PigZombieAngerEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PigZombieAngerEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PigZombieAngerEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        pig_zombie: impl Into<crate::entity::PigZombie<'mc>>,
        target: impl Into<crate::entity::Entity<'mc>>,
        new_anger: i32,
    ) -> Result<crate::event::entity::PigZombieAngerEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/PigZombie;Lorg/bukkit/entity/Entity;I)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(pig_zombie.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(target.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Int(new_anger);
        let cls = jni.find_class("org/bukkit/event/entity/PigZombieAngerEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::PigZombieAngerEvent::from_raw(&jni, res)
    }
    /// Gets the entity (if any) which triggered this anger update.
    pub fn target(&self) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTarget", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Entity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Gets the new anger resulting from this event.
    pub fn new_anger(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getNewAnger", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the new anger resulting from this event.
    pub fn set_new_anger(&self, new_anger: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(new_anger);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setNewAnger",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn entity(&self) -> Result<crate::entity::PigZombie<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/PigZombie;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::PigZombie::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/PigZombieAngerEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getTarget', 'getNewAnger', 'setNewAnger', 'getEntity', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PigZombieAngerEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PigZombieAngerEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for PigZombieAngerEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PigZombieAngerEvent into crate::event::entity::EntityEvent")
    }
}
#[repr(C)]
pub struct EntityTameEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityTameEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityTameEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityTameEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/EntityTameEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityTameEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityTameEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        entity: impl Into<crate::entity::LivingEntity<'mc>>,
        owner: impl Into<crate::entity::AnimalTamer<'mc>>,
    ) -> Result<crate::event::entity::EntityTameEvent<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/entity/AnimalTamer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(entity.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(owner.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityTameEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityTameEvent::from_raw(&jni, res)
    }

    pub fn entity(&self) -> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/LivingEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::LivingEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the owning AnimalTamer
    pub fn owner(&self) -> Result<crate::entity::AnimalTamer<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/AnimalTamer;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getOwner", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::AnimalTamer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityTameEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getEntity', 'isCancelled', 'setCancelled', 'getOwner', 'getHandlers', 'getHandlerList'])
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityTameEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityTameEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityTameEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityTameEvent into crate::event::entity::EntityEvent")
    }
}
#[repr(C)]
pub struct EntitySpawnEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntitySpawnEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntitySpawnEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntitySpawnEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/EntitySpawnEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntitySpawnEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntitySpawnEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        spawnee: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<crate::event::entity::EntitySpawnEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(spawnee.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntitySpawnEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntitySpawnEvent::from_raw(&jni, res)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the location at which the entity is spawning.
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLocation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntitySpawnEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['isCancelled', 'setCancelled', 'getLocation', 'getHandlers', 'getHandlerList'])
    /// Returns the Entity involved in this event
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntitySpawnEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntitySpawnEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntitySpawnEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntitySpawnEvent into crate::event::entity::EntityEvent")
    }
}
#[repr(C)]
pub struct EntityCombustEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityCombustEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityCombustEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityCombustEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityCombustEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityCombustEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityCombustEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        combustee: impl Into<crate::entity::Entity<'mc>>,
        duration: f32,
    ) -> Result<crate::event::entity::EntityCombustEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(combustee.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "F";
        let val_2 = jni::objects::JValueGen::Float(duration);
        args.push(val_2);
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/EntityCombustEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityCombustEvent::from_raw(&jni, res)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn duration(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDuration", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    #[deprecated]
    /// The number of seconds the combustee should be alight for.This value will only ever increase the combustion time, not decrease existing combustion times.
    pub fn set_duration(&self, duration: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(duration);
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setDuration", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityCombustEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['isCancelled', 'setCancelled', 'getDuration', 'setDuration', 'getHandlers', 'getHandlerList'])
    /// Returns the Entity involved in this event
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityCombustEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityCombustEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityCombustEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityCombustEvent into crate::event::entity::EntityEvent")
    }
}
#[repr(C)]
pub struct EntityPlaceEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityPlaceEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityPlaceEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityPlaceEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/EntityPlaceEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityPlaceEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityPlaceEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        entity: impl Into<crate::entity::Entity<'mc>>,
        player: impl Into<crate::entity::Player<'mc>>,
        block: impl Into<crate::block::Block<'mc>>,
        block_face: impl Into<crate::block::BlockFace<'mc>>,
        hand: std::option::Option<impl Into<crate::inventory::EquipmentSlot<'mc>>>,
    ) -> Result<crate::event::entity::EntityPlaceEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(entity.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/entity/Player;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(player.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "Lorg/bukkit/block/Block;";
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        args.push(val_3);
        sig += "Lorg/bukkit/block/BlockFace;";
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block_face.into().jni_object().clone())
        });
        args.push(val_4);
        if let Some(a) = hand {
            sig += "Lorg/bukkit/inventory/EquipmentSlot;";
            let val_5 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_5);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/EntityPlaceEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityPlaceEvent::from_raw(&jni, res)
    }
    /// Returns the player placing the entity
    pub fn player(&self) -> Result<Option<crate::entity::Player<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Player;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPlayer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Player::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Returns the block that the entity was placed on
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/Block;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBlock", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Block::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns the face of the block that the entity was placed on
    pub fn block_face(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBlockFace", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the hand used to place the entity.
    pub fn hand(&self) -> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/EquipmentSlot;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHand", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityPlaceEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getPlayer', 'getBlock', 'getBlockFace', 'getHand', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Returns the Entity involved in this event
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityPlaceEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityPlaceEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityPlaceEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityPlaceEvent into crate::event::entity::EntityEvent")
    }
}
#[repr(C)]
pub struct EntityCreatePortalEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityCreatePortalEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityCreatePortalEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityCreatePortalEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityCreatePortalEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityCreatePortalEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityCreatePortalEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        what: impl Into<crate::entity::LivingEntity<'mc>>,
        blocks: Vec<jni::objects::JObject<'mc>>,
        val_type: impl Into<crate::PortalType<'mc>>,
    ) -> Result<crate::event::entity::EntityCreatePortalEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from(
            "(Lorg/bukkit/entity/LivingEntity;Ljava/util/List;Lorg/bukkit/PortalType;)V",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(what.into().jni_object().clone())
        });
        let raw_val_2 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
        for v in blocks {
            let map_val_0 = jni::objects::JValueGen::Object(v);
            jni.call_method(
                &raw_val_2,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_type.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityCreatePortalEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityCreatePortalEvent::from_raw(&jni, res)
    }

    pub fn entity(&self) -> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/LivingEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::LivingEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a list of all blocks associated with the portal.
    pub fn blocks(&self) -> Result<Vec<crate::block::BlockState<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBlocks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::block::BlockState::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the type of portal that is trying to be created.
    pub fn portal_type(&self) -> Result<crate::PortalType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/PortalType;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPortalType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::PortalType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityCreatePortalEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getEntity', 'getBlocks', 'isCancelled', 'setCancelled', 'getPortalType', 'getHandlers', 'getHandlerList'])
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityCreatePortalEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityCreatePortalEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityCreatePortalEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting EntityCreatePortalEvent into crate::event::entity::EntityEvent",
        )
    }
}
#[repr(C)]
pub struct EntitySpellCastEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntitySpellCastEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntitySpellCastEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntitySpellCastEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntitySpellCastEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntitySpellCastEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntitySpellCastEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        what: impl Into<crate::entity::Spellcaster<'mc>>,
        spell: impl Into<crate::entity::SpellcasterSpell<'mc>>,
    ) -> Result<crate::event::entity::EntitySpellCastEvent<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/entity/Spellcaster;Lorg/bukkit/entity/Spellcaster/Spell;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(what.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(spell.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntitySpellCastEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntitySpellCastEvent::from_raw(&jni, res)
    }

    pub fn entity(&self) -> Result<crate::entity::Spellcaster<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Spellcaster;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Spellcaster::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the spell to be cast in this event.
    /// This is a convenience method equivalent to
    /// {@link Spellcaster#getSpell()}.
    pub fn spell(
        &self,
    ) -> Result<crate::entity::SpellcasterSpell<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Spellcaster/Spell;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSpell", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::SpellcasterSpell::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_cancelled(&self, cancelled: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancelled.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntitySpellCastEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getEntity', 'getSpell', 'setCancelled', 'isCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntitySpellCastEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntitySpellCastEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntitySpellCastEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntitySpellCastEvent into crate::event::entity::EntityEvent")
    }
}
#[repr(C)]
pub struct EntityPortalExitEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityPortalExitEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityPortalExitEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityPortalExitEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityPortalExitEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityPortalExitEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityPortalExitEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        entity: impl Into<crate::entity::Entity<'mc>>,
        from: impl Into<crate::Location<'mc>>,
        to: impl Into<crate::Location<'mc>>,
        before: impl Into<crate::util::Vector<'mc>>,
        after: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<crate::event::entity::EntityPortalExitEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;Lorg/bukkit/Location;Lorg/bukkit/Location;Lorg/bukkit/util/Vector;Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(entity.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(from.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(to.into().jni_object().clone())
        });
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(before.into().jni_object().clone())
        });
        let val_5 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(after.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityPortalExitEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
                jni::objects::JValueGen::from(val_5),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityPortalExitEvent::from_raw(&jni, res)
    }
    /// Gets a copy of the velocity that the entity has before entering the
    /// portal.
    pub fn before(&self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBefore", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a copy of the velocity that the entity will have after exiting the
    /// portal.
    pub fn after(&self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAfter", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the velocity that the entity will have after exiting the portal.
    pub fn set_after(
        &self,
        after: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(after.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAfter",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityPortalExitEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityTeleportEvent ( ['getBefore', 'getAfter', 'setAfter', 'getHandlers', 'getHandlerList'])

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityTeleportEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityTeleportEvent = temp_clone.into();
        real.is_cancelled()
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityTeleportEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityTeleportEvent = temp_clone.into();
        real.set_cancelled(cancel)
    }
    /// Gets the location that this entity moved from
    pub fn from(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityTeleportEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityTeleportEvent = temp_clone.into();
        real.from()
    }
    /// Sets the location that this entity moved from
    pub fn set_from(
        &self,
        from: impl Into<crate::Location<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityTeleportEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityTeleportEvent = temp_clone.into();
        real.set_from(from)
    }
    /// Gets the location that this entity moved to
    pub fn to(&self) -> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityTeleportEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityTeleportEvent = temp_clone.into();
        real.to()
    }
    /// Sets the location that this entity moved to
    pub fn set_to(
        &self,
        to: impl Into<crate::Location<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityTeleportEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityTeleportEvent = temp_clone.into();
        real.set_to(to)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntityTeleportEvent<'mc>> for EntityPortalExitEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityTeleportEvent<'mc> {
        crate::event::entity::EntityTeleportEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting EntityPortalExitEvent into crate::event::entity::EntityTeleportEvent",
        )
    }
}
#[repr(C)]
pub struct EntityDeathEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityDeathEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityDeathEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityDeathEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/EntityDeathEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityDeathEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityDeathEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        what: impl Into<crate::entity::LivingEntity<'mc>>,
        damage_source: impl Into<crate::damage::DamageSource<'mc>>,
        drops: Vec<jni::objects::JObject<'mc>>,
        dropped_exp: std::option::Option<i32>,
    ) -> Result<crate::event::entity::EntityDeathEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/LivingEntity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(what.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/damage/DamageSource;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(damage_source.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "Ljava/util/List;";
        let raw_val_3 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
        for v in drops {
            sig += "Ljava/lang/java/lang/Object;";
            let map_val_0 = jni::objects::JValueGen::Object(v);
            jni.call_method(
                &raw_val_3,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_3 = jni::objects::JValueGen::Object(raw_val_3);
        args.push(val_3);
        if let Some(a) = dropped_exp {
            sig += "I";
            let val_4 = jni::objects::JValueGen::Int(a);
            args.push(val_4);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/EntityDeathEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityDeathEvent::from_raw(&jni, res)
    }

    pub fn entity(&self) -> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/LivingEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::LivingEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the source of damage which caused the death.
    pub fn damage_source(
        &self,
    ) -> Result<crate::damage::DamageSource<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/damage/DamageSource;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDamageSource", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::damage::DamageSource::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets how much EXP should be dropped from this death.
    ///
    /// This does not indicate how much EXP should be taken from the entity in
    /// question, merely how much should be created after its death.
    pub fn dropped_exp(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDroppedExp", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets how much EXP should be dropped from this death.
    ///
    /// This does not indicate how much EXP should be taken from the entity in
    /// question, merely how much should be created after its death.
    pub fn set_dropped_exp(&self, exp: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(exp);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDroppedExp",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets all the items which will drop when the entity dies
    pub fn drops(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDrops", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::inventory::ItemStack::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityDeathEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getEntity', 'getDamageSource', 'getDroppedExp', 'setDroppedExp', 'getDrops', 'getHandlers', 'getHandlerList'])
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityDeathEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityDeathEvent into crate::event::entity::EntityEvent")
    }
}
#[repr(C)]
pub struct EntityToggleGlideEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityToggleGlideEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityToggleGlideEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityToggleGlideEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityToggleGlideEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityToggleGlideEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityToggleGlideEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        who: impl Into<crate::entity::LivingEntity<'mc>>,
        is_gliding: bool,
    ) -> Result<crate::event::entity::EntityToggleGlideEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/LivingEntity;Z)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(who.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Bool(is_gliding.into());
        let cls = jni.find_class("org/bukkit/event/entity/EntityToggleGlideEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityToggleGlideEvent::from_raw(&jni, res)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns true if the entity is now gliding or
    /// false if the entity stops gliding.
    pub fn is_gliding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isGliding", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityToggleGlideEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['isCancelled', 'setCancelled', 'isGliding', 'getHandlers', 'getHandlerList'])
    /// Returns the Entity involved in this event
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityToggleGlideEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityToggleGlideEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityToggleGlideEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting EntityToggleGlideEvent into crate::event::entity::EntityEvent",
        )
    }
}
#[repr(C)]
pub struct StriderTemperatureChangeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for StriderTemperatureChangeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for StriderTemperatureChangeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate StriderTemperatureChangeEvent from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/StriderTemperatureChangeEvent",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a StriderTemperatureChangeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> StriderTemperatureChangeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        what: impl Into<crate::entity::Strider<'mc>>,
        shivering: bool,
    ) -> Result<crate::event::entity::StriderTemperatureChangeEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/entity/Strider;Z)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(what.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Bool(shivering.into());
        let cls = jni.find_class("org/bukkit/event/entity/StriderTemperatureChangeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::StriderTemperatureChangeEvent::from_raw(&jni, res)
    }

    pub fn entity(&self) -> Result<crate::entity::Strider<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Strider;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Strider::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the Strider's new shivering state.
    pub fn is_shivering(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isShivering", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancelled: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancelled.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/StriderTemperatureChangeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getEntity', 'isShivering', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for StriderTemperatureChangeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting StriderTemperatureChangeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for StriderTemperatureChangeEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting StriderTemperatureChangeEvent into crate::event::entity::EntityEvent",
        )
    }
}
#[repr(C)]
pub struct EntityRemoveEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityRemoveEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityRemoveEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityRemoveEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/EntityRemoveEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityRemoveEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityRemoveEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        what: impl Into<crate::entity::Entity<'mc>>,
        cause: impl Into<crate::event::entity::EntityRemoveEventCause<'mc>>,
    ) -> Result<crate::event::entity::EntityRemoveEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/entity/Entity;Lorg/bukkit/event/entity/EntityRemoveEvent/Cause;)V",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(what.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(cause.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityRemoveEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityRemoveEvent::from_raw(&jni, res)
    }
    /// Gets the cause why the entity got removed.
    pub fn cause(
        &self,
    ) -> Result<crate::event::entity::EntityRemoveEventCause<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/entity/EntityRemoveEvent/Cause;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCause", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::entity::EntityRemoveEventCause::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityRemoveEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getCause', 'getHandlers', 'getHandlerList'])
    /// Returns the Entity involved in this event
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityRemoveEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityRemoveEvent into crate::event::entity::EntityEvent")
    }
}
pub enum EntityRemoveEventCause<'mc> {
    Death {
        inner: EntityRemoveEventCauseStruct<'mc>,
    },
    Despawn {
        inner: EntityRemoveEventCauseStruct<'mc>,
    },
    Drop {
        inner: EntityRemoveEventCauseStruct<'mc>,
    },
    EnterBlock {
        inner: EntityRemoveEventCauseStruct<'mc>,
    },
    Explode {
        inner: EntityRemoveEventCauseStruct<'mc>,
    },
    Hit {
        inner: EntityRemoveEventCauseStruct<'mc>,
    },
    Merge {
        inner: EntityRemoveEventCauseStruct<'mc>,
    },
    OutOfWorld {
        inner: EntityRemoveEventCauseStruct<'mc>,
    },
    Pickup {
        inner: EntityRemoveEventCauseStruct<'mc>,
    },
    PlayerQuit {
        inner: EntityRemoveEventCauseStruct<'mc>,
    },
    Plugin {
        inner: EntityRemoveEventCauseStruct<'mc>,
    },
    Transformation {
        inner: EntityRemoveEventCauseStruct<'mc>,
    },
    Unload {
        inner: EntityRemoveEventCauseStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for EntityRemoveEventCause<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityRemoveEventCause::Death { .. } => f.write_str("DEATH"),
            EntityRemoveEventCause::Despawn { .. } => f.write_str("DESPAWN"),
            EntityRemoveEventCause::Drop { .. } => f.write_str("DROP"),
            EntityRemoveEventCause::EnterBlock { .. } => f.write_str("ENTER_BLOCK"),
            EntityRemoveEventCause::Explode { .. } => f.write_str("EXPLODE"),
            EntityRemoveEventCause::Hit { .. } => f.write_str("HIT"),
            EntityRemoveEventCause::Merge { .. } => f.write_str("MERGE"),
            EntityRemoveEventCause::OutOfWorld { .. } => f.write_str("OUT_OF_WORLD"),
            EntityRemoveEventCause::Pickup { .. } => f.write_str("PICKUP"),
            EntityRemoveEventCause::PlayerQuit { .. } => f.write_str("PLAYER_QUIT"),
            EntityRemoveEventCause::Plugin { .. } => f.write_str("PLUGIN"),
            EntityRemoveEventCause::Transformation { .. } => f.write_str("TRANSFORMATION"),
            EntityRemoveEventCause::Unload { .. } => f.write_str("UNLOAD"),
        }
    }
}
impl<'mc> std::ops::Deref for EntityRemoveEventCause<'mc> {
    type Target = EntityRemoveEventCauseStruct<'mc>;
    fn deref(&self) -> &<EntityRemoveEventCause<'mc> as std::ops::Deref>::Target {
        match self {
            EntityRemoveEventCause::Death { inner } => inner,
            EntityRemoveEventCause::Despawn { inner } => inner,
            EntityRemoveEventCause::Drop { inner } => inner,
            EntityRemoveEventCause::EnterBlock { inner } => inner,
            EntityRemoveEventCause::Explode { inner } => inner,
            EntityRemoveEventCause::Hit { inner } => inner,
            EntityRemoveEventCause::Merge { inner } => inner,
            EntityRemoveEventCause::OutOfWorld { inner } => inner,
            EntityRemoveEventCause::Pickup { inner } => inner,
            EntityRemoveEventCause::PlayerQuit { inner } => inner,
            EntityRemoveEventCause::Plugin { inner } => inner,
            EntityRemoveEventCause::Transformation { inner } => inner,
            EntityRemoveEventCause::Unload { inner } => inner,
        }
    }
}

impl<'mc> EntityRemoveEventCause<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<EntityRemoveEventCause<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/EntityRemoveEvent/Cause");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/EntityRemoveEvent/Cause;",
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = env.translate_error(res)?;
        let obj = res.l()?;
        let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = env.translate_error(variant)?;
        let variant_str = env
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        match variant_str.as_str() {
            "DEATH" => Ok(EntityRemoveEventCause::Death {
                inner: EntityRemoveEventCauseStruct::from_raw(env, obj)?,
            }),
            "DESPAWN" => Ok(EntityRemoveEventCause::Despawn {
                inner: EntityRemoveEventCauseStruct::from_raw(env, obj)?,
            }),
            "DROP" => Ok(EntityRemoveEventCause::Drop {
                inner: EntityRemoveEventCauseStruct::from_raw(env, obj)?,
            }),
            "ENTER_BLOCK" => Ok(EntityRemoveEventCause::EnterBlock {
                inner: EntityRemoveEventCauseStruct::from_raw(env, obj)?,
            }),
            "EXPLODE" => Ok(EntityRemoveEventCause::Explode {
                inner: EntityRemoveEventCauseStruct::from_raw(env, obj)?,
            }),
            "HIT" => Ok(EntityRemoveEventCause::Hit {
                inner: EntityRemoveEventCauseStruct::from_raw(env, obj)?,
            }),
            "MERGE" => Ok(EntityRemoveEventCause::Merge {
                inner: EntityRemoveEventCauseStruct::from_raw(env, obj)?,
            }),
            "OUT_OF_WORLD" => Ok(EntityRemoveEventCause::OutOfWorld {
                inner: EntityRemoveEventCauseStruct::from_raw(env, obj)?,
            }),
            "PICKUP" => Ok(EntityRemoveEventCause::Pickup {
                inner: EntityRemoveEventCauseStruct::from_raw(env, obj)?,
            }),
            "PLAYER_QUIT" => Ok(EntityRemoveEventCause::PlayerQuit {
                inner: EntityRemoveEventCauseStruct::from_raw(env, obj)?,
            }),
            "PLUGIN" => Ok(EntityRemoveEventCause::Plugin {
                inner: EntityRemoveEventCauseStruct::from_raw(env, obj)?,
            }),
            "TRANSFORMATION" => Ok(EntityRemoveEventCause::Transformation {
                inner: EntityRemoveEventCauseStruct::from_raw(env, obj)?,
            }),
            "UNLOAD" => Ok(EntityRemoveEventCause::Unload {
                inner: EntityRemoveEventCauseStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct EntityRemoveEventCauseStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityRemoveEventCause<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Death { inner } => inner.0.clone(),
            Self::Despawn { inner } => inner.0.clone(),
            Self::Drop { inner } => inner.0.clone(),
            Self::EnterBlock { inner } => inner.0.clone(),
            Self::Explode { inner } => inner.0.clone(),
            Self::Hit { inner } => inner.0.clone(),
            Self::Merge { inner } => inner.0.clone(),
            Self::OutOfWorld { inner } => inner.0.clone(),
            Self::Pickup { inner } => inner.0.clone(),
            Self::PlayerQuit { inner } => inner.0.clone(),
            Self::Plugin { inner } => inner.0.clone(),
            Self::Transformation { inner } => inner.0.clone(),
            Self::Unload { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Death { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Despawn { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Drop { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::EnterBlock { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Explode { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Hit { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Merge { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::OutOfWorld { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Pickup { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::PlayerQuit { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Plugin { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Transformation { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Unload { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityRemoveEventCause<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityRemoveEventCause from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityRemoveEvent/Cause")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityRemoveEventCause object, got {}",
                name
            )
            .into())
        } else {
            let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
            let variant = env.translate_error(variant)?;
            let variant_str = env
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            match variant_str.as_str() {
                "DEATH" => Ok(EntityRemoveEventCause::Death {
                    inner: EntityRemoveEventCauseStruct::from_raw(env, obj)?,
                }),
                "DESPAWN" => Ok(EntityRemoveEventCause::Despawn {
                    inner: EntityRemoveEventCauseStruct::from_raw(env, obj)?,
                }),
                "DROP" => Ok(EntityRemoveEventCause::Drop {
                    inner: EntityRemoveEventCauseStruct::from_raw(env, obj)?,
                }),
                "ENTER_BLOCK" => Ok(EntityRemoveEventCause::EnterBlock {
                    inner: EntityRemoveEventCauseStruct::from_raw(env, obj)?,
                }),
                "EXPLODE" => Ok(EntityRemoveEventCause::Explode {
                    inner: EntityRemoveEventCauseStruct::from_raw(env, obj)?,
                }),
                "HIT" => Ok(EntityRemoveEventCause::Hit {
                    inner: EntityRemoveEventCauseStruct::from_raw(env, obj)?,
                }),
                "MERGE" => Ok(EntityRemoveEventCause::Merge {
                    inner: EntityRemoveEventCauseStruct::from_raw(env, obj)?,
                }),
                "OUT_OF_WORLD" => Ok(EntityRemoveEventCause::OutOfWorld {
                    inner: EntityRemoveEventCauseStruct::from_raw(env, obj)?,
                }),
                "PICKUP" => Ok(EntityRemoveEventCause::Pickup {
                    inner: EntityRemoveEventCauseStruct::from_raw(env, obj)?,
                }),
                "PLAYER_QUIT" => Ok(EntityRemoveEventCause::PlayerQuit {
                    inner: EntityRemoveEventCauseStruct::from_raw(env, obj)?,
                }),
                "PLUGIN" => Ok(EntityRemoveEventCause::Plugin {
                    inner: EntityRemoveEventCauseStruct::from_raw(env, obj)?,
                }),
                "TRANSFORMATION" => Ok(EntityRemoveEventCause::Transformation {
                    inner: EntityRemoveEventCauseStruct::from_raw(env, obj)?,
                }),
                "UNLOAD" => Ok(EntityRemoveEventCause::Unload {
                    inner: EntityRemoveEventCauseStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for EntityRemoveEventCauseStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityRemoveEventCauseStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityRemoveEventCauseStruct from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityRemoveEvent/Cause")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityRemoveEventCauseStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityRemoveEventCauseStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::entity::EntityRemoveEventCause<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/entity/EntityRemoveEvent/Cause;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityRemoveEvent/Cause");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::entity::EntityRemoveEventCause::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct EntityPoseChangeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityPoseChangeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityPoseChangeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityPoseChangeEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityPoseChangeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityPoseChangeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityPoseChangeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        who: impl Into<crate::entity::Entity<'mc>>,
        pose: impl Into<crate::entity::Pose<'mc>>,
    ) -> Result<crate::event::entity::EntityPoseChangeEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;Lorg/bukkit/entity/Pose;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(who.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(pose.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityPoseChangeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityPoseChangeEvent::from_raw(&jni, res)
    }
    /// Gets the entity's new pose.
    pub fn pose(&self) -> Result<crate::entity::Pose<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Pose;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPose", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Pose::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityPoseChangeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getPose', 'getHandlers', 'getHandlerList'])
    /// Returns the Entity involved in this event
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityPoseChangeEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityPoseChangeEvent into crate::event::entity::EntityEvent")
    }
}
#[repr(C)]
pub struct LingeringPotionSplashEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for LingeringPotionSplashEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for LingeringPotionSplashEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate LingeringPotionSplashEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/LingeringPotionSplashEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LingeringPotionSplashEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> LingeringPotionSplashEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        potion: impl Into<crate::entity::ThrownPotion<'mc>>,
        hit_entity: impl Into<crate::entity::Entity<'mc>>,
        hit_block: std::option::Option<impl Into<crate::block::Block<'mc>>>,
        hit_face: std::option::Option<impl Into<crate::block::BlockFace<'mc>>>,
        entity: std::option::Option<impl Into<crate::entity::AreaEffectCloud<'mc>>>,
    ) -> Result<crate::event::entity::LingeringPotionSplashEvent<'mc>, Box<dyn std::error::Error>>
    {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/ThrownPotion;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(potion.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/entity/Entity;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(hit_entity.into().jni_object().clone())
        });
        args.push(val_2);
        if let Some(a) = hit_block {
            sig += "Lorg/bukkit/block/Block;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        if let Some(a) = hit_face {
            sig += "Lorg/bukkit/block/BlockFace;";
            let val_4 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_4);
        }
        if let Some(a) = entity {
            sig += "Lorg/bukkit/entity/AreaEffectCloud;";
            let val_5 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_5);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/LingeringPotionSplashEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::LingeringPotionSplashEvent::from_raw(&jni, res)
    }

    pub fn entity(&self) -> Result<crate::entity::ThrownPotion<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/ThrownPotion;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::ThrownPotion::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the AreaEffectCloud spawned
    pub fn area_effect_cloud(
        &self,
    ) -> Result<crate::entity::AreaEffectCloud<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/AreaEffectCloud;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAreaEffectCloud",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::AreaEffectCloud::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/LingeringPotionSplashEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.ProjectileHitEvent ( ['getEntity', 'getAreaEffectCloud', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the block that was hit, if it was a block that was hit.
    pub fn hit_block(
        &self,
    ) -> Result<Option<crate::block::Block<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::ProjectileHitEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::ProjectileHitEvent = temp_clone.into();
        real.hit_block()
    }
    /// Gets the block face that was hit, if it was a block that was hit and the
    /// face was provided in the event.
    pub fn hit_block_face(
        &self,
    ) -> Result<Option<crate::block::BlockFace<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::ProjectileHitEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::ProjectileHitEvent = temp_clone.into();
        real.hit_block_face()
    }
    /// Gets the entity that was hit, if it was an entity that was hit.
    pub fn hit_entity(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::ProjectileHitEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::ProjectileHitEvent = temp_clone.into();
        real.hit_entity()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for LingeringPotionSplashEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting LingeringPotionSplashEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::ProjectileHitEvent<'mc>> for LingeringPotionSplashEvent<'mc> {
    fn into(self) -> crate::event::entity::ProjectileHitEvent<'mc> {
        crate::event::entity::ProjectileHitEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting LingeringPotionSplashEvent into crate::event::entity::ProjectileHitEvent")
    }
}
#[repr(C)]
pub struct EntityCombustByEntityEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityCombustByEntityEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityCombustByEntityEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityCombustByEntityEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityCombustByEntityEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityCombustByEntityEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityCombustByEntityEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        combuster: impl Into<crate::entity::Entity<'mc>>,
        combustee: impl Into<crate::entity::Entity<'mc>>,
        duration: f32,
    ) -> Result<crate::event::entity::EntityCombustByEntityEvent<'mc>, Box<dyn std::error::Error>>
    {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(combuster.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/entity/Entity;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(combustee.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "F";
        let val_3 = jni::objects::JValueGen::Float(duration);
        args.push(val_3);
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/EntityCombustByEntityEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityCombustByEntityEvent::from_raw(&jni, res)
    }
    /// Get the entity that caused the combustion event.
    pub fn combuster(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCombuster", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityCombustEvent ( ['getCombuster'])

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityCombustEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityCombustEvent = temp_clone.into();
        real.is_cancelled()
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityCombustEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityCombustEvent = temp_clone.into();
        real.set_cancelled(cancel)
    }

    pub fn duration(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityCombustEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityCombustEvent = temp_clone.into();
        real.duration()
    }
    /// The number of seconds the combustee should be alight for.This value will only ever increase the combustion time, not decrease existing combustion times.
    pub fn set_duration(&self, duration: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityCombustEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityCombustEvent = temp_clone.into();
        real.set_duration(duration)
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        crate::event::entity::EntityCombustEvent::handler_list(jni)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntityCombustEvent<'mc>> for EntityCombustByEntityEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityCombustEvent<'mc> {
        crate::event::entity::EntityCombustEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting EntityCombustByEntityEvent into crate::event::entity::EntityCombustEvent")
    }
}
#[repr(C)]
pub struct EntityToggleSwimEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityToggleSwimEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityToggleSwimEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityToggleSwimEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityToggleSwimEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityToggleSwimEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityToggleSwimEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        who: impl Into<crate::entity::LivingEntity<'mc>>,
        is_swimming: bool,
    ) -> Result<crate::event::entity::EntityToggleSwimEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/LivingEntity;Z)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(who.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Bool(is_swimming.into());
        let cls = jni.find_class("org/bukkit/event/entity/EntityToggleSwimEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityToggleSwimEvent::from_raw(&jni, res)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns true if the entity is now swims or
    /// false if the entity stops swimming.
    pub fn is_swimming(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isSwimming", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityToggleSwimEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['isCancelled', 'setCancelled', 'isSwimming', 'getHandlers', 'getHandlerList'])
    /// Returns the Entity involved in this event
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityToggleSwimEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityToggleSwimEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityToggleSwimEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityToggleSwimEvent into crate::event::entity::EntityEvent")
    }
}
#[repr(C)]
pub struct FireworkExplodeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for FireworkExplodeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for FireworkExplodeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate FireworkExplodeEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/FireworkExplodeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FireworkExplodeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> FireworkExplodeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        what: impl Into<crate::entity::Firework<'mc>>,
    ) -> Result<crate::event::entity::FireworkExplodeEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Firework;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(what.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/FireworkExplodeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::FireworkExplodeEvent::from_raw(&jni, res)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Set the cancelled state of this event. If the firework explosion is
    /// cancelled, the firework will still be removed, but no particles will be
    /// displayed.
    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn entity(&self) -> Result<crate::entity::Firework<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Firework;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Firework::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/FireworkExplodeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['isCancelled', 'setCancelled', 'getEntity', 'getHandlers', 'getHandlerList'])
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for FireworkExplodeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting FireworkExplodeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for FireworkExplodeEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting FireworkExplodeEvent into crate::event::entity::EntityEvent")
    }
}
#[repr(C)]
pub struct EntityChangeBlockEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityChangeBlockEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityChangeBlockEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityChangeBlockEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityChangeBlockEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityChangeBlockEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityChangeBlockEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        what: impl Into<crate::entity::Entity<'mc>>,
        block: impl Into<crate::block::Block<'mc>>,
        to: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::event::entity::EntityChangeBlockEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;Lorg/bukkit/block/Block;Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(what.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(to.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityChangeBlockEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityChangeBlockEvent::from_raw(&jni, res)
    }
    /// Gets the block the entity is changing
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/Block;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBlock", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Block::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the Material that the block is changing into
    pub fn to(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTo", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the data for the block that would be changed into
    pub fn block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBlockData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityChangeBlockEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getBlock', 'isCancelled', 'setCancelled', 'getTo', 'getBlockData', 'getHandlers', 'getHandlerList'])
    /// Returns the Entity involved in this event
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityChangeBlockEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityChangeBlockEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityChangeBlockEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting EntityChangeBlockEvent into crate::event::entity::EntityEvent",
        )
    }
}
#[repr(C)]
pub struct EntityDamageByEntityEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityDamageByEntityEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityDamageByEntityEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityDamageByEntityEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityDamageByEntityEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityDamageByEntityEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityDamageByEntityEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        damager: impl Into<crate::entity::Entity<'mc>>,
        damagee: impl Into<crate::entity::Entity<'mc>>,
        cause: impl Into<crate::event::entity::EntityDamageEventDamageCause<'mc>>,
        damage_source: impl Into<crate::damage::DamageSource<'mc>>,
        modifiers: std::option::Option<impl Into<blackboxmc_java::util::JavaMap<'mc>>>,
        modifier_functions: std::option::Option<impl Into<blackboxmc_java::util::JavaMap<'mc>>>,
    ) -> Result<crate::event::entity::EntityDamageByEntityEvent<'mc>, Box<dyn std::error::Error>>
    {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(damager.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/entity/Entity;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(damagee.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "Lorg/bukkit/event/entity/EntityDamageEvent/DamageCause;";
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(cause.into().jni_object().clone())
        });
        args.push(val_3);
        sig += "Lorg/bukkit/damage/DamageSource;";
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(damage_source.into().jni_object().clone())
        });
        args.push(val_4);
        if let Some(a) = modifiers {
            sig += "Ljava/util/Map;";
            let val_5 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_5);
        }
        if let Some(a) = modifier_functions {
            sig += "Ljava/util/Map;";
            let val_6 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_6);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/EntityDamageByEntityEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityDamageByEntityEvent::from_raw(&jni, res)
    }
    /// Returns the entity that damaged the defender.
    pub fn damager(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDamager", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityDamageEvent ( ['getDamager'])

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityDamageEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityDamageEvent = temp_clone.into();
        real.is_cancelled()
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityDamageEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityDamageEvent = temp_clone.into();
        real.set_cancelled(cancel)
    }
    /// Gets the original damage for the specified modifier, as defined at this
    /// event's construction.
    pub fn get_original_damage(
        &self,
        val_type: impl Into<crate::event::entity::EntityDamageEventDamageModifier<'mc>>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityDamageEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityDamageEvent = temp_clone.into();
        real.get_original_damage(val_type)
    }
    /// Sets the damage for the specified modifier.
    pub fn set_damage(
        &self,
        val_type: impl Into<crate::event::entity::EntityDamageEventDamageModifier<'mc>>,
        damage: std::option::Option<f64>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityDamageEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityDamageEvent = temp_clone.into();
        real.set_damage(val_type, damage)
    }
    /// Gets the raw amount of damage caused by the event
    pub fn damage(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityDamageEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityDamageEvent = temp_clone.into();
        real.damage()
    }
    /// This checks to see if a particular modifier is valid for this event's
    /// caller, such that, {@link #setDamage(DamageModifier, double)} will not
    /// throw an {@link UnsupportedOperationException}.
    ///
    /// {@link DamageModifier#BASE} is always applicable.
    pub fn is_applicable(
        &self,
        val_type: impl Into<crate::event::entity::EntityDamageEventDamageModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityDamageEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityDamageEvent = temp_clone.into();
        real.is_applicable(val_type)
    }
    /// Gets the amount of damage caused by the event after all damage
    /// reduction is applied.
    pub fn final_damage(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityDamageEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityDamageEvent = temp_clone.into();
        real.final_damage()
    }
    /// Gets the cause of the damage.
    ///
    /// While a DamageCause may indicate a specific Bukkit-assigned cause of damage,
    /// {@link #getDamageSource()} may expose additional types of damage such as custom
    /// damage types provided by data packs, as well as any direct or indirect entities,
    /// locations, or other contributing factors to the damage being inflicted. The
    /// alternative is generally preferred, but DamageCauses provided to this event
    /// should largely encompass most common use cases for developers if a simple cause
    /// is required.
    pub fn cause(
        &self,
    ) -> Result<crate::event::entity::EntityDamageEventDamageCause<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = crate::event::entity::EntityDamageEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityDamageEvent = temp_clone.into();
        real.cause()
    }
    /// Get the source of damage.
    pub fn damage_source(
        &self,
    ) -> Result<crate::damage::DamageSource<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityDamageEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityDamageEvent = temp_clone.into();
        real.damage_source()
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        crate::event::entity::EntityDamageEvent::handler_list(jni)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntityDamageEvent<'mc>> for EntityDamageByEntityEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityDamageEvent<'mc> {
        crate::event::entity::EntityDamageEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting EntityDamageByEntityEvent into crate::event::entity::EntityDamageEvent")
    }
}
#[repr(C)]
pub struct EntityBreedEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityBreedEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityBreedEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityBreedEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/EntityBreedEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityBreedEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityBreedEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        child: impl Into<crate::entity::LivingEntity<'mc>>,
        mother: impl Into<crate::entity::LivingEntity<'mc>>,
        father: impl Into<crate::entity::LivingEntity<'mc>>,
        breeder: impl Into<crate::entity::LivingEntity<'mc>>,
        bred_with: impl Into<crate::inventory::ItemStack<'mc>>,
        experience: i32,
    ) -> Result<crate::event::entity::EntityBreedEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/inventory/ItemStack;I)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(child.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(mother.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(father.into().jni_object().clone())
        });
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(breeder.into().jni_object().clone())
        });
        let val_5 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(bred_with.into().jni_object().clone())
        });
        let val_6 = jni::objects::JValueGen::Int(experience);
        let cls = jni.find_class("org/bukkit/event/entity/EntityBreedEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
                jni::objects::JValueGen::from(val_5),
                jni::objects::JValueGen::from(val_6),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityBreedEvent::from_raw(&jni, res)
    }

    pub fn entity(&self) -> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/LivingEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::LivingEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the parent creating this entity.
    pub fn mother(&self) -> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/LivingEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMother", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::LivingEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the other parent of the newly born entity.
    pub fn father(&self) -> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/LivingEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFather", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::LivingEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the Entity responsible for breeding. Breeder is null for spontaneous
    /// conception.
    pub fn breeder(
        &self,
    ) -> Result<Option<crate::entity::LivingEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/LivingEntity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBreeder", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::LivingEntity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// The ItemStack that was used to initiate breeding, if present.
    pub fn bred_with(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBredWith", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Get the amount of experience granted by breeding.
    pub fn experience(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getExperience", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set the amount of experience granted by breeding.
    pub fn set_experience(&self, experience: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(experience);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setExperience",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityBreedEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getEntity', 'getMother', 'getFather', 'getBreeder', 'getBredWith', 'getExperience', 'setExperience', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityBreedEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityBreedEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityBreedEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityBreedEvent into crate::event::entity::EntityEvent")
    }
}
#[repr(C)]
pub struct AreaEffectCloudApplyEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for AreaEffectCloudApplyEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for AreaEffectCloudApplyEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate AreaEffectCloudApplyEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/AreaEffectCloudApplyEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AreaEffectCloudApplyEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> AreaEffectCloudApplyEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        entity: impl Into<crate::entity::AreaEffectCloud<'mc>>,
        affected_entities: Vec<jni::objects::JObject<'mc>>,
    ) -> Result<crate::event::entity::AreaEffectCloudApplyEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/entity/AreaEffectCloud;Ljava/util/List;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(entity.into().jni_object().clone())
        });
        let raw_val_2 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
        for v in affected_entities {
            let map_val_0 = jni::objects::JValueGen::Object(v);
            jni.call_method(
                &raw_val_2,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        let cls = jni.find_class("org/bukkit/event/entity/AreaEffectCloudApplyEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::AreaEffectCloudApplyEvent::from_raw(&jni, res)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn entity(
        &self,
    ) -> Result<crate::entity::AreaEffectCloud<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/AreaEffectCloud;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::AreaEffectCloud::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Retrieves a mutable list of the effected entities
    ///
    /// It is important to note that not every entity in this list
    /// is guaranteed to be effected.The cloud may die during the
    /// application of its effects due to the depletion of {@link AreaEffectCloud#getDurationOnUse()}
    /// or {@link AreaEffectCloud#getRadiusOnUse()}
    pub fn affected_entities(
        &self,
    ) -> Result<Vec<crate::entity::LivingEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAffectedEntities",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::LivingEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/AreaEffectCloudApplyEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['isCancelled', 'setCancelled', 'getEntity', 'getAffectedEntities', 'getHandlers', 'getHandlerList'])
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for AreaEffectCloudApplyEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting AreaEffectCloudApplyEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for AreaEffectCloudApplyEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting AreaEffectCloudApplyEvent into crate::event::entity::EntityEvent",
        )
    }
}
#[repr(C)]
pub struct CreatureSpawnEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CreatureSpawnEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CreatureSpawnEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate CreatureSpawnEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/CreatureSpawnEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CreatureSpawnEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CreatureSpawnEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        spawnee: impl Into<crate::entity::LivingEntity<'mc>>,
        spawn_reason: impl Into<crate::event::entity::CreatureSpawnEventSpawnReason<'mc>>,
    ) -> Result<crate::event::entity::CreatureSpawnEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/event/entity/CreatureSpawnEvent/SpawnReason;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(spawnee.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(spawn_reason.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/CreatureSpawnEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::CreatureSpawnEvent::from_raw(&jni, res)
    }

    pub fn entity(&self) -> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/LivingEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::LivingEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the reason for why the creature is being spawned.
    pub fn spawn_reason(
        &self,
    ) -> Result<crate::event::entity::CreatureSpawnEventSpawnReason<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/event/entity/CreatureSpawnEvent/SpawnReason;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSpawnReason", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::entity::CreatureSpawnEventSpawnReason::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: org.bukkit.event.entity.EntitySpawnEvent ( ['getEntity', 'getSpawnReason'])

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntitySpawnEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntitySpawnEvent = temp_clone.into();
        real.is_cancelled()
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntitySpawnEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntitySpawnEvent = temp_clone.into();
        real.set_cancelled(cancel)
    }
    /// Gets the location at which the entity is spawning.
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntitySpawnEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntitySpawnEvent = temp_clone.into();
        real.location()
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        crate::event::entity::EntitySpawnEvent::handler_list(jni)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntitySpawnEvent<'mc>> for CreatureSpawnEvent<'mc> {
    fn into(self) -> crate::event::entity::EntitySpawnEvent<'mc> {
        crate::event::entity::EntitySpawnEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting CreatureSpawnEvent into crate::event::entity::EntitySpawnEvent",
        )
    }
}
pub enum CreatureSpawnEventSpawnReason<'mc> {
    Natural {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Jockey {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    ChunkGen {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Spawner {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    TrialSpawner {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Egg {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    SpawnerEgg {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Lightning {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    BuildSnowman {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    BuildIrongolem {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    BuildWither {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    VillageDefense {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    VillageInvasion {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Breeding {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    SlimeSplit {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Reinforcements {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    NetherPortal {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    DispenseEgg {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Infection {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Cured {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    OcelotBaby {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    SilverfishBlock {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Mount {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Trap {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    EnderPearl {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    ShoulderEntity {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Drowned {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Sheared {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Explosion {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Raid {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Patrol {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Beehive {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    PiglinZombified {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Spell {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Frozen {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Metamorphosis {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Duplication {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Command {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Enchantment {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    PotionEffect {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Custom {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
    Default {
        inner: CreatureSpawnEventSpawnReasonStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for CreatureSpawnEventSpawnReason<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreatureSpawnEventSpawnReason::Natural { .. } => f.write_str("NATURAL"),
            CreatureSpawnEventSpawnReason::Jockey { .. } => f.write_str("JOCKEY"),
            CreatureSpawnEventSpawnReason::ChunkGen { .. } => f.write_str("CHUNK_GEN"),
            CreatureSpawnEventSpawnReason::Spawner { .. } => f.write_str("SPAWNER"),
            CreatureSpawnEventSpawnReason::TrialSpawner { .. } => f.write_str("TRIAL_SPAWNER"),
            CreatureSpawnEventSpawnReason::Egg { .. } => f.write_str("EGG"),
            CreatureSpawnEventSpawnReason::SpawnerEgg { .. } => f.write_str("SPAWNER_EGG"),
            CreatureSpawnEventSpawnReason::Lightning { .. } => f.write_str("LIGHTNING"),
            CreatureSpawnEventSpawnReason::BuildSnowman { .. } => f.write_str("BUILD_SNOWMAN"),
            CreatureSpawnEventSpawnReason::BuildIrongolem { .. } => f.write_str("BUILD_IRONGOLEM"),
            CreatureSpawnEventSpawnReason::BuildWither { .. } => f.write_str("BUILD_WITHER"),
            CreatureSpawnEventSpawnReason::VillageDefense { .. } => f.write_str("VILLAGE_DEFENSE"),
            CreatureSpawnEventSpawnReason::VillageInvasion { .. } => {
                f.write_str("VILLAGE_INVASION")
            }
            CreatureSpawnEventSpawnReason::Breeding { .. } => f.write_str("BREEDING"),
            CreatureSpawnEventSpawnReason::SlimeSplit { .. } => f.write_str("SLIME_SPLIT"),
            CreatureSpawnEventSpawnReason::Reinforcements { .. } => f.write_str("REINFORCEMENTS"),
            CreatureSpawnEventSpawnReason::NetherPortal { .. } => f.write_str("NETHER_PORTAL"),
            CreatureSpawnEventSpawnReason::DispenseEgg { .. } => f.write_str("DISPENSE_EGG"),
            CreatureSpawnEventSpawnReason::Infection { .. } => f.write_str("INFECTION"),
            CreatureSpawnEventSpawnReason::Cured { .. } => f.write_str("CURED"),
            CreatureSpawnEventSpawnReason::OcelotBaby { .. } => f.write_str("OCELOT_BABY"),
            CreatureSpawnEventSpawnReason::SilverfishBlock { .. } => {
                f.write_str("SILVERFISH_BLOCK")
            }
            CreatureSpawnEventSpawnReason::Mount { .. } => f.write_str("MOUNT"),
            CreatureSpawnEventSpawnReason::Trap { .. } => f.write_str("TRAP"),
            CreatureSpawnEventSpawnReason::EnderPearl { .. } => f.write_str("ENDER_PEARL"),
            CreatureSpawnEventSpawnReason::ShoulderEntity { .. } => f.write_str("SHOULDER_ENTITY"),
            CreatureSpawnEventSpawnReason::Drowned { .. } => f.write_str("DROWNED"),
            CreatureSpawnEventSpawnReason::Sheared { .. } => f.write_str("SHEARED"),
            CreatureSpawnEventSpawnReason::Explosion { .. } => f.write_str("EXPLOSION"),
            CreatureSpawnEventSpawnReason::Raid { .. } => f.write_str("RAID"),
            CreatureSpawnEventSpawnReason::Patrol { .. } => f.write_str("PATROL"),
            CreatureSpawnEventSpawnReason::Beehive { .. } => f.write_str("BEEHIVE"),
            CreatureSpawnEventSpawnReason::PiglinZombified { .. } => {
                f.write_str("PIGLIN_ZOMBIFIED")
            }
            CreatureSpawnEventSpawnReason::Spell { .. } => f.write_str("SPELL"),
            CreatureSpawnEventSpawnReason::Frozen { .. } => f.write_str("FROZEN"),
            CreatureSpawnEventSpawnReason::Metamorphosis { .. } => f.write_str("METAMORPHOSIS"),
            CreatureSpawnEventSpawnReason::Duplication { .. } => f.write_str("DUPLICATION"),
            CreatureSpawnEventSpawnReason::Command { .. } => f.write_str("COMMAND"),
            CreatureSpawnEventSpawnReason::Enchantment { .. } => f.write_str("ENCHANTMENT"),
            CreatureSpawnEventSpawnReason::PotionEffect { .. } => f.write_str("POTION_EFFECT"),
            CreatureSpawnEventSpawnReason::Custom { .. } => f.write_str("CUSTOM"),
            CreatureSpawnEventSpawnReason::Default { .. } => f.write_str("DEFAULT"),
        }
    }
}
impl<'mc> std::ops::Deref for CreatureSpawnEventSpawnReason<'mc> {
    type Target = CreatureSpawnEventSpawnReasonStruct<'mc>;
    fn deref(&self) -> &<CreatureSpawnEventSpawnReason<'mc> as std::ops::Deref>::Target {
        match self {
            CreatureSpawnEventSpawnReason::Natural { inner } => inner,
            CreatureSpawnEventSpawnReason::Jockey { inner } => inner,
            CreatureSpawnEventSpawnReason::ChunkGen { inner } => inner,
            CreatureSpawnEventSpawnReason::Spawner { inner } => inner,
            CreatureSpawnEventSpawnReason::TrialSpawner { inner } => inner,
            CreatureSpawnEventSpawnReason::Egg { inner } => inner,
            CreatureSpawnEventSpawnReason::SpawnerEgg { inner } => inner,
            CreatureSpawnEventSpawnReason::Lightning { inner } => inner,
            CreatureSpawnEventSpawnReason::BuildSnowman { inner } => inner,
            CreatureSpawnEventSpawnReason::BuildIrongolem { inner } => inner,
            CreatureSpawnEventSpawnReason::BuildWither { inner } => inner,
            CreatureSpawnEventSpawnReason::VillageDefense { inner } => inner,
            CreatureSpawnEventSpawnReason::VillageInvasion { inner } => inner,
            CreatureSpawnEventSpawnReason::Breeding { inner } => inner,
            CreatureSpawnEventSpawnReason::SlimeSplit { inner } => inner,
            CreatureSpawnEventSpawnReason::Reinforcements { inner } => inner,
            CreatureSpawnEventSpawnReason::NetherPortal { inner } => inner,
            CreatureSpawnEventSpawnReason::DispenseEgg { inner } => inner,
            CreatureSpawnEventSpawnReason::Infection { inner } => inner,
            CreatureSpawnEventSpawnReason::Cured { inner } => inner,
            CreatureSpawnEventSpawnReason::OcelotBaby { inner } => inner,
            CreatureSpawnEventSpawnReason::SilverfishBlock { inner } => inner,
            CreatureSpawnEventSpawnReason::Mount { inner } => inner,
            CreatureSpawnEventSpawnReason::Trap { inner } => inner,
            CreatureSpawnEventSpawnReason::EnderPearl { inner } => inner,
            CreatureSpawnEventSpawnReason::ShoulderEntity { inner } => inner,
            CreatureSpawnEventSpawnReason::Drowned { inner } => inner,
            CreatureSpawnEventSpawnReason::Sheared { inner } => inner,
            CreatureSpawnEventSpawnReason::Explosion { inner } => inner,
            CreatureSpawnEventSpawnReason::Raid { inner } => inner,
            CreatureSpawnEventSpawnReason::Patrol { inner } => inner,
            CreatureSpawnEventSpawnReason::Beehive { inner } => inner,
            CreatureSpawnEventSpawnReason::PiglinZombified { inner } => inner,
            CreatureSpawnEventSpawnReason::Spell { inner } => inner,
            CreatureSpawnEventSpawnReason::Frozen { inner } => inner,
            CreatureSpawnEventSpawnReason::Metamorphosis { inner } => inner,
            CreatureSpawnEventSpawnReason::Duplication { inner } => inner,
            CreatureSpawnEventSpawnReason::Command { inner } => inner,
            CreatureSpawnEventSpawnReason::Enchantment { inner } => inner,
            CreatureSpawnEventSpawnReason::PotionEffect { inner } => inner,
            CreatureSpawnEventSpawnReason::Custom { inner } => inner,
            CreatureSpawnEventSpawnReason::Default { inner } => inner,
        }
    }
}

impl<'mc> CreatureSpawnEventSpawnReason<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<CreatureSpawnEventSpawnReason<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/CreatureSpawnEvent/SpawnReason");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/CreatureSpawnEvent/SpawnReason;",
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = env.translate_error(res)?;
        let obj = res.l()?;
        let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = env.translate_error(variant)?;
        let variant_str = env
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        match variant_str.as_str() {
            "NATURAL" => Ok(CreatureSpawnEventSpawnReason::Natural {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "JOCKEY" => Ok(CreatureSpawnEventSpawnReason::Jockey {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "CHUNK_GEN" => Ok(CreatureSpawnEventSpawnReason::ChunkGen {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "SPAWNER" => Ok(CreatureSpawnEventSpawnReason::Spawner {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "TRIAL_SPAWNER" => Ok(CreatureSpawnEventSpawnReason::TrialSpawner {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "EGG" => Ok(CreatureSpawnEventSpawnReason::Egg {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "SPAWNER_EGG" => Ok(CreatureSpawnEventSpawnReason::SpawnerEgg {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "LIGHTNING" => Ok(CreatureSpawnEventSpawnReason::Lightning {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "BUILD_SNOWMAN" => Ok(CreatureSpawnEventSpawnReason::BuildSnowman {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "BUILD_IRONGOLEM" => Ok(CreatureSpawnEventSpawnReason::BuildIrongolem {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "BUILD_WITHER" => Ok(CreatureSpawnEventSpawnReason::BuildWither {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_DEFENSE" => Ok(CreatureSpawnEventSpawnReason::VillageDefense {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_INVASION" => Ok(CreatureSpawnEventSpawnReason::VillageInvasion {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "BREEDING" => Ok(CreatureSpawnEventSpawnReason::Breeding {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "SLIME_SPLIT" => Ok(CreatureSpawnEventSpawnReason::SlimeSplit {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "REINFORCEMENTS" => Ok(CreatureSpawnEventSpawnReason::Reinforcements {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "NETHER_PORTAL" => Ok(CreatureSpawnEventSpawnReason::NetherPortal {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "DISPENSE_EGG" => Ok(CreatureSpawnEventSpawnReason::DispenseEgg {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "INFECTION" => Ok(CreatureSpawnEventSpawnReason::Infection {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "CURED" => Ok(CreatureSpawnEventSpawnReason::Cured {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "OCELOT_BABY" => Ok(CreatureSpawnEventSpawnReason::OcelotBaby {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "SILVERFISH_BLOCK" => Ok(CreatureSpawnEventSpawnReason::SilverfishBlock {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "MOUNT" => Ok(CreatureSpawnEventSpawnReason::Mount {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "TRAP" => Ok(CreatureSpawnEventSpawnReason::Trap {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "ENDER_PEARL" => Ok(CreatureSpawnEventSpawnReason::EnderPearl {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "SHOULDER_ENTITY" => Ok(CreatureSpawnEventSpawnReason::ShoulderEntity {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "DROWNED" => Ok(CreatureSpawnEventSpawnReason::Drowned {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "SHEARED" => Ok(CreatureSpawnEventSpawnReason::Sheared {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "EXPLOSION" => Ok(CreatureSpawnEventSpawnReason::Explosion {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "RAID" => Ok(CreatureSpawnEventSpawnReason::Raid {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "PATROL" => Ok(CreatureSpawnEventSpawnReason::Patrol {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "BEEHIVE" => Ok(CreatureSpawnEventSpawnReason::Beehive {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "PIGLIN_ZOMBIFIED" => Ok(CreatureSpawnEventSpawnReason::PiglinZombified {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "SPELL" => Ok(CreatureSpawnEventSpawnReason::Spell {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "FROZEN" => Ok(CreatureSpawnEventSpawnReason::Frozen {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "METAMORPHOSIS" => Ok(CreatureSpawnEventSpawnReason::Metamorphosis {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "DUPLICATION" => Ok(CreatureSpawnEventSpawnReason::Duplication {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "COMMAND" => Ok(CreatureSpawnEventSpawnReason::Command {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "ENCHANTMENT" => Ok(CreatureSpawnEventSpawnReason::Enchantment {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "POTION_EFFECT" => Ok(CreatureSpawnEventSpawnReason::PotionEffect {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "CUSTOM" => Ok(CreatureSpawnEventSpawnReason::Custom {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),
            "DEFAULT" => Ok(CreatureSpawnEventSpawnReason::Default {
                inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct CreatureSpawnEventSpawnReasonStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CreatureSpawnEventSpawnReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Natural { inner } => inner.0.clone(),
            Self::Jockey { inner } => inner.0.clone(),
            Self::ChunkGen { inner } => inner.0.clone(),
            Self::Spawner { inner } => inner.0.clone(),
            Self::TrialSpawner { inner } => inner.0.clone(),
            Self::Egg { inner } => inner.0.clone(),
            Self::SpawnerEgg { inner } => inner.0.clone(),
            Self::Lightning { inner } => inner.0.clone(),
            Self::BuildSnowman { inner } => inner.0.clone(),
            Self::BuildIrongolem { inner } => inner.0.clone(),
            Self::BuildWither { inner } => inner.0.clone(),
            Self::VillageDefense { inner } => inner.0.clone(),
            Self::VillageInvasion { inner } => inner.0.clone(),
            Self::Breeding { inner } => inner.0.clone(),
            Self::SlimeSplit { inner } => inner.0.clone(),
            Self::Reinforcements { inner } => inner.0.clone(),
            Self::NetherPortal { inner } => inner.0.clone(),
            Self::DispenseEgg { inner } => inner.0.clone(),
            Self::Infection { inner } => inner.0.clone(),
            Self::Cured { inner } => inner.0.clone(),
            Self::OcelotBaby { inner } => inner.0.clone(),
            Self::SilverfishBlock { inner } => inner.0.clone(),
            Self::Mount { inner } => inner.0.clone(),
            Self::Trap { inner } => inner.0.clone(),
            Self::EnderPearl { inner } => inner.0.clone(),
            Self::ShoulderEntity { inner } => inner.0.clone(),
            Self::Drowned { inner } => inner.0.clone(),
            Self::Sheared { inner } => inner.0.clone(),
            Self::Explosion { inner } => inner.0.clone(),
            Self::Raid { inner } => inner.0.clone(),
            Self::Patrol { inner } => inner.0.clone(),
            Self::Beehive { inner } => inner.0.clone(),
            Self::PiglinZombified { inner } => inner.0.clone(),
            Self::Spell { inner } => inner.0.clone(),
            Self::Frozen { inner } => inner.0.clone(),
            Self::Metamorphosis { inner } => inner.0.clone(),
            Self::Duplication { inner } => inner.0.clone(),
            Self::Command { inner } => inner.0.clone(),
            Self::Enchantment { inner } => inner.0.clone(),
            Self::PotionEffect { inner } => inner.0.clone(),
            Self::Custom { inner } => inner.0.clone(),
            Self::Default { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Natural { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Jockey { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::ChunkGen { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Spawner { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::TrialSpawner { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Egg { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SpawnerEgg { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Lightning { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BuildSnowman { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BuildIrongolem { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BuildWither { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageDefense { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageInvasion { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Breeding { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SlimeSplit { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Reinforcements { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::NetherPortal { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DispenseEgg { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Infection { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Cured { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::OcelotBaby { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SilverfishBlock { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Mount { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Trap { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::EnderPearl { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ShoulderEntity { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Drowned { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Sheared { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Explosion { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Raid { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Patrol { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Beehive { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::PiglinZombified { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Spell { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Frozen { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Metamorphosis { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Duplication { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Command { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Enchantment { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::PotionEffect { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Custom { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Default { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CreatureSpawnEventSpawnReason<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate CreatureSpawnEventSpawnReason from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/CreatureSpawnEvent/SpawnReason",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CreatureSpawnEventSpawnReason object, got {}",
                name
            )
            .into())
        } else {
            let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
            let variant = env.translate_error(variant)?;
            let variant_str = env
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            match variant_str.as_str() {
                "NATURAL" => Ok(CreatureSpawnEventSpawnReason::Natural {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "JOCKEY" => Ok(CreatureSpawnEventSpawnReason::Jockey {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "CHUNK_GEN" => Ok(CreatureSpawnEventSpawnReason::ChunkGen {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "SPAWNER" => Ok(CreatureSpawnEventSpawnReason::Spawner {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "TRIAL_SPAWNER" => Ok(CreatureSpawnEventSpawnReason::TrialSpawner {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "EGG" => Ok(CreatureSpawnEventSpawnReason::Egg {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "SPAWNER_EGG" => Ok(CreatureSpawnEventSpawnReason::SpawnerEgg {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "LIGHTNING" => Ok(CreatureSpawnEventSpawnReason::Lightning {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "BUILD_SNOWMAN" => Ok(CreatureSpawnEventSpawnReason::BuildSnowman {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "BUILD_IRONGOLEM" => Ok(CreatureSpawnEventSpawnReason::BuildIrongolem {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "BUILD_WITHER" => Ok(CreatureSpawnEventSpawnReason::BuildWither {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_DEFENSE" => Ok(CreatureSpawnEventSpawnReason::VillageDefense {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_INVASION" => Ok(CreatureSpawnEventSpawnReason::VillageInvasion {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "BREEDING" => Ok(CreatureSpawnEventSpawnReason::Breeding {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "SLIME_SPLIT" => Ok(CreatureSpawnEventSpawnReason::SlimeSplit {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "REINFORCEMENTS" => Ok(CreatureSpawnEventSpawnReason::Reinforcements {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "NETHER_PORTAL" => Ok(CreatureSpawnEventSpawnReason::NetherPortal {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "DISPENSE_EGG" => Ok(CreatureSpawnEventSpawnReason::DispenseEgg {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "INFECTION" => Ok(CreatureSpawnEventSpawnReason::Infection {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "CURED" => Ok(CreatureSpawnEventSpawnReason::Cured {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "OCELOT_BABY" => Ok(CreatureSpawnEventSpawnReason::OcelotBaby {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "SILVERFISH_BLOCK" => Ok(CreatureSpawnEventSpawnReason::SilverfishBlock {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "MOUNT" => Ok(CreatureSpawnEventSpawnReason::Mount {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "TRAP" => Ok(CreatureSpawnEventSpawnReason::Trap {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "ENDER_PEARL" => Ok(CreatureSpawnEventSpawnReason::EnderPearl {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "SHOULDER_ENTITY" => Ok(CreatureSpawnEventSpawnReason::ShoulderEntity {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "DROWNED" => Ok(CreatureSpawnEventSpawnReason::Drowned {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "SHEARED" => Ok(CreatureSpawnEventSpawnReason::Sheared {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "EXPLOSION" => Ok(CreatureSpawnEventSpawnReason::Explosion {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "RAID" => Ok(CreatureSpawnEventSpawnReason::Raid {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "PATROL" => Ok(CreatureSpawnEventSpawnReason::Patrol {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "BEEHIVE" => Ok(CreatureSpawnEventSpawnReason::Beehive {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "PIGLIN_ZOMBIFIED" => Ok(CreatureSpawnEventSpawnReason::PiglinZombified {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "SPELL" => Ok(CreatureSpawnEventSpawnReason::Spell {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "FROZEN" => Ok(CreatureSpawnEventSpawnReason::Frozen {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "METAMORPHOSIS" => Ok(CreatureSpawnEventSpawnReason::Metamorphosis {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "DUPLICATION" => Ok(CreatureSpawnEventSpawnReason::Duplication {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "COMMAND" => Ok(CreatureSpawnEventSpawnReason::Command {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "ENCHANTMENT" => Ok(CreatureSpawnEventSpawnReason::Enchantment {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "POTION_EFFECT" => Ok(CreatureSpawnEventSpawnReason::PotionEffect {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "CUSTOM" => Ok(CreatureSpawnEventSpawnReason::Custom {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                "DEFAULT" => Ok(CreatureSpawnEventSpawnReason::Default {
                    inner: CreatureSpawnEventSpawnReasonStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for CreatureSpawnEventSpawnReasonStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CreatureSpawnEventSpawnReasonStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate CreatureSpawnEventSpawnReasonStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/CreatureSpawnEvent/SpawnReason",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a CreatureSpawnEventSpawnReasonStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CreatureSpawnEventSpawnReasonStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::entity::CreatureSpawnEventSpawnReason<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/event/entity/CreatureSpawnEvent/SpawnReason;");
        let cls = jni.find_class("org/bukkit/event/entity/CreatureSpawnEvent/SpawnReason");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::entity::CreatureSpawnEventSpawnReason::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct HorseJumpEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for HorseJumpEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for HorseJumpEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate HorseJumpEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/HorseJumpEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a HorseJumpEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> HorseJumpEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        horse: impl Into<crate::entity::AbstractHorse<'mc>>,
        power: f32,
    ) -> Result<crate::event::entity::HorseJumpEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/AbstractHorse;F)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(horse.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Float(power);
        let cls = jni.find_class("org/bukkit/event/entity/HorseJumpEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::HorseJumpEvent::from_raw(&jni, res)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    #[deprecated]

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn entity(&self) -> Result<crate::entity::AbstractHorse<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/AbstractHorse;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::AbstractHorse::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the power of the jump.
    ///
    /// Power is a value that defines how much of the horse's jump strength
    /// should be used for the jump. Power is effectively multiplied times
    /// the horse's jump strength to determine how high the jump is; 0
    /// represents no jump strength while 1 represents full jump strength.
    /// Setting power to a value above 1 will use additional jump strength
    /// that the horse does not usually have.
    ///
    /// Power does not affect how high the horse is capable of jumping, only
    /// how much of its jumping capability will be used in this jump. To set
    /// the horse's overall jump strength, see {@link
    /// AbstractHorse#setJumpStrength(double)}.
    pub fn power(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPower", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    #[deprecated]
    /// Sets the power of the jump.Jump power can be set to a value above 1.0 which will increase the strength of this jump above the horse's actual jump strength.Setting the jump power to 0 will result in the jump animation still playing, but the horse not leaving the ground. Only canceling this event will result in no jump animation at all.
    pub fn set_power(&self, power: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(power);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPower",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/HorseJumpEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['isCancelled', 'setCancelled', 'getEntity', 'getPower', 'setPower', 'getHandlers', 'getHandlerList'])
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for HorseJumpEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting HorseJumpEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for HorseJumpEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting HorseJumpEvent into crate::event::entity::EntityEvent")
    }
}
#[repr(C)]
pub struct EntityDismountEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityDismountEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityDismountEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityDismountEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityDismountEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityDismountEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityDismountEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        what: impl Into<crate::entity::Entity<'mc>>,
        dismounted: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<crate::event::entity::EntityDismountEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;Lorg/bukkit/entity/Entity;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(what.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(dismounted.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityDismountEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityDismountEvent::from_raw(&jni, res)
    }
    /// Gets the entity which will no longer be ridden.
    pub fn dismounted(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDismounted", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityDismountEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getDismounted', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Returns the Entity involved in this event
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityDismountEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityDismountEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityDismountEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityDismountEvent into crate::event::entity::EntityEvent")
    }
}
#[repr(C)]
pub struct EntityResurrectEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityResurrectEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityResurrectEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityResurrectEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityResurrectEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityResurrectEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityResurrectEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        what: impl Into<crate::entity::LivingEntity<'mc>>,
        hand: std::option::Option<impl Into<crate::inventory::EquipmentSlot<'mc>>>,
    ) -> Result<crate::event::entity::EntityResurrectEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/LivingEntity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(what.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = hand {
            sig += "Lorg/bukkit/inventory/EquipmentSlot;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/EntityResurrectEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityResurrectEvent::from_raw(&jni, res)
    }

    pub fn entity(&self) -> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/LivingEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::LivingEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the hand in which the totem of undying was found, or null if the
    /// entity did not have a totem of undying.
    pub fn hand(
        &self,
    ) -> Result<Option<crate::inventory::EquipmentSlot<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/EquipmentSlot;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHand", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::EquipmentSlot::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancelled: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancelled.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityResurrectEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getEntity', 'getHand', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityResurrectEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityResurrectEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityResurrectEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityResurrectEvent into crate::event::entity::EntityEvent")
    }
}
#[repr(C)]
pub struct ProjectileLaunchEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ProjectileLaunchEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ProjectileLaunchEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ProjectileLaunchEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/ProjectileLaunchEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ProjectileLaunchEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ProjectileLaunchEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        what: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<crate::event::entity::ProjectileLaunchEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(what.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/ProjectileLaunchEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::ProjectileLaunchEvent::from_raw(&jni, res)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn entity(&self) -> Result<crate::entity::Projectile<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Projectile;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Projectile::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: org.bukkit.event.entity.EntitySpawnEvent ( ['isCancelled', 'setCancelled', 'getEntity'])
    /// Gets the location at which the entity is spawning.
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntitySpawnEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntitySpawnEvent = temp_clone.into();
        real.location()
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        crate::event::entity::EntitySpawnEvent::handler_list(jni)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for ProjectileLaunchEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ProjectileLaunchEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntitySpawnEvent<'mc>> for ProjectileLaunchEvent<'mc> {
    fn into(self) -> crate::event::entity::EntitySpawnEvent<'mc> {
        crate::event::entity::EntitySpawnEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting ProjectileLaunchEvent into crate::event::entity::EntitySpawnEvent",
        )
    }
}
#[repr(C)]
pub struct EntityRegainHealthEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityRegainHealthEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityRegainHealthEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityRegainHealthEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityRegainHealthEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityRegainHealthEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityRegainHealthEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        entity: impl Into<crate::entity::Entity<'mc>>,
        amount: f64,
        regain_reason: impl Into<crate::event::entity::EntityRegainHealthEventRegainReason<'mc>>,
    ) -> Result<crate::event::entity::EntityRegainHealthEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/entity/Entity;DLorg/bukkit/event/entity/EntityRegainHealthEvent/RegainReason;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(entity.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Double(amount);
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(regain_reason.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityRegainHealthEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityRegainHealthEvent::from_raw(&jni, res)
    }
    /// Gets the amount of regained health
    pub fn amount(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAmount", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Sets the amount of regained health
    pub fn set_amount(&self, amount: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(amount);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAmount",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the reason for why the entity is regaining health
    pub fn regain_reason(
        &self,
    ) -> Result<
        crate::event::entity::EntityRegainHealthEventRegainReason<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Lorg/bukkit/event/entity/EntityRegainHealthEvent/RegainReason;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRegainReason", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::entity::EntityRegainHealthEventRegainReason::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityRegainHealthEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getAmount', 'setAmount', 'isCancelled', 'setCancelled', 'getRegainReason', 'getHandlers', 'getHandlerList'])
    /// Returns the Entity involved in this event
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityRegainHealthEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityRegainHealthEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityRegainHealthEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting EntityRegainHealthEvent into crate::event::entity::EntityEvent",
        )
    }
}
pub enum EntityRegainHealthEventRegainReason<'mc> {
    Regen {
        inner: EntityRegainHealthEventRegainReasonStruct<'mc>,
    },
    Satiated {
        inner: EntityRegainHealthEventRegainReasonStruct<'mc>,
    },
    Eating {
        inner: EntityRegainHealthEventRegainReasonStruct<'mc>,
    },
    EnderCrystal {
        inner: EntityRegainHealthEventRegainReasonStruct<'mc>,
    },
    Magic {
        inner: EntityRegainHealthEventRegainReasonStruct<'mc>,
    },
    MagicRegen {
        inner: EntityRegainHealthEventRegainReasonStruct<'mc>,
    },
    WitherSpawn {
        inner: EntityRegainHealthEventRegainReasonStruct<'mc>,
    },
    Wither {
        inner: EntityRegainHealthEventRegainReasonStruct<'mc>,
    },
    Custom {
        inner: EntityRegainHealthEventRegainReasonStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for EntityRegainHealthEventRegainReason<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityRegainHealthEventRegainReason::Regen { .. } => f.write_str("REGEN"),
            EntityRegainHealthEventRegainReason::Satiated { .. } => f.write_str("SATIATED"),
            EntityRegainHealthEventRegainReason::Eating { .. } => f.write_str("EATING"),
            EntityRegainHealthEventRegainReason::EnderCrystal { .. } => {
                f.write_str("ENDER_CRYSTAL")
            }
            EntityRegainHealthEventRegainReason::Magic { .. } => f.write_str("MAGIC"),
            EntityRegainHealthEventRegainReason::MagicRegen { .. } => f.write_str("MAGIC_REGEN"),
            EntityRegainHealthEventRegainReason::WitherSpawn { .. } => f.write_str("WITHER_SPAWN"),
            EntityRegainHealthEventRegainReason::Wither { .. } => f.write_str("WITHER"),
            EntityRegainHealthEventRegainReason::Custom { .. } => f.write_str("CUSTOM"),
        }
    }
}
impl<'mc> std::ops::Deref for EntityRegainHealthEventRegainReason<'mc> {
    type Target = EntityRegainHealthEventRegainReasonStruct<'mc>;
    fn deref(&self) -> &<EntityRegainHealthEventRegainReason<'mc> as std::ops::Deref>::Target {
        match self {
            EntityRegainHealthEventRegainReason::Regen { inner } => inner,
            EntityRegainHealthEventRegainReason::Satiated { inner } => inner,
            EntityRegainHealthEventRegainReason::Eating { inner } => inner,
            EntityRegainHealthEventRegainReason::EnderCrystal { inner } => inner,
            EntityRegainHealthEventRegainReason::Magic { inner } => inner,
            EntityRegainHealthEventRegainReason::MagicRegen { inner } => inner,
            EntityRegainHealthEventRegainReason::WitherSpawn { inner } => inner,
            EntityRegainHealthEventRegainReason::Wither { inner } => inner,
            EntityRegainHealthEventRegainReason::Custom { inner } => inner,
        }
    }
}

impl<'mc> EntityRegainHealthEventRegainReason<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<EntityRegainHealthEventRegainReason<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/EntityRegainHealthEvent/RegainReason");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/EntityRegainHealthEvent/RegainReason;",
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = env.translate_error(res)?;
        let obj = res.l()?;
        let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = env.translate_error(variant)?;
        let variant_str = env
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        match variant_str.as_str() {
            "REGEN" => Ok(EntityRegainHealthEventRegainReason::Regen {
                inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
            }),
            "SATIATED" => Ok(EntityRegainHealthEventRegainReason::Satiated {
                inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
            }),
            "EATING" => Ok(EntityRegainHealthEventRegainReason::Eating {
                inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
            }),
            "ENDER_CRYSTAL" => Ok(EntityRegainHealthEventRegainReason::EnderCrystal {
                inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
            }),
            "MAGIC" => Ok(EntityRegainHealthEventRegainReason::Magic {
                inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
            }),
            "MAGIC_REGEN" => Ok(EntityRegainHealthEventRegainReason::MagicRegen {
                inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
            }),
            "WITHER_SPAWN" => Ok(EntityRegainHealthEventRegainReason::WitherSpawn {
                inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
            }),
            "WITHER" => Ok(EntityRegainHealthEventRegainReason::Wither {
                inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
            }),
            "CUSTOM" => Ok(EntityRegainHealthEventRegainReason::Custom {
                inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct EntityRegainHealthEventRegainReasonStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityRegainHealthEventRegainReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Regen { inner } => inner.0.clone(),
            Self::Satiated { inner } => inner.0.clone(),
            Self::Eating { inner } => inner.0.clone(),
            Self::EnderCrystal { inner } => inner.0.clone(),
            Self::Magic { inner } => inner.0.clone(),
            Self::MagicRegen { inner } => inner.0.clone(),
            Self::WitherSpawn { inner } => inner.0.clone(),
            Self::Wither { inner } => inner.0.clone(),
            Self::Custom { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Regen { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Satiated { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Eating { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::EnderCrystal { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Magic { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::MagicRegen { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WitherSpawn { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Wither { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Custom { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityRegainHealthEventRegainReason<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityRegainHealthEventRegainReason from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityRegainHealthEvent/RegainReason",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntityRegainHealthEventRegainReason object, got {}",
                    name
                )
                .into())
        } else {
            let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
            let variant = env.translate_error(variant)?;
            let variant_str = env
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            match variant_str.as_str() {
                "REGEN" => Ok(EntityRegainHealthEventRegainReason::Regen {
                    inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
                }),
                "SATIATED" => Ok(EntityRegainHealthEventRegainReason::Satiated {
                    inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
                }),
                "EATING" => Ok(EntityRegainHealthEventRegainReason::Eating {
                    inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
                }),
                "ENDER_CRYSTAL" => Ok(EntityRegainHealthEventRegainReason::EnderCrystal {
                    inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
                }),
                "MAGIC" => Ok(EntityRegainHealthEventRegainReason::Magic {
                    inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
                }),
                "MAGIC_REGEN" => Ok(EntityRegainHealthEventRegainReason::MagicRegen {
                    inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
                }),
                "WITHER_SPAWN" => Ok(EntityRegainHealthEventRegainReason::WitherSpawn {
                    inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
                }),
                "WITHER" => Ok(EntityRegainHealthEventRegainReason::Wither {
                    inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
                }),
                "CUSTOM" => Ok(EntityRegainHealthEventRegainReason::Custom {
                    inner: EntityRegainHealthEventRegainReasonStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for EntityRegainHealthEventRegainReasonStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityRegainHealthEventRegainReasonStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityRegainHealthEventRegainReasonStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityRegainHealthEvent/RegainReason",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntityRegainHealthEventRegainReasonStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityRegainHealthEventRegainReasonStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<
        crate::event::entity::EntityRegainHealthEventRegainReason<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Lorg/bukkit/event/entity/EntityRegainHealthEvent/RegainReason;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityRegainHealthEvent/RegainReason");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::entity::EntityRegainHealthEventRegainReason::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct SlimeSplitEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SlimeSplitEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SlimeSplitEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SlimeSplitEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/SlimeSplitEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SlimeSplitEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SlimeSplitEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        slime: impl Into<crate::entity::Slime<'mc>>,
        count: i32,
    ) -> Result<crate::event::entity::SlimeSplitEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Slime;I)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(slime.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(count);
        let cls = jni.find_class("org/bukkit/event/entity/SlimeSplitEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::SlimeSplitEvent::from_raw(&jni, res)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn entity(&self) -> Result<crate::entity::Slime<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Slime;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Slime::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the amount of smaller slimes to spawn
    pub fn count(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCount", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets how many smaller slimes will spawn on the split
    pub fn set_count(&self, count: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(count);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCount",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/SlimeSplitEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['isCancelled', 'setCancelled', 'getEntity', 'getCount', 'setCount', 'getHandlers', 'getHandlerList'])
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for SlimeSplitEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SlimeSplitEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for SlimeSplitEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SlimeSplitEvent into crate::event::entity::EntityEvent")
    }
}
#[repr(C)]
pub struct PlayerLeashEntityEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PlayerLeashEntityEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PlayerLeashEntityEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PlayerLeashEntityEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/PlayerLeashEntityEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PlayerLeashEntityEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PlayerLeashEntityEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        what: impl Into<crate::entity::Entity<'mc>>,
        leash_holder: impl Into<crate::entity::Entity<'mc>>,
        leasher: impl Into<crate::entity::Player<'mc>>,
        hand: std::option::Option<impl Into<crate::inventory::EquipmentSlot<'mc>>>,
    ) -> Result<crate::event::entity::PlayerLeashEntityEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(what.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/entity/Entity;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(leash_holder.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "Lorg/bukkit/entity/Player;";
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(leasher.into().jni_object().clone())
        });
        args.push(val_3);
        if let Some(a) = hand {
            sig += "Lorg/bukkit/inventory/EquipmentSlot;";
            let val_4 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_4);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/PlayerLeashEntityEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::PlayerLeashEntityEvent::from_raw(&jni, res)
    }
    /// Returns the entity that is holding the leash.
    pub fn leash_holder(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLeashHolder", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns the entity being leashed.
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns the player involved in this event
    pub fn player(&self) -> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Player;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPlayer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Player::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns the hand used by the player to leash the entity.
    pub fn hand(&self) -> Result<crate::inventory::EquipmentSlot<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/EquipmentSlot;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHand", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::EquipmentSlot::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/PlayerLeashEntityEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    // SUPER CLASS: org.bukkit.event.Event ( ['getLeashHolder', 'getEntity', 'getPlayer', 'getHand', 'getHandlers', 'getHandlerList', 'isCancelled', 'setCancelled'])
    /// Convenience method for providing a user-friendly identifier. By
    /// default, it is the event's class's {@linkplain Class#getSimpleName()
    /// simple name}.
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
    }
    /// Any custom event that should not by synchronized with other events must
    /// use the specific constructor. These are the caveats of using an
    /// asynchronous event:
    /// <ul>
    /// <li>The event is never fired from inside code triggered by a
    /// synchronous event. Attempting to do so results in an {@link
    /// java.lang.IllegalStateException}.
    /// <li>However, asynchronous event handlers may fire synchronous or
    /// asynchronous events
    /// <li>The event may be fired multiple times simultaneously and in any
    /// order.
    /// <li>Any newly registered or unregistered handler is ignored after an
    /// event starts execution.
    /// <li>The handlers for this event may block for any length of time.
    /// <li>Some implementations may selectively declare a specific event use
    /// as asynchronous. This behavior should be clearly defined.
    /// <li>Asynchronous calls are not calculated in the plugin timing system.
    /// </ul>
    pub fn is_asynchronous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAsynchronous", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PlayerLeashEntityEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PlayerLeashEntityEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::Event<'mc>> for PlayerLeashEntityEvent<'mc> {
    fn into(self) -> crate::event::Event<'mc> {
        crate::event::Event::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PlayerLeashEntityEvent into crate::event::Event")
    }
}
#[repr(C)]
pub struct EntityInteractEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityInteractEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityInteractEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityInteractEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityInteractEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityInteractEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityInteractEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        entity: impl Into<crate::entity::Entity<'mc>>,
        block: impl Into<crate::block::Block<'mc>>,
    ) -> Result<crate::event::entity::EntityInteractEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;Lorg/bukkit/block/Block;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(entity.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityInteractEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityInteractEvent::from_raw(&jni, res)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns the involved block
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/Block;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBlock", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Block::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityInteractEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['isCancelled', 'setCancelled', 'getBlock', 'getHandlers', 'getHandlerList'])
    /// Returns the Entity involved in this event
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityInteractEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityInteractEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityInteractEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityInteractEvent into crate::event::entity::EntityEvent")
    }
}
#[repr(C)]
pub struct ItemSpawnEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ItemSpawnEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ItemSpawnEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ItemSpawnEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/ItemSpawnEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ItemSpawnEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ItemSpawnEvent<'mc> {
    #[deprecated]

    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        spawnee: impl Into<crate::entity::Item<'mc>>,
        loc: std::option::Option<impl Into<crate::Location<'mc>>>,
    ) -> Result<crate::event::entity::ItemSpawnEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Item;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(spawnee.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = loc {
            sig += "Lorg/bukkit/Location;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/ItemSpawnEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::ItemSpawnEvent::from_raw(&jni, res)
    }

    pub fn entity(&self) -> Result<crate::entity::Item<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Item;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Item::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: org.bukkit.event.entity.EntitySpawnEvent ( ['getEntity'])

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntitySpawnEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntitySpawnEvent = temp_clone.into();
        real.is_cancelled()
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntitySpawnEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntitySpawnEvent = temp_clone.into();
        real.set_cancelled(cancel)
    }
    /// Gets the location at which the entity is spawning.
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntitySpawnEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntitySpawnEvent = temp_clone.into();
        real.location()
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        crate::event::entity::EntitySpawnEvent::handler_list(jni)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntitySpawnEvent<'mc>> for ItemSpawnEvent<'mc> {
    fn into(self) -> crate::event::entity::EntitySpawnEvent<'mc> {
        crate::event::entity::EntitySpawnEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ItemSpawnEvent into crate::event::entity::EntitySpawnEvent")
    }
}
#[repr(C)]
pub struct EnderDragonChangePhaseEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EnderDragonChangePhaseEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EnderDragonChangePhaseEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EnderDragonChangePhaseEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EnderDragonChangePhaseEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EnderDragonChangePhaseEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EnderDragonChangePhaseEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        ender_dragon: impl Into<crate::entity::EnderDragon<'mc>>,
        current_phase: impl Into<crate::entity::EnderDragonPhase<'mc>>,
        new_phase: impl Into<crate::entity::EnderDragonPhase<'mc>>,
    ) -> Result<crate::event::entity::EnderDragonChangePhaseEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/entity/EnderDragon;Lorg/bukkit/entity/EnderDragon/Phase;Lorg/bukkit/entity/EnderDragon/Phase;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(ender_dragon.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(current_phase.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(new_phase.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EnderDragonChangePhaseEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EnderDragonChangePhaseEvent::from_raw(&jni, res)
    }

    pub fn entity(&self) -> Result<crate::entity::EnderDragon<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/EnderDragon;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::EnderDragon::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the current phase that the dragon is in. This method will return null
    /// when a dragon is first spawned and hasn't yet been assigned a phase.
    pub fn current_phase(
        &self,
    ) -> Result<Option<crate::entity::EnderDragonPhase<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/EnderDragon/Phase;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCurrentPhase", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::EnderDragonPhase::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Gets the new phase that the dragon will switch to.
    pub fn new_phase(
        &self,
    ) -> Result<crate::entity::EnderDragonPhase<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/EnderDragon/Phase;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getNewPhase", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::EnderDragonPhase::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the new phase for the ender dragon.
    pub fn set_new_phase(
        &self,
        new_phase: impl Into<crate::entity::EnderDragonPhase<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/EnderDragon/Phase;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(new_phase.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setNewPhase",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EnderDragonChangePhaseEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getEntity', 'getCurrentPhase', 'getNewPhase', 'setNewPhase', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EnderDragonChangePhaseEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EnderDragonChangePhaseEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EnderDragonChangePhaseEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting EnderDragonChangePhaseEvent into crate::event::entity::EntityEvent",
        )
    }
}
#[repr(C)]
pub struct EntityExhaustionEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityExhaustionEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityExhaustionEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityExhaustionEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityExhaustionEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityExhaustionEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityExhaustionEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        who: impl Into<crate::entity::HumanEntity<'mc>>,
        exhaustion_reason: impl Into<crate::event::entity::EntityExhaustionEventExhaustionReason<'mc>>,
        exhaustion: f32,
    ) -> Result<crate::event::entity::EntityExhaustionEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/HumanEntity;Lorg/bukkit/event/entity/EntityExhaustionEvent/ExhaustionReason;F)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(who.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(exhaustion_reason.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Float(exhaustion);
        let cls = jni.find_class("org/bukkit/event/entity/EntityExhaustionEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityExhaustionEvent::from_raw(&jni, res)
    }
    /// Gets the {@link ExhaustionReason} for this event
    pub fn exhaustion_reason(
        &self,
    ) -> Result<
        crate::event::entity::EntityExhaustionEventExhaustionReason<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig =
            String::from("()Lorg/bukkit/event/entity/EntityExhaustionEvent/ExhaustionReason;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getExhaustionReason",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::entity::EntityExhaustionEventExhaustionReason::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )
    }
    /// Get the amount of exhaustion to add to the player's current exhaustion.
    pub fn exhaustion(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getExhaustion", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// Set the exhaustion to apply to the player.
    /// The maximum exhaustion that a player can have is 40. No error will be
    /// thrown if this limit is hit. This value may be negative, but there is
    /// unknown behavior for when exhaustion is below 0.
    pub fn set_exhaustion(&self, exhaustion: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(exhaustion);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setExhaustion",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn entity(&self) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/HumanEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::HumanEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityExhaustionEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getExhaustionReason', 'getExhaustion', 'setExhaustion', 'getEntity', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityExhaustionEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityExhaustionEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityExhaustionEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityExhaustionEvent into crate::event::entity::EntityEvent")
    }
}
pub enum EntityExhaustionEventExhaustionReason<'mc> {
    BlockMined {
        inner: EntityExhaustionEventExhaustionReasonStruct<'mc>,
    },
    HungerEffect {
        inner: EntityExhaustionEventExhaustionReasonStruct<'mc>,
    },
    Damaged {
        inner: EntityExhaustionEventExhaustionReasonStruct<'mc>,
    },
    Attack {
        inner: EntityExhaustionEventExhaustionReasonStruct<'mc>,
    },
    JumpSprint {
        inner: EntityExhaustionEventExhaustionReasonStruct<'mc>,
    },
    Jump {
        inner: EntityExhaustionEventExhaustionReasonStruct<'mc>,
    },
    Swim {
        inner: EntityExhaustionEventExhaustionReasonStruct<'mc>,
    },
    WalkUnderwater {
        inner: EntityExhaustionEventExhaustionReasonStruct<'mc>,
    },
    WalkOnWater {
        inner: EntityExhaustionEventExhaustionReasonStruct<'mc>,
    },
    Sprint {
        inner: EntityExhaustionEventExhaustionReasonStruct<'mc>,
    },
    Crouch {
        inner: EntityExhaustionEventExhaustionReasonStruct<'mc>,
    },
    Walk {
        inner: EntityExhaustionEventExhaustionReasonStruct<'mc>,
    },
    Regen {
        inner: EntityExhaustionEventExhaustionReasonStruct<'mc>,
    },
    Unknown {
        inner: EntityExhaustionEventExhaustionReasonStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for EntityExhaustionEventExhaustionReason<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityExhaustionEventExhaustionReason::BlockMined { .. } => f.write_str("BLOCK_MINED"),
            EntityExhaustionEventExhaustionReason::HungerEffect { .. } => {
                f.write_str("HUNGER_EFFECT")
            }
            EntityExhaustionEventExhaustionReason::Damaged { .. } => f.write_str("DAMAGED"),
            EntityExhaustionEventExhaustionReason::Attack { .. } => f.write_str("ATTACK"),
            EntityExhaustionEventExhaustionReason::JumpSprint { .. } => f.write_str("JUMP_SPRINT"),
            EntityExhaustionEventExhaustionReason::Jump { .. } => f.write_str("JUMP"),
            EntityExhaustionEventExhaustionReason::Swim { .. } => f.write_str("SWIM"),
            EntityExhaustionEventExhaustionReason::WalkUnderwater { .. } => {
                f.write_str("WALK_UNDERWATER")
            }
            EntityExhaustionEventExhaustionReason::WalkOnWater { .. } => {
                f.write_str("WALK_ON_WATER")
            }
            EntityExhaustionEventExhaustionReason::Sprint { .. } => f.write_str("SPRINT"),
            EntityExhaustionEventExhaustionReason::Crouch { .. } => f.write_str("CROUCH"),
            EntityExhaustionEventExhaustionReason::Walk { .. } => f.write_str("WALK"),
            EntityExhaustionEventExhaustionReason::Regen { .. } => f.write_str("REGEN"),
            EntityExhaustionEventExhaustionReason::Unknown { .. } => f.write_str("UNKNOWN"),
        }
    }
}
impl<'mc> std::ops::Deref for EntityExhaustionEventExhaustionReason<'mc> {
    type Target = EntityExhaustionEventExhaustionReasonStruct<'mc>;
    fn deref(&self) -> &<EntityExhaustionEventExhaustionReason<'mc> as std::ops::Deref>::Target {
        match self {
            EntityExhaustionEventExhaustionReason::BlockMined { inner } => inner,
            EntityExhaustionEventExhaustionReason::HungerEffect { inner } => inner,
            EntityExhaustionEventExhaustionReason::Damaged { inner } => inner,
            EntityExhaustionEventExhaustionReason::Attack { inner } => inner,
            EntityExhaustionEventExhaustionReason::JumpSprint { inner } => inner,
            EntityExhaustionEventExhaustionReason::Jump { inner } => inner,
            EntityExhaustionEventExhaustionReason::Swim { inner } => inner,
            EntityExhaustionEventExhaustionReason::WalkUnderwater { inner } => inner,
            EntityExhaustionEventExhaustionReason::WalkOnWater { inner } => inner,
            EntityExhaustionEventExhaustionReason::Sprint { inner } => inner,
            EntityExhaustionEventExhaustionReason::Crouch { inner } => inner,
            EntityExhaustionEventExhaustionReason::Walk { inner } => inner,
            EntityExhaustionEventExhaustionReason::Regen { inner } => inner,
            EntityExhaustionEventExhaustionReason::Unknown { inner } => inner,
        }
    }
}

impl<'mc> EntityExhaustionEventExhaustionReason<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<EntityExhaustionEventExhaustionReason<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/EntityExhaustionEvent/ExhaustionReason");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/EntityExhaustionEvent/ExhaustionReason;",
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = env.translate_error(res)?;
        let obj = res.l()?;
        let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = env.translate_error(variant)?;
        let variant_str = env
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        match variant_str.as_str() {
            "BLOCK_MINED" => Ok(EntityExhaustionEventExhaustionReason::BlockMined {
                inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "HUNGER_EFFECT" => Ok(EntityExhaustionEventExhaustionReason::HungerEffect {
                inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "DAMAGED" => Ok(EntityExhaustionEventExhaustionReason::Damaged {
                inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "ATTACK" => Ok(EntityExhaustionEventExhaustionReason::Attack {
                inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "JUMP_SPRINT" => Ok(EntityExhaustionEventExhaustionReason::JumpSprint {
                inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "JUMP" => Ok(EntityExhaustionEventExhaustionReason::Jump {
                inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "SWIM" => Ok(EntityExhaustionEventExhaustionReason::Swim {
                inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "WALK_UNDERWATER" => Ok(EntityExhaustionEventExhaustionReason::WalkUnderwater {
                inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "WALK_ON_WATER" => Ok(EntityExhaustionEventExhaustionReason::WalkOnWater {
                inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "SPRINT" => Ok(EntityExhaustionEventExhaustionReason::Sprint {
                inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "CROUCH" => Ok(EntityExhaustionEventExhaustionReason::Crouch {
                inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "WALK" => Ok(EntityExhaustionEventExhaustionReason::Walk {
                inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "REGEN" => Ok(EntityExhaustionEventExhaustionReason::Regen {
                inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
            }),
            "UNKNOWN" => Ok(EntityExhaustionEventExhaustionReason::Unknown {
                inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct EntityExhaustionEventExhaustionReasonStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityExhaustionEventExhaustionReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::BlockMined { inner } => inner.0.clone(),
            Self::HungerEffect { inner } => inner.0.clone(),
            Self::Damaged { inner } => inner.0.clone(),
            Self::Attack { inner } => inner.0.clone(),
            Self::JumpSprint { inner } => inner.0.clone(),
            Self::Jump { inner } => inner.0.clone(),
            Self::Swim { inner } => inner.0.clone(),
            Self::WalkUnderwater { inner } => inner.0.clone(),
            Self::WalkOnWater { inner } => inner.0.clone(),
            Self::Sprint { inner } => inner.0.clone(),
            Self::Crouch { inner } => inner.0.clone(),
            Self::Walk { inner } => inner.0.clone(),
            Self::Regen { inner } => inner.0.clone(),
            Self::Unknown { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::BlockMined { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::HungerEffect { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Damaged { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Attack { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::JumpSprint { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Jump { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Swim { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::WalkUnderwater { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WalkOnWater { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Sprint { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Crouch { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Walk { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Regen { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Unknown { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityExhaustionEventExhaustionReason<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityExhaustionEventExhaustionReason from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityExhaustionEvent/ExhaustionReason",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntityExhaustionEventExhaustionReason object, got {}",
                    name
                )
                .into())
        } else {
            let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
            let variant = env.translate_error(variant)?;
            let variant_str = env
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            match variant_str.as_str() {
                "BLOCK_MINED" => Ok(EntityExhaustionEventExhaustionReason::BlockMined {
                    inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "HUNGER_EFFECT" => Ok(EntityExhaustionEventExhaustionReason::HungerEffect {
                    inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "DAMAGED" => Ok(EntityExhaustionEventExhaustionReason::Damaged {
                    inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "ATTACK" => Ok(EntityExhaustionEventExhaustionReason::Attack {
                    inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "JUMP_SPRINT" => Ok(EntityExhaustionEventExhaustionReason::JumpSprint {
                    inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "JUMP" => Ok(EntityExhaustionEventExhaustionReason::Jump {
                    inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "SWIM" => Ok(EntityExhaustionEventExhaustionReason::Swim {
                    inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "WALK_UNDERWATER" => Ok(EntityExhaustionEventExhaustionReason::WalkUnderwater {
                    inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "WALK_ON_WATER" => Ok(EntityExhaustionEventExhaustionReason::WalkOnWater {
                    inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "SPRINT" => Ok(EntityExhaustionEventExhaustionReason::Sprint {
                    inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "CROUCH" => Ok(EntityExhaustionEventExhaustionReason::Crouch {
                    inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "WALK" => Ok(EntityExhaustionEventExhaustionReason::Walk {
                    inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "REGEN" => Ok(EntityExhaustionEventExhaustionReason::Regen {
                    inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                "UNKNOWN" => Ok(EntityExhaustionEventExhaustionReason::Unknown {
                    inner: EntityExhaustionEventExhaustionReasonStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for EntityExhaustionEventExhaustionReasonStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityExhaustionEventExhaustionReasonStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                    "Tried to instantiate EntityExhaustionEventExhaustionReasonStruct from null object.")
                .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityExhaustionEvent/ExhaustionReason",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntityExhaustionEventExhaustionReasonStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityExhaustionEventExhaustionReasonStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<
        crate::event::entity::EntityExhaustionEventExhaustionReason<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig =
            String::from("()Lorg/bukkit/event/entity/EntityExhaustionEvent/ExhaustionReason;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityExhaustionEvent/ExhaustionReason");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::entity::EntityExhaustionEventExhaustionReason::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct BatToggleSleepEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BatToggleSleepEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BatToggleSleepEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BatToggleSleepEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/BatToggleSleepEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BatToggleSleepEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BatToggleSleepEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        what: impl Into<crate::entity::Bat<'mc>>,
        awake: bool,
    ) -> Result<crate::event::entity::BatToggleSleepEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Bat;Z)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(what.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Bool(awake.into());
        let cls = jni.find_class("org/bukkit/event/entity/BatToggleSleepEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::BatToggleSleepEvent::from_raw(&jni, res)
    }
    /// Get whether or not the bat is attempting to awaken.
    pub fn is_awake(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isAwake", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/BatToggleSleepEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['isAwake', 'setCancelled', 'isCancelled', 'getHandlers', 'getHandlerList'])
    /// Returns the Entity involved in this event
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for BatToggleSleepEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BatToggleSleepEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for BatToggleSleepEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BatToggleSleepEvent into crate::event::entity::EntityEvent")
    }
}
#[repr(C)]
pub struct VillagerCareerChangeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for VillagerCareerChangeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for VillagerCareerChangeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate VillagerCareerChangeEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/VillagerCareerChangeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a VillagerCareerChangeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> VillagerCareerChangeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        what: impl Into<crate::entity::Villager<'mc>>,
        profession: impl Into<crate::entity::VillagerProfession<'mc>>,
        reason: impl Into<crate::event::entity::VillagerCareerChangeEventChangeReason<'mc>>,
    ) -> Result<crate::event::entity::VillagerCareerChangeEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/entity/Villager;Lorg/bukkit/entity/Villager/Profession;Lorg/bukkit/event/entity/VillagerCareerChangeEvent/ChangeReason;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(what.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(profession.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(reason.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/VillagerCareerChangeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::VillagerCareerChangeEvent::from_raw(&jni, res)
    }

    pub fn entity(&self) -> Result<crate::entity::Villager<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Villager;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Villager::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the future profession of the villager.
    pub fn profession(
        &self,
    ) -> Result<crate::entity::VillagerProfession<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Villager/Profession;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getProfession", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::VillagerProfession::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the profession the villager will become from this event.
    pub fn set_profession(
        &self,
        profession: impl Into<crate::entity::VillagerProfession<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Villager/Profession;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(profession.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setProfession",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the reason for why the villager's career is changing.
    pub fn reason(
        &self,
    ) -> Result<
        crate::event::entity::VillagerCareerChangeEventChangeReason<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig =
            String::from("()Lorg/bukkit/event/entity/VillagerCareerChangeEvent/ChangeReason;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getReason", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::entity::VillagerCareerChangeEventChangeReason::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/VillagerCareerChangeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getEntity', 'getProfession', 'setProfession', 'getReason', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for VillagerCareerChangeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting VillagerCareerChangeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for VillagerCareerChangeEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting VillagerCareerChangeEvent into crate::event::entity::EntityEvent",
        )
    }
}
pub enum VillagerCareerChangeEventChangeReason<'mc> {
    LosingJob {
        inner: VillagerCareerChangeEventChangeReasonStruct<'mc>,
    },
    Employed {
        inner: VillagerCareerChangeEventChangeReasonStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for VillagerCareerChangeEventChangeReason<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VillagerCareerChangeEventChangeReason::LosingJob { .. } => f.write_str("LOSING_JOB"),
            VillagerCareerChangeEventChangeReason::Employed { .. } => f.write_str("EMPLOYED"),
        }
    }
}
impl<'mc> std::ops::Deref for VillagerCareerChangeEventChangeReason<'mc> {
    type Target = VillagerCareerChangeEventChangeReasonStruct<'mc>;
    fn deref(&self) -> &<VillagerCareerChangeEventChangeReason<'mc> as std::ops::Deref>::Target {
        match self {
            VillagerCareerChangeEventChangeReason::LosingJob { inner } => inner,
            VillagerCareerChangeEventChangeReason::Employed { inner } => inner,
        }
    }
}

impl<'mc> VillagerCareerChangeEventChangeReason<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<VillagerCareerChangeEventChangeReason<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/VillagerCareerChangeEvent/ChangeReason");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/VillagerCareerChangeEvent/ChangeReason;",
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = env.translate_error(res)?;
        let obj = res.l()?;
        let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = env.translate_error(variant)?;
        let variant_str = env
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        match variant_str.as_str() {
            "LOSING_JOB" => Ok(VillagerCareerChangeEventChangeReason::LosingJob {
                inner: VillagerCareerChangeEventChangeReasonStruct::from_raw(env, obj)?,
            }),
            "EMPLOYED" => Ok(VillagerCareerChangeEventChangeReason::Employed {
                inner: VillagerCareerChangeEventChangeReasonStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct VillagerCareerChangeEventChangeReasonStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for VillagerCareerChangeEventChangeReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::LosingJob { inner } => inner.0.clone(),
            Self::Employed { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::LosingJob { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Employed { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for VillagerCareerChangeEventChangeReason<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate VillagerCareerChangeEventChangeReason from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/VillagerCareerChangeEvent/ChangeReason",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a VillagerCareerChangeEventChangeReason object, got {}",
                    name
                )
                .into())
        } else {
            let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
            let variant = env.translate_error(variant)?;
            let variant_str = env
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            match variant_str.as_str() {
                "LOSING_JOB" => Ok(VillagerCareerChangeEventChangeReason::LosingJob {
                    inner: VillagerCareerChangeEventChangeReasonStruct::from_raw(env, obj)?,
                }),
                "EMPLOYED" => Ok(VillagerCareerChangeEventChangeReason::Employed {
                    inner: VillagerCareerChangeEventChangeReasonStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for VillagerCareerChangeEventChangeReasonStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for VillagerCareerChangeEventChangeReasonStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                    "Tried to instantiate VillagerCareerChangeEventChangeReasonStruct from null object.")
                .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/VillagerCareerChangeEvent/ChangeReason",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a VillagerCareerChangeEventChangeReasonStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> VillagerCareerChangeEventChangeReasonStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<
        crate::event::entity::VillagerCareerChangeEventChangeReason<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig =
            String::from("()Lorg/bukkit/event/entity/VillagerCareerChangeEvent/ChangeReason;");
        let cls = jni.find_class("org/bukkit/event/entity/VillagerCareerChangeEvent/ChangeReason");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::entity::VillagerCareerChangeEventChangeReason::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct EntityDamageByBlockEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityDamageByBlockEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityDamageByBlockEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityDamageByBlockEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityDamageByBlockEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityDamageByBlockEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityDamageByBlockEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        damager: impl Into<crate::block::Block<'mc>>,
        damager_state: impl Into<crate::block::BlockState<'mc>>,
        damagee: impl Into<crate::entity::Entity<'mc>>,
        cause: impl Into<crate::event::entity::EntityDamageEventDamageCause<'mc>>,
        damage_source: std::option::Option<impl Into<crate::damage::DamageSource<'mc>>>,
        modifiers: std::option::Option<impl Into<blackboxmc_java::util::JavaMap<'mc>>>,
        modifier_functions: std::option::Option<impl Into<blackboxmc_java::util::JavaMap<'mc>>>,
    ) -> Result<crate::event::entity::EntityDamageByBlockEvent<'mc>, Box<dyn std::error::Error>>
    {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/block/Block;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(damager.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/block/BlockState;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(damager_state.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "Lorg/bukkit/entity/Entity;";
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(damagee.into().jni_object().clone())
        });
        args.push(val_3);
        sig += "Lorg/bukkit/event/entity/EntityDamageEvent/DamageCause;";
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(cause.into().jni_object().clone())
        });
        args.push(val_4);
        if let Some(a) = damage_source {
            sig += "Lorg/bukkit/damage/DamageSource;";
            let val_5 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_5);
        }
        if let Some(a) = modifiers {
            sig += "Ljava/util/Map;";
            let val_6 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_6);
        }
        if let Some(a) = modifier_functions {
            sig += "Ljava/util/Map;";
            let val_7 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_7);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/EntityDamageByBlockEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityDamageByBlockEvent::from_raw(&jni, res)
    }
    /// Returns the block that damaged the player.
    pub fn damager(&self) -> Result<Option<crate::block::Block<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/Block;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDamager", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::block::Block::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Returns the captured BlockState of the block that damaged the player.
    pub fn damager_block_state(
        &self,
    ) -> Result<Option<crate::block::BlockState<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockState;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDamagerBlockState",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::block::BlockState::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityDamageEvent ( ['getDamager', 'getDamagerBlockState'])

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityDamageEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityDamageEvent = temp_clone.into();
        real.is_cancelled()
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityDamageEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityDamageEvent = temp_clone.into();
        real.set_cancelled(cancel)
    }
    /// Gets the original damage for the specified modifier, as defined at this
    /// event's construction.
    pub fn get_original_damage(
        &self,
        val_type: impl Into<crate::event::entity::EntityDamageEventDamageModifier<'mc>>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityDamageEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityDamageEvent = temp_clone.into();
        real.get_original_damage(val_type)
    }
    /// Sets the damage for the specified modifier.
    pub fn set_damage(
        &self,
        val_type: impl Into<crate::event::entity::EntityDamageEventDamageModifier<'mc>>,
        damage: std::option::Option<f64>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityDamageEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityDamageEvent = temp_clone.into();
        real.set_damage(val_type, damage)
    }
    /// Gets the raw amount of damage caused by the event
    pub fn damage(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityDamageEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityDamageEvent = temp_clone.into();
        real.damage()
    }
    /// This checks to see if a particular modifier is valid for this event's
    /// caller, such that, {@link #setDamage(DamageModifier, double)} will not
    /// throw an {@link UnsupportedOperationException}.
    ///
    /// {@link DamageModifier#BASE} is always applicable.
    pub fn is_applicable(
        &self,
        val_type: impl Into<crate::event::entity::EntityDamageEventDamageModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityDamageEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityDamageEvent = temp_clone.into();
        real.is_applicable(val_type)
    }
    /// Gets the amount of damage caused by the event after all damage
    /// reduction is applied.
    pub fn final_damage(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityDamageEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityDamageEvent = temp_clone.into();
        real.final_damage()
    }
    /// Gets the cause of the damage.
    ///
    /// While a DamageCause may indicate a specific Bukkit-assigned cause of damage,
    /// {@link #getDamageSource()} may expose additional types of damage such as custom
    /// damage types provided by data packs, as well as any direct or indirect entities,
    /// locations, or other contributing factors to the damage being inflicted. The
    /// alternative is generally preferred, but DamageCauses provided to this event
    /// should largely encompass most common use cases for developers if a simple cause
    /// is required.
    pub fn cause(
        &self,
    ) -> Result<crate::event::entity::EntityDamageEventDamageCause<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = crate::event::entity::EntityDamageEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityDamageEvent = temp_clone.into();
        real.cause()
    }
    /// Get the source of damage.
    pub fn damage_source(
        &self,
    ) -> Result<crate::damage::DamageSource<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityDamageEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityDamageEvent = temp_clone.into();
        real.damage_source()
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        crate::event::entity::EntityDamageEvent::handler_list(jni)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntityDamageEvent<'mc>> for EntityDamageByBlockEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityDamageEvent<'mc> {
        crate::event::entity::EntityDamageEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting EntityDamageByBlockEvent into crate::event::entity::EntityDamageEvent")
    }
}
#[repr(C)]
pub struct EntityTargetEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityTargetEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityTargetEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityTargetEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/EntityTargetEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityTargetEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityTargetEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        entity: impl Into<crate::entity::Entity<'mc>>,
        target: impl Into<crate::entity::Entity<'mc>>,
        reason: impl Into<crate::event::entity::EntityTargetEventTargetReason<'mc>>,
    ) -> Result<crate::event::entity::EntityTargetEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;Lorg/bukkit/entity/Entity;Lorg/bukkit/event/entity/EntityTargetEvent/TargetReason;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(entity.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(target.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(reason.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityTargetEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityTargetEvent::from_raw(&jni, res)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns the reason for the targeting
    pub fn reason(
        &self,
    ) -> Result<crate::event::entity::EntityTargetEventTargetReason<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/event/entity/EntityTargetEvent/TargetReason;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getReason", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::entity::EntityTargetEventTargetReason::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the entity that this is targeting.
    ///
    /// This will be null in the case that the event is called when the mob
    /// forgets its target.
    pub fn target(&self) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTarget", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Entity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Set the entity that you want the mob to target instead.
    ///
    /// It is possible to be null, null will cause the entity to be
    /// target-less.
    ///
    /// This is different from cancelling the event. Cancelling the event will
    /// cause the entity to keep an original target, while setting to be null
    /// will cause the entity to be reset.
    pub fn set_target(
        &self,
        target: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(target.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTarget",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityTargetEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['isCancelled', 'setCancelled', 'getReason', 'getTarget', 'setTarget', 'getHandlers', 'getHandlerList'])
    /// Returns the Entity involved in this event
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityTargetEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityTargetEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityTargetEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityTargetEvent into crate::event::entity::EntityEvent")
    }
}
pub enum EntityTargetEventTargetReason<'mc> {
    TargetDied {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
    ClosestPlayer {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
    TargetAttackedEntity {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
    PigZombieTarget {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
    ForgotTarget {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
    TargetAttackedOwner {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
    OwnerAttackedTarget {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
    RandomTarget {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
    DefendVillage {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
    TargetAttackedNearbyEntity {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
    ReinforcementTarget {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
    Collision {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
    Custom {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
    ClosestEntity {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
    FollowLeader {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
    Tempt {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
    Unknown {
        inner: EntityTargetEventTargetReasonStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for EntityTargetEventTargetReason<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityTargetEventTargetReason::TargetDied { .. } => f.write_str("TARGET_DIED"),
            EntityTargetEventTargetReason::ClosestPlayer { .. } => f.write_str("CLOSEST_PLAYER"),
            EntityTargetEventTargetReason::TargetAttackedEntity { .. } => {
                f.write_str("TARGET_ATTACKED_ENTITY")
            }
            EntityTargetEventTargetReason::PigZombieTarget { .. } => {
                f.write_str("PIG_ZOMBIE_TARGET")
            }
            EntityTargetEventTargetReason::ForgotTarget { .. } => f.write_str("FORGOT_TARGET"),
            EntityTargetEventTargetReason::TargetAttackedOwner { .. } => {
                f.write_str("TARGET_ATTACKED_OWNER")
            }
            EntityTargetEventTargetReason::OwnerAttackedTarget { .. } => {
                f.write_str("OWNER_ATTACKED_TARGET")
            }
            EntityTargetEventTargetReason::RandomTarget { .. } => f.write_str("RANDOM_TARGET"),
            EntityTargetEventTargetReason::DefendVillage { .. } => f.write_str("DEFEND_VILLAGE"),
            EntityTargetEventTargetReason::TargetAttackedNearbyEntity { .. } => {
                f.write_str("TARGET_ATTACKED_NEARBY_ENTITY")
            }
            EntityTargetEventTargetReason::ReinforcementTarget { .. } => {
                f.write_str("REINFORCEMENT_TARGET")
            }
            EntityTargetEventTargetReason::Collision { .. } => f.write_str("COLLISION"),
            EntityTargetEventTargetReason::Custom { .. } => f.write_str("CUSTOM"),
            EntityTargetEventTargetReason::ClosestEntity { .. } => f.write_str("CLOSEST_ENTITY"),
            EntityTargetEventTargetReason::FollowLeader { .. } => f.write_str("FOLLOW_LEADER"),
            EntityTargetEventTargetReason::Tempt { .. } => f.write_str("TEMPT"),
            EntityTargetEventTargetReason::Unknown { .. } => f.write_str("UNKNOWN"),
        }
    }
}
impl<'mc> std::ops::Deref for EntityTargetEventTargetReason<'mc> {
    type Target = EntityTargetEventTargetReasonStruct<'mc>;
    fn deref(&self) -> &<EntityTargetEventTargetReason<'mc> as std::ops::Deref>::Target {
        match self {
            EntityTargetEventTargetReason::TargetDied { inner } => inner,
            EntityTargetEventTargetReason::ClosestPlayer { inner } => inner,
            EntityTargetEventTargetReason::TargetAttackedEntity { inner } => inner,
            EntityTargetEventTargetReason::PigZombieTarget { inner } => inner,
            EntityTargetEventTargetReason::ForgotTarget { inner } => inner,
            EntityTargetEventTargetReason::TargetAttackedOwner { inner } => inner,
            EntityTargetEventTargetReason::OwnerAttackedTarget { inner } => inner,
            EntityTargetEventTargetReason::RandomTarget { inner } => inner,
            EntityTargetEventTargetReason::DefendVillage { inner } => inner,
            EntityTargetEventTargetReason::TargetAttackedNearbyEntity { inner } => inner,
            EntityTargetEventTargetReason::ReinforcementTarget { inner } => inner,
            EntityTargetEventTargetReason::Collision { inner } => inner,
            EntityTargetEventTargetReason::Custom { inner } => inner,
            EntityTargetEventTargetReason::ClosestEntity { inner } => inner,
            EntityTargetEventTargetReason::FollowLeader { inner } => inner,
            EntityTargetEventTargetReason::Tempt { inner } => inner,
            EntityTargetEventTargetReason::Unknown { inner } => inner,
        }
    }
}

impl<'mc> EntityTargetEventTargetReason<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<EntityTargetEventTargetReason<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/EntityTargetEvent/TargetReason");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/EntityTargetEvent/TargetReason;",
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = env.translate_error(res)?;
        let obj = res.l()?;
        let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = env.translate_error(variant)?;
        let variant_str = env
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        match variant_str.as_str() {
            "TARGET_DIED" => Ok(EntityTargetEventTargetReason::TargetDied {
                inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
            }),
            "CLOSEST_PLAYER" => Ok(EntityTargetEventTargetReason::ClosestPlayer {
                inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
            }),
            "TARGET_ATTACKED_ENTITY" => Ok(EntityTargetEventTargetReason::TargetAttackedEntity {
                inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
            }),
            "PIG_ZOMBIE_TARGET" => Ok(EntityTargetEventTargetReason::PigZombieTarget {
                inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
            }),
            "FORGOT_TARGET" => Ok(EntityTargetEventTargetReason::ForgotTarget {
                inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
            }),
            "TARGET_ATTACKED_OWNER" => Ok(EntityTargetEventTargetReason::TargetAttackedOwner {
                inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
            }),
            "OWNER_ATTACKED_TARGET" => Ok(EntityTargetEventTargetReason::OwnerAttackedTarget {
                inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
            }),
            "RANDOM_TARGET" => Ok(EntityTargetEventTargetReason::RandomTarget {
                inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
            }),
            "DEFEND_VILLAGE" => Ok(EntityTargetEventTargetReason::DefendVillage {
                inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
            }),
            "TARGET_ATTACKED_NEARBY_ENTITY" => {
                Ok(EntityTargetEventTargetReason::TargetAttackedNearbyEntity {
                    inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                })
            }
            "REINFORCEMENT_TARGET" => Ok(EntityTargetEventTargetReason::ReinforcementTarget {
                inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
            }),
            "COLLISION" => Ok(EntityTargetEventTargetReason::Collision {
                inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
            }),
            "CUSTOM" => Ok(EntityTargetEventTargetReason::Custom {
                inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
            }),
            "CLOSEST_ENTITY" => Ok(EntityTargetEventTargetReason::ClosestEntity {
                inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
            }),
            "FOLLOW_LEADER" => Ok(EntityTargetEventTargetReason::FollowLeader {
                inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
            }),
            "TEMPT" => Ok(EntityTargetEventTargetReason::Tempt {
                inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
            }),
            "UNKNOWN" => Ok(EntityTargetEventTargetReason::Unknown {
                inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct EntityTargetEventTargetReasonStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityTargetEventTargetReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::TargetDied { inner } => inner.0.clone(),
            Self::ClosestPlayer { inner } => inner.0.clone(),
            Self::TargetAttackedEntity { inner } => inner.0.clone(),
            Self::PigZombieTarget { inner } => inner.0.clone(),
            Self::ForgotTarget { inner } => inner.0.clone(),
            Self::TargetAttackedOwner { inner } => inner.0.clone(),
            Self::OwnerAttackedTarget { inner } => inner.0.clone(),
            Self::RandomTarget { inner } => inner.0.clone(),
            Self::DefendVillage { inner } => inner.0.clone(),
            Self::TargetAttackedNearbyEntity { inner } => inner.0.clone(),
            Self::ReinforcementTarget { inner } => inner.0.clone(),
            Self::Collision { inner } => inner.0.clone(),
            Self::Custom { inner } => inner.0.clone(),
            Self::ClosestEntity { inner } => inner.0.clone(),
            Self::FollowLeader { inner } => inner.0.clone(),
            Self::Tempt { inner } => inner.0.clone(),
            Self::Unknown { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::TargetDied { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ClosestPlayer { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TargetAttackedEntity { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::PigZombieTarget { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ForgotTarget { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TargetAttackedOwner { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::OwnerAttackedTarget { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::RandomTarget { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DefendVillage { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TargetAttackedNearbyEntity { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ReinforcementTarget { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Collision { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Custom { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::ClosestEntity { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::FollowLeader { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Tempt { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Unknown { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityTargetEventTargetReason<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityTargetEventTargetReason from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityTargetEvent/TargetReason",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityTargetEventTargetReason object, got {}",
                name
            )
            .into())
        } else {
            let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
            let variant = env.translate_error(variant)?;
            let variant_str = env
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            match variant_str.as_str() {
                "TARGET_DIED" => Ok(EntityTargetEventTargetReason::TargetDied {
                    inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                }),
                "CLOSEST_PLAYER" => Ok(EntityTargetEventTargetReason::ClosestPlayer {
                    inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                }),
                "TARGET_ATTACKED_ENTITY" => {
                    Ok(EntityTargetEventTargetReason::TargetAttackedEntity {
                        inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                    })
                }
                "PIG_ZOMBIE_TARGET" => Ok(EntityTargetEventTargetReason::PigZombieTarget {
                    inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                }),
                "FORGOT_TARGET" => Ok(EntityTargetEventTargetReason::ForgotTarget {
                    inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                }),
                "TARGET_ATTACKED_OWNER" => Ok(EntityTargetEventTargetReason::TargetAttackedOwner {
                    inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                }),
                "OWNER_ATTACKED_TARGET" => Ok(EntityTargetEventTargetReason::OwnerAttackedTarget {
                    inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                }),
                "RANDOM_TARGET" => Ok(EntityTargetEventTargetReason::RandomTarget {
                    inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                }),
                "DEFEND_VILLAGE" => Ok(EntityTargetEventTargetReason::DefendVillage {
                    inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                }),
                "TARGET_ATTACKED_NEARBY_ENTITY" => {
                    Ok(EntityTargetEventTargetReason::TargetAttackedNearbyEntity {
                        inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                    })
                }
                "REINFORCEMENT_TARGET" => Ok(EntityTargetEventTargetReason::ReinforcementTarget {
                    inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                }),
                "COLLISION" => Ok(EntityTargetEventTargetReason::Collision {
                    inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                }),
                "CUSTOM" => Ok(EntityTargetEventTargetReason::Custom {
                    inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                }),
                "CLOSEST_ENTITY" => Ok(EntityTargetEventTargetReason::ClosestEntity {
                    inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                }),
                "FOLLOW_LEADER" => Ok(EntityTargetEventTargetReason::FollowLeader {
                    inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                }),
                "TEMPT" => Ok(EntityTargetEventTargetReason::Tempt {
                    inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                }),
                "UNKNOWN" => Ok(EntityTargetEventTargetReason::Unknown {
                    inner: EntityTargetEventTargetReasonStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for EntityTargetEventTargetReasonStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityTargetEventTargetReasonStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityTargetEventTargetReasonStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityTargetEvent/TargetReason",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntityTargetEventTargetReasonStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityTargetEventTargetReasonStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::entity::EntityTargetEventTargetReason<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/event/entity/EntityTargetEvent/TargetReason;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityTargetEvent/TargetReason");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::entity::EntityTargetEventTargetReason::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct ExpBottleEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ExpBottleEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ExpBottleEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ExpBottleEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/ExpBottleEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ExpBottleEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ExpBottleEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        bottle: impl Into<crate::entity::ThrownExpBottle<'mc>>,
        hit_entity: impl Into<crate::entity::Entity<'mc>>,
        hit_block: std::option::Option<impl Into<crate::block::Block<'mc>>>,
        hit_face: std::option::Option<impl Into<crate::block::BlockFace<'mc>>>,
        exp: std::option::Option<i32>,
    ) -> Result<crate::event::entity::ExpBottleEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/ThrownExpBottle;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(bottle.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/entity/Entity;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(hit_entity.into().jni_object().clone())
        });
        args.push(val_2);
        if let Some(a) = hit_block {
            sig += "Lorg/bukkit/block/Block;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        if let Some(a) = hit_face {
            sig += "Lorg/bukkit/block/BlockFace;";
            let val_4 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_4);
        }
        if let Some(a) = exp {
            sig += "I";
            let val_5 = jni::objects::JValueGen::Int(a);
            args.push(val_5);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/ExpBottleEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::ExpBottleEvent::from_raw(&jni, res)
    }

    pub fn entity(
        &self,
    ) -> Result<crate::entity::ThrownExpBottle<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/ThrownExpBottle;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::ThrownExpBottle::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// This method indicates if the particle effect should be shown.
    pub fn show_effect(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getShowEffect", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// This method sets if the particle effect will be shown.
    ///
    /// This does not change the experience created.
    pub fn set_show_effect(&self, show_effect: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(show_effect.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setShowEffect",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// This method retrieves the amount of experience to be created.
    ///
    /// The number indicates a total amount to be divided into orbs.
    pub fn experience(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getExperience", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// This method sets the amount of experience to be created.
    ///
    /// The number indicates a total amount to be divided into orbs.
    pub fn set_experience(&self, exp: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(exp);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setExperience",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/ExpBottleEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.ProjectileHitEvent ( ['getEntity', 'getShowEffect', 'setShowEffect', 'getExperience', 'setExperience', 'getHandlers', 'getHandlerList'])
    /// Gets the block that was hit, if it was a block that was hit.
    pub fn hit_block(
        &self,
    ) -> Result<Option<crate::block::Block<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::ProjectileHitEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::ProjectileHitEvent = temp_clone.into();
        real.hit_block()
    }
    /// Gets the block face that was hit, if it was a block that was hit and the
    /// face was provided in the event.
    pub fn hit_block_face(
        &self,
    ) -> Result<Option<crate::block::BlockFace<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::ProjectileHitEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::ProjectileHitEvent = temp_clone.into();
        real.hit_block_face()
    }
    /// Gets the entity that was hit, if it was an entity that was hit.
    pub fn hit_entity(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::ProjectileHitEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::ProjectileHitEvent = temp_clone.into();
        real.hit_entity()
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::ProjectileHitEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::ProjectileHitEvent = temp_clone.into();
        real.is_cancelled()
    }
    /// Whether to cancel the action that occurs when the projectile hits.
    /// In the case of an entity, it will not collide (unless it's a firework,
    /// then use {@link FireworkExplodeEvent}).
    ///
    /// In the case of a block, some blocks (eg target block, bell) will not
    /// perform the action associated.
    ///
    /// This does NOT prevent block collisions, and explosions will still occur
    /// unless their respective events are cancelled.
    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::ProjectileHitEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::ProjectileHitEvent = temp_clone.into();
        real.set_cancelled(cancel)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::ProjectileHitEvent<'mc>> for ExpBottleEvent<'mc> {
    fn into(self) -> crate::event::entity::ProjectileHitEvent<'mc> {
        crate::event::entity::ProjectileHitEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ExpBottleEvent into crate::event::entity::ProjectileHitEvent")
    }
}
#[repr(C)]
pub struct EntityMountEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityMountEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityMountEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityMountEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/EntityMountEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityMountEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityMountEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        what: impl Into<crate::entity::Entity<'mc>>,
        mount: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<crate::event::entity::EntityMountEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;Lorg/bukkit/entity/Entity;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(what.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(mount.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityMountEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityMountEvent::from_raw(&jni, res)
    }
    /// Gets the entity which will be ridden.
    pub fn mount(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMount", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityMountEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getMount', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Returns the Entity involved in this event
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityMountEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityMountEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityMountEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityMountEvent into crate::event::entity::EntityEvent")
    }
}
#[repr(C)]
pub struct EntityBreakDoorEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityBreakDoorEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityBreakDoorEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityBreakDoorEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityBreakDoorEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityBreakDoorEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityBreakDoorEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        entity: impl Into<crate::entity::LivingEntity<'mc>>,
        target_block: impl Into<crate::block::Block<'mc>>,
    ) -> Result<crate::event::entity::EntityBreakDoorEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/block/Block;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(entity.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(target_block.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityBreakDoorEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityBreakDoorEvent::from_raw(&jni, res)
    }

    pub fn entity(&self) -> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/LivingEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::LivingEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityChangeBlockEvent ( ['getEntity'])
    /// Gets the block the entity is changing
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityChangeBlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityChangeBlockEvent = temp_clone.into();
        real.block()
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityChangeBlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityChangeBlockEvent = temp_clone.into();
        real.is_cancelled()
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityChangeBlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityChangeBlockEvent = temp_clone.into();
        real.set_cancelled(cancel)
    }
    /// Gets the Material that the block is changing into
    pub fn to(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityChangeBlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityChangeBlockEvent = temp_clone.into();
        real.to()
    }
    /// Gets the data for the block that would be changed into
    pub fn block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityChangeBlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityChangeBlockEvent = temp_clone.into();
        real.block_data()
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        crate::event::entity::EntityChangeBlockEvent::handler_list(jni)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntityChangeBlockEvent<'mc>> for EntityBreakDoorEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityChangeBlockEvent<'mc> {
        crate::event::entity::EntityChangeBlockEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting EntityBreakDoorEvent into crate::event::entity::EntityChangeBlockEvent")
    }
}
#[repr(C)]
pub struct EntityEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate EntityEvent from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/EntityEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        what: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<crate::event::entity::EntityEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(what.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityEvent::from_raw(&jni, res)
    }
    /// Returns the Entity involved in this event
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/EntityType;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEntityType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::EntityType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: org.bukkit.event.Event ( ['getEntity', 'getEntityType'])
    /// Convenience method for providing a user-friendly identifier. By
    /// default, it is the event's class's {@linkplain Class#getSimpleName()
    /// simple name}.
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
    }
    /// Any custom event that should not by synchronized with other events must
    /// use the specific constructor. These are the caveats of using an
    /// asynchronous event:
    /// <ul>
    /// <li>The event is never fired from inside code triggered by a
    /// synchronous event. Attempting to do so results in an {@link
    /// java.lang.IllegalStateException}.
    /// <li>However, asynchronous event handlers may fire synchronous or
    /// asynchronous events
    /// <li>The event may be fired multiple times simultaneously and in any
    /// order.
    /// <li>Any newly registered or unregistered handler is ignored after an
    /// event starts execution.
    /// <li>The handlers for this event may block for any length of time.
    /// <li>Some implementations may selectively declare a specific event use
    /// as asynchronous. This behavior should be clearly defined.
    /// <li>Asynchronous calls are not calculated in the plugin timing system.
    /// </ul>
    pub fn is_asynchronous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAsynchronous", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Event<'mc>> for EntityEvent<'mc> {
    fn into(self) -> crate::event::Event<'mc> {
        crate::event::Event::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityEvent into crate::event::Event")
    }
}
#[repr(C)]
pub struct PlayerDeathEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PlayerDeathEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PlayerDeathEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PlayerDeathEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/PlayerDeathEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PlayerDeathEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PlayerDeathEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        player: impl Into<crate::entity::Player<'mc>>,
        damage_source: impl Into<crate::damage::DamageSource<'mc>>,
        drops: Vec<jni::objects::JObject<'mc>>,
        dropped_exp: i32,
        new_exp: i32,
        new_total_exp: std::option::Option<i32>,
        new_level: std::option::Option<i32>,
        death_message: std::option::Option<impl Into<String>>,
    ) -> Result<crate::event::entity::PlayerDeathEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Player;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(player.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/damage/DamageSource;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(damage_source.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "Ljava/util/List;";
        let raw_val_3 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
        for v in drops {
            sig += "Ljava/lang/java/lang/Object;";
            let map_val_0 = jni::objects::JValueGen::Object(v);
            jni.call_method(
                &raw_val_3,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_3 = jni::objects::JValueGen::Object(raw_val_3);
        args.push(val_3);
        sig += "I";
        let val_4 = jni::objects::JValueGen::Int(dropped_exp);
        args.push(val_4);
        sig += "I";
        let val_5 = jni::objects::JValueGen::Int(new_exp);
        args.push(val_5);
        if let Some(a) = new_total_exp {
            sig += "I";
            let val_6 = jni::objects::JValueGen::Int(a);
            args.push(val_6);
        }
        if let Some(a) = new_level {
            sig += "I";
            let val_7 = jni::objects::JValueGen::Int(a);
            args.push(val_7);
        }
        if let Some(a) = death_message {
            sig += "Ljava/lang/String;";
            let val_8 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                jni.new_string(a.into())?,
            ));
            args.push(val_8);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/PlayerDeathEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::PlayerDeathEvent::from_raw(&jni, res)
    }

    pub fn entity(&self) -> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Player;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Player::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Set the death message that will appear to everyone on the server.
    pub fn set_death_message(
        &self,
        death_message: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(death_message.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDeathMessage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the death message that will appear to everyone on the server.
    pub fn death_message(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDeathMessage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(
            self.jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
                .to_string_lossy()
                .to_string(),
        ))
    }
    /// Gets how much EXP the Player should have at respawn.
    ///
    /// This does not indicate how much EXP should be dropped, please see
    /// {@link #getDroppedExp()} for that.
    pub fn new_exp(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getNewExp", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets how much EXP the Player should have at respawn.
    ///
    /// This does not indicate how much EXP should be dropped, please see
    /// {@link #setDroppedExp(int)} for that.
    pub fn set_new_exp(&self, exp: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(exp);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setNewExp",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the Level the Player should have at respawn.
    pub fn new_level(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getNewLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the Level the Player should have at respawn.
    pub fn set_new_level(&self, level: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(level);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setNewLevel",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the Total EXP the Player should have at respawn.
    pub fn new_total_exp(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getNewTotalExp", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the Total EXP the Player should have at respawn.
    pub fn set_new_total_exp(&self, total_exp: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(total_exp);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setNewTotalExp",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets if the Player should keep all EXP at respawn.
    ///
    /// This flag overrides other EXP settings
    pub fn keep_level(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getKeepLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets if the Player should keep all EXP at respawn.
    ///
    /// This overrides all other EXP settings
    ///
    /// <b>This doesn't prevent the EXP from dropping.
    /// {@link #setDroppedExp(int)} should be used stop the
    /// EXP from dropping.</b>
    pub fn set_keep_level(&self, keep_level: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(keep_level.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setKeepLevel",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets if the Player keeps inventory on death.
    ///
    /// <b>This doesn't prevent the items from dropping.
    /// {@code getDrops().clear()} should be used stop the
    /// items from dropping.</b>
    pub fn set_keep_inventory(
        &self,
        keep_inventory: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(keep_inventory.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setKeepInventory",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets if the Player keeps inventory on death.
    pub fn keep_inventory(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getKeepInventory",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityDeathEvent ( ['getEntity', 'setDeathMessage', 'getDeathMessage', 'getNewExp', 'setNewExp', 'getNewLevel', 'setNewLevel', 'getNewTotalExp', 'setNewTotalExp', 'getKeepLevel', 'setKeepLevel', 'setKeepInventory', 'getKeepInventory'])
    /// Gets the source of damage which caused the death.
    pub fn damage_source(
        &self,
    ) -> Result<crate::damage::DamageSource<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityDeathEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityDeathEvent = temp_clone.into();
        real.damage_source()
    }
    /// Gets how much EXP should be dropped from this death.
    ///
    /// This does not indicate how much EXP should be taken from the entity in
    /// question, merely how much should be created after its death.
    pub fn dropped_exp(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityDeathEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityDeathEvent = temp_clone.into();
        real.dropped_exp()
    }
    /// Sets how much EXP should be dropped from this death.
    ///
    /// This does not indicate how much EXP should be taken from the entity in
    /// question, merely how much should be created after its death.
    pub fn set_dropped_exp(&self, exp: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityDeathEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityDeathEvent = temp_clone.into();
        real.set_dropped_exp(exp)
    }
    /// Gets all the items which will drop when the entity dies
    pub fn drops(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDrops", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::inventory::ItemStack::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        crate::event::entity::EntityDeathEvent::handler_list(jni)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntityDeathEvent<'mc>> for PlayerDeathEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityDeathEvent<'mc> {
        crate::event::entity::EntityDeathEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PlayerDeathEvent into crate::event::entity::EntityDeathEvent")
    }
}
#[repr(C)]
pub struct EntityPotionEffectEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityPotionEffectEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityPotionEffectEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityPotionEffectEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityPotionEffectEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityPotionEffectEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityPotionEffectEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        living_entity: impl Into<crate::entity::LivingEntity<'mc>>,
        old_effect: impl Into<crate::potion::PotionEffect<'mc>>,
        new_effect: impl Into<crate::potion::PotionEffect<'mc>>,
        cause: impl Into<crate::event::entity::EntityPotionEffectEventCause<'mc>>,
        action: impl Into<crate::event::entity::EntityPotionEffectEventAction<'mc>>,
        val_override: bool,
    ) -> Result<crate::event::entity::EntityPotionEffectEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/potion/PotionEffect;Lorg/bukkit/potion/PotionEffect;Lorg/bukkit/event/entity/EntityPotionEffectEvent/Cause;Lorg/bukkit/event/entity/EntityPotionEffectEvent/Action;Z)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(living_entity.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(old_effect.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(new_effect.into().jni_object().clone())
        });
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(cause.into().jni_object().clone())
        });
        let val_5 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(action.into().jni_object().clone())
        });
        let val_6 = jni::objects::JValueGen::Bool(val_override.into());
        let cls = jni.find_class("org/bukkit/event/entity/EntityPotionEffectEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
                jni::objects::JValueGen::from(val_5),
                jni::objects::JValueGen::from(val_6),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityPotionEffectEvent::from_raw(&jni, res)
    }
    /// Gets the old potion effect of the changed type, which will be removed.
    pub fn old_effect(
        &self,
    ) -> Result<Option<crate::potion::PotionEffect<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/potion/PotionEffect;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getOldEffect", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::potion::PotionEffect::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Gets new potion effect of the changed type to be applied.
    pub fn new_effect(
        &self,
    ) -> Result<Option<crate::potion::PotionEffect<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/potion/PotionEffect;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getNewEffect", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::potion::PotionEffect::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Gets the cause why the effect has changed.
    pub fn cause(
        &self,
    ) -> Result<crate::event::entity::EntityPotionEffectEventCause<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/event/entity/EntityPotionEffectEvent/Cause;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCause", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::entity::EntityPotionEffectEventCause::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the action which will be performed on the potion effect type.
    pub fn action(
        &self,
    ) -> Result<crate::event::entity::EntityPotionEffectEventAction<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/event/entity/EntityPotionEffectEvent/Action;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAction", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::entity::EntityPotionEffectEventAction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the modified potion effect type.
    pub fn modified_type(
        &self,
    ) -> Result<crate::potion::PotionEffectType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/potion/PotionEffectType;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getModifiedType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::potion::PotionEffectType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns if the new potion effect will override the old potion effect
    /// (Only applicable for the CHANGED Action).
    pub fn is_override(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isOverride", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets if the new potion effect will override the old potion effect (Only
    /// applicable for the CHANGED action).
    pub fn set_override(&self, val_override: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(val_override.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOverride",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityPotionEffectEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getOldEffect', 'getNewEffect', 'getCause', 'getAction', 'getModifiedType', 'isOverride', 'setOverride', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Returns the Entity involved in this event
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityPotionEffectEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityPotionEffectEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityPotionEffectEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting EntityPotionEffectEvent into crate::event::entity::EntityEvent",
        )
    }
}
pub enum EntityPotionEffectEventAction<'mc> {
    Added {
        inner: EntityPotionEffectEventActionStruct<'mc>,
    },
    Changed {
        inner: EntityPotionEffectEventActionStruct<'mc>,
    },
    Cleared {
        inner: EntityPotionEffectEventActionStruct<'mc>,
    },
    Removed {
        inner: EntityPotionEffectEventActionStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for EntityPotionEffectEventAction<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityPotionEffectEventAction::Added { .. } => f.write_str("ADDED"),
            EntityPotionEffectEventAction::Changed { .. } => f.write_str("CHANGED"),
            EntityPotionEffectEventAction::Cleared { .. } => f.write_str("CLEARED"),
            EntityPotionEffectEventAction::Removed { .. } => f.write_str("REMOVED"),
        }
    }
}
impl<'mc> std::ops::Deref for EntityPotionEffectEventAction<'mc> {
    type Target = EntityPotionEffectEventActionStruct<'mc>;
    fn deref(&self) -> &<EntityPotionEffectEventAction<'mc> as std::ops::Deref>::Target {
        match self {
            EntityPotionEffectEventAction::Added { inner } => inner,
            EntityPotionEffectEventAction::Changed { inner } => inner,
            EntityPotionEffectEventAction::Cleared { inner } => inner,
            EntityPotionEffectEventAction::Removed { inner } => inner,
        }
    }
}

impl<'mc> EntityPotionEffectEventAction<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<EntityPotionEffectEventAction<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/EntityPotionEffectEvent/Action");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/EntityPotionEffectEvent/Action;",
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = env.translate_error(res)?;
        let obj = res.l()?;
        let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = env.translate_error(variant)?;
        let variant_str = env
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        match variant_str.as_str() {
            "ADDED" => Ok(EntityPotionEffectEventAction::Added {
                inner: EntityPotionEffectEventActionStruct::from_raw(env, obj)?,
            }),
            "CHANGED" => Ok(EntityPotionEffectEventAction::Changed {
                inner: EntityPotionEffectEventActionStruct::from_raw(env, obj)?,
            }),
            "CLEARED" => Ok(EntityPotionEffectEventAction::Cleared {
                inner: EntityPotionEffectEventActionStruct::from_raw(env, obj)?,
            }),
            "REMOVED" => Ok(EntityPotionEffectEventAction::Removed {
                inner: EntityPotionEffectEventActionStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct EntityPotionEffectEventActionStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityPotionEffectEventAction<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Added { inner } => inner.0.clone(),
            Self::Changed { inner } => inner.0.clone(),
            Self::Cleared { inner } => inner.0.clone(),
            Self::Removed { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Added { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Changed { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Cleared { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Removed { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityPotionEffectEventAction<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityPotionEffectEventAction from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityPotionEffectEvent/Action",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityPotionEffectEventAction object, got {}",
                name
            )
            .into())
        } else {
            let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
            let variant = env.translate_error(variant)?;
            let variant_str = env
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            match variant_str.as_str() {
                "ADDED" => Ok(EntityPotionEffectEventAction::Added {
                    inner: EntityPotionEffectEventActionStruct::from_raw(env, obj)?,
                }),
                "CHANGED" => Ok(EntityPotionEffectEventAction::Changed {
                    inner: EntityPotionEffectEventActionStruct::from_raw(env, obj)?,
                }),
                "CLEARED" => Ok(EntityPotionEffectEventAction::Cleared {
                    inner: EntityPotionEffectEventActionStruct::from_raw(env, obj)?,
                }),
                "REMOVED" => Ok(EntityPotionEffectEventAction::Removed {
                    inner: EntityPotionEffectEventActionStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for EntityPotionEffectEventActionStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityPotionEffectEventActionStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityPotionEffectEventActionStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityPotionEffectEvent/Action",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntityPotionEffectEventActionStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityPotionEffectEventActionStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::entity::EntityPotionEffectEventAction<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/event/entity/EntityPotionEffectEvent/Action;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityPotionEffectEvent/Action");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::entity::EntityPotionEffectEventAction::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub enum EntityPotionEffectEventCause<'mc> {
    AreaEffectCloud {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Arrow {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Attack {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Axolotl {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Beacon {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Command {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Conduit {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Conversion {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Death {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Dolphin {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Expiration {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Food {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Illusion {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Milk {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    PatrolCaptain {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Plugin {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    PotionDrink {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    PotionSplash {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    SpiderSpawn {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Totem {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    TurtleHelmet {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Unknown {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    VillagerTrade {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    Warden {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
    WitherRose {
        inner: EntityPotionEffectEventCauseStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for EntityPotionEffectEventCause<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityPotionEffectEventCause::AreaEffectCloud { .. } => {
                f.write_str("AREA_EFFECT_CLOUD")
            }
            EntityPotionEffectEventCause::Arrow { .. } => f.write_str("ARROW"),
            EntityPotionEffectEventCause::Attack { .. } => f.write_str("ATTACK"),
            EntityPotionEffectEventCause::Axolotl { .. } => f.write_str("AXOLOTL"),
            EntityPotionEffectEventCause::Beacon { .. } => f.write_str("BEACON"),
            EntityPotionEffectEventCause::Command { .. } => f.write_str("COMMAND"),
            EntityPotionEffectEventCause::Conduit { .. } => f.write_str("CONDUIT"),
            EntityPotionEffectEventCause::Conversion { .. } => f.write_str("CONVERSION"),
            EntityPotionEffectEventCause::Death { .. } => f.write_str("DEATH"),
            EntityPotionEffectEventCause::Dolphin { .. } => f.write_str("DOLPHIN"),
            EntityPotionEffectEventCause::Expiration { .. } => f.write_str("EXPIRATION"),
            EntityPotionEffectEventCause::Food { .. } => f.write_str("FOOD"),
            EntityPotionEffectEventCause::Illusion { .. } => f.write_str("ILLUSION"),
            EntityPotionEffectEventCause::Milk { .. } => f.write_str("MILK"),
            EntityPotionEffectEventCause::PatrolCaptain { .. } => f.write_str("PATROL_CAPTAIN"),
            EntityPotionEffectEventCause::Plugin { .. } => f.write_str("PLUGIN"),
            EntityPotionEffectEventCause::PotionDrink { .. } => f.write_str("POTION_DRINK"),
            EntityPotionEffectEventCause::PotionSplash { .. } => f.write_str("POTION_SPLASH"),
            EntityPotionEffectEventCause::SpiderSpawn { .. } => f.write_str("SPIDER_SPAWN"),
            EntityPotionEffectEventCause::Totem { .. } => f.write_str("TOTEM"),
            EntityPotionEffectEventCause::TurtleHelmet { .. } => f.write_str("TURTLE_HELMET"),
            EntityPotionEffectEventCause::Unknown { .. } => f.write_str("UNKNOWN"),
            EntityPotionEffectEventCause::VillagerTrade { .. } => f.write_str("VILLAGER_TRADE"),
            EntityPotionEffectEventCause::Warden { .. } => f.write_str("WARDEN"),
            EntityPotionEffectEventCause::WitherRose { .. } => f.write_str("WITHER_ROSE"),
        }
    }
}
impl<'mc> std::ops::Deref for EntityPotionEffectEventCause<'mc> {
    type Target = EntityPotionEffectEventCauseStruct<'mc>;
    fn deref(&self) -> &<EntityPotionEffectEventCause<'mc> as std::ops::Deref>::Target {
        match self {
            EntityPotionEffectEventCause::AreaEffectCloud { inner } => inner,
            EntityPotionEffectEventCause::Arrow { inner } => inner,
            EntityPotionEffectEventCause::Attack { inner } => inner,
            EntityPotionEffectEventCause::Axolotl { inner } => inner,
            EntityPotionEffectEventCause::Beacon { inner } => inner,
            EntityPotionEffectEventCause::Command { inner } => inner,
            EntityPotionEffectEventCause::Conduit { inner } => inner,
            EntityPotionEffectEventCause::Conversion { inner } => inner,
            EntityPotionEffectEventCause::Death { inner } => inner,
            EntityPotionEffectEventCause::Dolphin { inner } => inner,
            EntityPotionEffectEventCause::Expiration { inner } => inner,
            EntityPotionEffectEventCause::Food { inner } => inner,
            EntityPotionEffectEventCause::Illusion { inner } => inner,
            EntityPotionEffectEventCause::Milk { inner } => inner,
            EntityPotionEffectEventCause::PatrolCaptain { inner } => inner,
            EntityPotionEffectEventCause::Plugin { inner } => inner,
            EntityPotionEffectEventCause::PotionDrink { inner } => inner,
            EntityPotionEffectEventCause::PotionSplash { inner } => inner,
            EntityPotionEffectEventCause::SpiderSpawn { inner } => inner,
            EntityPotionEffectEventCause::Totem { inner } => inner,
            EntityPotionEffectEventCause::TurtleHelmet { inner } => inner,
            EntityPotionEffectEventCause::Unknown { inner } => inner,
            EntityPotionEffectEventCause::VillagerTrade { inner } => inner,
            EntityPotionEffectEventCause::Warden { inner } => inner,
            EntityPotionEffectEventCause::WitherRose { inner } => inner,
        }
    }
}

impl<'mc> EntityPotionEffectEventCause<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<EntityPotionEffectEventCause<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/entity/EntityPotionEffectEvent/Cause");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/entity/EntityPotionEffectEvent/Cause;",
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = env.translate_error(res)?;
        let obj = res.l()?;
        let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = env.translate_error(variant)?;
        let variant_str = env
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        match variant_str.as_str() {
            "AREA_EFFECT_CLOUD" => Ok(EntityPotionEffectEventCause::AreaEffectCloud {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "ARROW" => Ok(EntityPotionEffectEventCause::Arrow {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "ATTACK" => Ok(EntityPotionEffectEventCause::Attack {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "AXOLOTL" => Ok(EntityPotionEffectEventCause::Axolotl {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "BEACON" => Ok(EntityPotionEffectEventCause::Beacon {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "COMMAND" => Ok(EntityPotionEffectEventCause::Command {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "CONDUIT" => Ok(EntityPotionEffectEventCause::Conduit {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "CONVERSION" => Ok(EntityPotionEffectEventCause::Conversion {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "DEATH" => Ok(EntityPotionEffectEventCause::Death {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "DOLPHIN" => Ok(EntityPotionEffectEventCause::Dolphin {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "EXPIRATION" => Ok(EntityPotionEffectEventCause::Expiration {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "FOOD" => Ok(EntityPotionEffectEventCause::Food {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "ILLUSION" => Ok(EntityPotionEffectEventCause::Illusion {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "MILK" => Ok(EntityPotionEffectEventCause::Milk {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "PATROL_CAPTAIN" => Ok(EntityPotionEffectEventCause::PatrolCaptain {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "PLUGIN" => Ok(EntityPotionEffectEventCause::Plugin {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "POTION_DRINK" => Ok(EntityPotionEffectEventCause::PotionDrink {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "POTION_SPLASH" => Ok(EntityPotionEffectEventCause::PotionSplash {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "SPIDER_SPAWN" => Ok(EntityPotionEffectEventCause::SpiderSpawn {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "TOTEM" => Ok(EntityPotionEffectEventCause::Totem {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "TURTLE_HELMET" => Ok(EntityPotionEffectEventCause::TurtleHelmet {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "UNKNOWN" => Ok(EntityPotionEffectEventCause::Unknown {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "VILLAGER_TRADE" => Ok(EntityPotionEffectEventCause::VillagerTrade {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "WARDEN" => Ok(EntityPotionEffectEventCause::Warden {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),
            "WITHER_ROSE" => Ok(EntityPotionEffectEventCause::WitherRose {
                inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct EntityPotionEffectEventCauseStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityPotionEffectEventCause<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::AreaEffectCloud { inner } => inner.0.clone(),
            Self::Arrow { inner } => inner.0.clone(),
            Self::Attack { inner } => inner.0.clone(),
            Self::Axolotl { inner } => inner.0.clone(),
            Self::Beacon { inner } => inner.0.clone(),
            Self::Command { inner } => inner.0.clone(),
            Self::Conduit { inner } => inner.0.clone(),
            Self::Conversion { inner } => inner.0.clone(),
            Self::Death { inner } => inner.0.clone(),
            Self::Dolphin { inner } => inner.0.clone(),
            Self::Expiration { inner } => inner.0.clone(),
            Self::Food { inner } => inner.0.clone(),
            Self::Illusion { inner } => inner.0.clone(),
            Self::Milk { inner } => inner.0.clone(),
            Self::PatrolCaptain { inner } => inner.0.clone(),
            Self::Plugin { inner } => inner.0.clone(),
            Self::PotionDrink { inner } => inner.0.clone(),
            Self::PotionSplash { inner } => inner.0.clone(),
            Self::SpiderSpawn { inner } => inner.0.clone(),
            Self::Totem { inner } => inner.0.clone(),
            Self::TurtleHelmet { inner } => inner.0.clone(),
            Self::Unknown { inner } => inner.0.clone(),
            Self::VillagerTrade { inner } => inner.0.clone(),
            Self::Warden { inner } => inner.0.clone(),
            Self::WitherRose { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::AreaEffectCloud { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Arrow { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Attack { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Axolotl { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Beacon { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Command { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Conduit { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Conversion { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Death { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Dolphin { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Expiration { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Food { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Illusion { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Milk { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::PatrolCaptain { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Plugin { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::PotionDrink { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::PotionSplash { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SpiderSpawn { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Totem { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::TurtleHelmet { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Unknown { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::VillagerTrade { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Warden { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::WitherRose { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityPotionEffectEventCause<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityPotionEffectEventCause from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityPotionEffectEvent/Cause",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityPotionEffectEventCause object, got {}",
                name
            )
            .into())
        } else {
            let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
            let variant = env.translate_error(variant)?;
            let variant_str = env
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            match variant_str.as_str() {
                "AREA_EFFECT_CLOUD" => Ok(EntityPotionEffectEventCause::AreaEffectCloud {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "ARROW" => Ok(EntityPotionEffectEventCause::Arrow {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "ATTACK" => Ok(EntityPotionEffectEventCause::Attack {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "AXOLOTL" => Ok(EntityPotionEffectEventCause::Axolotl {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "BEACON" => Ok(EntityPotionEffectEventCause::Beacon {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "COMMAND" => Ok(EntityPotionEffectEventCause::Command {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "CONDUIT" => Ok(EntityPotionEffectEventCause::Conduit {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "CONVERSION" => Ok(EntityPotionEffectEventCause::Conversion {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "DEATH" => Ok(EntityPotionEffectEventCause::Death {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "DOLPHIN" => Ok(EntityPotionEffectEventCause::Dolphin {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "EXPIRATION" => Ok(EntityPotionEffectEventCause::Expiration {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "FOOD" => Ok(EntityPotionEffectEventCause::Food {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "ILLUSION" => Ok(EntityPotionEffectEventCause::Illusion {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "MILK" => Ok(EntityPotionEffectEventCause::Milk {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "PATROL_CAPTAIN" => Ok(EntityPotionEffectEventCause::PatrolCaptain {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "PLUGIN" => Ok(EntityPotionEffectEventCause::Plugin {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "POTION_DRINK" => Ok(EntityPotionEffectEventCause::PotionDrink {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "POTION_SPLASH" => Ok(EntityPotionEffectEventCause::PotionSplash {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "SPIDER_SPAWN" => Ok(EntityPotionEffectEventCause::SpiderSpawn {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "TOTEM" => Ok(EntityPotionEffectEventCause::Totem {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "TURTLE_HELMET" => Ok(EntityPotionEffectEventCause::TurtleHelmet {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "UNKNOWN" => Ok(EntityPotionEffectEventCause::Unknown {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "VILLAGER_TRADE" => Ok(EntityPotionEffectEventCause::VillagerTrade {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "WARDEN" => Ok(EntityPotionEffectEventCause::Warden {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                "WITHER_ROSE" => Ok(EntityPotionEffectEventCause::WitherRose {
                    inner: EntityPotionEffectEventCauseStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for EntityPotionEffectEventCauseStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityPotionEffectEventCauseStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityPotionEffectEventCauseStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/entity/EntityPotionEffectEvent/Cause",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntityPotionEffectEventCauseStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityPotionEffectEventCauseStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::entity::EntityPotionEffectEventCause<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/event/entity/EntityPotionEffectEvent/Cause;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityPotionEffectEvent/Cause");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::entity::EntityPotionEffectEventCause::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct EntityDropItemEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityDropItemEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityDropItemEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityDropItemEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityDropItemEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityDropItemEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityDropItemEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        entity: impl Into<crate::entity::Entity<'mc>>,
        drop: impl Into<crate::entity::Item<'mc>>,
    ) -> Result<crate::event::entity::EntityDropItemEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;Lorg/bukkit/entity/Item;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(entity.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(drop.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityDropItemEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityDropItemEvent::from_raw(&jni, res)
    }
    /// Gets the Item created by the entity
    pub fn item_drop(&self) -> Result<crate::entity::Item<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Item;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemDrop", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Item::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityDropItemEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getItemDrop', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Returns the Entity involved in this event
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityDropItemEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityDropItemEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityDropItemEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityDropItemEvent into crate::event::entity::EntityEvent")
    }
}
#[repr(C)]
pub struct SpawnerSpawnEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SpawnerSpawnEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SpawnerSpawnEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SpawnerSpawnEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/SpawnerSpawnEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SpawnerSpawnEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SpawnerSpawnEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        spawnee: impl Into<crate::entity::Entity<'mc>>,
        spawner: impl Into<crate::block::CreatureSpawner<'mc>>,
    ) -> Result<crate::event::entity::SpawnerSpawnEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;Lorg/bukkit/block/CreatureSpawner;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(spawnee.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(spawner.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/SpawnerSpawnEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::SpawnerSpawnEvent::from_raw(&jni, res)
    }

    pub fn spawner(
        &self,
    ) -> Result<crate::block::CreatureSpawner<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/CreatureSpawner;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSpawner", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::CreatureSpawner::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: org.bukkit.event.entity.EntitySpawnEvent ( ['getSpawner'])

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntitySpawnEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntitySpawnEvent = temp_clone.into();
        real.is_cancelled()
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntitySpawnEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntitySpawnEvent = temp_clone.into();
        real.set_cancelled(cancel)
    }
    /// Gets the location at which the entity is spawning.
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntitySpawnEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntitySpawnEvent = temp_clone.into();
        real.location()
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        crate::event::entity::EntitySpawnEvent::handler_list(jni)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntitySpawnEvent<'mc>> for SpawnerSpawnEvent<'mc> {
    fn into(self) -> crate::event::entity::EntitySpawnEvent<'mc> {
        crate::event::entity::EntitySpawnEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting SpawnerSpawnEvent into crate::event::entity::EntitySpawnEvent",
        )
    }
}
#[repr(C)]
pub struct EntityKnockbackByEntityEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityKnockbackByEntityEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityKnockbackByEntityEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityKnockbackByEntityEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityKnockbackByEntityEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityKnockbackByEntityEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityKnockbackByEntityEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        entity: impl Into<crate::entity::LivingEntity<'mc>>,
        source: impl Into<crate::entity::Entity<'mc>>,
        cause: impl Into<crate::event::entity::EntityKnockbackEventKnockbackCause<'mc>>,
        force: f64,
        raw_knockback: impl Into<crate::util::Vector<'mc>>,
        knockback: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<crate::event::entity::EntityKnockbackByEntityEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/entity/LivingEntity;Lorg/bukkit/entity/Entity;Lorg/bukkit/event/entity/EntityKnockbackEvent/KnockbackCause;DLorg/bukkit/util/Vector;Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(entity.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(source.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(cause.into().jni_object().clone())
        });
        let val_4 = jni::objects::JValueGen::Double(force);
        let val_5 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(raw_knockback.into().jni_object().clone())
        });
        let val_6 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(knockback.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityKnockbackByEntityEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
                jni::objects::JValueGen::from(val_5),
                jni::objects::JValueGen::from(val_6),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityKnockbackByEntityEvent::from_raw(&jni, res)
    }
    /// Get the entity that has caused knockback to the defender.
    pub fn source_entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSourceEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityKnockbackEvent ( ['getSourceEntity'])

    pub fn entity(&self) -> Result<crate::entity::LivingEntity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityKnockbackEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityKnockbackEvent = temp_clone.into();
        real.entity()
    }
    /// Gets the cause of the knockback.
    pub fn cause(
        &self,
    ) -> Result<
        crate::event::entity::EntityKnockbackEventKnockbackCause<'mc>,
        Box<dyn std::error::Error>,
    > {
        let temp_clone = crate::event::entity::EntityKnockbackEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityKnockbackEvent = temp_clone.into();
        real.cause()
    }
    /// Gets the raw force of the knockback.
    ///
    /// This value is based on factors such as the {@link Enchantment#KNOCKBACK}
    /// level of an attacker and the
    /// {@link Attribute#GENERIC_KNOCKBACK_RESISTANCE} of the entity.
    pub fn force(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityKnockbackEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityKnockbackEvent = temp_clone.into();
        real.force()
    }
    /// Gets the raw knockback force that will be applied to the entity.
    ///
    /// This value is read-only, changes made to it <b>will not</b> have any
    /// effect on the final knockback received.
    pub fn knockback(&self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityKnockbackEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityKnockbackEvent = temp_clone.into();
        real.knockback()
    }
    /// Gets the force that will be applied to the entity.
    ///
    /// In contrast to {@link EntityKnockbackEvent#getKnockback()} this value is
    /// affected by the entities current velocity and whether they are touching
    /// the ground.
    ///
    /// <b>Note:</b> this method returns a copy, changes must be applied with
    /// {@link #setFinalKnockback(Vector)}.
    pub fn final_knockback(&self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityKnockbackEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityKnockbackEvent = temp_clone.into();
        real.final_knockback()
    }
    /// Sets the force that will be applied to the entity.
    pub fn set_final_knockback(
        &self,
        knockback: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityKnockbackEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityKnockbackEvent = temp_clone.into();
        real.set_final_knockback(knockback)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityKnockbackEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityKnockbackEvent = temp_clone.into();
        real.is_cancelled()
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityKnockbackEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityKnockbackEvent = temp_clone.into();
        real.set_cancelled(cancel)
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        crate::event::entity::EntityKnockbackEvent::handler_list(jni)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntityKnockbackEvent<'mc>>
    for EntityKnockbackByEntityEvent<'mc>
{
    fn into(self) -> crate::event::entity::EntityKnockbackEvent<'mc> {
        crate::event::entity::EntityKnockbackEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting EntityKnockbackByEntityEvent into crate::event::entity::EntityKnockbackEvent")
    }
}
#[repr(C)]
pub struct PigZapEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PigZapEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PigZapEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PigZapEvent from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/PigZapEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PigZapEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PigZapEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        pig: impl Into<crate::entity::Pig<'mc>>,
        bolt: impl Into<crate::entity::LightningStrike<'mc>>,
        pigzombie: impl Into<crate::entity::PigZombie<'mc>>,
    ) -> Result<crate::event::entity::PigZapEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Pig;Lorg/bukkit/entity/LightningStrike;Lorg/bukkit/entity/PigZombie;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(pig.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(bolt.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(pigzombie.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/PigZapEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::PigZapEvent::from_raw(&jni, res)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn entity(&self) -> Result<crate::entity::Pig<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Pig;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Pig::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the bolt which is striking the pig.
    pub fn lightning(
        &self,
    ) -> Result<crate::entity::LightningStrike<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/LightningStrike;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLightning", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::LightningStrike::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]
    /// Gets the zombie pig that will replace the pig, provided the event is not cancelled first.
    pub fn pig_zombie(&self) -> Result<crate::entity::PigZombie<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/PigZombie;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPigZombie", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::PigZombie::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/PigZapEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityTransformEvent ( ['isCancelled', 'setCancelled', 'getEntity', 'getLightning', 'getPigZombie', 'getHandlers', 'getHandlerList'])
    /// Gets the entity that the original entity was transformed to.
    /// This returns the first entity in the transformed entity list.
    pub fn transformed_entity(
        &self,
    ) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityTransformEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityTransformEvent = temp_clone.into();
        real.transformed_entity()
    }
    /// Gets the entities that the original entity was transformed to.
    pub fn transformed_entities(
        &self,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTransformedEntities",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::Entity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    /// Gets the reason for the conversion that has occurred.
    pub fn transform_reason(
        &self,
    ) -> Result<
        crate::event::entity::EntityTransformEventTransformReason<'mc>,
        Box<dyn std::error::Error>,
    > {
        let temp_clone = crate::event::entity::EntityTransformEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityTransformEvent = temp_clone.into();
        real.transform_reason()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PigZapEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PigZapEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityTransformEvent<'mc>> for PigZapEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityTransformEvent<'mc> {
        crate::event::entity::EntityTransformEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PigZapEvent into crate::event::entity::EntityTransformEvent")
    }
}
#[repr(C)]
pub struct ItemDespawnEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ItemDespawnEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ItemDespawnEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ItemDespawnEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/ItemDespawnEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ItemDespawnEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ItemDespawnEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        despawnee: impl Into<crate::entity::Item<'mc>>,
        loc: impl Into<crate::Location<'mc>>,
    ) -> Result<crate::event::entity::ItemDespawnEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Item;Lorg/bukkit/Location;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(despawnee.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(loc.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/ItemDespawnEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::ItemDespawnEvent::from_raw(&jni, res)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn entity(&self) -> Result<crate::entity::Item<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Item;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Item::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the location at which the item is despawning.
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLocation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/ItemDespawnEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['isCancelled', 'setCancelled', 'getEntity', 'getLocation', 'getHandlers', 'getHandlerList'])
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for ItemDespawnEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ItemDespawnEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for ItemDespawnEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ItemDespawnEvent into crate::event::entity::EntityEvent")
    }
}
#[repr(C)]
pub struct ProjectileHitEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ProjectileHitEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ProjectileHitEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ProjectileHitEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/ProjectileHitEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ProjectileHitEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ProjectileHitEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        projectile: impl Into<crate::entity::Projectile<'mc>>,
        hit_entity: std::option::Option<impl Into<crate::entity::Entity<'mc>>>,
        hit_block: std::option::Option<impl Into<crate::block::Block<'mc>>>,
        hit_face: std::option::Option<impl Into<crate::block::BlockFace<'mc>>>,
    ) -> Result<crate::event::entity::ProjectileHitEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Projectile;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(projectile.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = hit_entity {
            sig += "Lorg/bukkit/entity/Entity;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        if let Some(a) = hit_block {
            sig += "Lorg/bukkit/block/Block;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        if let Some(a) = hit_face {
            sig += "Lorg/bukkit/block/BlockFace;";
            let val_4 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_4);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/ProjectileHitEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::ProjectileHitEvent::from_raw(&jni, res)
    }

    pub fn entity(&self) -> Result<crate::entity::Projectile<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Projectile;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Projectile::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the block that was hit, if it was a block that was hit.
    pub fn hit_block(
        &self,
    ) -> Result<Option<crate::block::Block<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/Block;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHitBlock", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::block::Block::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Gets the block face that was hit, if it was a block that was hit and the
    /// face was provided in the event.
    pub fn hit_block_face(
        &self,
    ) -> Result<Option<crate::block::BlockFace<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHitBlockFace", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Gets the entity that was hit, if it was an entity that was hit.
    pub fn hit_entity(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHitEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Entity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Whether to cancel the action that occurs when the projectile hits.
    /// In the case of an entity, it will not collide (unless it's a firework,
    /// then use {@link FireworkExplodeEvent}).
    ///
    /// In the case of a block, some blocks (eg target block, bell) will not
    /// perform the action associated.
    ///
    /// This does NOT prevent block collisions, and explosions will still occur
    /// unless their respective events are cancelled.
    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/ProjectileHitEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getEntity', 'getHitBlock', 'getHitBlockFace', 'getHitEntity', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for ProjectileHitEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ProjectileHitEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for ProjectileHitEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ProjectileHitEvent into crate::event::entity::EntityEvent")
    }
}
#[repr(C)]
pub struct EntityExplodeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityExplodeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityExplodeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityExplodeEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityExplodeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityExplodeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityExplodeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        what: impl Into<crate::entity::Entity<'mc>>,
        location: impl Into<crate::Location<'mc>>,
        blocks: Vec<jni::objects::JObject<'mc>>,
        val_yield: f32,
        result: impl Into<crate::ExplosionResult<'mc>>,
    ) -> Result<crate::event::entity::EntityExplodeEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;Lorg/bukkit/Location;Ljava/util/List;FLorg/bukkit/ExplosionResult;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(what.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(location.into().jni_object().clone())
        });
        let raw_val_3 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
        for v in blocks {
            let map_val_0 = jni::objects::JValueGen::Object(v);
            jni.call_method(
                &raw_val_3,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_3 = jni::objects::JValueGen::Object(raw_val_3);
        let val_4 = jni::objects::JValueGen::Float(val_yield);
        let val_5 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(result.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityExplodeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
                jni::objects::JValueGen::from(val_5),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityExplodeEvent::from_raw(&jni, res)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns the result of the explosion if it is not cancelled.
    pub fn explosion_result(
        &self,
    ) -> Result<crate::ExplosionResult<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/ExplosionResult;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getExplosionResult",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::ExplosionResult::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns the list of blocks that would have been removed or were removed
    /// from the explosion event.
    pub fn block_list(&self) -> Result<Vec<crate::block::Block<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "blockList", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::block::Block::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    /// Returns the location where the explosion happened.
    ///
    /// It is not possible to get this value from the Entity as the Entity no
    /// longer exists in the world.
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLocation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns the percentage of blocks to drop from this explosion
    pub fn get_yield(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getYield", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// Sets the percentage of blocks to drop from this explosion
    pub fn set_yield(&self, val_yield: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(val_yield);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setYield",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityExplodeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['isCancelled', 'setCancelled', 'getExplosionResult', 'blockList', 'getLocation', 'getYield', 'setYield', 'getHandlers', 'getHandlerList'])
    /// Returns the Entity involved in this event
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityExplodeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityExplodeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityExplodeEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityExplodeEvent into crate::event::entity::EntityEvent")
    }
}
#[repr(C)]
pub struct ItemMergeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ItemMergeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ItemMergeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ItemMergeEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/ItemMergeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ItemMergeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ItemMergeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        item: impl Into<crate::entity::Item<'mc>>,
        target: impl Into<crate::entity::Item<'mc>>,
    ) -> Result<crate::event::entity::ItemMergeEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Item;Lorg/bukkit/entity/Item;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(target.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/ItemMergeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::ItemMergeEvent::from_raw(&jni, res)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancelled: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancelled.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn entity(&self) -> Result<crate::entity::Item<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Item;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Item::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the Item entity the main Item is being merged into.
    pub fn target(&self) -> Result<crate::entity::Item<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Item;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTarget", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Item::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/ItemMergeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['isCancelled', 'setCancelled', 'getEntity', 'getTarget', 'getHandlers', 'getHandlerList'])
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for ItemMergeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ItemMergeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for ItemMergeEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ItemMergeEvent into crate::event::entity::EntityEvent")
    }
}
#[repr(C)]
pub struct EntityPortalEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityPortalEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityPortalEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityPortalEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/EntityPortalEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityPortalEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityPortalEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        entity: impl Into<crate::entity::Entity<'mc>>,
        from: impl Into<crate::Location<'mc>>,
        to: impl Into<crate::Location<'mc>>,
        search_radius: std::option::Option<i32>,
        can_create_portal: std::option::Option<bool>,
        creation_radius: std::option::Option<i32>,
    ) -> Result<crate::event::entity::EntityPortalEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(entity.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/Location;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(from.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "Lorg/bukkit/Location;";
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(to.into().jni_object().clone())
        });
        args.push(val_3);
        if let Some(a) = search_radius {
            sig += "I";
            let val_4 = jni::objects::JValueGen::Int(a);
            args.push(val_4);
        }
        if let Some(a) = can_create_portal {
            sig += "Z";
            let val_5 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_5);
        }
        if let Some(a) = creation_radius {
            sig += "I";
            let val_6 = jni::objects::JValueGen::Int(a);
            args.push(val_6);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/EntityPortalEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityPortalEvent::from_raw(&jni, res)
    }
    /// Set the Block radius to search in for available portals.
    pub fn set_search_radius(&self, search_radius: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(search_radius);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSearchRadius",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the search radius value for finding an available portal.
    pub fn search_radius(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSearchRadius", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Returns whether the server will attempt to create a destination portal or
    /// not.
    pub fn can_create_portal(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCanCreatePortal",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether the server should attempt to create a destination portal or
    /// not.
    pub fn set_can_create_portal(
        &self,
        can_create_portal: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(can_create_portal.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCanCreatePortal",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets the maximum radius the world is searched for a free space from the
    /// given location.
    /// If enough free space is found then the portal will be created there, if
    /// not it will force create with air-space at the target location.
    /// Does not apply to end portal target platforms which will always appear at
    /// the target location.
    pub fn set_creation_radius(
        &self,
        creation_radius: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(creation_radius);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCreationRadius",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the maximum radius the world is searched for a free space from the
    /// given location.
    /// If enough free space is found then the portal will be created there, if
    /// not it will force create with air-space at the target location.
    /// Does not apply to end portal target platforms which will always appear at
    /// the target location.
    pub fn creation_radius(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCreationRadius",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityPortalEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityTeleportEvent ( ['setSearchRadius', 'getSearchRadius', 'getCanCreatePortal', 'setCanCreatePortal', 'setCreationRadius', 'getCreationRadius', 'getHandlers', 'getHandlerList'])

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityTeleportEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityTeleportEvent = temp_clone.into();
        real.is_cancelled()
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityTeleportEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityTeleportEvent = temp_clone.into();
        real.set_cancelled(cancel)
    }
    /// Gets the location that this entity moved from
    pub fn from(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityTeleportEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityTeleportEvent = temp_clone.into();
        real.from()
    }
    /// Sets the location that this entity moved from
    pub fn set_from(
        &self,
        from: impl Into<crate::Location<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityTeleportEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityTeleportEvent = temp_clone.into();
        real.set_from(from)
    }
    /// Gets the location that this entity moved to
    pub fn to(&self) -> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityTeleportEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityTeleportEvent = temp_clone.into();
        real.to()
    }
    /// Sets the location that this entity moved to
    pub fn set_to(
        &self,
        to: impl Into<crate::Location<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityTeleportEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityTeleportEvent = temp_clone.into();
        real.set_to(to)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::entity::EntityTeleportEvent<'mc>> for EntityPortalEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityTeleportEvent<'mc> {
        crate::event::entity::EntityTeleportEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting EntityPortalEvent into crate::event::entity::EntityTeleportEvent",
        )
    }
}
#[repr(C)]
pub struct VillagerReplenishTradeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for VillagerReplenishTradeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for VillagerReplenishTradeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate VillagerReplenishTradeEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/VillagerReplenishTradeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a VillagerReplenishTradeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> VillagerReplenishTradeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        what: impl Into<crate::entity::AbstractVillager<'mc>>,
        recipe: impl Into<crate::inventory::MerchantRecipe<'mc>>,
    ) -> Result<crate::event::entity::VillagerReplenishTradeEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from(
            "(Lorg/bukkit/entity/AbstractVillager;Lorg/bukkit/inventory/MerchantRecipe;)V",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(what.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(recipe.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/VillagerReplenishTradeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::VillagerReplenishTradeEvent::from_raw(&jni, res)
    }
    /// Get the recipe to replenish.
    pub fn recipe(
        &self,
    ) -> Result<crate::inventory::MerchantRecipe<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/MerchantRecipe;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRecipe", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::MerchantRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Set the recipe to replenish.
    pub fn set_recipe(
        &self,
        recipe: impl Into<crate::inventory::MerchantRecipe<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/MerchantRecipe;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(recipe.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRecipe",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    /// Get the bonus uses added.
    pub fn bonus(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBonus", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    #[deprecated]
    /// Set the bonus uses added.
    pub fn set_bonus(&self, bonus: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(bonus);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBonus",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn entity(
        &self,
    ) -> Result<crate::entity::AbstractVillager<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/AbstractVillager;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::AbstractVillager::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/VillagerReplenishTradeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getRecipe', 'setRecipe', 'getBonus', 'setBonus', 'isCancelled', 'setCancelled', 'getEntity', 'getHandlers', 'getHandlerList'])
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for VillagerReplenishTradeEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting VillagerReplenishTradeEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for VillagerReplenishTradeEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting VillagerReplenishTradeEvent into crate::event::entity::EntityEvent",
        )
    }
}
#[repr(C)]
pub struct EntityEnterBlockEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityEnterBlockEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityEnterBlockEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EntityEnterBlockEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/entity/EntityEnterBlockEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityEnterBlockEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityEnterBlockEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        entity: impl Into<crate::entity::Entity<'mc>>,
        block: impl Into<crate::block::Block<'mc>>,
    ) -> Result<crate::event::entity::EntityEnterBlockEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;Lorg/bukkit/block/Block;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(entity.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/entity/EntityEnterBlockEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::EntityEnterBlockEvent::from_raw(&jni, res)
    }
    /// Get the block the entity will enter.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/Block;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBlock", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Block::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/EntityEnterBlockEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['getBlock', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Returns the Entity involved in this event
    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity()
    }
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EntityEnterBlockEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityEnterBlockEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for EntityEnterBlockEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityEnterBlockEvent into crate::event::entity::EntityEvent")
    }
}
#[repr(C)]
pub struct SheepDyeWoolEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SheepDyeWoolEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SheepDyeWoolEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SheepDyeWoolEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/entity/SheepDyeWoolEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SheepDyeWoolEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SheepDyeWoolEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        sheep: impl Into<crate::entity::Sheep<'mc>>,
        color: impl Into<crate::DyeColor<'mc>>,
        player: std::option::Option<impl Into<crate::entity::Player<'mc>>>,
    ) -> Result<crate::event::entity::SheepDyeWoolEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Sheep;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(sheep.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/DyeColor;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(color.into().jni_object().clone())
        });
        args.push(val_2);
        if let Some(a) = player {
            sig += "Lorg/bukkit/entity/Player;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/entity/SheepDyeWoolEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::entity::SheepDyeWoolEvent::from_raw(&jni, res)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn entity(&self) -> Result<crate::entity::Sheep<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Sheep;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Sheep::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns the player dyeing the sheep, if available.
    pub fn player(&self) -> Result<Option<crate::entity::Player<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Player;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPlayer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Player::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Gets the DyeColor the sheep is being dyed
    pub fn color(&self) -> Result<crate::DyeColor<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/DyeColor;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::DyeColor::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the DyeColor the sheep is being dyed
    pub fn set_color(
        &self,
        color: impl Into<crate::DyeColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/DyeColor;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(color.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/entity/SheepDyeWoolEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.entity.EntityEvent ( ['isCancelled', 'setCancelled', 'getEntity', 'getPlayer', 'getColor', 'setColor', 'getHandlers', 'getHandlerList'])
    /// Gets the EntityType of the Entity involved in this event.
    pub fn entity_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::entity::EntityEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::entity::EntityEvent = temp_clone.into();
        real.entity_type()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for SheepDyeWoolEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SheepDyeWoolEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::entity::EntityEvent<'mc>> for SheepDyeWoolEvent<'mc> {
    fn into(self) -> crate::event::entity::EntityEvent<'mc> {
        crate::event::entity::EntityEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SheepDyeWoolEvent into crate::event::entity::EntityEvent")
    }
}
